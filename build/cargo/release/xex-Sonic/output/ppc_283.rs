pub fn sub_8318B4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318B4E8 size=252
    let mut pc: u32 = 0x8318B4E8;
    'dispatch: loop {
        match pc {
            0x8318B4E8 => {
    //   block [0x8318B4E8..0x8318B5E4)
	// 8318B4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B4F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B4F4: C1A3003C  lfs f13, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8318B4F8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8318B4FC: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8318B500: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8318B504: 4BFFFDD5  bl 0x8318b2d8
	ctx.lr = 0x8318B508;
	sub_8318B2D8(ctx, base);
	// 8318B508: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318B50C: 409A0038  bne cr6, 0x8318b544
	if !ctx.cr[6].eq {
	pc = 0x8318B544; continue 'dispatch;
	}
	// 8318B510: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 8318B514: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B518: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318B51C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318B520: 55088BFE  srwi r8, r8, 0xf
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318B524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318B528: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8318B52C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8318B530: 409AFFE4  bne cr6, 0x8318b514
	if !ctx.cr[6].eq {
	pc = 0x8318B514; continue 'dispatch;
	}
	// 8318B534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318B538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B540: 4E800020  blr
	return;
	// 8318B544: FD606828  fsub f11, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 8318B548: 3D008205  lis r8, -0x7dfb
	ctx.r[8].s64 = -2113601536;
	// 8318B54C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8318B550: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 8318B554: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8318B558: C988E3A0  lfd f12, -0x1c60(r8)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-7264 as u32) ) };
	// 8318B55C: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8318B560: FD8C5824  fdiv f12, f12, f11
	ctx.f[12].f64 = ctx.f[12].f64 / ctx.f[11].f64;
	// 8318B564: FD8C0032  fmul f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 8318B568: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 8318B56C: C9A8C4E8  lfd f13, -0x3b18(r8)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-15128 as u32) ) };
	// 8318B570: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 8318B574: FD8C0372  fmul f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 8318B578: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 8318B57C: C968BA50  lfd f11, -0x45b0(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(-17840 as u32) ) };
	// 8318B580: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B584: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8318B588: 409A0008  bne cr6, 0x8318b590
	if !ctx.cr[6].eq {
	pc = 0x8318B590; continue 'dispatch;
	}
	// 8318B58C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8318B590: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B594: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318B598: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B59C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318B5A0: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8318B5A4: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8318B5A8: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 8318B5AC: FD4A0032  fmul f10, f10, f0
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 8318B5B0: FD4A02F2  fmul f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[11].f64;
	// 8318B5B4: FD4D5024  fdiv f10, f13, f10
	ctx.f[10].f64 = ctx.f[13].f64 / ctx.f[10].f64;
	// 8318B5B8: FD4C5028  fsub f10, f12, f10
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[10].f64;
	// 8318B5BC: FD40565E  fctidz f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 8318B5C0: D9410058  stfd f10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[10].u64 ) };
	// 8318B5C4: A101005E  lhz r8, 0x5e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 8318B5C8: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8318B5CC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8318B5D0: 409AFFB0  bne cr6, 0x8318b580
	if !ctx.cr[6].eq {
	pc = 0x8318B580; continue 'dispatch;
	}
	// 8318B5D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318B5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318B5E8 size=240
    let mut pc: u32 = 0x8318B5E8;
    'dispatch: loop {
        match pc {
            0x8318B5E8 => {
    //   block [0x8318B5E8..0x8318B6D8)
	// 8318B5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B5F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B5F4: C1A3003C  lfs f13, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8318B5F8: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8318B5FC: 4BFFFCDD  bl 0x8318b2d8
	ctx.lr = 0x8318B600;
	sub_8318B2D8(ctx, base);
	// 8318B600: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318B604: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 8318B608: 409A0038  bne cr6, 0x8318b640
	if !ctx.cr[6].eq {
	pc = 0x8318B640; continue 'dispatch;
	}
	// 8318B60C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8318B610: 7D252050  subf r9, r5, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 8318B614: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8318B618: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318B61C: 5508082E  rlwinm r8, r8, 1, 0, 0x17
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x7FFFFFFFu64;
	// 8318B620: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318B624: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8318B628: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B62C: 409AFFE8  bne cr6, 0x8318b614
	if !ctx.cr[6].eq {
	pc = 0x8318B614; continue 'dispatch;
	}
	// 8318B630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B63C: 4E800020  blr
	return;
	// 8318B640: FD606828  fsub f11, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 8318B644: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 8318B648: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318B64C: 7D042850  subf r8, r4, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 8318B650: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8318B654: C989E3A0  lfd f12, -0x1c60(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-7264 as u32) ) };
	// 8318B658: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8318B65C: FD8C5824  fdiv f12, f12, f11
	ctx.f[12].f64 = ctx.f[12].f64 / ctx.f[11].f64;
	// 8318B660: FD8C0032  fmul f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 8318B664: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 8318B668: C9A9BA58  lfd f13, -0x45a8(r9)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-17832 as u32) ) };
	// 8318B66C: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8318B670: FD8C0372  fmul f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 8318B674: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 8318B678: C969BA50  lfd f11, -0x45b0(r9)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-17840 as u32) ) };
	// 8318B67C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B680: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8318B684: 409A0008  bne cr6, 0x8318b68c
	if !ctx.cr[6].eq {
	pc = 0x8318B68C; continue 'dispatch;
	}
	// 8318B688: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8318B68C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B690: 7CC85A14  add r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8318B694: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318B698: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B69C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318B6A0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8318B6A4: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8318B6A8: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 8318B6AC: FD4A0032  fmul f10, f10, f0
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 8318B6B0: FD4A02F2  fmul f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[11].f64;
	// 8318B6B4: FD4D5024  fdiv f10, f13, f10
	ctx.f[10].f64 = ctx.f[13].f64 / ctx.f[10].f64;
	// 8318B6B8: FD4C5028  fsub f10, f12, f10
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[10].f64;
	// 8318B6BC: FD40565E  fctidz f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 8318B6C0: 7D4037AE  stfiwx f10, 0, r6
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32, tmp.u32) };
	// 8318B6C4: 409AFFB8  bne cr6, 0x8318b67c
	if !ctx.cr[6].eq {
	pc = 0x8318B67C; continue 'dispatch;
	}
	// 8318B6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B6D8 size=4
    let mut pc: u32 = 0x8318B6D8;
    'dispatch: loop {
        match pc {
            0x8318B6D8 => {
    //   block [0x8318B6D8..0x8318B6DC)
	// 8318B6D8: 4BFFFC10  b 0x8318b2e8
	sub_8318B2E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318B6E0 size=160
    let mut pc: u32 = 0x8318B6E0;
    'dispatch: loop {
        match pc {
            0x8318B6E0 => {
    //   block [0x8318B6E0..0x8318B780)
	// 8318B6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B6E4: 4801CA81  bl 0x831a8164
	ctx.lr = 0x8318B6E8;
	sub_831A8130(ctx, base);
	// 8318B6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B6EC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8318B6F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318B6F4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8318B6F8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8318B6FC: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 8318B700: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318B704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318B708: 3BBE0400  addi r29, r30, 0x400
	ctx.r[29].s64 = ctx.r[30].s64 + 1024;
	// 8318B70C: 4801CAD5  bl 0x831a81e0
	ctx.lr = 0x8318B710;
	sub_831A81E0(ctx, base);
	// 8318B710: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8318B714: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8318B718: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8318B71C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B720: 4BFFFD79  bl 0x8318b498
	ctx.lr = 0x8318B724;
	sub_8318B498(ctx, base);
	// 8318B724: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318B728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318B72C: 409A0034  bne cr6, 0x8318b760
	if !ctx.cr[6].eq {
	pc = 0x8318B760; continue 'dispatch;
	}
	// 8318B730: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318B734: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318B738: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318B73C: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 8318B740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B744: 409A0010  bne cr6, 0x8318b754
	if !ctx.cr[6].eq {
	pc = 0x8318B754; continue 'dispatch;
	}
	// 8318B748: 4BFFFDA1  bl 0x8318b4e8
	ctx.lr = 0x8318B74C;
	sub_8318B4E8(ctx, base);
	// 8318B74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318B750: 4801CA64  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318B754: 4BFFFE95  bl 0x8318b5e8
	ctx.lr = 0x8318B758;
	sub_8318B5E8(ctx, base);
	// 8318B758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318B75C: 4801CA58  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318B760: C05F0040  lfs f2, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8318B764: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8318B768: C03F003C  lfs f1, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8318B76C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318B770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318B774: 4E800421  bctrl
	ctx.lr = 0x8318B778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318B778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318B77C: 4801CA38  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B780 size=84
    let mut pc: u32 = 0x8318B780;
    'dispatch: loop {
        match pc {
            0x8318B780 => {
    //   block [0x8318B780..0x8318B7D4)
	// 8318B780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318B78C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318B790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B794: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318B798: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318B79C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8318B7A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318B7A4: 4BFFFC05  bl 0x8318b3a8
	ctx.lr = 0x8318B7A8;
	sub_8318B3A8(ctx, base);
	// 8318B7A8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8318B7AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B7B0: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318B7B4: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318B7B8: 4BFFFF29  bl 0x8318b6e0
	ctx.lr = 0x8318B7BC;
	sub_8318B6E0(ctx, base);
	// 8318B7BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318B7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B7C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318B7CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B7D8 size=32
    let mut pc: u32 = 0x8318B7D8;
    'dispatch: loop {
        match pc {
            0x8318B7D8 => {
    //   block [0x8318B7D8..0x8318B7F8)
	// 8318B7D8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8318B7DC: 409A0024  bne cr6, 0x8318b800
	if !ctx.cr[6].eq {
		sub_8318B800(ctx, base);
		return;
	}
	// 8318B7E0: 7C6B2278  xor r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 ^ ctx.r[4].u64;
	// 8318B7E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318B7E8: 41980010  blt cr6, 0x8318b7f8
	if ctx.cr[6].lt {
		sub_8318B7F8(ctx, base);
		return;
	}
	// 8318B7EC: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 8318B7F0: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8318B7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B7F8 size=8
    let mut pc: u32 = 0x8318B7F8;
    'dispatch: loop {
        match pc {
            0x8318B7F8 => {
    //   block [0x8318B7F8..0x8318B800)
	// 8318B7F8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8318B7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B800 size=28
    let mut pc: u32 = 0x8318B800;
    'dispatch: loop {
        match pc {
            0x8318B800 => {
    //   block [0x8318B800..0x8318B81C)
	// 8318B800: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8318B804: 7C8A07B4  extsw r10, r4
	ctx.r[10].s64 = ctx.r[4].s32 as i64;
	// 8318B808: 7CA907B4  extsw r9, r5
	ctx.r[9].s64 = ctx.r[5].s32 as i64;
	// 8318B80C: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8318B810: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8318B814: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 8318B818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B820 size=116
    let mut pc: u32 = 0x8318B820;
    'dispatch: loop {
        match pc {
            0x8318B820 => {
    //   block [0x8318B820..0x8318B894)
	// 8318B820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318B82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B830: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318B834: 3BEBD7AC  addi r31, r11, -0x2854
	ctx.r[31].s64 = ctx.r[11].s64 + -10324;
	// 8318B838: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 8318B83C: 395F0024  addi r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 + 36;
	// 8318B840: 393F0024  addi r9, r31, 0x24
	ctx.r[9].s64 = ctx.r[31].s64 + 36;
	// 8318B844: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B848: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318B84C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318B850: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B854: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318B858: 409A0028  bne cr6, 0x8318b880
	if !ctx.cr[6].eq {
	pc = 0x8318B880; continue 'dispatch;
	}
	// 8318B85C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8318B860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B864: 4BFA362D  bl 0x8312ee90
	ctx.lr = 0x8318B868;
	sub_8312EE90(ctx, base);
	// 8318B868: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 8318B86C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318B870: 409A0010  bne cr6, 0x8318b880
	if !ctx.cr[6].eq {
	pc = 0x8318B880; continue 'dispatch;
	}
	// 8318B874: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318B878: 60840F61  ori r4, r4, 0xf61
	ctx.r[4].u64 = ctx.r[4].u64 | 3937;
	// 8318B87C: 4BFFBC7D  bl 0x831874f8
	ctx.lr = 0x8318B880;
	sub_831874F8(ctx, base);
	// 8318B880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B88C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B898 size=104
    let mut pc: u32 = 0x8318B898;
    'dispatch: loop {
        match pc {
            0x8318B898 => {
    //   block [0x8318B898..0x8318B900)
	// 8318B898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B8A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318B8A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B8A8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318B8AC: 3BEBD7CC  addi r31, r11, -0x2834
	ctx.r[31].s64 = ctx.r[11].s64 + -10292;
	// 8318B8B0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8318B8B4: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8318B8B8: 393F0004  addi r9, r31, 4
	ctx.r[9].s64 = ctx.r[31].s64 + 4;
	// 8318B8BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B8C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318B8C4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318B8C8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B8CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318B8D0: 409A001C  bne cr6, 0x8318b8ec
	if !ctx.cr[6].eq {
	pc = 0x8318B8EC; continue 'dispatch;
	}
	// 8318B8D4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B8D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318B8DC: 419A0010  beq cr6, 0x8318b8ec
	if ctx.cr[6].eq {
	pc = 0x8318B8EC; continue 'dispatch;
	}
	// 8318B8E0: 4BFA3691  bl 0x8312ef70
	ctx.lr = 0x8318B8E4;
	sub_8312EF70(ctx, base);
	// 8318B8E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318B8E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318B8EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B900 size=64
    let mut pc: u32 = 0x8318B900;
    'dispatch: loop {
        match pc {
            0x8318B900 => {
    //   block [0x8318B900..0x8318B940)
	// 8318B900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B90C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318B910: 806BD7CC  lwz r3, -0x2834(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10292 as u32) ) } as u64;
	// 8318B914: 4BFA36ED  bl 0x8312f000
	ctx.lr = 0x8318B918;
	sub_8312F000(ctx, base);
	// 8318B918: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318B91C: 40980014  bge cr6, 0x8318b930
	if !ctx.cr[6].lt {
	pc = 0x8318B930; continue 'dispatch;
	}
	// 8318B920: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318B924: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318B928: 60840F62  ori r4, r4, 0xf62
	ctx.r[4].u64 = ctx.r[4].u64 | 3938;
	// 8318B92C: 4BFFBBCD  bl 0x831874f8
	ctx.lr = 0x8318B930;
	sub_831874F8(ctx, base);
	// 8318B930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B940 size=64
    let mut pc: u32 = 0x8318B940;
    'dispatch: loop {
        match pc {
            0x8318B940 => {
    //   block [0x8318B940..0x8318B980)
	// 8318B940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B94C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318B950: 806BD7CC  lwz r3, -0x2834(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10292 as u32) ) } as u64;
	// 8318B954: 4BFA3745  bl 0x8312f098
	ctx.lr = 0x8318B958;
	sub_8312F098(ctx, base);
	// 8318B958: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318B95C: 40980014  bge cr6, 0x8318b970
	if !ctx.cr[6].lt {
	pc = 0x8318B970; continue 'dispatch;
	}
	// 8318B960: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318B964: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318B968: 60840F63  ori r4, r4, 0xf63
	ctx.r[4].u64 = ctx.r[4].u64 | 3939;
	// 8318B96C: 4BFFBB8D  bl 0x831874f8
	ctx.lr = 0x8318B970;
	sub_831874F8(ctx, base);
	// 8318B970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B980 size=108
    let mut pc: u32 = 0x8318B980;
    'dispatch: loop {
        match pc {
            0x8318B980 => {
    //   block [0x8318B980..0x8318B9EC)
	// 8318B980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318B98C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318B994: 4BFFB94D  bl 0x831872e0
	ctx.lr = 0x8318B998;
	sub_831872E0(ctx, base);
	// 8318B998: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318B99C: 419A0028  beq cr6, 0x8318b9c4
	if ctx.cr[6].eq {
	pc = 0x8318B9C4; continue 'dispatch;
	}
	// 8318B9A0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318B9A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318B9A8: 60840161  ori r4, r4, 0x161
	ctx.r[4].u64 = ctx.r[4].u64 | 353;
	// 8318B9AC: 4BFFBB4D  bl 0x831874f8
	ctx.lr = 0x8318B9B0;
	sub_831874F8(ctx, base);
	// 8318B9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B9BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B9C0: 4E800020  blr
	return;
	// 8318B9C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318B9C8: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 8318B9CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B9D0: 4BFEDA89  bl 0x83179458
	ctx.lr = 0x8318B9D4;
	sub_83179458(ctx, base);
	// 8318B9D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318B9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B9E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B9F0 size=48
    let mut pc: u32 = 0x8318B9F0;
    'dispatch: loop {
        match pc {
            0x8318B9F0 => {
    //   block [0x8318B9F0..0x8318BA20)
	// 8318B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B9FC: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 8318BA00: 4BFEDA11  bl 0x83179410
	ctx.lr = 0x8318BA04;
	sub_83179410(ctx, base);
	// 8318BA04: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8318BA08: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318BA0C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8318BA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318BA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BA20 size=84
    let mut pc: u32 = 0x8318BA20;
    'dispatch: loop {
        match pc {
            0x8318BA20 => {
    //   block [0x8318BA20..0x8318BA74)
	// 8318BA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318BA28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318BA2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BA30: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 8318BA34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318BA38: 4BFED9D9  bl 0x83179410
	ctx.lr = 0x8318BA3C;
	sub_83179410(ctx, base);
	// 8318BA3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318BA40: 409A001C  bne cr6, 0x8318ba5c
	if !ctx.cr[6].eq {
	pc = 0x8318BA5C; continue 'dispatch;
	}
	// 8318BA44: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 8318BA48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318BA4C: 4BFED9C5  bl 0x83179410
	ctx.lr = 0x8318BA50;
	sub_83179410(ctx, base);
	// 8318BA50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318BA54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318BA58: 419A0008  beq cr6, 0x8318ba60
	if ctx.cr[6].eq {
	pc = 0x8318BA60; continue 'dispatch;
	}
	// 8318BA5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318BA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BA78 size=84
    let mut pc: u32 = 0x8318BA78;
    'dispatch: loop {
        match pc {
            0x8318BA78 => {
    //   block [0x8318BA78..0x8318BACC)
	// 8318BA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318BA80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318BA84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BA88: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 8318BA8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318BA90: 4BFED981  bl 0x83179410
	ctx.lr = 0x8318BA94;
	sub_83179410(ctx, base);
	// 8318BA94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318BA98: 409A001C  bne cr6, 0x8318bab4
	if !ctx.cr[6].eq {
	pc = 0x8318BAB4; continue 'dispatch;
	}
	// 8318BA9C: 38800039  li r4, 0x39
	ctx.r[4].s64 = 57;
	// 8318BAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318BAA4: 4BFED96D  bl 0x83179410
	ctx.lr = 0x8318BAA8;
	sub_83179410(ctx, base);
	// 8318BAA8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318BAAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318BAB0: 419A0008  beq cr6, 0x8318bab8
	if ctx.cr[6].eq {
	pc = 0x8318BAB8; continue 'dispatch;
	}
	// 8318BAB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318BAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318BABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318BAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318BAD0 size=124
    let mut pc: u32 = 0x8318BAD0;
    'dispatch: loop {
        match pc {
            0x8318BAD0 => {
    //   block [0x8318BAD0..0x8318BB4C)
	// 8318BAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318BAD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318BADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318BAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BAE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318BAE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BAEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318BAF0: 3BEB0D88  addi r31, r11, 0xd88
	ctx.r[31].s64 = ctx.r[11].s64 + 3464;
	// 8318BAF4: 4BFFFE0D  bl 0x8318b900
	ctx.lr = 0x8318BAF8;
	sub_8318B900(ctx, base);
	// 8318BAF8: 817F0168  lwz r11, 0x168(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 8318BAFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BB00: 815F0164  lwz r10, 0x164(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8318BB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318BB08: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8318BB0C: 7D692E70  srawi r9, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 8318BB10: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8318BB14: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318BB18: 915F0164  stw r10, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 8318BB1C: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8318BB20: 3929005B  addi r9, r9, 0x5b
	ctx.r[9].s64 = ctx.r[9].s64 + 91;
	// 8318BB24: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318BB28: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8318BB2C: 917F0168  stw r11, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 8318BB30: 4BFFFE11  bl 0x8318b940
	ctx.lr = 0x8318BB34;
	sub_8318B940(ctx, base);
	// 8318BB34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318BB38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BB3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BB40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318BB44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318BB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318BB50 size=160
    let mut pc: u32 = 0x8318BB50;
    'dispatch: loop {
        match pc {
            0x8318BB50 => {
    //   block [0x8318BB50..0x8318BBF0)
	// 8318BB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BB54: 4801C615  bl 0x831a8168
	ctx.lr = 0x8318BB58;
	sub_831A8130(ctx, base);
	// 8318BB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BB5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318BB60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BB64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318BB68: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318BB6C: 3BEB0D88  addi r31, r11, 0xd88
	ctx.r[31].s64 = ctx.r[11].s64 + 3464;
	// 8318BB70: 4BFFFD91  bl 0x8318b900
	ctx.lr = 0x8318BB74;
	sub_8318B900(ctx, base);
	// 8318BB74: 817F01F8  lwz r11, 0x1f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 8318BB78: 815F01FC  lwz r10, 0x1fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8318BB7C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8318BB80: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318BB84: 4198001C  blt cr6, 0x8318bba0
	if ctx.cr[6].lt {
	pc = 0x8318BBA0; continue 'dispatch;
	}
	// 8318BB88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BB8C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8318BB90: 4BFFFDB1  bl 0x8318b940
	ctx.lr = 0x8318BB94;
	sub_8318B940(ctx, base);
	// 8318BB94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318BB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318BB9C: 4801C61C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8318BBA0: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 8318BBA4: 93BF01EC  stw r29, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[29].u32 ) };
	// 8318BBA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BBAC: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8318BBB0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8318BBB4: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318BBB8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8318BBBC: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 8318BBC0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318BBC4: 7FCBF92E  stwx r30, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 8318BBC8: 815F01F8  lwz r10, 0x1f8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 8318BBCC: 817F01F0  lwz r11, 0x1f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 8318BBD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318BBD4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8318BBD8: 915F01F8  stw r10, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[10].u32 ) };
	// 8318BBDC: 917F01F0  stw r11, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 8318BBE0: 4BFFFD61  bl 0x8318b940
	ctx.lr = 0x8318BBE4;
	sub_8318B940(ctx, base);
	// 8318BBE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318BBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318BBEC: 4801C5CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318BBF0 size=164
    let mut pc: u32 = 0x8318BBF0;
    'dispatch: loop {
        match pc {
            0x8318BBF0 => {
    //   block [0x8318BBF0..0x8318BC94)
	// 8318BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BBF4: 4801C575  bl 0x831a8168
	ctx.lr = 0x8318BBF8;
	sub_831A8130(ctx, base);
	// 8318BBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BBFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318BC00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BC04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318BC08: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318BC0C: 3BEB0D88  addi r31, r11, 0xd88
	ctx.r[31].s64 = ctx.r[11].s64 + 3464;
	// 8318BC10: 4BFFFCF1  bl 0x8318b900
	ctx.lr = 0x8318BC14;
	sub_8318B900(ctx, base);
	// 8318BC14: 817F01F8  lwz r11, 0x1f8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 8318BC18: 815F01FC  lwz r10, 0x1fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8318BC1C: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318BC20: 41810024  bgt 0x8318bc44
	if ctx.cr[0].gt {
	pc = 0x8318BC44; continue 'dispatch;
	}
	// 8318BC24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8318BC28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BC2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8318BC30: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318BC34: 4BFFFD0D  bl 0x8318b940
	ctx.lr = 0x8318BC38;
	sub_8318B940(ctx, base);
	// 8318BC38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318BC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318BC40: 4801C578  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8318BC44: 817F01EC  lwz r11, 0x1ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(492 as u32) ) } as u64;
	// 8318BC48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318BC4C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8318BC50: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318BC54: 817F01FC  lwz r11, 0x1fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8318BC58: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 8318BC5C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8318BC60: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318BC64: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8318BC68: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 8318BC6C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318BC70: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8318BC74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318BC78: 817F01FC  lwz r11, 0x1fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8318BC7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318BC80: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 8318BC84: 4BFFFCBD  bl 0x8318b940
	ctx.lr = 0x8318BC88;
	sub_8318B940(ctx, base);
	// 8318BC88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318BC8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318BC90: 4801C528  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BC98 size=104
    let mut pc: u32 = 0x8318BC98;
    'dispatch: loop {
        match pc {
            0x8318BC98 => {
    //   block [0x8318BC98..0x8318BD00)
	// 8318BC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BC9C: 4801C4D1  bl 0x831a816c
	ctx.lr = 0x8318BCA0;
	sub_831A8130(ctx, base);
	// 8318BCA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BCA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318BCA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8318BCAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318BCB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8318BCB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318BCB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318BCBC: 419A003C  beq cr6, 0x8318bcf8
	if ctx.cr[6].eq {
	pc = 0x8318BCF8; continue 'dispatch;
	}
	// 8318BCC0: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318BCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318BCC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318BCCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318BCD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318BCD4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8318BCD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318BCDC: 4E800421  bctrl
	ctx.lr = 0x8318BCE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318BCE0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318BCE4: 409A0014  bne cr6, 0x8318bcf8
	if !ctx.cr[6].eq {
	pc = 0x8318BCF8; continue 'dispatch;
	}
	// 8318BCE8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8318BCEC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318BCF0: 2F1E000F  cmpwi cr6, r30, 0xf
	ctx.cr[6].compare_i32(ctx.r[30].s32, 15, &mut ctx.xer);
	// 8318BCF4: 4198FFC0  blt cr6, 0x8318bcb4
	if ctx.cr[6].lt {
	pc = 0x8318BCB4; continue 'dispatch;
	}
	// 8318BCF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318BCFC: 4801C4C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BD00 size=48
    let mut pc: u32 = 0x8318BD00;
    'dispatch: loop {
        match pc {
            0x8318BD00 => {
    //   block [0x8318BD00..0x8318BD30)
	// 8318BD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318BD04: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8318BD08: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8318BD0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318BD10: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318BD14: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8318BD18: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318BD1C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318BD20: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318BD24: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318BD28: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8318BD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BD30 size=40
    let mut pc: u32 = 0x8318BD30;
    'dispatch: loop {
        match pc {
            0x8318BD30 => {
    //   block [0x8318BD30..0x8318BD58)
	// 8318BD30: 1D660074  mulli r11, r6, 0x74
	ctx.r[11].s64 = ctx.r[6].s64 * 116;
	// 8318BD34: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BD38: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318BD3C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8318BD40: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8318BD44: 396B05D3  addi r11, r11, 0x5d3
	ctx.r[11].s64 = ctx.r[11].s64 + 1491;
	// 8318BD48: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318BD4C: 7CCB192E  stwx r6, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u32) };
	// 8318BD50: 908A13E4  stw r4, 0x13e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5092 as u32), ctx.r[4].u32 ) };
	// 8318BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BD58 size=28
    let mut pc: u32 = 0x8318BD58;
    'dispatch: loop {
        match pc {
            0x8318BD58 => {
    //   block [0x8318BD58..0x8318BD74)
	// 8318BD58: 1D440074  mulli r10, r4, 0x74
	ctx.r[10].s64 = ctx.r[4].s64 * 116;
	// 8318BD5C: 1D650044  mulli r11, r5, 0x44
	ctx.r[11].s64 = ctx.r[5].s64 * 68;
	// 8318BD60: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8318BD64: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BD68: 90AA13E8  stw r5, 0x13e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5096 as u32), ctx.r[5].u32 ) };
	// 8318BD6C: 908B1748  stw r4, 0x1748(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5960 as u32), ctx.r[4].u32 ) };
	// 8318BD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BD78 size=112
    let mut pc: u32 = 0x8318BD78;
    'dispatch: loop {
        match pc {
            0x8318BD78 => {
    //   block [0x8318BD78..0x8318BDE8)
	// 8318BD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BD7C: 4801C3ED  bl 0x831a8168
	ctx.lr = 0x8318BD80;
	sub_831A8130(ctx, base);
	// 8318BD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BD84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318BD88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8318BD8C: 397F1738  addi r11, r31, 0x1738
	ctx.r[11].s64 = ctx.r[31].s64 + 5944;
	// 8318BD90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318BD94: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8318BD98: 3BCB000C  addi r30, r11, 0xc
	ctx.r[30].s64 = ctx.r[11].s64 + 12;
	// 8318BD9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318BDA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318BDA4: 419A002C  beq cr6, 0x8318bdd0
	if ctx.cr[6].eq {
	pc = 0x8318BDD0; continue 'dispatch;
	}
	// 8318BDA8: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318BDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318BDB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318BDB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318BDB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318BDBC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8318BDC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318BDC4: 4E800421  bctrl
	ctx.lr = 0x8318BDC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318BDC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318BDCC: 409A0014  bne cr6, 0x8318bde0
	if !ctx.cr[6].eq {
	pc = 0x8318BDE0; continue 'dispatch;
	}
	// 8318BDD0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8318BDD4: 3BDE0044  addi r30, r30, 0x44
	ctx.r[30].s64 = ctx.r[30].s64 + 68;
	// 8318BDD8: 2F1D0009  cmpwi cr6, r29, 9
	ctx.cr[6].compare_i32(ctx.r[29].s32, 9, &mut ctx.xer);
	// 8318BDDC: 4198FFC0  blt cr6, 0x8318bd9c
	if ctx.cr[6].lt {
	pc = 0x8318BD9C; continue 'dispatch;
	}
	// 8318BDE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318BDE4: 4801C3D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BDE8 size=40
    let mut pc: u32 = 0x8318BDE8;
    'dispatch: loop {
        match pc {
            0x8318BDE8 => {
    //   block [0x8318BDE8..0x8318BE10)
	// 8318BDE8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318BDEC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8318BDF0: 1D6B0044  mulli r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 * 68;
	// 8318BDF4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BDF8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8318BDFC: 816B1744  lwz r11, 0x1744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5956 as u32) ) } as u64;
	// 8318BE00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318BE04: 409A000C  bne cr6, 0x8318be10
	if !ctx.cr[6].eq {
		sub_8318BE10(ctx, base);
		return;
	}
	// 8318BE08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318BE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BE10 size=24
    let mut pc: u32 = 0x8318BE10;
    'dispatch: loop {
        match pc {
            0x8318BE10 => {
    //   block [0x8318BE10..0x8318BE28)
	// 8318BE10: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318BE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318BE18: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 8318BE1C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8318BE20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318BE24: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BE28 size=16
    let mut pc: u32 = 0x8318BE28;
    'dispatch: loop {
        match pc {
            0x8318BE28 => {
    //   block [0x8318BE28..0x8318BE38)
	// 8318BE28: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 8318BE2C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BE30: 90AB1738  stw r5, 0x1738(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5944 as u32), ctx.r[5].u32 ) };
	// 8318BE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BE38 size=16
    let mut pc: u32 = 0x8318BE38;
    'dispatch: loop {
        match pc {
            0x8318BE38 => {
    //   block [0x8318BE38..0x8318BE48)
	// 8318BE38: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 8318BE3C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BE40: 806B1738  lwz r3, 0x1738(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5944 as u32) ) } as u64;
	// 8318BE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BE48 size=16
    let mut pc: u32 = 0x8318BE48;
    'dispatch: loop {
        match pc {
            0x8318BE48 => {
    //   block [0x8318BE48..0x8318BE58)
	// 8318BE48: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 8318BE4C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BE50: 90AB173C  stw r5, 0x173c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5948 as u32), ctx.r[5].u32 ) };
	// 8318BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BE58 size=16
    let mut pc: u32 = 0x8318BE58;
    'dispatch: loop {
        match pc {
            0x8318BE58 => {
    //   block [0x8318BE58..0x8318BE68)
	// 8318BE58: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 8318BE5C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BE60: 806B173C  lwz r3, 0x173c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5948 as u32) ) } as u64;
	// 8318BE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BE68 size=28
    let mut pc: u32 = 0x8318BE68;
    'dispatch: loop {
        match pc {
            0x8318BE68 => {
    //   block [0x8318BE68..0x8318BE84)
	// 8318BE68: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 8318BE6C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318BE70: 816B1744  lwz r11, 0x1744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5956 as u32) ) } as u64;
	// 8318BE74: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318BE78: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318BE7C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8318BE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BE88 size=60
    let mut pc: u32 = 0x8318BE88;
    'dispatch: loop {
        match pc {
            0x8318BE88 => {
    //   block [0x8318BE88..0x8318BEC4)
	// 8318BE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318BE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318BE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BE98: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 8318BE9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318BEA0: 4801C671  bl 0x831a8510
	ctx.lr = 0x8318BEA4;
	sub_831A8510(ctx, base);
	// 8318BEA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318BEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318BEAC: 4BFFFDED  bl 0x8318bc98
	ctx.lr = 0x8318BEB0;
	sub_8318BC98(ctx, base);
	// 8318BEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318BEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BEBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318BEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BEC8 size=8
    let mut pc: u32 = 0x8318BEC8;
    'dispatch: loop {
        match pc {
            0x8318BEC8 => {
    //   block [0x8318BEC8..0x8318BED0)
	// 8318BEC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318BECC: 4BFFFDCC  b 0x8318bc98
	sub_8318BC98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BED0 size=12
    let mut pc: u32 = 0x8318BED0;
    'dispatch: loop {
        match pc {
            0x8318BED0 => {
    //   block [0x8318BED0..0x8318BEDC)
	// 8318BED0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8318BED4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8318BED8: 4BFFFE80  b 0x8318bd58
	sub_8318BD58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BEE0 size=12
    let mut pc: u32 = 0x8318BEE0;
    'dispatch: loop {
        match pc {
            0x8318BEE0 => {
    //   block [0x8318BEE0..0x8318BEEC)
	// 8318BEE0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8318BEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318BEE8: 4BFFFE48  b 0x8318bd30
	sub_8318BD30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BEF0 size=12
    let mut pc: u32 = 0x8318BEF0;
    'dispatch: loop {
        match pc {
            0x8318BEF0 => {
    //   block [0x8318BEF0..0x8318BEFC)
	// 8318BEF0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8318BEF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318BEF8: 4BFFFE38  b 0x8318bd30
	sub_8318BD30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318BF00 size=12
    let mut pc: u32 = 0x8318BF00;
    'dispatch: loop {
        match pc {
            0x8318BF00 => {
    //   block [0x8318BF00..0x8318BF0C)
	// 8318BF00: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8318BF04: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8318BF08: 4BFFFE28  b 0x8318bd30
	sub_8318BD30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BF10 size=104
    let mut pc: u32 = 0x8318BF10;
    'dispatch: loop {
        match pc {
            0x8318BF10 => {
    //   block [0x8318BF10..0x8318BF78)
	// 8318BF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318BF18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BF1C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8318BF20: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8318BF24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318BF28: 4BFFFE31  bl 0x8318bd58
	ctx.lr = 0x8318BF2C;
	sub_8318BD58(ctx, base);
	// 8318BF2C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8318BF30: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8318BF34: 4BFFFFAD  bl 0x8318bee0
	ctx.lr = 0x8318BF38;
	sub_8318BEE0(ctx, base);
	// 8318BF38: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318BF3C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8318BF40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318BF44: 419A001C  beq cr6, 0x8318bf60
	if ctx.cr[6].eq {
	pc = 0x8318BF60; continue 'dispatch;
	}
	// 8318BF48: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8318BF4C: 4BFFFE0D  bl 0x8318bd58
	ctx.lr = 0x8318BF50;
	sub_8318BD58(ctx, base);
	// 8318BF50: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8318BF54: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8318BF58: 4BFFFF89  bl 0x8318bee0
	ctx.lr = 0x8318BF5C;
	sub_8318BEE0(ctx, base);
	// 8318BF5C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8318BF60: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8318BF64: 4BFFFDF5  bl 0x8318bd58
	ctx.lr = 0x8318BF68;
	sub_8318BD58(ctx, base);
	// 8318BF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318BF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BF78 size=104
    let mut pc: u32 = 0x8318BF78;
    'dispatch: loop {
        match pc {
            0x8318BF78 => {
    //   block [0x8318BF78..0x8318BFE0)
	// 8318BF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318BF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BF84: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8318BF88: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8318BF8C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8318BF90: 4BFFFDC9  bl 0x8318bd58
	ctx.lr = 0x8318BF94;
	sub_8318BD58(ctx, base);
	// 8318BF94: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8318BF98: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8318BF9C: 4BFFFF45  bl 0x8318bee0
	ctx.lr = 0x8318BFA0;
	sub_8318BEE0(ctx, base);
	// 8318BFA0: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318BFA4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8318BFA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318BFAC: 419A001C  beq cr6, 0x8318bfc8
	if ctx.cr[6].eq {
	pc = 0x8318BFC8; continue 'dispatch;
	}
	// 8318BFB0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8318BFB4: 4BFFFDA5  bl 0x8318bd58
	ctx.lr = 0x8318BFB8;
	sub_8318BD58(ctx, base);
	// 8318BFB8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8318BFBC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8318BFC0: 4BFFFF21  bl 0x8318bee0
	ctx.lr = 0x8318BFC4;
	sub_8318BEE0(ctx, base);
	// 8318BFC4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318BFC8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8318BFCC: 4BFFFD8D  bl 0x8318bd58
	ctx.lr = 0x8318BFD0;
	sub_8318BD58(ctx, base);
	// 8318BFD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318BFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318BFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318BFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318BFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318BFE0 size=184
    let mut pc: u32 = 0x8318BFE0;
    'dispatch: loop {
        match pc {
            0x8318BFE0 => {
    //   block [0x8318BFE0..0x8318C098)
	// 8318BFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318BFE4: 4801C189  bl 0x831a816c
	ctx.lr = 0x8318BFE8;
	sub_831A8130(ctx, base);
	// 8318BFE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318BFEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318BFF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318BFF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318BFF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318BFFC: 4BFFFD5D  bl 0x8318bd58
	ctx.lr = 0x8318C000;
	sub_8318BD58(ctx, base);
	// 8318C000: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318C004: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8318C008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C00C: 419A0018  beq cr6, 0x8318c024
	if ctx.cr[6].eq {
	pc = 0x8318C024; continue 'dispatch;
	}
	// 8318C010: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318C014: 4BFFFECD  bl 0x8318bee0
	ctx.lr = 0x8318C018;
	sub_8318BEE0(ctx, base);
	// 8318C018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318C01C: 4BFFFEF5  bl 0x8318bf10
	ctx.lr = 0x8318C020;
	sub_8318BF10(ctx, base);
	// 8318C020: 48000014  b 0x8318c034
	pc = 0x8318C034; continue 'dispatch;
	// 8318C024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C028: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8318C02C: 4BFED42D  bl 0x83179458
	ctx.lr = 0x8318C030;
	sub_83179458(ctx, base);
	// 8318C030: 93BF0BB0  stw r29, 0xbb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2992 as u32), ctx.r[29].u32 ) };
	// 8318C034: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318C038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C03C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C040: 419A001C  beq cr6, 0x8318c05c
	if ctx.cr[6].eq {
	pc = 0x8318C05C; continue 'dispatch;
	}
	// 8318C044: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8318C048: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318C04C: 4BFFFEA5  bl 0x8318bef0
	ctx.lr = 0x8318C050;
	sub_8318BEF0(ctx, base);
	// 8318C050: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318C054: 4BFFFF25  bl 0x8318bf78
	ctx.lr = 0x8318C058;
	sub_8318BF78(ctx, base);
	// 8318C058: 48000014  b 0x8318c06c
	pc = 0x8318C06C; continue 'dispatch;
	// 8318C05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C060: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318C064: 4BFED3F5  bl 0x83179458
	ctx.lr = 0x8318C068;
	sub_83179458(ctx, base);
	// 8318C068: 93BF0BB4  stw r29, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[29].u32 ) };
	// 8318C06C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8318C070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C074: 419A001C  beq cr6, 0x8318c090
	if ctx.cr[6].eq {
	pc = 0x8318C090; continue 'dispatch;
	}
	// 8318C078: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8318C07C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318C080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C084: 4BFFFE7D  bl 0x8318bf00
	ctx.lr = 0x8318C088;
	sub_8318BF00(ctx, base);
	// 8318C088: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318C08C: 4BFFFE45  bl 0x8318bed0
	ctx.lr = 0x8318C090;
	sub_8318BED0(ctx, base);
	// 8318C090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318C094: 4801C128  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C098 size=364
    let mut pc: u32 = 0x8318C098;
    'dispatch: loop {
        match pc {
            0x8318C098 => {
    //   block [0x8318C098..0x8318C204)
	// 8318C098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C0A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318C0A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C0A8: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8318C0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C0B0: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318C0B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C0B8: 419A0030  beq cr6, 0x8318c0e8
	if ctx.cr[6].eq {
	pc = 0x8318C0E8; continue 'dispatch;
	}
	// 8318C0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C0C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318C0C4: 4BFFFE1D  bl 0x8318bee0
	ctx.lr = 0x8318C0C8;
	sub_8318BEE0(ctx, base);
	// 8318C0C8: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8318C0CC: 4BFFFF15  bl 0x8318bfe0
	ctx.lr = 0x8318C0D0;
	sub_8318BFE0(ctx, base);
	// 8318C0D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C0D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C0E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C0E4: 4E800020  blr
	return;
	// 8318C0E8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318C0EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C0F0: 419A0048  beq cr6, 0x8318c138
	if ctx.cr[6].eq {
	pc = 0x8318C138; continue 'dispatch;
	}
	// 8318C0F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318C0F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318C0FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C100: 4BFFFDE1  bl 0x8318bee0
	ctx.lr = 0x8318C104;
	sub_8318BEE0(ctx, base);
	// 8318C104: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8318C108: 4BFFFE09  bl 0x8318bf10
	ctx.lr = 0x8318C10C;
	sub_8318BF10(ctx, base);
	// 8318C10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C110: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318C114: 4BFED345  bl 0x83179458
	ctx.lr = 0x8318C118;
	sub_83179458(ctx, base);
	// 8318C118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C11C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C120: 917F0BB4  stw r11, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[11].u32 ) };
	// 8318C124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C12C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C134: 4E800020  blr
	return;
	// 8318C138: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318C13C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C140: 419A0048  beq cr6, 0x8318c188
	if ctx.cr[6].eq {
	pc = 0x8318C188; continue 'dispatch;
	}
	// 8318C144: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8318C148: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318C14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C150: 4BFFFD91  bl 0x8318bee0
	ctx.lr = 0x8318C154;
	sub_8318BEE0(ctx, base);
	// 8318C154: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8318C158: 4BFFFE21  bl 0x8318bf78
	ctx.lr = 0x8318C15C;
	sub_8318BF78(ctx, base);
	// 8318C15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C160: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8318C164: 4BFED2F5  bl 0x83179458
	ctx.lr = 0x8318C168;
	sub_83179458(ctx, base);
	// 8318C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C16C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C170: 917F0BB0  stw r11, 0xbb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2992 as u32), ctx.r[11].u32 ) };
	// 8318C174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C184: 4E800020  blr
	return;
	// 8318C188: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 8318C18C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C190: 419A005C  beq cr6, 0x8318c1ec
	if ctx.cr[6].eq {
	pc = 0x8318C1EC; continue 'dispatch;
	}
	// 8318C194: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8318C198: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318C19C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C1A0: 4BFFFD41  bl 0x8318bee0
	ctx.lr = 0x8318C1A4;
	sub_8318BEE0(ctx, base);
	// 8318C1A4: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8318C1A8: 4BFFFD29  bl 0x8318bed0
	ctx.lr = 0x8318C1AC;
	sub_8318BED0(ctx, base);
	// 8318C1AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C1B0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8318C1B4: 4BFED2A5  bl 0x83179458
	ctx.lr = 0x8318C1B8;
	sub_83179458(ctx, base);
	// 8318C1B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318C1BC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8318C1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C1C4: 4BFED295  bl 0x83179458
	ctx.lr = 0x8318C1C8;
	sub_83179458(ctx, base);
	// 8318C1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C1CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C1D0: 917F0BB0  stw r11, 0xbb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2992 as u32), ctx.r[11].u32 ) };
	// 8318C1D4: 917F0BB4  stw r11, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[11].u32 ) };
	// 8318C1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C1E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C1E8: 4E800020  blr
	return;
	// 8318C1EC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318C1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C1FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C208 size=152
    let mut pc: u32 = 0x8318C208;
    'dispatch: loop {
        match pc {
            0x8318C208 => {
    //   block [0x8318C208..0x8318C2A0)
	// 8318C208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318C214: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C218: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C21C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C220: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8318C224: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8318C228: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 8318C22C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8318C230: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8318C234: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C238: 4BFFFAC9  bl 0x8318bd00
	ctx.lr = 0x8318C23C;
	sub_8318BD00(ctx, base);
	// 8318C23C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8318C240: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8318C244: 38630044  addi r3, r3, 0x44
	ctx.r[3].s64 = ctx.r[3].s64 + 68;
	// 8318C248: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8318C24C: 409AFFE4  bne cr6, 0x8318c230
	if !ctx.cr[6].eq {
	pc = 0x8318C230; continue 'dispatch;
	}
	// 8318C250: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8318C254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C258: 4BFFFE41  bl 0x8318c098
	ctx.lr = 0x8318C25C;
	sub_8318C098(ctx, base);
	// 8318C25C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C260: 419A0028  beq cr6, 0x8318c288
	if ctx.cr[6].eq {
	pc = 0x8318C288; continue 'dispatch;
	}
	// 8318C264: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318C268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C26C: 60840302  ori r4, r4, 0x302
	ctx.r[4].u64 = ctx.r[4].u64 | 770;
	// 8318C270: 4BFFB289  bl 0x831874f8
	ctx.lr = 0x8318C274;
	sub_831874F8(ctx, base);
	// 8318C274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C284: 4E800020  blr
	return;
	// 8318C288: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C28C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2A0 size=20
    let mut pc: u32 = 0x8318C2A0;
    'dispatch: loop {
        match pc {
            0x8318C2A0 => {
    //   block [0x8318C2A0..0x8318C2B4)
	// 8318C2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C2A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318C2A8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318C2AC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318C2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2B8 size=12
    let mut pc: u32 = 0x8318C2B8;
    'dispatch: loop {
        match pc {
            0x8318C2B8 => {
    //   block [0x8318C2B8..0x8318C2C4)
	// 8318C2B8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8318C2BC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8318C2C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2C4 size=12
    let mut pc: u32 = 0x8318C2C4;
    'dispatch: loop {
        match pc {
            0x8318C2C4 => {
    //   block [0x8318C2C4..0x8318C2D0)
	// 8318C2C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C2C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318C2CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2D0 size=12
    let mut pc: u32 = 0x8318C2D0;
    'dispatch: loop {
        match pc {
            0x8318C2D0 => {
    //   block [0x8318C2D0..0x8318C2DC)
	// 8318C2D0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318C2D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318C2D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2DC size=4
    let mut pc: u32 = 0x8318C2DC;
    'dispatch: loop {
        match pc {
            0x8318C2DC => {
    //   block [0x8318C2DC..0x8318C2E0)
	// 8318C2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2E0 size=12
    let mut pc: u32 = 0x8318C2E0;
    'dispatch: loop {
        match pc {
            0x8318C2E0 => {
    //   block [0x8318C2E0..0x8318C2EC)
	// 8318C2E0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8318C2E4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8318C2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C2F0 size=16
    let mut pc: u32 = 0x8318C2F0;
    'dispatch: loop {
        match pc {
            0x8318C2F0 => {
    //   block [0x8318C2F0..0x8318C300)
	// 8318C2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318C2F4: 409A000C  bne cr6, 0x8318c300
	if !ctx.cr[6].eq {
		sub_8318C300(ctx, base);
		return;
	}
	// 8318C2F8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C300 size=28
    let mut pc: u32 = 0x8318C300;
    'dispatch: loop {
        match pc {
            0x8318C300 => {
    //   block [0x8318C300..0x8318C31C)
	// 8318C300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C304: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318C308: 419AFFF0  beq cr6, 0x8318c2f8
	if ctx.cr[6].eq {
		sub_8318C2F0(ctx, base);
		return;
	}
	// 8318C30C: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318C310: 906BD7D8  stw r3, -0x2828(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-10280 as u32), ctx.r[3].u32 ) };
	// 8318C314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C320 size=40
    let mut pc: u32 = 0x8318C320;
    'dispatch: loop {
        match pc {
            0x8318C320 => {
    //   block [0x8318C320..0x8318C348)
	// 8318C320: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318C324: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C328: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8318C32C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318C330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C334: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318C338: 40990020  ble cr6, 0x8318c358
	if !ctx.cr[6].gt {
		sub_8318C348(ctx, base);
		return;
	}
	// 8318C33C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C340: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8318C344: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C348 size=24
    let mut pc: u32 = 0x8318C348;
    'dispatch: loop {
        match pc {
            0x8318C348 => {
    //   block [0x8318C348..0x8318C360)
	// 8318C348: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318C34C: 38630100  addi r3, r3, 0x100
	ctx.r[3].s64 = ctx.r[3].s64 + 256;
	// 8318C350: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8318C354: 4198FFE8  blt cr6, 0x8318c33c
	if ctx.cr[6].lt {
		sub_8318C320(ctx, base);
		return;
	}
	// 8318C358: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C360 size=20
    let mut pc: u32 = 0x8318C360;
    'dispatch: loop {
        match pc {
            0x8318C360 => {
    //   block [0x8318C360..0x8318C374)
	// 8318C360: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8318C364: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8318C368: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318C36C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318C370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C378 size=40
    let mut pc: u32 = 0x8318C378;
    'dispatch: loop {
        match pc {
            0x8318C378 => {
    //   block [0x8318C378..0x8318C3A0)
	// 8318C378: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8318C37C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318C380: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318C384: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318C388: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318C38C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318C390: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318C394: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318C398: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318C39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C3A0 size=40
    let mut pc: u32 = 0x8318C3A0;
    'dispatch: loop {
        match pc {
            0x8318C3A0 => {
    //   block [0x8318C3A0..0x8318C3C8)
	// 8318C3A0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8318C3A4: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8318C3A8: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8318C3AC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318C3B0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318C3B4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318C3B8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318C3BC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318C3C0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318C3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C3C8 size=4
    let mut pc: u32 = 0x8318C3C8;
    'dispatch: loop {
        match pc {
            0x8318C3C8 => {
    //   block [0x8318C3C8..0x8318C3CC)
	// 8318C3C8: 4BFFFED8  b 0x8318c2a0
	sub_8318C2A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C3D0 size=72
    let mut pc: u32 = 0x8318C3D0;
    'dispatch: loop {
        match pc {
            0x8318C3D0 => {
    //   block [0x8318C3D0..0x8318C418)
	// 8318C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C3D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318C3DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C3E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318C3E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318C3E8: 409A0010  bne cr6, 0x8318c3f8
	if !ctx.cr[6].eq {
	pc = 0x8318C3F8; continue 'dispatch;
	}
	// 8318C3EC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318C3F0: 806B994C  lwz r3, -0x66b4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C3F4: 48000008  b 0x8318c3fc
	pc = 0x8318C3FC; continue 'dispatch;
	// 8318C3F8: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8318C3FC: 4BFFFEBD  bl 0x8318c2b8
	ctx.lr = 0x8318C400;
	sub_8318C2B8(ctx, base);
	// 8318C400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C418 size=112
    let mut pc: u32 = 0x8318C418;
    'dispatch: loop {
        match pc {
            0x8318C418 => {
    //   block [0x8318C418..0x8318C488)
	// 8318C418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C424: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318C428: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318C42C: 409A0010  bne cr6, 0x8318c43c
	if !ctx.cr[6].eq {
	pc = 0x8318C43C; continue 'dispatch;
	}
	// 8318C430: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318C434: 806B994C  lwz r3, -0x66b4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C438: 48000038  b 0x8318c470
	pc = 0x8318C470; continue 'dispatch;
	// 8318C43C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8318C440: 4BFFFEB1  bl 0x8318c2f0
	ctx.lr = 0x8318C444;
	sub_8318C2F0(ctx, base);
	// 8318C444: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C448: 419A0024  beq cr6, 0x8318c46c
	if ctx.cr[6].eq {
	pc = 0x8318C46C; continue 'dispatch;
	}
	// 8318C44C: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318C450: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C454: 60840101  ori r4, r4, 0x101
	ctx.r[4].u64 = ctx.r[4].u64 | 257;
	// 8318C458: 4BFFFF79  bl 0x8318c3d0
	ctx.lr = 0x8318C45C;
	sub_8318C3D0(ctx, base);
	// 8318C45C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C468: 4E800020  blr
	return;
	// 8318C46C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8318C470: 4BFFFE71  bl 0x8318c2e0
	ctx.lr = 0x8318C474;
	sub_8318C2E0(ctx, base);
	// 8318C474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C488 size=164
    let mut pc: u32 = 0x8318C488;
    'dispatch: loop {
        match pc {
            0x8318C488 => {
    //   block [0x8318C488..0x8318C52C)
	// 8318C488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318C494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C498: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8318C49C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318C4A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C4A4: 48006BE5  bl 0x83193088
	ctx.lr = 0x8318C4A8;
	sub_83193088(ctx, base);
	// 8318C4A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8318C4AC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8318C4B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318C4B4: 4BFFFDED  bl 0x8318c2a0
	ctx.lr = 0x8318C4B8;
	sub_8318C2A0(ctx, base);
	// 8318C4B8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8318C4BC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8318C4C0: 4BFFFEA1  bl 0x8318c360
	ctx.lr = 0x8318C4C4;
	sub_8318C360(ctx, base);
	// 8318C4C4: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8318C4C8: 4BFFFEB1  bl 0x8318c378
	ctx.lr = 0x8318C4CC;
	sub_8318C378(ctx, base);
	// 8318C4CC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8318C4D0: 4BFFFEA9  bl 0x8318c378
	ctx.lr = 0x8318C4D4;
	sub_8318C378(ctx, base);
	// 8318C4D4: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8318C4D8: 4BFFFEA1  bl 0x8318c378
	ctx.lr = 0x8318C4DC;
	sub_8318C378(ctx, base);
	// 8318C4DC: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 8318C4E0: 4BFFFE99  bl 0x8318c378
	ctx.lr = 0x8318C4E4;
	sub_8318C378(ctx, base);
	// 8318C4E4: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8318C4E8: 4BFFFEB9  bl 0x8318c3a0
	ctx.lr = 0x8318C4EC;
	sub_8318C3A0(ctx, base);
	// 8318C4EC: 3D408319  lis r10, -0x7ce7
	ctx.r[10].s64 = -2095513600;
	// 8318C4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C4F4: 394AD8D0  addi r10, r10, -0x2730
	ctx.r[10].s64 = ctx.r[10].s64 + -10032;
	// 8318C4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C4FC: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 8318C500: 915F00D4  stw r10, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 8318C504: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 8318C508: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8318C50C: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8318C510: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8318C514: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 8318C518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C530 size=84
    let mut pc: u32 = 0x8318C530;
    'dispatch: loop {
        match pc {
            0x8318C530 => {
    //   block [0x8318C530..0x8318C584)
	// 8318C530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C53C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318C540: 4BFFFDB1  bl 0x8318c2f0
	ctx.lr = 0x8318C544;
	sub_8318C2F0(ctx, base);
	// 8318C544: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C548: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C54C: 419A0020  beq cr6, 0x8318c56c
	if ctx.cr[6].eq {
	pc = 0x8318C56C; continue 'dispatch;
	}
	// 8318C550: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318C554: 60840103  ori r4, r4, 0x103
	ctx.r[4].u64 = ctx.r[4].u64 | 259;
	// 8318C558: 4BFFFE79  bl 0x8318c3d0
	ctx.lr = 0x8318C55C;
	sub_8318C3D0(ctx, base);
	// 8318C55C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C568: 4E800020  blr
	return;
	// 8318C56C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318C570: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318C574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C588 size=144
    let mut pc: u32 = 0x8318C588;
    'dispatch: loop {
        match pc {
            0x8318C588 => {
    //   block [0x8318C588..0x8318C618)
	// 8318C588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318C594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318C598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C59C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318C5A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8318C5A4: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8318C5A8: 3FE08345  lis r31, -0x7cbb
	ctx.r[31].s64 = -2092630016;
	// 8318C5AC: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318C5B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318C5B4: 396B0110  addi r11, r11, 0x110
	ctx.r[11].s64 = ctx.r[11].s64 + 272;
	// 8318C5B8: 5565F0BE  srwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8318C5BC: 907F994C  stw r3, -0x66b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26292 as u32), ctx.r[3].u32 ) };
	// 8318C5C0: 48006AC9  bl 0x83193088
	ctx.lr = 0x8318C5C4;
	sub_83193088(ctx, base);
	// 8318C5C4: 807F994C  lwz r3, -0x66b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C5C8: 4BFFFE01  bl 0x8318c3c8
	ctx.lr = 0x8318C5CC;
	sub_8318C3C8(ctx, base);
	// 8318C5CC: 817F994C  lwz r11, -0x66b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C5D0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8318C5D4: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8318C5D8: 817F994C  lwz r11, -0x66b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C5DC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8318C5E0: 4099001C  ble cr6, 0x8318c5fc
	if !ctx.cr[6].gt {
	pc = 0x8318C5FC; continue 'dispatch;
	}
	// 8318C5E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8318C5E8: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8318C5EC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318C5F0: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 8318C5F4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8318C5F8: 409AFFF0  bne cr6, 0x8318c5e8
	if !ctx.cr[6].eq {
	pc = 0x8318C5E8; continue 'dispatch;
	}
	// 8318C5FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318C604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C60C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318C610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C618 size=112
    let mut pc: u32 = 0x8318C618;
    'dispatch: loop {
        match pc {
            0x8318C618 => {
    //   block [0x8318C618..0x8318C688)
	// 8318C618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C620: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318C624: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318C628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C62C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318C630: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8318C634: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8318C638: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318C63C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8318C640: 40990028  ble cr6, 0x8318c668
	if !ctx.cr[6].gt {
	pc = 0x8318C668; continue 'dispatch;
	}
	// 8318C644: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C648: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318C64C: 419A000C  beq cr6, 0x8318c658
	if ctx.cr[6].eq {
	pc = 0x8318C658; continue 'dispatch;
	}
	// 8318C650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C654: 4BFFFEDD  bl 0x8318c530
	ctx.lr = 0x8318C658;
	sub_8318C530(ctx, base);
	// 8318C658: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8318C65C: 3BFF0100  addi r31, r31, 0x100
	ctx.r[31].s64 = ctx.r[31].s64 + 256;
	// 8318C660: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8318C664: 409AFFE0  bne cr6, 0x8318c644
	if !ctx.cr[6].eq {
	pc = 0x8318C644; continue 'dispatch;
	}
	// 8318C668: 4BF3B479  bl 0x830c7ae0
	ctx.lr = 0x8318C66C;
	sub_830C7AE0(ctx, base);
	// 8318C66C: 4BF3B475  bl 0x830c7ae0
	ctx.lr = 0x8318C670;
	sub_830C7AE0(ctx, base);
	// 8318C670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318C674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C67C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318C680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318C684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C688 size=60
    let mut pc: u32 = 0x8318C688;
    'dispatch: loop {
        match pc {
            0x8318C688 => {
    //   block [0x8318C688..0x8318C6C4)
	// 8318C688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C694: 4BFFFC8D  bl 0x8318c320
	ctx.lr = 0x8318C698;
	sub_8318C320(ctx, base);
	// 8318C698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318C69C: 409A0014  bne cr6, 0x8318c6b0
	if !ctx.cr[6].eq {
	pc = 0x8318C6B0; continue 'dispatch;
	}
	// 8318C6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C6AC: 4E800020  blr
	return;
	// 8318C6B0: 4BFFFDD9  bl 0x8318c488
	ctx.lr = 0x8318C6B4;
	sub_8318C488(ctx, base);
	// 8318C6B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C6C8 size=68
    let mut pc: u32 = 0x8318C6C8;
    'dispatch: loop {
        match pc {
            0x8318C6C8 => {
    //   block [0x8318C6C8..0x8318C70C)
	// 8318C6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318C6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C6D4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318C6D8: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 8318C6DC: 396BBA60  addi r11, r11, -0x45a0
	ctx.r[11].s64 = ctx.r[11].s64 + -17824;
	// 8318C6E0: 916AD7D4  stw r11, -0x282c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10284 as u32), ctx.r[11].u32 ) };
	// 8318C6E4: 4BFFFEA5  bl 0x8318c588
	ctx.lr = 0x8318C6E8;
	sub_8318C588(ctx, base);
	// 8318C6E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C6EC: 409A0010  bne cr6, 0x8318c6fc
	if !ctx.cr[6].eq {
	pc = 0x8318C6FC; continue 'dispatch;
	}
	// 8318C6F0: 4BF3B3F1  bl 0x830c7ae0
	ctx.lr = 0x8318C6F4;
	sub_830C7AE0(ctx, base);
	// 8318C6F4: 4BF3B3ED  bl 0x830c7ae0
	ctx.lr = 0x8318C6F8;
	sub_830C7AE0(ctx, base);
	// 8318C6F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C6FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318C700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318C704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318C708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C710 size=52
    let mut pc: u32 = 0x8318C710;
    'dispatch: loop {
        match pc {
            0x8318C710 => {
    //   block [0x8318C710..0x8318C744)
	// 8318C710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C714: 4801BA59  bl 0x831a816c
	ctx.lr = 0x8318C718;
	sub_831A8130(ctx, base);
	// 8318C718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C71C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318C724: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318C728: 4BFFFBC9  bl 0x8318c2f0
	ctx.lr = 0x8318C72C;
	sub_8318C2F0(ctx, base);
	// 8318C72C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C730: 409A000C  bne cr6, 0x8318c73c
	if !ctx.cr[6].eq {
	pc = 0x8318C73C; continue 'dispatch;
	}
	// 8318C734: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 8318C738: 93BF00E8  stw r29, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[29].u32 ) };
	// 8318C73C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318C740: 4801BA7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C748 size=52
    let mut pc: u32 = 0x8318C748;
    'dispatch: loop {
        match pc {
            0x8318C748 => {
    //   block [0x8318C748..0x8318C77C)
	// 8318C748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C74C: 4801BA21  bl 0x831a816c
	ctx.lr = 0x8318C750;
	sub_831A8130(ctx, base);
	// 8318C750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C758: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318C75C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318C760: 4BFFFB91  bl 0x8318c2f0
	ctx.lr = 0x8318C764;
	sub_8318C2F0(ctx, base);
	// 8318C764: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C768: 409A000C  bne cr6, 0x8318c774
	if !ctx.cr[6].eq {
	pc = 0x8318C774; continue 'dispatch;
	}
	// 8318C76C: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 8318C770: 93BF00F0  stw r29, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[29].u32 ) };
	// 8318C774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318C778: 4801BA44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C780 size=52
    let mut pc: u32 = 0x8318C780;
    'dispatch: loop {
        match pc {
            0x8318C780 => {
    //   block [0x8318C780..0x8318C7B4)
	// 8318C780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C784: 4801B9E9  bl 0x831a816c
	ctx.lr = 0x8318C788;
	sub_831A8130(ctx, base);
	// 8318C788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318C794: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318C798: 4BFFFB59  bl 0x8318c2f0
	ctx.lr = 0x8318C79C;
	sub_8318C2F0(ctx, base);
	// 8318C79C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C7A0: 409A000C  bne cr6, 0x8318c7ac
	if !ctx.cr[6].eq {
	pc = 0x8318C7AC; continue 'dispatch;
	}
	// 8318C7A4: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8318C7A8: 93BF00F8  stw r29, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[29].u32 ) };
	// 8318C7AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318C7B0: 4801BA0C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318C7B8 size=120
    let mut pc: u32 = 0x8318C7B8;
    'dispatch: loop {
        match pc {
            0x8318C7B8 => {
    //   block [0x8318C7B8..0x8318C830)
	// 8318C7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318C7BC: 4801B9A9  bl 0x831a8164
	ctx.lr = 0x8318C7C0;
	sub_831A8130(ctx, base);
	// 8318C7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318C7C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318C7C8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8318C7CC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8318C7D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318C7D4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8318C7D8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8318C7DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318C7E0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318C7E4: 4BFFFB0D  bl 0x8318c2f0
	ctx.lr = 0x8318C7E8;
	sub_8318C2F0(ctx, base);
	// 8318C7E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318C7EC: 419A001C  beq cr6, 0x8318c808
	if ctx.cr[6].eq {
	pc = 0x8318C808; continue 'dispatch;
	}
	// 8318C7F0: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318C7F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318C7F8: 60840301  ori r4, r4, 0x301
	ctx.r[4].u64 = ctx.r[4].u64 | 769;
	// 8318C7FC: 4BFFFBD5  bl 0x8318c3d0
	ctx.lr = 0x8318C800;
	sub_8318C3D0(ctx, base);
	// 8318C800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318C804: 4801B9B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318C808: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8318C80C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8318C810: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8318C814: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8318C818: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8318C81C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318C820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318C824: 4E800421  bctrl
	ctx.lr = 0x8318C828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318C828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318C82C: 4801B988  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318C830 size=652
    let mut pc: u32 = 0x8318C830;
    'dispatch: loop {
        match pc {
            0x8318C830 => {
    //   block [0x8318C830..0x8318CABC)
	// 8318C830: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8318C834: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8318C838: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 8318C83C: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8318C840: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8318C844: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8318C848: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C84C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8318C850: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 8318C854: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C858: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C85C: 2F0A001E  cmpwi cr6, r10, 0x1e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 30, &mut ctx.xer);
	// 8318C860: 7D095030  slw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C864: 41980038  blt cr6, 0x8318c89c
	if ctx.cr[6].lt {
	pc = 0x8318C89C; continue 'dispatch;
	}
	// 8318C868: 390AFFE2  addi r8, r10, -0x1e
	ctx.r[8].s64 = ctx.r[10].s64 + -30;
	// 8318C86C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8318C870: 419A001C  beq cr6, 0x8318c88c
	if ctx.cr[6].eq {
	pc = 0x8318C88C; continue 'dispatch;
	}
	// 8318C874: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C878: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318C87C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C880: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C884: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C888: 4800001C  b 0x8318c8a4
	pc = 0x8318C8A4; continue 'dispatch;
	// 8318C88C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318C890: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C894: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C898: 4800000C  b 0x8318c8a4
	pc = 0x8318C8A4; continue 'dispatch;
	// 8318C89C: 390A0002  addi r8, r10, 2
	ctx.r[8].s64 = ctx.r[10].s64 + 2;
	// 8318C8A0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C8A4: 553E17BE  srwi r30, r9, 0x1e
	ctx.r[30].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8318C8A8: 39280002  addi r9, r8, 2
	ctx.r[9].s64 = ctx.r[8].s64 + 2;
	// 8318C8AC: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318C8B0: 41980018  blt cr6, 0x8318c8c8
	if ctx.cr[6].lt {
	pc = 0x8318C8C8; continue 'dispatch;
	}
	// 8318C8B4: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318C8B8: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C8BC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C8C0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C8C4: 48000008  b 0x8318c8cc
	pc = 0x8318C8CC; continue 'dispatch;
	// 8318C8C8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C8CC: 2F09001D  cmpwi cr6, r9, 0x1d
	ctx.cr[6].compare_i32(ctx.r[9].s32, 29, &mut ctx.xer);
	// 8318C8D0: 41980044  blt cr6, 0x8318c914
	if ctx.cr[6].lt {
	pc = 0x8318C914; continue 'dispatch;
	}
	// 8318C8D4: 3929FFE3  addi r9, r9, -0x1d
	ctx.r[9].s64 = ctx.r[9].s64 + -29;
	// 8318C8D8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318C8DC: 419A0024  beq cr6, 0x8318c900
	if ctx.cr[6].eq {
	pc = 0x8318C900; continue 'dispatch;
	}
	// 8318C8E0: 21090003  subfic r8, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[9].s64;
	// 8318C8E4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C8E8: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318C8EC: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C8F0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C8F4: 551F1F7E  srwi r31, r8, 0x1d
	ctx.r[31].u32 = ctx.r[8].u32.wrapping_shr(29);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8318C8F8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C8FC: 48000024  b 0x8318c920
	pc = 0x8318C920; continue 'dispatch;
	// 8318C900: 555F1F7E  srwi r31, r10, 0x1d
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8318C904: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318C908: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C90C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C910: 48000010  b 0x8318c920
	pc = 0x8318C920; continue 'dispatch;
	// 8318C914: 555F1F7E  srwi r31, r10, 0x1d
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8318C918: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 8318C91C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C920: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318C924: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318C928: 41980018  blt cr6, 0x8318c940
	if ctx.cr[6].lt {
	pc = 0x8318C940; continue 'dispatch;
	}
	// 8318C92C: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318C930: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C934: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C938: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C93C: 48000008  b 0x8318c944
	pc = 0x8318C944; continue 'dispatch;
	// 8318C940: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C944: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8318C948: 41980044  blt cr6, 0x8318c98c
	if ctx.cr[6].lt {
	pc = 0x8318C98C; continue 'dispatch;
	}
	// 8318C94C: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8318C950: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318C954: 419A0024  beq cr6, 0x8318c978
	if ctx.cr[6].eq {
	pc = 0x8318C978; continue 'dispatch;
	}
	// 8318C958: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8318C95C: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C960: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318C964: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C968: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C96C: 55047C7E  srwi r4, r8, 0x11
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shr(17);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8318C970: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C974: 48000024  b 0x8318c998
	pc = 0x8318C998; continue 'dispatch;
	// 8318C978: 55447C7E  srwi r4, r10, 0x11
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8318C97C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318C980: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C984: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C988: 48000010  b 0x8318c998
	pc = 0x8318C998; continue 'dispatch;
	// 8318C98C: 55447C7E  srwi r4, r10, 0x11
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8318C990: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8318C994: 554A7820  slwi r10, r10, 0xf
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C998: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318C99C: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318C9A0: 41980018  blt cr6, 0x8318c9b8
	if ctx.cr[6].lt {
	pc = 0x8318C9B8; continue 'dispatch;
	}
	// 8318C9A4: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318C9A8: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C9AC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C9B0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C9B4: 48000008  b 0x8318c9bc
	pc = 0x8318C9BC; continue 'dispatch;
	// 8318C9B8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318C9BC: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8318C9C0: 4198003C  blt cr6, 0x8318c9fc
	if ctx.cr[6].lt {
	pc = 0x8318C9FC; continue 'dispatch;
	}
	// 8318C9C4: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8318C9C8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318C9CC: 419A0020  beq cr6, 0x8318c9ec
	if ctx.cr[6].eq {
	pc = 0x8318C9EC; continue 'dispatch;
	}
	// 8318C9D0: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8318C9D4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C9D8: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318C9DC: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318C9E0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C9E4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C9E8: 4800001C  b 0x8318ca04
	pc = 0x8318CA04; continue 'dispatch;
	// 8318C9EC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318C9F0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318C9F4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318C9F8: 4800000C  b 0x8318ca04
	pc = 0x8318CA04; continue 'dispatch;
	// 8318C9FC: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8318CA00: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CA04: 55467C7E  srwi r6, r10, 0x11
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318CA08: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8318CA0C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318CA10: 41980018  blt cr6, 0x8318ca28
	if ctx.cr[6].lt {
	pc = 0x8318CA28; continue 'dispatch;
	}
	// 8318CA14: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318CA18: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CA1C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CA20: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CA24: 48000008  b 0x8318ca2c
	pc = 0x8318CA2C; continue 'dispatch;
	// 8318CA28: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318CA2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318CA30: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318CA34: 41980014  blt cr6, 0x8318ca48
	if ctx.cr[6].lt {
	pc = 0x8318CA48; continue 'dispatch;
	}
	// 8318CA38: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318CA3C: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CA40: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CA44: 48000008  b 0x8318ca4c
	pc = 0x8318CA4C; continue 'dispatch;
	// 8318CA48: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318CA4C: 2F0A000A  cmpwi cr6, r10, 0xa
	ctx.cr[6].compare_i32(ctx.r[10].s32, 10, &mut ctx.xer);
	// 8318CA50: 41980024  blt cr6, 0x8318ca74
	if ctx.cr[6].lt {
	pc = 0x8318CA74; continue 'dispatch;
	}
	// 8318CA54: 394AFFF6  addi r10, r10, -0xa
	ctx.r[10].s64 = ctx.r[10].s64 + -10;
	// 8318CA58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318CA5C: 419A0018  beq cr6, 0x8318ca74
	if ctx.cr[6].eq {
	pc = 0x8318CA74; continue 'dispatch;
	}
	// 8318CA60: 214A0016  subfic r10, r10, 0x16
	ctx.xer.ca = ctx.r[10].u32 <= 22 as u32;
	ctx.r[10].s64 = (22 as i64) - ctx.r[10].s64;
	// 8318CA64: 7D6B5430  srw r11, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CA68: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8318CA6C: 556BB2BE  srwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318CA70: 48000008  b 0x8318ca78
	pc = 0x8318CA78; continue 'dispatch;
	// 8318CA74: 552BB2BE  srwi r11, r9, 0xa
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318CA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318CA7C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318CA80: 78890020  clrldi r9, r4, 0x20
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 8318CA84: 7BEA7C4C  rldimi r10, r31, 0xf, 0x11
	ctx.r[10].u64 = ((ctx.r[31].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[10].u64 & 0xFFFF800000007FFF);
	// 8318CA88: 78C80020  clrldi r8, r6, 0x20
	ctx.r[8].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	// 8318CA8C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318CA90: 7FC90034  cntlzw r9, r30
	ctx.r[9].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 8318CA94: 794A7C24  sldi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(15);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8318CA98: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8318CA9C: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8318CAA0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8318CAA4: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8318CAA8: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 8318CAAC: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8318CAB0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318CAB4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8318CAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318CAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318CAC0 size=1348
    let mut pc: u32 = 0x8318CAC0;
    'dispatch: loop {
        match pc {
            0x8318CAC0 => {
    //   block [0x8318CAC0..0x8318D004)
	// 8318CAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318CAC4: 4801B699  bl 0x831a815c
	ctx.lr = 0x8318CAC8;
	sub_831A8130(ctx, base);
	// 8318CAC8: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318CACC: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 8318CAD0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8318CAD4: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8318CAD8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8318CADC: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8318CAE0: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8318CAE4: 3BBB0028  addi r29, r27, 0x28
	ctx.r[29].s64 = ctx.r[27].s64 + 40;
	// 8318CAE8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CAEC: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8318CAF0: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 8318CAF4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CAF8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CAFC: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8318CB00: 7D095030  slw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CB04: 4198004C  blt cr6, 0x8318cb50
	if ctx.cr[6].lt {
	pc = 0x8318CB50; continue 'dispatch;
	}
	// 8318CB08: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 8318CB0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318CB10: 419A0028  beq cr6, 0x8318cb38
	if ctx.cr[6].eq {
	pc = 0x8318CB38; continue 'dispatch;
	}
	// 8318CB14: 210A0010  subfic r8, r10, 0x10
	ctx.xer.ca = ctx.r[10].u32 <= 16 as u32;
	ctx.r[8].s64 = (16 as i64) - ctx.r[10].s64;
	// 8318CB18: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CB1C: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8318CB20: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CB24: 552B843E  srwi r11, r9, 0x10
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318CB28: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318CB2C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CB30: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CB34: 4800002C  b 0x8318cb60
	pc = 0x8318CB60; continue 'dispatch;
	// 8318CB38: 5529843E  srwi r9, r9, 0x10
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318CB3C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318CB40: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318CB44: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CB48: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CB4C: 48000014  b 0x8318cb60
	pc = 0x8318CB60; continue 'dispatch;
	// 8318CB50: 5526843E  srwi r6, r9, 0x10
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(16);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318CB54: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8318CB58: 5528801E  slwi r8, r9, 0x10
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CB5C: 90DD0000  stw r6, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8318CB60: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8318CB64: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318CB68: 41980018  blt cr6, 0x8318cb80
	if ctx.cr[6].lt {
	pc = 0x8318CB80; continue 'dispatch;
	}
	// 8318CB6C: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318CB70: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CB74: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CB78: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CB7C: 48000008  b 0x8318cb84
	pc = 0x8318CB84; continue 'dispatch;
	// 8318CB80: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CB84: 2F09000A  cmpwi cr6, r9, 0xa
	ctx.cr[6].compare_i32(ctx.r[9].s32, 10, &mut ctx.xer);
	// 8318CB88: 4198004C  blt cr6, 0x8318cbd4
	if ctx.cr[6].lt {
	pc = 0x8318CBD4; continue 'dispatch;
	}
	// 8318CB8C: 3929FFF6  addi r9, r9, -0xa
	ctx.r[9].s64 = ctx.r[9].s64 + -10;
	// 8318CB90: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318CB94: 419A0028  beq cr6, 0x8318cbbc
	if ctx.cr[6].eq {
	pc = 0x8318CBBC; continue 'dispatch;
	}
	// 8318CB98: 21090016  subfic r8, r9, 0x16
	ctx.xer.ca = ctx.r[9].u32 <= 22 as u32;
	ctx.r[8].s64 = (22 as i64) - ctx.r[9].s64;
	// 8318CB9C: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CBA0: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318CBA4: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CBA8: 550BB2BE  srwi r11, r8, 0xa
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318CBAC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318CBB0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CBB4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CBB8: 4800002C  b 0x8318cbe4
	pc = 0x8318CBE4; continue 'dispatch;
	// 8318CBBC: 5548B2BE  srwi r8, r10, 0xa
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CBC0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318CBC4: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8318CBC8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CBCC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CBD0: 48000014  b 0x8318cbe4
	pc = 0x8318CBE4; continue 'dispatch;
	// 8318CBD4: 5548B2BE  srwi r8, r10, 0xa
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CBD8: 39290016  addi r9, r9, 0x16
	ctx.r[9].s64 = ctx.r[9].s64 + 22;
	// 8318CBDC: 554AB012  slwi r10, r10, 0x16
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(22);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CBE0: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8318CBE4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318CBE8: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318CBEC: 41980018  blt cr6, 0x8318cc04
	if ctx.cr[6].lt {
	pc = 0x8318CC04; continue 'dispatch;
	}
	// 8318CBF0: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318CBF4: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CBF8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CBFC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CC00: 48000008  b 0x8318cc08
	pc = 0x8318CC08; continue 'dispatch;
	// 8318CC04: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CC08: 2F09001A  cmpwi cr6, r9, 0x1a
	ctx.cr[6].compare_i32(ctx.r[9].s32, 26, &mut ctx.xer);
	// 8318CC0C: 4198004C  blt cr6, 0x8318cc58
	if ctx.cr[6].lt {
	pc = 0x8318CC58; continue 'dispatch;
	}
	// 8318CC10: 3929FFE6  addi r9, r9, -0x1a
	ctx.r[9].s64 = ctx.r[9].s64 + -26;
	// 8318CC14: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318CC18: 419A0028  beq cr6, 0x8318cc40
	if ctx.cr[6].eq {
	pc = 0x8318CC40; continue 'dispatch;
	}
	// 8318CC1C: 21090006  subfic r8, r9, 6
	ctx.xer.ca = ctx.r[9].u32 <= 6 as u32;
	ctx.r[8].s64 = (6 as i64) - ctx.r[9].s64;
	// 8318CC20: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CC24: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318CC28: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CC2C: 550B36BE  srwi r11, r8, 0x1a
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(26);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318CC30: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318CC34: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CC38: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CC3C: 4800002C  b 0x8318cc68
	pc = 0x8318CC68; continue 'dispatch;
	// 8318CC40: 554836BE  srwi r8, r10, 0x1a
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(26);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CC44: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318CC48: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8318CC4C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CC50: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CC54: 48000014  b 0x8318cc68
	pc = 0x8318CC68; continue 'dispatch;
	// 8318CC58: 554836BE  srwi r8, r10, 0x1a
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(26);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CC5C: 39290006  addi r9, r9, 6
	ctx.r[9].s64 = ctx.r[9].s64 + 6;
	// 8318CC60: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CC64: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8318CC68: 7D480034  cntlzw r8, r10
	ctx.r[8].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8318CC6C: 2F09001F  cmpwi cr6, r9, 0x1f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 31, &mut ctx.xer);
	// 8318CC70: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8318CC74: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8318CC78: 911D0010  stw r8, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8318CC7C: 409A0018  bne cr6, 0x8318cc94
	if !ctx.cr[6].eq {
	pc = 0x8318CC94; continue 'dispatch;
	}
	// 8318CC80: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318CC84: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CC88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8318CC8C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CC90: 4800000C  b 0x8318cc9c
	pc = 0x8318CC9C; continue 'dispatch;
	// 8318CC94: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318CC98: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CC9C: 7D0A0034  cntlzw r10, r8
	ctx.r[10].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8318CCA0: 2F09001F  cmpwi cr6, r9, 0x1f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 31, &mut ctx.xer);
	// 8318CCA4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8318CCA8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8318CCAC: 915D0014  stw r10, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8318CCB0: 409A0018  bne cr6, 0x8318ccc8
	if !ctx.cr[6].eq {
	pc = 0x8318CCC8; continue 'dispatch;
	}
	// 8318CCB4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8318CCB8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CCBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318CCC0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CCC4: 4800000C  b 0x8318ccd0
	pc = 0x8318CCD0; continue 'dispatch;
	// 8318CCC8: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8318CCCC: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318CCD0: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8318CCD4: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8318CCD8: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8318CCDC: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8318CCE0: 911D0018  stw r8, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8318CCE4: 409A0018  bne cr6, 0x8318ccfc
	if !ctx.cr[6].eq {
	pc = 0x8318CCFC; continue 'dispatch;
	}
	// 8318CCE8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8318CCEC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318CCF4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CCF8: 4800000C  b 0x8318cd04
	pc = 0x8318CD04; continue 'dispatch;
	// 8318CCFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318CD00: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318CD04: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8318CD08: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8318CD0C: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8318CD10: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8318CD14: 911D001C  stw r8, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8318CD18: 409A0018  bne cr6, 0x8318cd30
	if !ctx.cr[6].eq {
	pc = 0x8318CD30; continue 'dispatch;
	}
	// 8318CD1C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318CD20: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CD24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318CD28: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CD2C: 4800000C  b 0x8318cd38
	pc = 0x8318CD38; continue 'dispatch;
	// 8318CD30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318CD34: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CD38: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8318CD3C: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318CD40: 41980018  blt cr6, 0x8318cd58
	if ctx.cr[6].lt {
	pc = 0x8318CD58; continue 'dispatch;
	}
	// 8318CD44: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318CD48: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CD4C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CD50: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CD54: 48000008  b 0x8318cd5c
	pc = 0x8318CD5C; continue 'dispatch;
	// 8318CD58: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CD5C: 2F09001B  cmpwi cr6, r9, 0x1b
	ctx.cr[6].compare_i32(ctx.r[9].s32, 27, &mut ctx.xer);
	// 8318CD60: 4198004C  blt cr6, 0x8318cdac
	if ctx.cr[6].lt {
	pc = 0x8318CDAC; continue 'dispatch;
	}
	// 8318CD64: 3929FFE5  addi r9, r9, -0x1b
	ctx.r[9].s64 = ctx.r[9].s64 + -27;
	// 8318CD68: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318CD6C: 419A0028  beq cr6, 0x8318cd94
	if ctx.cr[6].eq {
	pc = 0x8318CD94; continue 'dispatch;
	}
	// 8318CD70: 21090005  subfic r8, r9, 5
	ctx.xer.ca = ctx.r[9].u32 <= 5 as u32;
	ctx.r[8].s64 = (5 as i64) - ctx.r[9].s64;
	// 8318CD74: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CD78: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318CD7C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CD80: 550B2EFE  srwi r11, r8, 0x1b
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(27);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318CD84: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318CD88: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CD8C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CD90: 4800002C  b 0x8318cdbc
	pc = 0x8318CDBC; continue 'dispatch;
	// 8318CD94: 55482EFE  srwi r8, r10, 0x1b
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(27);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CD98: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318CD9C: 911D000C  stw r8, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8318CDA0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CDA4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CDA8: 48000014  b 0x8318cdbc
	pc = 0x8318CDBC; continue 'dispatch;
	// 8318CDAC: 55482EFE  srwi r8, r10, 0x1b
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(27);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CDB0: 39290005  addi r9, r9, 5
	ctx.r[9].s64 = ctx.r[9].s64 + 5;
	// 8318CDB4: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CDB8: 911D000C  stw r8, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8318CDBC: 2F090018  cmpwi cr6, r9, 0x18
	ctx.cr[6].compare_i32(ctx.r[9].s32, 24, &mut ctx.xer);
	// 8318CDC0: 41980044  blt cr6, 0x8318ce04
	if ctx.cr[6].lt {
	pc = 0x8318CE04; continue 'dispatch;
	}
	// 8318CDC4: 3929FFE8  addi r9, r9, -0x18
	ctx.r[9].s64 = ctx.r[9].s64 + -24;
	// 8318CDC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318CDCC: 419A0024  beq cr6, 0x8318cdf0
	if ctx.cr[6].eq {
	pc = 0x8318CDF0; continue 'dispatch;
	}
	// 8318CDD0: 21090008  subfic r8, r9, 8
	ctx.xer.ca = ctx.r[9].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[9].s64;
	// 8318CDD4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CDD8: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318CDDC: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CDE0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CDE4: 551E463E  srwi r30, r8, 0x18
	ctx.r[30].u32 = ctx.r[8].u32.wrapping_shr(24);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8318CDE8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CDEC: 48000024  b 0x8318ce10
	pc = 0x8318CE10; continue 'dispatch;
	// 8318CDF0: 555E463E  srwi r30, r10, 0x18
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8318CDF4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318CDF8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CDFC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CE00: 48000010  b 0x8318ce10
	pc = 0x8318CE10; continue 'dispatch;
	// 8318CE04: 555E463E  srwi r30, r10, 0x18
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8318CE08: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8318CE0C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CE10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318CE14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318CE18: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8318CE1C: 409800E4  bge cr6, 0x8318cf00
	if !ctx.cr[6].lt {
	pc = 0x8318CF00; continue 'dispatch;
	}
	// 8318CE20: 2F090018  cmpwi cr6, r9, 0x18
	ctx.cr[6].compare_i32(ctx.r[9].s32, 24, &mut ctx.xer);
	// 8318CE24: 4198003C  blt cr6, 0x8318ce60
	if ctx.cr[6].lt {
	pc = 0x8318CE60; continue 'dispatch;
	}
	// 8318CE28: 3929FFE8  addi r9, r9, -0x18
	ctx.r[9].s64 = ctx.r[9].s64 + -24;
	// 8318CE2C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318CE30: 419A0020  beq cr6, 0x8318ce50
	if ctx.cr[6].eq {
	pc = 0x8318CE50; continue 'dispatch;
	}
	// 8318CE34: 21090008  subfic r8, r9, 8
	ctx.xer.ca = ctx.r[9].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[9].s64;
	// 8318CE38: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CE3C: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318CE40: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CE44: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CE48: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CE4C: 4800001C  b 0x8318ce68
	pc = 0x8318CE68; continue 'dispatch;
	// 8318CE50: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318CE54: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CE58: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CE5C: 4800000C  b 0x8318ce68
	pc = 0x8318CE68; continue 'dispatch;
	// 8318CE60: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8318CE64: 5548402E  slwi r8, r10, 8
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CE68: 5545463E  srwi r5, r10, 0x18
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8318CE6C: 39490002  addi r10, r9, 2
	ctx.r[10].s64 = ctx.r[9].s64 + 2;
	// 8318CE70: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318CE74: 41980018  blt cr6, 0x8318ce8c
	if ctx.cr[6].lt {
	pc = 0x8318CE8C; continue 'dispatch;
	}
	// 8318CE78: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318CE7C: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CE80: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CE84: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CE88: 48000008  b 0x8318ce90
	pc = 0x8318CE90; continue 'dispatch;
	// 8318CE8C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CE90: 7D090034  cntlzw r9, r8
	ctx.r[9].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8318CE94: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8318CE98: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8318CE9C: 5526DFFE  rlwinm r6, r9, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8318CEA0: 409A0114  bne cr6, 0x8318cfb4
	if !ctx.cr[6].eq {
	pc = 0x8318CFB4; continue 'dispatch;
	}
	// 8318CEA4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318CEA8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8318CEB0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CEB4: 55486CFE  srwi r8, r10, 0x13
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CEB8: 3929000D  addi r9, r9, 0xd
	ctx.r[9].s64 = ctx.r[9].s64 + 13;
	// 8318CEBC: 554A6824  slwi r10, r10, 0xd
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(13);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CEC0: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8318CEC4: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 8318CEC8: 3B410061  addi r26, r1, 0x61
	ctx.r[26].s64 = ctx.r[1].s64 + 97;
	// 8318CECC: 3B210062  addi r25, r1, 0x62
	ctx.r[25].s64 = ctx.r[1].s64 + 98;
	// 8318CED0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318CED4: 7CA3F9AE  stbx r5, r3, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u8) };
	// 8318CED8: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318CEDC: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8318CEE0: 7CC5D1AE  stbx r6, r5, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[26].u32), ctx.r[6].u8) };
	// 8318CEE4: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318CEE8: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318CEEC: 7D06CB2E  sthx r8, r6, r25
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[25].u32), ctx.r[8].u16) };
	// 8318CEF0: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318CEF4: 38680001  addi r3, r8, 1
	ctx.r[3].s64 = ctx.r[8].s64 + 1;
	// 8318CEF8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8318CEFC: 4198FF24  blt cr6, 0x8318ce20
	if ctx.cr[6].lt {
	pc = 0x8318CE20; continue 'dispatch;
	}
	// 8318CF00: 39690007  addi r11, r9, 7
	ctx.r[11].s64 = ctx.r[9].s64 + 7;
	// 8318CF04: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 8318CF08: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8318CF0C: 3BEBFFF8  addi r31, r11, -8
	ctx.r[31].s64 = ctx.r[11].s64 + -8;
	// 8318CF10: 7D64F850  subf r11, r4, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[4].s64;
	// 8318CF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318CF18: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318CF1C: 4800205D  bl 0x8318ef78
	ctx.lr = 0x8318CF20;
	sub_8318EF78(ctx, base);
	// 8318CF20: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318CF24: 409A0024  bne cr6, 0x8318cf48
	if !ctx.cr[6].eq {
	pc = 0x8318CF48; continue 'dispatch;
	}
	// 8318CF28: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 8318CF2C: 4800204D  bl 0x8318ef78
	ctx.lr = 0x8318CF30;
	sub_8318EF78(ctx, base);
	// 8318CF30: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 8318CF34: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318CF38: 409A0010  bne cr6, 0x8318cf48
	if !ctx.cr[6].eq {
	pc = 0x8318CF48; continue 'dispatch;
	}
	// 8318CF3C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CF40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318CF44: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318CF48: 817B00E4  lwz r11, 0xe4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(228 as u32) ) } as u64;
	// 8318CF4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318CF50: 419A005C  beq cr6, 0x8318cfac
	if ctx.cr[6].eq {
	pc = 0x8318CFAC; continue 'dispatch;
	}
	// 8318CF54: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318CF58: 57C8CFFE  rlwinm r8, r30, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	// 8318CF5C: 813D0010  lwz r9, 0x10(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318CF60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318CF64: 807B00E8  lwz r3, 0xe8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(232 as u32) ) } as u64;
	// 8318CF68: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8318CF6C: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318CF70: 9901005A  stb r8, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[8].u8 ) };
	// 8318CF74: 811D0014  lwz r8, 0x14(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318CF78: 99210055  stb r9, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[9].u8 ) };
	// 8318CF7C: 813D001C  lwz r9, 0x1c(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318CF80: 99410057  stb r10, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[10].u8 ) };
	// 8318CF84: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318CF88: 99010056  stb r8, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[8].u8 ) };
	// 8318CF8C: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318CF90: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 8318CF94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8318CF98: 57CA067E  clrlwi r10, r30, 0x19
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	// 8318CF9C: 99010059  stb r8, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[8].u8 ) };
	// 8318CFA0: 9941005B  stb r10, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[10].u8 ) };
	// 8318CFA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318CFA8: 4E800421  bctrl
	ctx.lr = 0x8318CFAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318CFAC: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8318CFB0: 4801B1FC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8318CFB4: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8318CFB8: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318CFBC: 2F090013  cmpwi cr6, r9, 0x13
	ctx.cr[6].compare_i32(ctx.r[9].s32, 19, &mut ctx.xer);
	// 8318CFC0: 4198FEF4  blt cr6, 0x8318ceb4
	if ctx.cr[6].lt {
	pc = 0x8318CEB4; continue 'dispatch;
	}
	// 8318CFC4: 3929FFED  addi r9, r9, -0x13
	ctx.r[9].s64 = ctx.r[9].s64 + -19;
	// 8318CFC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318CFCC: 419A0024  beq cr6, 0x8318cff0
	if ctx.cr[6].eq {
	pc = 0x8318CFF0; continue 'dispatch;
	}
	// 8318CFD0: 2109000D  subfic r8, r9, 0xd
	ctx.xer.ca = ctx.r[9].u32 <= 13 as u32;
	ctx.r[8].s64 = (13 as i64) - ctx.r[9].s64;
	// 8318CFD4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CFD8: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318CFDC: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318CFE0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CFE4: 55086CFE  srwi r8, r8, 0x13
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CFE8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318CFEC: 4BFFFED4  b 0x8318cec0
	pc = 0x8318CEC0; continue 'dispatch;
	// 8318CFF0: 55486CFE  srwi r8, r10, 0x13
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318CFF4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318CFF8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318CFFC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8318D000: 4BFFFEC0  b 0x8318cec0
	pc = 0x8318CEC0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318D008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318D008 size=2040
    let mut pc: u32 = 0x8318D008;
    'dispatch: loop {
        match pc {
            0x8318D008 => {
    //   block [0x8318D008..0x8318D800)
	// 8318D008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318D00C: 4801B161  bl 0x831a816c
	ctx.lr = 0x8318D010;
	sub_831A8130(ctx, base);
	// 8318D010: 39640003  addi r11, r4, 3
	ctx.r[11].s64 = ctx.r[4].s64 + 3;
	// 8318D014: 3BC300A8  addi r30, r3, 0xa8
	ctx.r[30].s64 = ctx.r[3].s64 + 168;
	// 8318D018: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8318D01C: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8318D020: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8318D024: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D028: 396A0003  addi r11, r10, 3
	ctx.r[11].s64 = ctx.r[10].s64 + 3;
	// 8318D02C: 3BE90004  addi r31, r9, 4
	ctx.r[31].s64 = ctx.r[9].s64 + 4;
	// 8318D030: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D034: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D038: 2F0A0018  cmpwi cr6, r10, 0x18
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24, &mut ctx.xer);
	// 8318D03C: 7D095030  slw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D040: 4198003C  blt cr6, 0x8318d07c
	if ctx.cr[6].lt {
	pc = 0x8318D07C; continue 'dispatch;
	}
	// 8318D044: 394AFFE8  addi r10, r10, -0x18
	ctx.r[10].s64 = ctx.r[10].s64 + -24;
	// 8318D048: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D04C: 419A0020  beq cr6, 0x8318d06c
	if ctx.cr[6].eq {
	pc = 0x8318D06C; continue 'dispatch;
	}
	// 8318D050: 210A0008  subfic r8, r10, 8
	ctx.xer.ca = ctx.r[10].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[10].s64;
	// 8318D054: 7D675030  slw r7, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D058: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D05C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D060: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D064: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8318D068: 4800001C  b 0x8318d084
	pc = 0x8318D084; continue 'dispatch;
	// 8318D06C: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 8318D070: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D074: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D078: 4800000C  b 0x8318d084
	pc = 0x8318D084; continue 'dispatch;
	// 8318D07C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8318D080: 5527402E  slwi r7, r9, 8
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8318D084: 5523463E  srwi r3, r9, 0x18
	ctx.r[3].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8318D088: 3903FF20  addi r8, r3, -0xe0
	ctx.r[8].s64 = ctx.r[3].s64 + -224;
	// 8318D08C: 2B08000F  cmplwi cr6, r8, 0xf
	ctx.cr[6].compare_u32(ctx.r[8].u32, 15 as u32, &mut ctx.xer);
	// 8318D090: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8318D094: 4199000C  bgt cr6, 0x8318d0a0
	if ctx.cr[6].gt {
	pc = 0x8318D0A0; continue 'dispatch;
	}
	// 8318D098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8318D09C: 48000054  b 0x8318d0f0
	pc = 0x8318D0F0; continue 'dispatch;
	// 8318D0A0: 3903FF40  addi r8, r3, -0xc0
	ctx.r[8].s64 = ctx.r[3].s64 + -192;
	// 8318D0A4: 2B08001F  cmplwi cr6, r8, 0x1f
	ctx.cr[6].compare_u32(ctx.r[8].u32, 31 as u32, &mut ctx.xer);
	// 8318D0A8: 4199000C  bgt cr6, 0x8318d0b4
	if ctx.cr[6].gt {
	pc = 0x8318D0B4; continue 'dispatch;
	}
	// 8318D0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8318D0B0: 48000040  b 0x8318d0f0
	pc = 0x8318D0F0; continue 'dispatch;
	// 8318D0B4: 2F0300BD  cmpwi cr6, r3, 0xbd
	ctx.cr[6].compare_i32(ctx.r[3].s32, 189, &mut ctx.xer);
	// 8318D0B8: 409A0010  bne cr6, 0x8318d0c8
	if !ctx.cr[6].eq {
	pc = 0x8318D0C8; continue 'dispatch;
	}
	// 8318D0BC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8318D0C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8318D0C4: 4800002C  b 0x8318d0f0
	pc = 0x8318D0F0; continue 'dispatch;
	// 8318D0C8: 2F0300BF  cmpwi cr6, r3, 0xbf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 191, &mut ctx.xer);
	// 8318D0CC: 409A0010  bne cr6, 0x8318d0dc
	if !ctx.cr[6].eq {
	pc = 0x8318D0DC; continue 'dispatch;
	}
	// 8318D0D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8318D0D4: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8318D0D8: 48000018  b 0x8318d0f0
	pc = 0x8318D0F0; continue 'dispatch;
	// 8318D0DC: 2F0300BE  cmpwi cr6, r3, 0xbe
	ctx.cr[6].compare_i32(ctx.r[3].s32, 190, &mut ctx.xer);
	// 8318D0E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8318D0E4: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8318D0E8: 419A0008  beq cr6, 0x8318d0f0
	if ctx.cr[6].eq {
	pc = 0x8318D0F0; continue 'dispatch;
	}
	// 8318D0EC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8318D0F0: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 8318D0F4: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8318D0F8: 911E0018  stw r8, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8318D0FC: 409A0074  bne cr6, 0x8318d170
	if !ctx.cr[6].eq {
	pc = 0x8318D170; continue 'dispatch;
	}
	// 8318D100: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8318D104: 41980054  blt cr6, 0x8318d158
	if ctx.cr[6].lt {
	pc = 0x8318D158; continue 'dispatch;
	}
	// 8318D108: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 8318D10C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D110: 419A002C  beq cr6, 0x8318d13c
	if ctx.cr[6].eq {
	pc = 0x8318D13C; continue 'dispatch;
	}
	// 8318D114: 212A0010  subfic r9, r10, 0x10
	ctx.xer.ca = ctx.r[10].u32 <= 16 as u32;
	ctx.r[9].s64 = (16 as i64) - ctx.r[10].s64;
	// 8318D118: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 8318D11C: 7D694C30  srw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D120: 7D283B78  or r8, r9, r7
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8318D124: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D128: 550B843E  srwi r11, r8, 0x10
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318D12C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318D130: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D134: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D138: 4800006C  b 0x8318d1a4
	pc = 0x8318D1A4; continue 'dispatch;
	// 8318D13C: 54E8843E  srwi r8, r7, 0x10
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D140: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8318D144: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 8318D148: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8318D14C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D150: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D154: 48000050  b 0x8318d1a4
	pc = 0x8318D1A4; continue 'dispatch;
	// 8318D158: 54E8843E  srwi r8, r7, 0x10
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D15C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8318D160: 54E9801E  slwi r9, r7, 0x10
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D164: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 8318D168: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8318D16C: 48000038  b 0x8318d1a4
	pc = 0x8318D1A4; continue 'dispatch;
	// 8318D170: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D174: 419A001C  beq cr6, 0x8318d190
	if ctx.cr[6].eq {
	pc = 0x8318D190; continue 'dispatch;
	}
	// 8318D178: 212A0020  subfic r9, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[10].s64;
	// 8318D17C: 7D694C30  srw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D180: 7D283B78  or r8, r9, r7
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8318D184: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8318D188: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D18C: 4800000C  b 0x8318d198
	pc = 0x8318D198; continue 'dispatch;
	// 8318D190: 90FE001C  stw r7, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 8318D194: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8318D198: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D19C: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 8318D1A0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D1A4: 2F0300BF  cmpwi cr6, r3, 0xbf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 191, &mut ctx.xer);
	// 8318D1A8: 419A0648  beq cr6, 0x8318d7f0
	if ctx.cr[6].eq {
	pc = 0x8318D7F0; continue 'dispatch;
	}
	// 8318D1AC: 2F0300BE  cmpwi cr6, r3, 0xbe
	ctx.cr[6].compare_i32(ctx.r[3].s32, 190, &mut ctx.xer);
	// 8318D1B0: 419A0640  beq cr6, 0x8318d7f0
	if ctx.cr[6].eq {
	pc = 0x8318D7F0; continue 'dispatch;
	}
	// 8318D1B4: 5528463E  srwi r8, r9, 0x18
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D1B8: 2F0A0018  cmpwi cr6, r10, 0x18
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24, &mut ctx.xer);
	// 8318D1BC: 40990010  ble cr6, 0x8318d1cc
	if !ctx.cr[6].gt {
	pc = 0x8318D1CC; continue 'dispatch;
	}
	// 8318D1C0: 20EA0038  subfic r7, r10, 0x38
	ctx.xer.ca = ctx.r[10].u32 <= 56 as u32;
	ctx.r[7].s64 = (56 as i64) - ctx.r[10].s64;
	// 8318D1C4: 7D673C30  srw r7, r11, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D1C8: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8318D1CC: 2B0800FF  cmplwi cr6, r8, 0xff
	ctx.cr[6].compare_u32(ctx.r[8].u32, 255 as u32, &mut ctx.xer);
	// 8318D1D0: 409A002C  bne cr6, 0x8318d1fc
	if !ctx.cr[6].eq {
	pc = 0x8318D1FC; continue 'dispatch;
	}
	// 8318D1D4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8318D1D8: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318D1DC: 41980018  blt cr6, 0x8318d1f4
	if ctx.cr[6].lt {
	pc = 0x8318D1F4; continue 'dispatch;
	}
	// 8318D1E0: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318D1E4: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D1E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D1EC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D1F0: 4BFFFFC4  b 0x8318d1b4
	pc = 0x8318D1B4; continue 'dispatch;
	// 8318D1F4: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D1F8: 4BFFFFBC  b 0x8318d1b4
	pc = 0x8318D1B4; continue 'dispatch;
	// 8318D1FC: 552817BE  srwi r8, r9, 0x1e
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D200: 2F0A001E  cmpwi cr6, r10, 0x1e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 30, &mut ctx.xer);
	// 8318D204: 40990010  ble cr6, 0x8318d214
	if !ctx.cr[6].gt {
	pc = 0x8318D214; continue 'dispatch;
	}
	// 8318D208: 20EA003E  subfic r7, r10, 0x3e
	ctx.xer.ca = ctx.r[10].u32 <= 62 as u32;
	ctx.r[7].s64 = (62 as i64) - ctx.r[10].s64;
	// 8318D20C: 7D673C30  srw r7, r11, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D210: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8318D214: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 8318D218: 409A006C  bne cr6, 0x8318d284
	if !ctx.cr[6].eq {
	pc = 0x8318D284; continue 'dispatch;
	}
	// 8318D21C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8318D220: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318D224: 41980018  blt cr6, 0x8318d23c
	if ctx.cr[6].lt {
	pc = 0x8318D23C; continue 'dispatch;
	}
	// 8318D228: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318D22C: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D230: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D234: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D238: 48000008  b 0x8318d240
	pc = 0x8318D240; continue 'dispatch;
	// 8318D23C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D240: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8318D244: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8318D248: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8318D24C: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8318D250: 409A0074  bne cr6, 0x8318d2c4
	if !ctx.cr[6].eq {
	pc = 0x8318D2C4; continue 'dispatch;
	}
	// 8318D254: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8318D258: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D25C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318D260: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D264: 55286CFE  srwi r8, r9, 0x13
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D268: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 8318D26C: 55296824  slwi r9, r9, 0xd
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(13);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D270: 55083830  slwi r8, r8, 7
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D274: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8318D278: 419A0008  beq cr6, 0x8318d280
	if ctx.cr[6].eq {
	pc = 0x8318D280; continue 'dispatch;
	}
	// 8318D27C: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D280: 911E0020  stw r8, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8318D284: 5528273E  srwi r8, r9, 0x1c
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(28);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D288: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 8318D28C: 40990010  ble cr6, 0x8318d29c
	if !ctx.cr[6].gt {
	pc = 0x8318D29C; continue 'dispatch;
	}
	// 8318D290: 20EA003C  subfic r7, r10, 0x3c
	ctx.xer.ca = ctx.r[10].u32 <= 60 as u32;
	ctx.r[7].s64 = (60 as i64) - ctx.r[10].s64;
	// 8318D294: 7D673C30  srw r7, r11, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D298: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8318D29C: 2B080002  cmplwi cr6, r8, 2
	ctx.cr[6].compare_u32(ctx.r[8].u32, 2 as u32, &mut ctx.xer);
	// 8318D2A0: 409A01DC  bne cr6, 0x8318d47c
	if !ctx.cr[6].eq {
	pc = 0x8318D47C; continue 'dispatch;
	}
	// 8318D2A4: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 8318D2A8: 2F080020  cmpwi cr6, r8, 0x20
	ctx.cr[6].compare_i32(ctx.r[8].s32, 32, &mut ctx.xer);
	// 8318D2AC: 41980068  blt cr6, 0x8318d314
	if ctx.cr[6].lt {
	pc = 0x8318D314; continue 'dispatch;
	}
	// 8318D2B0: 3908FFE0  addi r8, r8, -0x20
	ctx.r[8].s64 = ctx.r[8].s64 + -32;
	// 8318D2B4: 7D6A4030  slw r10, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D2B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D2BC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D2C0: 48000058  b 0x8318d318
	pc = 0x8318D318; continue 'dispatch;
	// 8318D2C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318D2C8: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D2CC: 2F0A0013  cmpwi cr6, r10, 0x13
	ctx.cr[6].compare_i32(ctx.r[10].s32, 19, &mut ctx.xer);
	// 8318D2D0: 4198FF94  blt cr6, 0x8318d264
	if ctx.cr[6].lt {
	pc = 0x8318D264; continue 'dispatch;
	}
	// 8318D2D4: 394AFFED  addi r10, r10, -0x13
	ctx.r[10].s64 = ctx.r[10].s64 + -19;
	// 8318D2D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D2DC: 419A0024  beq cr6, 0x8318d300
	if ctx.cr[6].eq {
	pc = 0x8318D300; continue 'dispatch;
	}
	// 8318D2E0: 210A000D  subfic r8, r10, 0xd
	ctx.xer.ca = ctx.r[10].u32 <= 13 as u32;
	ctx.r[8].s64 = (13 as i64) - ctx.r[10].s64;
	// 8318D2E4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D2E8: 7D084B78  or r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8318D2EC: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D2F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D2F4: 55086CFE  srwi r8, r8, 0x13
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D2F8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D2FC: 4BFFFF74  b 0x8318d270
	pc = 0x8318D270; continue 'dispatch;
	// 8318D300: 55286CFE  srwi r8, r9, 0x13
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D304: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8318D308: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D30C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D310: 4BFFFF60  b 0x8318d270
	pc = 0x8318D270; continue 'dispatch;
	// 8318D314: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D318: 2F08001D  cmpwi cr6, r8, 0x1d
	ctx.cr[6].compare_i32(ctx.r[8].s32, 29, &mut ctx.xer);
	// 8318D31C: 41980044  blt cr6, 0x8318d360
	if ctx.cr[6].lt {
	pc = 0x8318D360; continue 'dispatch;
	}
	// 8318D320: 3928FFE3  addi r9, r8, -0x1d
	ctx.r[9].s64 = ctx.r[8].s64 + -29;
	// 8318D324: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318D328: 419A0024  beq cr6, 0x8318d34c
	if ctx.cr[6].eq {
	pc = 0x8318D34C; continue 'dispatch;
	}
	// 8318D32C: 21090003  subfic r8, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[9].s64;
	// 8318D330: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D334: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318D338: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D33C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D340: 55061F7E  srwi r6, r8, 0x1d
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D344: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D348: 48000024  b 0x8318d36c
	pc = 0x8318D36C; continue 'dispatch;
	// 8318D34C: 55461F7E  srwi r6, r10, 0x1d
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D350: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318D354: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D358: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D35C: 48000010  b 0x8318d36c
	pc = 0x8318D36C; continue 'dispatch;
	// 8318D360: 55461F7E  srwi r6, r10, 0x1d
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D364: 39280003  addi r9, r8, 3
	ctx.r[9].s64 = ctx.r[8].s64 + 3;
	// 8318D368: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D36C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318D370: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318D374: 41980018  blt cr6, 0x8318d38c
	if ctx.cr[6].lt {
	pc = 0x8318D38C; continue 'dispatch;
	}
	// 8318D378: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318D37C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D380: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D384: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D388: 48000008  b 0x8318d390
	pc = 0x8318D390; continue 'dispatch;
	// 8318D38C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D390: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8318D394: 4198003C  blt cr6, 0x8318d3d0
	if ctx.cr[6].lt {
	pc = 0x8318D3D0; continue 'dispatch;
	}
	// 8318D398: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8318D39C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318D3A0: 419A0020  beq cr6, 0x8318d3c0
	if ctx.cr[6].eq {
	pc = 0x8318D3C0; continue 'dispatch;
	}
	// 8318D3A4: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8318D3A8: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D3AC: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318D3B0: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D3B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D3B8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D3BC: 4800001C  b 0x8318d3d8
	pc = 0x8318D3D8; continue 'dispatch;
	// 8318D3C0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318D3C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D3C8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D3CC: 4800000C  b 0x8318d3d8
	pc = 0x8318D3D8; continue 'dispatch;
	// 8318D3D0: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8318D3D4: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D3D8: 55477C7E  srwi r7, r10, 0x11
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8318D3DC: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8318D3E0: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318D3E4: 41980018  blt cr6, 0x8318d3fc
	if ctx.cr[6].lt {
	pc = 0x8318D3FC; continue 'dispatch;
	}
	// 8318D3E8: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318D3EC: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D3F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D3F4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D3F8: 48000008  b 0x8318d400
	pc = 0x8318D400; continue 'dispatch;
	// 8318D3FC: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D400: 2F0A0011  cmpwi cr6, r10, 0x11
	ctx.cr[6].compare_i32(ctx.r[10].s32, 17, &mut ctx.xer);
	// 8318D404: 41980034  blt cr6, 0x8318d438
	if ctx.cr[6].lt {
	pc = 0x8318D438; continue 'dispatch;
	}
	// 8318D408: 394AFFEF  addi r10, r10, -0x11
	ctx.r[10].s64 = ctx.r[10].s64 + -17;
	// 8318D40C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D410: 419A001C  beq cr6, 0x8318d42c
	if ctx.cr[6].eq {
	pc = 0x8318D42C; continue 'dispatch;
	}
	// 8318D414: 210A000F  subfic r8, r10, 0xf
	ctx.xer.ca = ctx.r[10].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[10].s64;
	// 8318D418: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D41C: 7D6B4430  srw r11, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D420: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8318D424: 55697C7E  srwi r9, r11, 0x11
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D428: 48000018  b 0x8318d440
	pc = 0x8318D440; continue 'dispatch;
	// 8318D42C: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D430: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D434: 4800000C  b 0x8318d440
	pc = 0x8318D440; continue 'dispatch;
	// 8318D438: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8318D43C: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D440: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8318D444: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8318D448: 4198000C  blt cr6, 0x8318d454
	if ctx.cr[6].lt {
	pc = 0x8318D454; continue 'dispatch;
	}
	// 8318D44C: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8318D450: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318D458: 78E80020  clrldi r8, r7, 0x20
	ctx.r[8].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8318D45C: 78CA7C4C  rldimi r10, r6, 0xf, 0x11
	ctx.r[10].u64 = ((ctx.r[6].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[10].u64 & 0xFFFF800000007FFF);
	// 8318D460: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 8318D464: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8318D468: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8318D46C: 794A7C24  sldi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(15);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8318D470: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318D474: F91E0008  std r8, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8318D478: 48000348  b 0x8318d7c0
	pc = 0x8318D7C0; continue 'dispatch;
	// 8318D47C: 2B080003  cmplwi cr6, r8, 3
	ctx.cr[6].compare_u32(ctx.r[8].u32, 3 as u32, &mut ctx.xer);
	// 8318D480: 409A0324  bne cr6, 0x8318d7a4
	if !ctx.cr[6].eq {
	pc = 0x8318D7A4; continue 'dispatch;
	}
	// 8318D484: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 8318D488: 2F080020  cmpwi cr6, r8, 0x20
	ctx.cr[6].compare_i32(ctx.r[8].s32, 32, &mut ctx.xer);
	// 8318D48C: 41980018  blt cr6, 0x8318d4a4
	if ctx.cr[6].lt {
	pc = 0x8318D4A4; continue 'dispatch;
	}
	// 8318D490: 3908FFE0  addi r8, r8, -0x20
	ctx.r[8].s64 = ctx.r[8].s64 + -32;
	// 8318D494: 7D6A4030  slw r10, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D498: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D49C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D4A0: 48000008  b 0x8318d4a8
	pc = 0x8318D4A8; continue 'dispatch;
	// 8318D4A4: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D4A8: 2F08001D  cmpwi cr6, r8, 0x1d
	ctx.cr[6].compare_i32(ctx.r[8].s32, 29, &mut ctx.xer);
	// 8318D4AC: 41980044  blt cr6, 0x8318d4f0
	if ctx.cr[6].lt {
	pc = 0x8318D4F0; continue 'dispatch;
	}
	// 8318D4B0: 3928FFE3  addi r9, r8, -0x1d
	ctx.r[9].s64 = ctx.r[8].s64 + -29;
	// 8318D4B4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318D4B8: 419A0024  beq cr6, 0x8318d4dc
	if ctx.cr[6].eq {
	pc = 0x8318D4DC; continue 'dispatch;
	}
	// 8318D4BC: 21090003  subfic r8, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[9].s64;
	// 8318D4C0: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D4C4: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318D4C8: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D4CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D4D0: 55031F7E  srwi r3, r8, 0x1d
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8318D4D4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D4D8: 48000024  b 0x8318d4fc
	pc = 0x8318D4FC; continue 'dispatch;
	// 8318D4DC: 55431F7E  srwi r3, r10, 0x1d
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8318D4E0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318D4E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D4E8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D4EC: 48000010  b 0x8318d4fc
	pc = 0x8318D4FC; continue 'dispatch;
	// 8318D4F0: 55431F7E  srwi r3, r10, 0x1d
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8318D4F4: 39280003  addi r9, r8, 3
	ctx.r[9].s64 = ctx.r[8].s64 + 3;
	// 8318D4F8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D4FC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318D500: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318D504: 41980018  blt cr6, 0x8318d51c
	if ctx.cr[6].lt {
	pc = 0x8318D51C; continue 'dispatch;
	}
	// 8318D508: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318D50C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D510: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D514: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D518: 48000008  b 0x8318d520
	pc = 0x8318D520; continue 'dispatch;
	// 8318D51C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D520: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8318D524: 41980044  blt cr6, 0x8318d568
	if ctx.cr[6].lt {
	pc = 0x8318D568; continue 'dispatch;
	}
	// 8318D528: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8318D52C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318D530: 419A0024  beq cr6, 0x8318d554
	if ctx.cr[6].eq {
	pc = 0x8318D554; continue 'dispatch;
	}
	// 8318D534: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8318D538: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D53C: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318D540: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D544: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D548: 55067C7E  srwi r6, r8, 0x11
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D54C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D550: 48000024  b 0x8318d574
	pc = 0x8318D574; continue 'dispatch;
	// 8318D554: 55467C7E  srwi r6, r10, 0x11
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D558: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8318D55C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D560: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D564: 48000010  b 0x8318d574
	pc = 0x8318D574; continue 'dispatch;
	// 8318D568: 55467C7E  srwi r6, r10, 0x11
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D56C: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8318D570: 554A7820  slwi r10, r10, 0xf
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D574: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8318D578: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318D57C: 41980018  blt cr6, 0x8318d594
	if ctx.cr[6].lt {
	pc = 0x8318D594; continue 'dispatch;
	}
	// 8318D580: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318D584: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D58C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D590: 48000008  b 0x8318d598
	pc = 0x8318D598; continue 'dispatch;
	// 8318D594: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D598: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8318D59C: 4198003C  blt cr6, 0x8318d5d8
	if ctx.cr[6].lt {
	pc = 0x8318D5D8; continue 'dispatch;
	}
	// 8318D5A0: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8318D5A4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318D5A8: 419A0020  beq cr6, 0x8318d5c8
	if ctx.cr[6].eq {
	pc = 0x8318D5C8; continue 'dispatch;
	}
	// 8318D5AC: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8318D5B0: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D5B4: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318D5B8: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D5BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D5C0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D5C4: 4800001C  b 0x8318d5e0
	pc = 0x8318D5E0; continue 'dispatch;
	// 8318D5C8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318D5CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D5D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D5D4: 4800000C  b 0x8318d5e0
	pc = 0x8318D5E0; continue 'dispatch;
	// 8318D5D8: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8318D5DC: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D5E0: 55477C7E  srwi r7, r10, 0x11
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8318D5E4: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8318D5E8: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318D5EC: 41980018  blt cr6, 0x8318d604
	if ctx.cr[6].lt {
	pc = 0x8318D604; continue 'dispatch;
	}
	// 8318D5F0: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318D5F4: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D5F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D5FC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D600: 48000008  b 0x8318d608
	pc = 0x8318D608; continue 'dispatch;
	// 8318D604: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D608: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8318D60C: 78C60020  clrldi r6, r6, 0x20
	ctx.r[6].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	// 8318D610: 78687C4C  rldimi r8, r3, 0xf, 0x11
	ctx.r[8].u64 = ((ctx.r[3].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[8].u64 & 0xFFFF800000007FFF);
	// 8318D614: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8318D618: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 8318D61C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318D620: 79087C24  sldi r8, r8, 0xf
	ctx.r[8].u64 = ctx.r[8].u64.wrapping_shl(15);
	ctx.r[8].u32 = ctx.r[8].u64 as u32;
	// 8318D624: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318D628: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 8318D62C: F91E0000  std r8, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8318D630: 41980018  blt cr6, 0x8318d648
	if ctx.cr[6].lt {
	pc = 0x8318D648; continue 'dispatch;
	}
	// 8318D634: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318D638: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D63C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D640: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D644: 48000008  b 0x8318d64c
	pc = 0x8318D64C; continue 'dispatch;
	// 8318D648: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D64C: 2F0A001D  cmpwi cr6, r10, 0x1d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 29, &mut ctx.xer);
	// 8318D650: 4198003C  blt cr6, 0x8318d68c
	if ctx.cr[6].lt {
	pc = 0x8318D68C; continue 'dispatch;
	}
	// 8318D654: 394AFFE3  addi r10, r10, -0x1d
	ctx.r[10].s64 = ctx.r[10].s64 + -29;
	// 8318D658: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D65C: 419A0020  beq cr6, 0x8318d67c
	if ctx.cr[6].eq {
	pc = 0x8318D67C; continue 'dispatch;
	}
	// 8318D660: 210A0003  subfic r8, r10, 3
	ctx.xer.ca = ctx.r[10].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[10].s64;
	// 8318D664: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D668: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8318D66C: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D674: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D678: 4800001C  b 0x8318d694
	pc = 0x8318D694; continue 'dispatch;
	// 8318D67C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318D680: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D684: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D688: 4800000C  b 0x8318d694
	pc = 0x8318D694; continue 'dispatch;
	// 8318D68C: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 8318D690: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D694: 55261F7E  srwi r6, r9, 0x1d
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318D698: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8318D69C: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8318D6A0: 41980018  blt cr6, 0x8318d6b8
	if ctx.cr[6].lt {
	pc = 0x8318D6B8; continue 'dispatch;
	}
	// 8318D6A4: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8318D6A8: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D6AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D6B0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D6B4: 48000008  b 0x8318d6bc
	pc = 0x8318D6BC; continue 'dispatch;
	// 8318D6B8: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D6BC: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8318D6C0: 4198003C  blt cr6, 0x8318d6fc
	if ctx.cr[6].lt {
	pc = 0x8318D6FC; continue 'dispatch;
	}
	// 8318D6C4: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8318D6C8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318D6CC: 419A0020  beq cr6, 0x8318d6ec
	if ctx.cr[6].eq {
	pc = 0x8318D6EC; continue 'dispatch;
	}
	// 8318D6D0: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8318D6D4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D6D8: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8318D6DC: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D6E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D6E4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D6E8: 4800001C  b 0x8318d704
	pc = 0x8318D704; continue 'dispatch;
	// 8318D6EC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318D6F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D6F4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D6F8: 4800000C  b 0x8318d704
	pc = 0x8318D704; continue 'dispatch;
	// 8318D6FC: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8318D700: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318D704: 55477C7E  srwi r7, r10, 0x11
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8318D708: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8318D70C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8318D710: 41980018  blt cr6, 0x8318d728
	if ctx.cr[6].lt {
	pc = 0x8318D728; continue 'dispatch;
	}
	// 8318D714: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318D718: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D71C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D720: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D724: 48000008  b 0x8318d72c
	pc = 0x8318D72C; continue 'dispatch;
	// 8318D728: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D72C: 2F0A0011  cmpwi cr6, r10, 0x11
	ctx.cr[6].compare_i32(ctx.r[10].s32, 17, &mut ctx.xer);
	// 8318D730: 41980034  blt cr6, 0x8318d764
	if ctx.cr[6].lt {
	pc = 0x8318D764; continue 'dispatch;
	}
	// 8318D734: 394AFFEF  addi r10, r10, -0x11
	ctx.r[10].s64 = ctx.r[10].s64 + -17;
	// 8318D738: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D73C: 419A001C  beq cr6, 0x8318d758
	if ctx.cr[6].eq {
	pc = 0x8318D758; continue 'dispatch;
	}
	// 8318D740: 210A000F  subfic r8, r10, 0xf
	ctx.xer.ca = ctx.r[10].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[10].s64;
	// 8318D744: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D748: 7D6B4430  srw r11, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8318D74C: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8318D750: 55697C7E  srwi r9, r11, 0x11
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D754: 48000018  b 0x8318d76c
	pc = 0x8318D76C; continue 'dispatch;
	// 8318D758: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D75C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D760: 4800000C  b 0x8318d76c
	pc = 0x8318D76C; continue 'dispatch;
	// 8318D764: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8318D768: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318D76C: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8318D770: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8318D774: 4198000C  blt cr6, 0x8318d780
	if ctx.cr[6].lt {
	pc = 0x8318D780; continue 'dispatch;
	}
	// 8318D778: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8318D77C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318D784: 78E80020  clrldi r8, r7, 0x20
	ctx.r[8].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8318D788: 78CA7C4C  rldimi r10, r6, 0xf, 0x11
	ctx.r[10].u64 = ((ctx.r[6].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[10].u64 & 0xFFFF800000007FFF);
	// 8318D78C: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 8318D790: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8318D794: 794A7C24  sldi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(15);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8318D798: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318D79C: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8318D7A0: 48000024  b 0x8318d7c4
	pc = 0x8318D7C4; continue 'dispatch;
	// 8318D7A4: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 8318D7A8: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8318D7AC: 4198000C  blt cr6, 0x8318d7b8
	if ctx.cr[6].lt {
	pc = 0x8318D7B8; continue 'dispatch;
	}
	// 8318D7B0: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8318D7B4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8318D7B8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8318D7BC: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8318D7C0: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8318D7C4: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 8318D7C8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 8318D7CC: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8318D7D0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8318D7D4: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 8318D7D8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318D7DC: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318D7E0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8318D7E4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8318D7E8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318D7EC: 4801A9D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318D7F0: 93A50000  stw r29, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8318D7F4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318D7F8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318D7FC: 4801A9C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318D800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318D800 size=208
    let mut pc: u32 = 0x8318D800;
    'dispatch: loop {
        match pc {
            0x8318D800 => {
    //   block [0x8318D800..0x8318D8D0)
	// 8318D800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318D804: 4801A961  bl 0x831a8164
	ctx.lr = 0x8318D808;
	sub_831A8130(ctx, base);
	// 8318D808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318D80C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8318D810: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8318D814: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318D818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318D81C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318D820: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8318D824: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8318D828: 48001751  bl 0x8318ef78
	ctx.lr = 0x8318D82C;
	sub_8318EF78(ctx, base);
	// 8318D82C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8318D830: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8318D834: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318D838: 419A007C  beq cr6, 0x8318d8b4
	if ctx.cr[6].eq {
	pc = 0x8318D8B4; continue 'dispatch;
	}
	// 8318D83C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8318D840: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318D844: 419A0054  beq cr6, 0x8318d898
	if ctx.cr[6].eq {
	pc = 0x8318D898; continue 'dispatch;
	}
	// 8318D848: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 8318D84C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318D850: 409A003C  bne cr6, 0x8318d88c
	if !ctx.cr[6].eq {
	pc = 0x8318D88C; continue 'dispatch;
	}
	// 8318D854: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318D858: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318D85C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318D860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318D864: 4BFFF7A5  bl 0x8318d008
	ctx.lr = 0x8318D868;
	sub_8318D008(ctx, base);
	// 8318D868: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8318D86C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318D870: 419A001C  beq cr6, 0x8318d88c
	if ctx.cr[6].eq {
	pc = 0x8318D88C; continue 'dispatch;
	}
	// 8318D874: 815F00B8  lwz r10, 0xb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 8318D878: E8BF00A8  ld r5, 0xa8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	// 8318D87C: 807F00F8  lwz r3, 0xf8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 8318D880: 5544063E  clrlwi r4, r10, 0x18
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8318D884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318D888: 4E800421  bctrl
	ctx.lr = 0x8318D88C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318D88C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8318D890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318D894: 4801A920  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318D898: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318D89C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318D8A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318D8A4: 4BFFF21D  bl 0x8318cac0
	ctx.lr = 0x8318D8A8;
	sub_8318CAC0(ctx, base);
	// 8318D8A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318D8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318D8B0: 4801A904  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318D8B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318D8B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318D8BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318D8C0: 4BFFEF71  bl 0x8318c830
	ctx.lr = 0x8318D8C4;
	sub_8318C830(ctx, base);
	// 8318D8C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318D8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318D8CC: 4801A8E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318D8D0 size=228
    let mut pc: u32 = 0x8318D8D0;
    'dispatch: loop {
        match pc {
            0x8318D8D0 => {
    //   block [0x8318D8D0..0x8318D9B4)
	// 8318D8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318D8D4: 4801A891  bl 0x831a8164
	ctx.lr = 0x8318D8D8;
	sub_831A8130(ctx, base);
	// 8318D8D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318D8DC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8318D8E0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8318D8E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318D8E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8318D8EC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8318D8F0: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8318D8F4: 41980054  blt cr6, 0x8318d948
	if ctx.cr[6].lt {
	pc = 0x8318D948; continue 'dispatch;
	}
	// 8318D8F8: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8318D8FC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318D900: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318D904: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318D908: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8318D90C: 4BFFFEF5  bl 0x8318d800
	ctx.lr = 0x8318D910;
	sub_8318D800(ctx, base);
	// 8318D910: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D914: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318D918: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318D91C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318D920: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318D924: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8318D928: 7FEBF850  subf r31, r11, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8318D92C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318D930: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D934: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318D938: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318D93C: 419A000C  beq cr6, 0x8318d948
	if ctx.cr[6].eq {
	pc = 0x8318D948; continue 'dispatch;
	}
	// 8318D940: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8318D944: 4098FFB4  bge cr6, 0x8318d8f8
	if !ctx.cr[6].lt {
	pc = 0x8318D8F8; continue 'dispatch;
	}
	// 8318D948: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D94C: 556B039C  rlwinm r11, r11, 0, 0xe, 0xe
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8318D950: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318D954: 419A0054  beq cr6, 0x8318d9a8
	if ctx.cr[6].eq {
	pc = 0x8318D9A8; continue 'dispatch;
	}
	// 8318D958: 397B0028  addi r11, r27, 0x28
	ctx.r[11].s64 = ctx.r[27].s64 + 40;
	// 8318D95C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318D960: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318D964: 419A000C  beq cr6, 0x8318d970
	if ctx.cr[6].eq {
	pc = 0x8318D970; continue 'dispatch;
	}
	// 8318D968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318D96C: 48000014  b 0x8318d980
	pc = 0x8318D980; continue 'dispatch;
	// 8318D970: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318D974: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8318D978: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8318D97C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318D980: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318D984: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8318D988: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 8318D98C: 394A0048  addi r10, r10, 0x48
	ctx.r[10].s64 = ctx.r[10].s64 + 72;
	// 8318D990: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318D994: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D998: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318D99C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318D9A0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318D9A4: 4200FFF0  bdnz 0x8318d994
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318D994; continue 'dispatch;
	}
	// 8318D9A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318D9AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318D9B0: 4801A804  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318D9B8 size=92
    let mut pc: u32 = 0x8318D9B8;
    'dispatch: loop {
        match pc {
            0x8318D9B8 => {
    //   block [0x8318D9B8..0x8318DA14)
	// 8318D9B8: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8318D9BC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318D9C0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8318D9C4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318D9C8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318D9CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318D9D0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318D9D4: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318D9D8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318D9DC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318D9E0: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318D9E4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318D9E8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318D9EC: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318D9F0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318D9F4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318D9F8: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318D9FC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318DA00: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318DA04: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318DA08: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318DA0C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318DA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DA18 size=128
    let mut pc: u32 = 0x8318DA18;
    'dispatch: loop {
        match pc {
            0x8318DA18 => {
    //   block [0x8318DA18..0x8318DA98)
	// 8318DA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318DA20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318DA24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DA28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318DA2C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8318DA30: 7C862851  subf. r4, r6, r5
	ctx.r[4].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8318DA34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318DA38: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8318DA3C: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8318DA40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DA44: 41810014  bgt 0x8318da58
	if ctx.cr[0].gt {
	pc = 0x8318DA58; continue 'dispatch;
	}
	// 8318DA48: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318DA4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DA50: 6084040C  ori r4, r4, 0x40c
	ctx.r[4].u64 = ctx.r[4].u64 | 1036;
	// 8318DA54: 4800002C  b 0x8318da80
	pc = 0x8318DA80; continue 'dispatch;
	// 8318DA58: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318DA5C: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 8318DA60: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318DA64: 4BFA6D6D  bl 0x831347d0
	ctx.lr = 0x8318DA68;
	sub_831347D0(ctx, base);
	// 8318DA68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318DA6C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8318DA70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DA74: 409A0010  bne cr6, 0x8318da84
	if !ctx.cr[6].eq {
	pc = 0x8318DA84; continue 'dispatch;
	}
	// 8318DA78: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318DA7C: 6084040A  ori r4, r4, 0x40a
	ctx.r[4].u64 = ctx.r[4].u64 | 1034;
	// 8318DA80: 4BFF9A79  bl 0x831874f8
	ctx.lr = 0x8318DA84;
	sub_831874F8(ctx, base);
	// 8318DA84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318DA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318DA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318DA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318DA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DA98 size=100
    let mut pc: u32 = 0x8318DA98;
    'dispatch: loop {
        match pc {
            0x8318DA98 => {
    //   block [0x8318DA98..0x8318DAFC)
	// 8318DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318DAA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318DAA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DAA8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DAAC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DAB0: 396B1398  addi r11, r11, 0x1398
	ctx.r[11].s64 = ctx.r[11].s64 + 5016;
	// 8318DAB4: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8318DAB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DABC: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 8318DAC0: 409A0028  bne cr6, 0x8318dae8
	if !ctx.cr[6].eq {
	pc = 0x8318DAE8; continue 'dispatch;
	}
	// 8318DAC4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318DAC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318DACC: 419A001C  beq cr6, 0x8318dae8
	if ctx.cr[6].eq {
	pc = 0x8318DAE8; continue 'dispatch;
	}
	// 8318DAD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DAD4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318DAD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DADC: 4E800421  bctrl
	ctx.lr = 0x8318DAE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318DAE4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318DAE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318DAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318DAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318DAF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318DAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DB00 size=36
    let mut pc: u32 = 0x8318DB00;
    'dispatch: loop {
        match pc {
            0x8318DB00 => {
    //   block [0x8318DB00..0x8318DB24)
	// 8318DB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318DB04: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8318DB08: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 8318DB0C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8318DB10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318DB14: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8318DB18: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8318DB1C: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318DB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DB28 size=56
    let mut pc: u32 = 0x8318DB28;
    'dispatch: loop {
        match pc {
            0x8318DB28 => {
    //   block [0x8318DB28..0x8318DB60)
	// 8318DB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318DB2C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DB30: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318DB34: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318DB38: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318DB3C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318DB40: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318DB44: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318DB48: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318DB4C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318DB50: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318DB54: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8318DB58: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318DB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DB60 size=20
    let mut pc: u32 = 0x8318DB60;
    'dispatch: loop {
        match pc {
            0x8318DB60 => {
    //   block [0x8318DB60..0x8318DB74)
	// 8318DB60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318DB64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318DB68: 409A000C  bne cr6, 0x8318db74
	if !ctx.cr[6].eq {
		sub_8318DB74(ctx, base);
		return;
	}
	// 8318DB6C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318DB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DB74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DB74 size=52
    let mut pc: u32 = 0x8318DB74;
    'dispatch: loop {
        match pc {
            0x8318DB74 => {
    //   block [0x8318DB74..0x8318DBA8)
	// 8318DB74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DB78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318DB7C: 409A002C  bne cr6, 0x8318dba8
	if !ctx.cr[6].eq {
		sub_8318DBA8(ctx, base);
		return;
	}
	// 8318DB80: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318DB84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318DB88: 419AFFE4  beq cr6, 0x8318db6c
	if ctx.cr[6].eq {
		sub_8318DB60(ctx, base);
		return;
	}
	// 8318DB8C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318DB90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318DB94: 4099FFD8  ble cr6, 0x8318db6c
	if !ctx.cr[6].gt {
		sub_8318DB60(ctx, base);
		return;
	}
	// 8318DB98: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318DB9C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318DBA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318DBA4: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DBA8 size=8
    let mut pc: u32 = 0x8318DBA8;
    'dispatch: loop {
        match pc {
            0x8318DBA8 => {
    //   block [0x8318DBA8..0x8318DBB0)
	// 8318DBA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DBB0 size=16
    let mut pc: u32 = 0x8318DBB0;
    'dispatch: loop {
        match pc {
            0x8318DBB0 => {
    //   block [0x8318DBB0..0x8318DBC0)
	// 8318DBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318DBB4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DBB8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318DBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DBC0 size=56
    let mut pc: u32 = 0x8318DBC0;
    'dispatch: loop {
        match pc {
            0x8318DBC0 => {
    //   block [0x8318DBC0..0x8318DBF8)
	// 8318DBC0: 54AA2036  slwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318DBC4: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DBC8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8318DBCC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DBD0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DBD4: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 8318DBD8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318DBDC: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318DBE0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318DBE4: 81460008  lwz r10, 8(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318DBE8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318DBEC: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318DBF0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8318DBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DBF8 size=56
    let mut pc: u32 = 0x8318DBF8;
    'dispatch: loop {
        match pc {
            0x8318DBF8 => {
    //   block [0x8318DBF8..0x8318DC30)
	// 8318DBF8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DBFC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DC00: 54AA2036  slwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318DC04: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 8318DC08: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318DC0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DC10: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318DC14: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318DC18: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318DC1C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318DC20: 91460008  stw r10, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318DC24: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318DC28: 9166000C  stw r11, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318DC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DC30 size=216
    let mut pc: u32 = 0x8318DC30;
    'dispatch: loop {
        match pc {
            0x8318DC30 => {
    //   block [0x8318DC30..0x8318DD08)
	// 8318DC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DC34: 4801A531  bl 0x831a8164
	ctx.lr = 0x8318DC38;
	sub_831A8130(ctx, base);
	// 8318DC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DC3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318DC40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318DC44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8318DC48: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8318DC4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DC50: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318DC54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DC58: 4E800421  bctrl
	ctx.lr = 0x8318DC5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DC5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DC60: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 8318DC64: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8318DC68: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8318DC6C: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8318DC70: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318DC74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318DC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318DC7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DC80: 4E800421  bctrl
	ctx.lr = 0x8318DC84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DC84: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318DC88: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8318DC8C: 40980048  bge cr6, 0x8318dcd4
	if !ctx.cr[6].lt {
	pc = 0x8318DCD4; continue 'dispatch;
	}
	// 8318DC90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DC94: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 8318DC98: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8318DC9C: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8318DCA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318DCA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318DCA8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318DCAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DCB0: 4E800421  bctrl
	ctx.lr = 0x8318DCB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DCB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DCB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318DCBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318DCC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318DCC4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318DCC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DCCC: 4E800421  bctrl
	ctx.lr = 0x8318DCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DCD0: 48000010  b 0x8318dce0
	pc = 0x8318DCE0; continue 'dispatch;
	// 8318DCD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318DCD8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DCDC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318DCE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DCE4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8318DCE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318DCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318DCF0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318DCF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DCF8: 4E800421  bctrl
	ctx.lr = 0x8318DCFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DCFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8318DD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318DD04: 4801A4B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DD08 size=108
    let mut pc: u32 = 0x8318DD08;
    'dispatch: loop {
        match pc {
            0x8318DD08 => {
    //   block [0x8318DD08..0x8318DD74)
	// 8318DD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318DD10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318DD14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318DD18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DD1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318DD20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318DD24: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318DD28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DD2C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318DD30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DD34: 4E800421  bctrl
	ctx.lr = 0x8318DD38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DD38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DD3C: 7FCA0034  cntlzw r10, r30
	ctx.r[10].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 8318DD40: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318DD44: 5544DFFE  rlwinm r4, r10, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8318DD48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318DD4C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8318DD50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DD54: 4E800421  bctrl
	ctx.lr = 0x8318DD58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DD58: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318DD5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318DD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318DD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318DD68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318DD6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318DD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DD78 size=68
    let mut pc: u32 = 0x8318DD78;
    'dispatch: loop {
        match pc {
            0x8318DD78 => {
    //   block [0x8318DD78..0x8318DDBC)
	// 8318DD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DD7C: 4801A3F1  bl 0x831a816c
	ctx.lr = 0x8318DD80;
	sub_831A8130(ctx, base);
	// 8318DD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DD84: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DD88: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DD8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318DD90: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318DD94: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8318DD98: 4BFF9659  bl 0x831873f0
	ctx.lr = 0x8318DD9C;
	sub_831873F0(ctx, base);
	// 8318DD9C: 817F13C0  lwz r11, 0x13c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5056 as u32) ) } as u64;
	// 8318DDA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318DDA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DDA8: 817F13C4  lwz r11, 0x13c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5060 as u32) ) } as u64;
	// 8318DDAC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DDB0: 4BFF9651  bl 0x83187400
	ctx.lr = 0x8318DDB4;
	sub_83187400(ctx, base);
	// 8318DDB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318DDB8: 4801A404  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DDC0 size=60
    let mut pc: u32 = 0x8318DDC0;
    'dispatch: loop {
        match pc {
            0x8318DDC0 => {
    //   block [0x8318DDC0..0x8318DDFC)
	// 8318DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DDC4: 4801A3A9  bl 0x831a816c
	ctx.lr = 0x8318DDC8;
	sub_831A8130(ctx, base);
	// 8318DDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DDCC: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DDD0: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DDD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318DDD8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318DDDC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8318DDE0: 4BFF9611  bl 0x831873f0
	ctx.lr = 0x8318DDE4;
	sub_831873F0(ctx, base);
	// 8318DDE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318DDE8: 93DF13C0  stw r30, 0x13c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5056 as u32), ctx.r[30].u32 ) };
	// 8318DDEC: 93BF13C4  stw r29, 0x13c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5060 as u32), ctx.r[29].u32 ) };
	// 8318DDF0: 4BFF9611  bl 0x83187400
	ctx.lr = 0x8318DDF4;
	sub_83187400(ctx, base);
	// 8318DDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318DDF8: 4801A3C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DE00 size=16
    let mut pc: u32 = 0x8318DE00;
    'dispatch: loop {
        match pc {
            0x8318DE00 => {
    //   block [0x8318DE00..0x8318DE10)
	// 8318DE00: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DE04: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DE08: 806B13B4  lwz r3, 0x13b4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5044 as u32) ) } as u64;
	// 8318DE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DE10 size=16
    let mut pc: u32 = 0x8318DE10;
    'dispatch: loop {
        match pc {
            0x8318DE10 => {
    //   block [0x8318DE10..0x8318DE20)
	// 8318DE10: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DE14: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DE18: 806B13CC  lwz r3, 0x13cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5068 as u32) ) } as u64;
	// 8318DE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DE20 size=116
    let mut pc: u32 = 0x8318DE20;
    'dispatch: loop {
        match pc {
            0x8318DE20 => {
    //   block [0x8318DE20..0x8318DE94)
	// 8318DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DE24: 4801A349  bl 0x831a816c
	ctx.lr = 0x8318DE28;
	sub_831A8130(ctx, base);
	// 8318DE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DE2C: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DE30: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DE34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318DE38: 4BFF95B9  bl 0x831873f0
	ctx.lr = 0x8318DE3C;
	sub_831873F0(ctx, base);
	// 8318DE3C: 83DF13C8  lwz r30, 0x13c8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5064 as u32) ) } as u64;
	// 8318DE40: 83BF13CC  lwz r29, 0x13cc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5068 as u32) ) } as u64;
	// 8318DE44: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8318DE48: 409A002C  bne cr6, 0x8318de74
	if !ctx.cr[6].eq {
	pc = 0x8318DE74; continue 'dispatch;
	}
	// 8318DE4C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8318DE50: 419A0030  beq cr6, 0x8318de80
	if ctx.cr[6].eq {
	pc = 0x8318DE80; continue 'dispatch;
	}
	// 8318DE54: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 8318DE58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318DE5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318DE60: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318DE64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318DE68: 4E800421  bctrl
	ctx.lr = 0x8318DE6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318DE6C: 7FC3EA14  add r30, r3, r29
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 8318DE70: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8318DE74: 4098000C  bge cr6, 0x8318de80
	if !ctx.cr[6].lt {
	pc = 0x8318DE80; continue 'dispatch;
	}
	// 8318DE78: 3FC07FFF  lis r30, 0x7fff
	ctx.r[30].s64 = 2147418112;
	// 8318DE7C: 63DEFFFF  ori r30, r30, 0xffff
	ctx.r[30].u64 = ctx.r[30].u64 | 65535;
	// 8318DE80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318DE84: 4BFF957D  bl 0x83187400
	ctx.lr = 0x8318DE88;
	sub_83187400(ctx, base);
	// 8318DE88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318DE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318DE90: 4801A32C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DE98 size=40
    let mut pc: u32 = 0x8318DE98;
    'dispatch: loop {
        match pc {
            0x8318DE98 => {
    //   block [0x8318DE98..0x8318DEC0)
	// 8318DE98: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DE9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318DEA0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DEA4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318DEA8: 814B139C  lwz r10, 0x139c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5020 as u32) ) } as u64;
	// 8318DEAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318DEB0: 409A0010  bne cr6, 0x8318dec0
	if !ctx.cr[6].eq {
		sub_8318DEC0(ctx, base);
		return;
	}
	// 8318DEB4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318DEB8: 60840401  ori r4, r4, 0x401
	ctx.r[4].u64 = ctx.r[4].u64 | 1025;
	// 8318DEBC: 4BFF963C  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DEC0 size=16
    let mut pc: u32 = 0x8318DEC0;
    'dispatch: loop {
        match pc {
            0x8318DEC0 => {
    //   block [0x8318DEC0..0x8318DED0)
	// 8318DEC0: 816B13AC  lwz r11, 0x13ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5036 as u32) ) } as u64;
	// 8318DEC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DEC8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318DECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DED0 size=24
    let mut pc: u32 = 0x8318DED0;
    'dispatch: loop {
        match pc {
            0x8318DED0 => {
    //   block [0x8318DED0..0x8318DEE8)
	// 8318DED0: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DED4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DED8: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 8318DEDC: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318DEE0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318DEE4: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DEE8 size=12
    let mut pc: u32 = 0x8318DEE8;
    'dispatch: loop {
        match pc {
            0x8318DEE8 => {
    //   block [0x8318DEE8..0x8318DEF4)
	// 8318DEE8: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 8318DEEC: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8318DEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DEF8 size=40
    let mut pc: u32 = 0x8318DEF8;
    'dispatch: loop {
        match pc {
            0x8318DEF8 => {
    //   block [0x8318DEF8..0x8318DF20)
	// 8318DEF8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DEFC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DF00: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8318DF04: 814B139C  lwz r10, 0x139c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5020 as u32) ) } as u64;
	// 8318DF08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318DF0C: 409A0014  bne cr6, 0x8318df20
	if !ctx.cr[6].eq {
		sub_8318DF20(ctx, base);
		return;
	}
	// 8318DF10: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8318DF14: 808B13E4  lwz r4, 0x13e4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5092 as u32) ) } as u64;
	// 8318DF18: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8318DF1C: 4BFFDECC  b 0x8318bde8
	sub_8318BDE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DF20 size=8
    let mut pc: u32 = 0x8318DF20;
    'dispatch: loop {
        match pc {
            0x8318DF20 => {
    //   block [0x8318DF20..0x8318DF28)
	// 8318DF20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DF28 size=96
    let mut pc: u32 = 0x8318DF28;
    'dispatch: loop {
        match pc {
            0x8318DF28 => {
    //   block [0x8318DF28..0x8318DF88)
	// 8318DF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318DF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318DF34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318DF38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318DF3C: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DF40: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8318DF44: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8318DF48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DF4C: 814B139C  lwz r10, 0x139c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5020 as u32) ) } as u64;
	// 8318DF50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318DF54: 409A0018  bne cr6, 0x8318df6c
	if !ctx.cr[6].eq {
	pc = 0x8318DF6C; continue 'dispatch;
	}
	// 8318DF58: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8318DF5C: 808B13E4  lwz r4, 0x13e4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5092 as u32) ) } as u64;
	// 8318DF60: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8318DF64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318DF68: 4BFFDE81  bl 0x8318bde8
	ctx.lr = 0x8318DF6C;
	sub_8318BDE8(ctx, base);
	// 8318DF6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318DF70: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8318DF74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318DF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318DF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318DF80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318DF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DF88 size=8
    let mut pc: u32 = 0x8318DF88;
    'dispatch: loop {
        match pc {
            0x8318DF88 => {
    //   block [0x8318DF88..0x8318DF90)
	// 8318DF88: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8318DF8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DF90 size=16
    let mut pc: u32 = 0x8318DF90;
    'dispatch: loop {
        match pc {
            0x8318DF90 => {
    //   block [0x8318DF90..0x8318DFA0)
	// 8318DF90: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DF94: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DF98: 90AB13A0  stw r5, 0x13a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5024 as u32), ctx.r[5].u32 ) };
	// 8318DF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DFA0 size=16
    let mut pc: u32 = 0x8318DFA0;
    'dispatch: loop {
        match pc {
            0x8318DFA0 => {
    //   block [0x8318DFA0..0x8318DFB0)
	// 8318DFA0: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8318DFA4: 409A000C  bne cr6, 0x8318dfb0
	if !ctx.cr[6].eq {
		sub_8318DFB0(ctx, base);
		return;
	}
	// 8318DFA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318DFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DFB0 size=16
    let mut pc: u32 = 0x8318DFB0;
    'dispatch: loop {
        match pc {
            0x8318DFB0 => {
    //   block [0x8318DFB0..0x8318DFC0)
	// 8318DFB0: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DFB4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DFB8: 806B13A0  lwz r3, 0x13a0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5024 as u32) ) } as u64;
	// 8318DFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DFC0 size=8
    let mut pc: u32 = 0x8318DFC0;
    'dispatch: loop {
        match pc {
            0x8318DFC0 => {
    //   block [0x8318DFC0..0x8318DFC8)
	// 8318DFC0: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8318DFC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DFC8 size=16
    let mut pc: u32 = 0x8318DFC8;
    'dispatch: loop {
        match pc {
            0x8318DFC8 => {
    //   block [0x8318DFC8..0x8318DFD8)
	// 8318DFC8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DFCC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DFD0: 90AB13A4  stw r5, 0x13a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5028 as u32), ctx.r[5].u32 ) };
	// 8318DFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DFD8 size=16
    let mut pc: u32 = 0x8318DFD8;
    'dispatch: loop {
        match pc {
            0x8318DFD8 => {
    //   block [0x8318DFD8..0x8318DFE8)
	// 8318DFD8: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8318DFDC: 409A000C  bne cr6, 0x8318dfe8
	if !ctx.cr[6].eq {
		sub_8318DFE8(ctx, base);
		return;
	}
	// 8318DFE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318DFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318DFE8 size=16
    let mut pc: u32 = 0x8318DFE8;
    'dispatch: loop {
        match pc {
            0x8318DFE8 => {
    //   block [0x8318DFE8..0x8318DFF8)
	// 8318DFE8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318DFEC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318DFF0: 806B13A4  lwz r3, 0x13a4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5028 as u32) ) } as u64;
	// 8318DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318DFF8 size=168
    let mut pc: u32 = 0x8318DFF8;
    'dispatch: loop {
        match pc {
            0x8318DFF8 => {
    //   block [0x8318DFF8..0x8318E0A0)
	// 8318DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318DFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318E004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318E010: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8318E014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318E018: 4BFA67B9  bl 0x831347d0
	ctx.lr = 0x8318E01C;
	sub_831347D0(ctx, base);
	// 8318E01C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E020: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E024: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318E028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E02C: 4E800421  bctrl
	ctx.lr = 0x8318E030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E030: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318E034: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E038: 3BCBD7DC  addi r30, r11, -0x2824
	ctx.r[30].s64 = ctx.r[11].s64 + -10276;
	// 8318E03C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318E040: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8318E044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E048: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E04C: 4E800421  bctrl
	ctx.lr = 0x8318E050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E050: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8318E054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318E058: 4BFA5A41  bl 0x83133a98
	ctx.lr = 0x8318E05C;
	sub_83133A98(ctx, base);
	// 8318E05C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E060: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E064: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318E068: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E06C: 4E800421  bctrl
	ctx.lr = 0x8318E070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E070: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E074: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8318E078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E07C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318E080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E084: 4E800421  bctrl
	ctx.lr = 0x8318E088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E094: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318E098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E0A0 size=64
    let mut pc: u32 = 0x8318E0A0;
    'dispatch: loop {
        match pc {
            0x8318E0A0 => {
    //   block [0x8318E0A0..0x8318E0E0)
	// 8318E0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E0AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E0B0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318E0B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E0B8: 4E800421  bctrl
	ctx.lr = 0x8318E0BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E0BC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318E0C0: 816BD7E0  lwz r11, -0x2820(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10272 as u32) ) } as u64;
	// 8318E0C4: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8318E0C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318E0CC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318E0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E0E0 size=64
    let mut pc: u32 = 0x8318E0E0;
    'dispatch: loop {
        match pc {
            0x8318E0E0 => {
    //   block [0x8318E0E0..0x8318E120)
	// 8318E0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E0EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E0F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318E0F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E0F8: 4E800421  bctrl
	ctx.lr = 0x8318E0FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E0FC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318E100: 816BD7DC  lwz r11, -0x2824(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10276 as u32) ) } as u64;
	// 8318E104: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8318E108: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318E10C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318E110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E120 size=44
    let mut pc: u32 = 0x8318E120;
    'dispatch: loop {
        match pc {
            0x8318E120 => {
    //   block [0x8318E120..0x8318E14C)
	// 8318E120: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318E124: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318E128: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318E12C: 41980008  blt cr6, 0x8318e134
	if ctx.cr[6].lt {
	pc = 0x8318E134; continue 'dispatch;
	}
	// 8318E130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E134: 786A07C4  rldicr r10, r3, 0, 0x1f
	ctx.r[10].u64 = (ctx.r[3].u64).rotate_left(0) & 0xFFFFFFFF00000000;
	// 8318E138: 796B07C6  sldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(32);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 8318E13C: 78890020  clrldi r9, r4, 0x20
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 8318E140: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8318E144: 7D634B78  or r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8318E148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E150 size=4
    let mut pc: u32 = 0x8318E150;
    'dispatch: loop {
        match pc {
            0x8318E150 => {
    //   block [0x8318E150..0x8318E154)
	// 8318E150: 4BFFFEA8  b 0x8318dff8
	sub_8318DFF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E158 size=72
    let mut pc: u32 = 0x8318E158;
    'dispatch: loop {
        match pc {
            0x8318E158 => {
    //   block [0x8318E158..0x8318E1A0)
	// 8318E158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E160: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E164: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318E16C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E170: 4BFFF929  bl 0x8318da98
	ctx.lr = 0x8318E174;
	sub_8318DA98(ctx, base);
	// 8318E174: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318E178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E17C: 4BFFF91D  bl 0x8318da98
	ctx.lr = 0x8318E180;
	sub_8318DA98(ctx, base);
	// 8318E180: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8318E184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E188: 4BFFF911  bl 0x8318da98
	ctx.lr = 0x8318E18C;
	sub_8318DA98(ctx, base);
	// 8318E18C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E1A0 size=176
    let mut pc: u32 = 0x8318E1A0;
    'dispatch: loop {
        match pc {
            0x8318E1A0 => {
    //   block [0x8318E1A0..0x8318E250)
	// 8318E1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E1A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318E1AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E1B4: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318E1B8: 1D670074  mulli r11, r7, 0x74
	ctx.r[11].s64 = ctx.r[7].s64 * 116;
	// 8318E1BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E1C0: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8318E1C4: 7D68302E  lwzx r11, r8, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 8318E1C8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318E1CC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318E1D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318E1D4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318E1D8: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 8318E1DC: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 8318E1E0: 4BFFF921  bl 0x8318db00
	ctx.lr = 0x8318E1E4;
	sub_8318DB00(ctx, base);
	// 8318E1E4: 7CE8F02E  lwzx r7, r8, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8318E1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318E1EC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8318E1F0: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8318E1F4: 7D08302E  lwzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 8318E1F8: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318E1FC: 9149000C  stw r10, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8318E200: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8318E204: 811F1FBC  lwz r8, 0x1fbc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8124 as u32) ) } as u64;
	// 8318E208: 91090010  stw r8, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8318E20C: 811F1FB8  lwz r8, 0x1fb8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 8318E210: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8318E214: 40990024  ble cr6, 0x8318e238
	if !ctx.cr[6].gt {
	pc = 0x8318E238; continue 'dispatch;
	}
	// 8318E218: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8318E21C: 80E90010  lwz r7, 0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318E220: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318E224: 7D47412E  stwx r10, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 8318E228: 39080088  addi r8, r8, 0x88
	ctx.r[8].s64 = ctx.r[8].s64 + 136;
	// 8318E22C: 80FF1FB8  lwz r7, 0x1fb8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8120 as u32) ) } as u64;
	// 8318E230: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8318E234: 4198FFE8  blt cr6, 0x8318e21c
	if ctx.cr[6].lt {
	pc = 0x8318E21C; continue 'dispatch;
	}
	// 8318E238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E244: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318E248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E250 size=132
    let mut pc: u32 = 0x8318E250;
    'dispatch: loop {
        match pc {
            0x8318E250 => {
    //   block [0x8318E250..0x8318E2D4)
	// 8318E250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E25C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 8318E260: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318E264: 1D660074  mulli r11, r6, 0x74
	ctx.r[11].s64 = ctx.r[6].s64 * 116;
	// 8318E268: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318E26C: 7D69402E  lwzx r11, r9, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8318E270: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 8318E274: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8318E278: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318E27C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318E280: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 8318E284: 4BFFF87D  bl 0x8318db00
	ctx.lr = 0x8318E288;
	sub_8318DB00(ctx, base);
	// 8318E288: 7D49382E  lwzx r10, r9, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8318E28C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E290: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8318E294: 7D49402E  lwzx r10, r9, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8318E298: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318E29C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318E2A0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318E2A4: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318E2A8: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8318E2AC: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8318E2B0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318E2B4: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8318E2B8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8318E2BC: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8318E2C0: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8318E2C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E2D8 size=56
    let mut pc: u32 = 0x8318E2D8;
    'dispatch: loop {
        match pc {
            0x8318E2D8 => {
    //   block [0x8318E2D8..0x8318E310)
	// 8318E2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E2E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E2E4: 1D660074  mulli r11, r6, 0x74
	ctx.r[11].s64 = ctx.r[6].s64 * 116;
	// 8318E2E8: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318E2EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318E2F0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8318E2F4: 4BFFF80D  bl 0x8318db00
	ctx.lr = 0x8318E2F8;
	sub_8318DB00(ctx, base);
	// 8318E2F8: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 8318E2FC: 4BFFF82D  bl 0x8318db28
	ctx.lr = 0x8318E300;
	sub_8318DB28(ctx, base);
	// 8318E300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E310 size=112
    let mut pc: u32 = 0x8318E310;
    'dispatch: loop {
        match pc {
            0x8318E310 => {
    //   block [0x8318E310..0x8318E380)
	// 8318E310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E314: 48019E55  bl 0x831a8168
	ctx.lr = 0x8318E318;
	sub_831A8130(ctx, base);
	// 8318E318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E31C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318E324: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318E328: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318E32C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8318E330: 4BFF90C1  bl 0x831873f0
	ctx.lr = 0x8318E334;
	sub_831873F0(ctx, base);
	// 8318E334: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8318E338: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8318E33C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8318E340: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8318E344: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318E348: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E34C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318E350: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318E354: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318E358: 4200FFF0  bdnz 0x8318e348
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318E348; continue 'dispatch;
	}
	// 8318E35C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8318E360: 4BFFF851  bl 0x8318dbb0
	ctx.lr = 0x8318E364;
	sub_8318DBB0(ctx, base);
	// 8318E364: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318E36C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318E370: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318E374: 4BFF908D  bl 0x83187400
	ctx.lr = 0x8318E378;
	sub_83187400(ctx, base);
	// 8318E378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318E37C: 48019E3C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E380 size=160
    let mut pc: u32 = 0x8318E380;
    'dispatch: loop {
        match pc {
            0x8318E380 => {
    //   block [0x8318E380..0x8318E420)
	// 8318E380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E38C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E390: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318E394: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8318E398: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 8318E39C: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318E3A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E3A4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8318E3A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E3AC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318E3B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318E3B4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318E3B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318E3BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318E3C0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318E3C4: 816A139C  lwz r11, 0x139c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5020 as u32) ) } as u64;
	// 8318E3C8: 806A13AC  lwz r3, 0x13ac(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5036 as u32) ) } as u64;
	// 8318E3CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318E3D0: 419A0038  beq cr6, 0x8318e408
	if ctx.cr[6].eq {
	pc = 0x8318E408; continue 'dispatch;
	}
	// 8318E3D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318E3D8: 419A0030  beq cr6, 0x8318e408
	if ctx.cr[6].eq {
	pc = 0x8318E408; continue 'dispatch;
	}
	// 8318E3DC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8318E3E0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318E3E4: 4BFFF84D  bl 0x8318dc30
	ctx.lr = 0x8318E3E8;
	sub_8318DC30(ctx, base);
	// 8318E3E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318E3EC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318E3F0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318E3F4: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318E3F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E3FC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318E400: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8318E404: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8318E408: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318E40C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E420 size=132
    let mut pc: u32 = 0x8318E420;
    'dispatch: loop {
        match pc {
            0x8318E420 => {
    //   block [0x8318E420..0x8318E4A4)
	// 8318E420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E42C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E434: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8318E438: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318E43C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318E440: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318E444: 4BFFF7ED  bl 0x8318dc30
	ctx.lr = 0x8318E448;
	sub_8318DC30(ctx, base);
	// 8318E448: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318E44C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318E450: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8318E454: 41980014  blt cr6, 0x8318e468
	if ctx.cr[6].lt {
	pc = 0x8318E468; continue 'dispatch;
	}
	// 8318E458: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318E45C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8318E460: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8318E464: 4198002C  blt cr6, 0x8318e490
	if ctx.cr[6].lt {
	pc = 0x8318E490; continue 'dispatch;
	}
	// 8318E468: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318E46C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8318E470: 41980014  blt cr6, 0x8318e484
	if ctx.cr[6].lt {
	pc = 0x8318E484; continue 'dispatch;
	}
	// 8318E474: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318E478: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8318E47C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8318E480: 41980010  blt cr6, 0x8318e490
	if ctx.cr[6].lt {
	pc = 0x8318E490; continue 'dispatch;
	}
	// 8318E484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E488: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318E48C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318E490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E4A8 size=180
    let mut pc: u32 = 0x8318E4A8;
    'dispatch: loop {
        match pc {
            0x8318E4A8 => {
    //   block [0x8318E4A8..0x8318E55C)
	// 8318E4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E4AC: 48019CC1  bl 0x831a816c
	ctx.lr = 0x8318E4B0;
	sub_831A8130(ctx, base);
	// 8318E4B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E4B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318E4BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318E4C0: 4BFFFBE1  bl 0x8318e0a0
	ctx.lr = 0x8318E4C4;
	sub_8318E0A0(ctx, base);
	// 8318E4C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E4CC: 419A0034  beq cr6, 0x8318e500
	if ctx.cr[6].eq {
	pc = 0x8318E500; continue 'dispatch;
	}
	// 8318E4D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318E4D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318E4D8: 4BFA6271  bl 0x83134748
	ctx.lr = 0x8318E4DC;
	sub_83134748(ctx, base);
	// 8318E4DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318E4E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318E4E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318E4E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E4EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E4F0: 4BFA6259  bl 0x83134748
	ctx.lr = 0x8318E4F4;
	sub_83134748(ctx, base);
	// 8318E4F4: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8318E4F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E4FC: 48019CC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318E500: 4BFFFBE1  bl 0x8318e0e0
	ctx.lr = 0x8318E504;
	sub_8318E0E0(ctx, base);
	// 8318E504: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E508: 419A0040  beq cr6, 0x8318e548
	if ctx.cr[6].eq {
	pc = 0x8318E548; continue 'dispatch;
	}
	// 8318E50C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E510: 4BFA5509  bl 0x83133a18
	ctx.lr = 0x8318E514;
	sub_83133A18(ctx, base);
	// 8318E514: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318E518: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318E51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E520: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E528: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318E52C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318E530: 4E800421  bctrl
	ctx.lr = 0x8318E534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318E534: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E538: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8318E53C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E544: 48019C78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318E548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E54C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E550: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E558: 48019C64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E560 size=144
    let mut pc: u32 = 0x8318E560;
    'dispatch: loop {
        match pc {
            0x8318E560 => {
    //   block [0x8318E560..0x8318E5F0)
	// 8318E560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E564: 48019C05  bl 0x831a8168
	ctx.lr = 0x8318E568;
	sub_831A8130(ctx, base);
	// 8318E568: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E56C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8318E570: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318E574: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 8318E578: 7CAA282E  lwzx r5, r10, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8318E57C: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318E580: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8318E584: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 8318E588: 409A0010  bne cr6, 0x8318e598
	if !ctx.cr[6].eq {
	pc = 0x8318E598; continue 'dispatch;
	}
	// 8318E58C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8318E590: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 8318E594: 48000040  b 0x8318e5d4
	pc = 0x8318E5D4; continue 'dispatch;
	// 8318E598: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8318E59C: 7C8A202E  lwzx r4, r10, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8318E5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318E5A4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8318E5A8: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 8318E5AC: 4BFFF46D  bl 0x8318da18
	ctx.lr = 0x8318E5B0;
	sub_8318DA18(ctx, base);
	// 8318E5B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E5B4: 409A0034  bne cr6, 0x8318e5e8
	if !ctx.cr[6].eq {
	pc = 0x8318E5E8; continue 'dispatch;
	}
	// 8318E5B8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8318E5BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E5C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E5C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318E5C8: 4BFFFD49  bl 0x8318e310
	ctx.lr = 0x8318E5CC;
	sub_8318E310(ctx, base);
	// 8318E5CC: 387C0028  addi r3, r28, 0x28
	ctx.r[3].s64 = ctx.r[28].s64 + 40;
	// 8318E5D0: 480003D1  bl 0x8318e9a0
	ctx.lr = 0x8318E5D4;
	sub_8318E9A0(ctx, base);
	// 8318E5D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318E5D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318E5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E5E0: 4BFFF521  bl 0x8318db00
	ctx.lr = 0x8318E5E4;
	sub_8318DB00(ctx, base);
	// 8318E5E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318E5E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318E5EC: 48019BCC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E5F0 size=132
    let mut pc: u32 = 0x8318E5F0;
    'dispatch: loop {
        match pc {
            0x8318E5F0 => {
    //   block [0x8318E5F0..0x8318E674)
	// 8318E5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E5F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E5FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E600: 1D650074  mulli r11, r5, 0x74
	ctx.r[11].s64 = ctx.r[5].s64 * 116;
	// 8318E604: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318E608: 38AB1398  addi r5, r11, 0x1398
	ctx.r[5].s64 = ctx.r[11].s64 + 5016;
	// 8318E60C: 3BE50010  addi r31, r5, 0x10
	ctx.r[31].s64 = ctx.r[5].s64 + 16;
	// 8318E610: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E614: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8318E618: 419A0024  beq cr6, 0x8318e63c
	if ctx.cr[6].eq {
	pc = 0x8318E63C; continue 'dispatch;
	}
	// 8318E61C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318E620: 60840409  ori r4, r4, 0x409
	ctx.r[4].u64 = ctx.r[4].u64 | 1033;
	// 8318E624: 4BFF8ED5  bl 0x831874f8
	ctx.lr = 0x8318E628;
	sub_831874F8(ctx, base);
	// 8318E628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E638: 4E800020  blr
	return;
	// 8318E63C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318E640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E644: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318E648: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318E64C: 69660001  xori r6, r11, 1
	ctx.r[6].u64 = ctx.r[11].u64 ^ 1;
	// 8318E650: 4BFFFCC1  bl 0x8318e310
	ctx.lr = 0x8318E654;
	sub_8318E310(ctx, base);
	// 8318E654: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8318E658: 48000369  bl 0x8318e9c0
	ctx.lr = 0x8318E65C;
	sub_8318E9C0(ctx, base);
	// 8318E65C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E66C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E678 size=8
    let mut pc: u32 = 0x8318E678;
    'dispatch: loop {
        match pc {
            0x8318E678 => {
    //   block [0x8318E678..0x8318E680)
	// 8318E678: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318E67C: 4BFFFD04  b 0x8318e380
	sub_8318E380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E680 size=8
    let mut pc: u32 = 0x8318E680;
    'dispatch: loop {
        match pc {
            0x8318E680 => {
    //   block [0x8318E680..0x8318E688)
	// 8318E680: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8318E684: 4BFFFCFC  b 0x8318e380
	sub_8318E380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E688 size=264
    let mut pc: u32 = 0x8318E688;
    'dispatch: loop {
        match pc {
            0x8318E688 => {
    //   block [0x8318E688..0x8318E790)
	// 8318E688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E68C: 48019ACD  bl 0x831a8158
	ctx.lr = 0x8318E690;
	sub_831A8130(ctx, base);
	// 8318E690: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E694: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8318E698: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8318E69C: 1D7A0074  mulli r11, r26, 0x74
	ctx.r[11].s64 = ctx.r[26].s64 * 116;
	// 8318E6A0: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8318E6A4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8318E6A8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8318E6AC: 396B1398  addi r11, r11, 0x1398
	ctx.r[11].s64 = ctx.r[11].s64 + 5016;
	// 8318E6B0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8318E6B4: 3B8B0010  addi r28, r11, 0x10
	ctx.r[28].s64 = ctx.r[11].s64 + 16;
	// 8318E6B8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8318E6BC: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318E6C0: 419A00C4  beq cr6, 0x8318e784
	if ctx.cr[6].eq {
	pc = 0x8318E784; continue 'dispatch;
	}
	// 8318E6C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318E6C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318E6CC: 419A00B8  beq cr6, 0x8318e784
	if ctx.cr[6].eq {
	pc = 0x8318E784; continue 'dispatch;
	}
	// 8318E6D0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8318E6D4: 419A00B0  beq cr6, 0x8318e784
	if ctx.cr[6].eq {
	pc = 0x8318E784; continue 'dispatch;
	}
	// 8318E6D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318E6DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E6E0: 4BFFF629  bl 0x8318dd08
	ctx.lr = 0x8318E6E4;
	sub_8318DD08(ctx, base);
	// 8318E6E4: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8318E6E8: 40980034  bge cr6, 0x8318e71c
	if !ctx.cr[6].lt {
	pc = 0x8318E71C; continue 'dispatch;
	}
	// 8318E6EC: 7F63F850  subf r27, r3, r31
	ctx.r[27].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 8318E6F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318E6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E6F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8318E6FC: 4BFFF60D  bl 0x8318dd08
	ctx.lr = 0x8318E700;
	sub_8318DD08(ctx, base);
	// 8318E700: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8318E704: 40980018  bge cr6, 0x8318e71c
	if !ctx.cr[6].lt {
	pc = 0x8318E71C; continue 'dispatch;
	}
	// 8318E708: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318E70C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8318E710: 6084040B  ori r4, r4, 0x40b
	ctx.r[4].u64 = ctx.r[4].u64 | 1035;
	// 8318E714: 4BFF8DE5  bl 0x831874f8
	ctx.lr = 0x8318E718;
	sub_831874F8(ctx, base);
	// 8318E718: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8318E71C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8318E720: 409A003C  bne cr6, 0x8318e75c
	if !ctx.cr[6].eq {
	pc = 0x8318E75C; continue 'dispatch;
	}
	// 8318E724: 2F1A0001  cmpwi cr6, r26, 1
	ctx.cr[6].compare_i32(ctx.r[26].s32, 1, &mut ctx.xer);
	// 8318E728: 409A000C  bne cr6, 0x8318e734
	if !ctx.cr[6].eq {
	pc = 0x8318E734; continue 'dispatch;
	}
	// 8318E72C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318E730: 4BFFFCF1  bl 0x8318e420
	ctx.lr = 0x8318E734;
	sub_8318E420(ctx, base);
	// 8318E734: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318E738: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318E73C: 41980034  blt cr6, 0x8318e770
	if ctx.cr[6].lt {
	pc = 0x8318E770; continue 'dispatch;
	}
	// 8318E740: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8318E744: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8318E748: 917C0024  stw r11, 0x24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318E74C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318E750: 91790044  stw r11, 0x44(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8318E754: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318E758: 48019A50  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8318E75C: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 8318E760: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318E764: 4198000C  blt cr6, 0x8318e770
	if ctx.cr[6].lt {
	pc = 0x8318E770; continue 'dispatch;
	}
	// 8318E768: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8318E76C: 917C0020  stw r11, 0x20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318E770: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318E774: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8318E778: 91790044  stw r11, 0x44(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8318E77C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318E780: 48019A28  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8318E784: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318E788: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318E78C: 48019A1C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E790 size=48
    let mut pc: u32 = 0x8318E790;
    'dispatch: loop {
        match pc {
            0x8318E790 => {
    //   block [0x8318E790..0x8318E7C0)
	// 8318E790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E79C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318E7A0: 4BFFFEE1  bl 0x8318e680
	ctx.lr = 0x8318E7A4;
	sub_8318E680(ctx, base);
	// 8318E7A4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318E7A8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318E7AC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8318E7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318E7C0 size=276
    let mut pc: u32 = 0x8318E7C0;
    'dispatch: loop {
        match pc {
            0x8318E7C0 => {
    //   block [0x8318E7C0..0x8318E8D4)
	// 8318E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E7C4: 480199A9  bl 0x831a816c
	ctx.lr = 0x8318E7C8;
	sub_831A8130(ctx, base);
	// 8318E7C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E7CC: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8318E7D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8318E7D4: 3BE90008  addi r31, r9, 8
	ctx.r[31].s64 = ctx.r[9].s64 + 8;
	// 8318E7D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318E7DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318E7E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318E7E4: 80A90004  lwz r5, 4(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318E7E8: 4BFFF1D1  bl 0x8318d9b8
	ctx.lr = 0x8318E7EC;
	sub_8318D9B8(ctx, base);
	// 8318E7EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318E7F0: 81490028  lwz r10, 0x28(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 8318E7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318E7F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E7FC: 7D2B53D6  divw r9, r11, r10
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8318E800: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E804: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8318E808: 7CEA5850  subf r7, r10, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8318E80C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E810: 4BFFFD51  bl 0x8318e560
	ctx.lr = 0x8318E814;
	sub_8318E560(ctx, base);
	// 8318E814: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E818: 409A00B4  bne cr6, 0x8318e8cc
	if !ctx.cr[6].eq {
	pc = 0x8318E8CC; continue 'dispatch;
	}
	// 8318E81C: 38E00800  li r7, 0x800
	ctx.r[7].s64 = 2048;
	// 8318E820: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8318E824: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E828: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E82C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E830: 4BFFFD31  bl 0x8318e560
	ctx.lr = 0x8318E834;
	sub_8318E560(ctx, base);
	// 8318E834: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E838: 409A0094  bne cr6, 0x8318e8cc
	if !ctx.cr[6].eq {
	pc = 0x8318E8CC; continue 'dispatch;
	}
	// 8318E83C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8318E840: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8318E844: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E848: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E84C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E850: 4BFFFD11  bl 0x8318e560
	ctx.lr = 0x8318E854;
	sub_8318E560(ctx, base);
	// 8318E854: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E858: 409A0074  bne cr6, 0x8318e8cc
	if !ctx.cr[6].eq {
	pc = 0x8318E8CC; continue 'dispatch;
	}
	// 8318E85C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8318E860: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8318E864: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318E868: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318E86C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318E870: 4BFFF931  bl 0x8318e1a0
	ctx.lr = 0x8318E874;
	sub_8318E1A0(ctx, base);
	// 8318E874: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8318E878: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E87C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E880: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E884: 4BFFF9CD  bl 0x8318e250
	ctx.lr = 0x8318E888;
	sub_8318E250(ctx, base);
	// 8318E888: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8318E88C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8318E890: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318E894: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318E898: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318E89C: 4BFFF905  bl 0x8318e1a0
	ctx.lr = 0x8318E8A0;
	sub_8318E1A0(ctx, base);
	// 8318E8A0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8318E8A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E8A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E8AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E8B0: 4BFFF9A1  bl 0x8318e250
	ctx.lr = 0x8318E8B4;
	sub_8318E250(ctx, base);
	// 8318E8B4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8318E8B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8318E8BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318E8C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E8C4: 4BFFFA15  bl 0x8318e2d8
	ctx.lr = 0x8318E8C8;
	sub_8318E2D8(ctx, base);
	// 8318E8C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318E8CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318E8D0: 480198EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318E8D8 size=176
    let mut pc: u32 = 0x8318E8D8;
    'dispatch: loop {
        match pc {
            0x8318E8D8 => {
    //   block [0x8318E8D8..0x8318E988)
	// 8318E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318E8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318E8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318E8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E8EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318E8F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E8F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318E8F8: 4BFFF269  bl 0x8318db60
	ctx.lr = 0x8318E8FC;
	sub_8318DB60(ctx, base);
	// 8318E8FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E904: 419A0014  beq cr6, 0x8318e918
	if ctx.cr[6].eq {
	pc = 0x8318E918; continue 'dispatch;
	}
	// 8318E908: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318E90C: 60840408  ori r4, r4, 0x408
	ctx.r[4].u64 = ctx.r[4].u64 | 1032;
	// 8318E910: 4BFF8BE9  bl 0x831874f8
	ctx.lr = 0x8318E914;
	sub_831874F8(ctx, base);
	// 8318E914: 4800005C  b 0x8318e970
	pc = 0x8318E970; continue 'dispatch;
	// 8318E918: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8318E91C: 4BFFD54D  bl 0x8318be68
	ctx.lr = 0x8318E920;
	sub_8318BE68(ctx, base);
	// 8318E920: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E924: 419A000C  beq cr6, 0x8318e930
	if ctx.cr[6].eq {
	pc = 0x8318E930; continue 'dispatch;
	}
	// 8318E928: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318E92C: 48000038  b 0x8318e964
	pc = 0x8318E964; continue 'dispatch;
	// 8318E930: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8318E934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E938: 4BFFD531  bl 0x8318be68
	ctx.lr = 0x8318E93C;
	sub_8318BE68(ctx, base);
	// 8318E93C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318E940: 419A000C  beq cr6, 0x8318e94c
	if ctx.cr[6].eq {
	pc = 0x8318E94C; continue 'dispatch;
	}
	// 8318E944: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318E948: 4800001C  b 0x8318e964
	pc = 0x8318E964; continue 'dispatch;
	// 8318E94C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8318E950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E954: 4BFFD515  bl 0x8318be68
	ctx.lr = 0x8318E958;
	sub_8318BE68(ctx, base);
	// 8318E958: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 8318E95C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8318E960: 556507BC  rlwinm r5, r11, 0, 0x1e, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8318E964: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318E968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318E96C: 4BFFFC85  bl 0x8318e5f0
	ctx.lr = 0x8318E970;
	sub_8318E5F0(ctx, base);
	// 8318E970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318E97C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318E980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318E984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E988 size=8
    let mut pc: u32 = 0x8318E988;
    'dispatch: loop {
        match pc {
            0x8318E988 => {
    //   block [0x8318E988..0x8318E990)
	// 8318E988: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318E98C: 4BFFFCFC  b 0x8318e688
	sub_8318E688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E990 size=8
    let mut pc: u32 = 0x8318E990;
    'dispatch: loop {
        match pc {
            0x8318E990 => {
    //   block [0x8318E990..0x8318E998)
	// 8318E990: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8318E994: 4BFFFCF4  b 0x8318e688
	sub_8318E688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E998 size=4
    let mut pc: u32 = 0x8318E998;
    'dispatch: loop {
        match pc {
            0x8318E998 => {
    //   block [0x8318E998..0x8318E99C)
	// 8318E998: 48019B78  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E9A0 size=28
    let mut pc: u32 = 0x8318E9A0;
    'dispatch: loop {
        match pc {
            0x8318E9A0 => {
    //   block [0x8318E9A0..0x8318E9BC)
	// 8318E9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E9A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318E9A8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318E9AC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318E9B0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318E9B4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318E9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318E9C0 size=20
    let mut pc: u32 = 0x8318E9C0;
    'dispatch: loop {
        match pc {
            0x8318E9C0 => {
    //   block [0x8318E9C0..0x8318E9D4)
	// 8318E9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318E9C4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318E9C8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318E9CC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318E9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318E9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318E9D8 size=92
    let mut pc: u32 = 0x8318E9D8;
    'dispatch: loop {
        match pc {
            0x8318E9D8 => {
    //   block [0x8318E9D8..0x8318EA34)
	// 8318E9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318E9DC: 48019791  bl 0x831a816c
	ctx.lr = 0x8318E9E0;
	sub_831A8130(ctx, base);
	// 8318E9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318E9E4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318E9E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318E9EC: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 8318E9F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318E9F4: 555E0038  rlwinm r30, r10, 0, 0, 0x1c
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8318E9F8: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8318E9FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318EA00: 7FAB2A14  add r29, r11, r5
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8318EA04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318EA08: 480197D9  bl 0x831a81e0
	ctx.lr = 0x8318EA0C;
	sub_831A81E0(ctx, base);
	// 8318EA0C: 7FAB2670  srawi r11, r29, 4
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 4) as i64;
	// 8318EA10: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8318EA14: 7D4B0194  addze r10, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8318EA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318EA1C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318EA20: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318EA24: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318EA28: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318EA2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EA30: 4801978C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EA38 size=32
    let mut pc: u32 = 0x8318EA38;
    'dispatch: loop {
        match pc {
            0x8318EA38 => {
    //   block [0x8318EA38..0x8318EA58)
	// 8318EA38: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318EA3C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318EA40: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8318EA44: 409A0014  bne cr6, 0x8318ea58
	if !ctx.cr[6].eq {
		sub_8318EA58(ctx, base);
		return;
	}
	// 8318EA48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318EA4C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318EA50: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318EA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EA58 size=100
    let mut pc: u32 = 0x8318EA58;
    'dispatch: loop {
        match pc {
            0x8318EA58 => {
    //   block [0x8318EA58..0x8318EABC)
	// 8318EA58: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318EA5C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318EA60: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318EA64: E8E40000  ld r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8318EA68: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 8318EA6C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8318EA70: F8EA0000  std r7, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8318EA74: E9040008  ld r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8318EA78: F90A0008  std r8, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8318EA7C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318EA80: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8318EA84: 4198000C  blt cr6, 0x8318ea90
	if ctx.cr[6].lt {
	pc = 0x8318EA90; continue 'dispatch;
	}
	// 8318EA88: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8318EA8C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 8318EA90: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318EA94: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8318EA98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318EA9C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8318EAA0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318EAA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318EAA8: 40980008  bge cr6, 0x8318eab0
	if !ctx.cr[6].lt {
	pc = 0x8318EAB0; continue 'dispatch;
	}
	// 8318EAAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318EAB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EAB4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318EAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EAC0 size=184
    let mut pc: u32 = 0x8318EAC0;
    'dispatch: loop {
        match pc {
            0x8318EAC0 => {
    //   block [0x8318EAC0..0x8318EB78)
	// 8318EAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318EAC4: 480196A5  bl 0x831a8168
	ctx.lr = 0x8318EAC8;
	sub_831A8130(ctx, base);
	// 8318EAC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318EACC: 7FE53214  add r31, r5, r6
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 8318EAD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EAD4: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318EAD8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318EADC: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318EAE0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8318EAE4: 4099008C  ble cr6, 0x8318eb70
	if !ctx.cr[6].gt {
	pc = 0x8318EB70; continue 'dispatch;
	}
	// 8318EAE8: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318EAEC: 55092036  slwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8318EAF0: 54FD2036  slwi r29, r7, 4
	ctx.r[29].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8318EAF4: 7D69F214  add r11, r9, r30
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 8318EAF8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318EAFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318EB00: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8318EB04: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8318EB08: 41990010  bgt cr6, 0x8318eb18
	if ctx.cr[6].gt {
	pc = 0x8318EB18; continue 'dispatch;
	}
	// 8318EB0C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8318EB10: 4199002C  bgt cr6, 0x8318eb3c
	if ctx.cr[6].gt {
	pc = 0x8318EB3C; continue 'dispatch;
	}
	// 8318EB14: 48000020  b 0x8318eb34
	pc = 0x8318EB34; continue 'dispatch;
	// 8318EB18: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8318EB1C: 4199000C  bgt cr6, 0x8318eb28
	if ctx.cr[6].gt {
	pc = 0x8318EB28; continue 'dispatch;
	}
	// 8318EB20: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8318EB24: 41980050  blt cr6, 0x8318eb74
	if ctx.cr[6].lt {
	pc = 0x8318EB74; continue 'dispatch;
	}
	// 8318EB28: 7F052040  cmplw cr6, r5, r4
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8318EB2C: 41990010  bgt cr6, 0x8318eb3c
	if ctx.cr[6].gt {
	pc = 0x8318EB3C; continue 'dispatch;
	}
	// 8318EB30: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 8318EB34: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318EB38: 4198003C  blt cr6, 0x8318eb74
	if ctx.cr[6].lt {
	pc = 0x8318EB74; continue 'dispatch;
	}
	// 8318EB3C: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 8318EB40: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8318EB44: 40980010  bge cr6, 0x8318eb54
	if !ctx.cr[6].lt {
	pc = 0x8318EB54; continue 'dispatch;
	}
	// 8318EB48: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8318EB4C: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 8318EB50: 48000014  b 0x8318eb64
	pc = 0x8318EB64; continue 'dispatch;
	// 8318EB54: 7D7D4850  subf r11, r29, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 8318EB58: 7D474050  subf r10, r7, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8318EB5C: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 8318EB60: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 8318EB64: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8318EB68: 7F03E000  cmpw cr6, r3, r28
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8318EB6C: 4198FF88  blt cr6, 0x8318eaf4
	if ctx.cr[6].lt {
	pc = 0x8318EAF4; continue 'dispatch;
	}
	// 8318EB70: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318EB74: 48019644  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EB78 size=40
    let mut pc: u32 = 0x8318EB78;
    'dispatch: loop {
        match pc {
            0x8318EB78 => {
    //   block [0x8318EB78..0x8318EBA0)
	// 8318EB78: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318EB7C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318EB80: 814B13D0  lwz r10, 0x13d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5072 as u32) ) } as u64;
	// 8318EB84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318EB88: 419A0018  beq cr6, 0x8318eba0
	if ctx.cr[6].eq {
		sub_8318EBA0(ctx, base);
		return;
	}
	// 8318EB8C: 814B13D8  lwz r10, 0x13d8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5080 as u32) ) } as u64;
	// 8318EB90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318EB94: 816B13D4  lwz r11, 0x13d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5076 as u32) ) } as u64;
	// 8318EB98: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8318EB9C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EBA0 size=8
    let mut pc: u32 = 0x8318EBA0;
    'dispatch: loop {
        match pc {
            0x8318EBA0 => {
    //   block [0x8318EBA0..0x8318EBA8)
	// 8318EBA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318EBA8 size=104
    let mut pc: u32 = 0x8318EBA8;
    'dispatch: loop {
        match pc {
            0x8318EBA8 => {
    //   block [0x8318EBA8..0x8318EC10)
	// 8318EBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318EBAC: 480195C1  bl 0x831a816c
	ctx.lr = 0x8318EBB0;
	sub_831A8130(ctx, base);
	// 8318EBB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318EBB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318EBB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318EBBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318EBC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8318EBC4: 419A0040  beq cr6, 0x8318ec04
	if ctx.cr[6].eq {
	pc = 0x8318EC04; continue 'dispatch;
	}
	// 8318EBC8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8318EBCC: 40990038  ble cr6, 0x8318ec04
	if !ctx.cr[6].gt {
	pc = 0x8318EC04; continue 'dispatch;
	}
	// 8318EBD0: 4BFF8711  bl 0x831872e0
	ctx.lr = 0x8318EBD4;
	sub_831872E0(ctx, base);
	// 8318EBD4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318EBD8: 419A001C  beq cr6, 0x8318ebf4
	if ctx.cr[6].eq {
	pc = 0x8318EBF4; continue 'dispatch;
	}
	// 8318EBDC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318EBE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EBE4: 60840165  ori r4, r4, 0x165
	ctx.r[4].u64 = ctx.r[4].u64 | 357;
	// 8318EBE8: 4BFF8911  bl 0x831874f8
	ctx.lr = 0x8318EBEC;
	sub_831874F8(ctx, base);
	// 8318EBEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EBF0: 480195CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318EBF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318EBF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318EBFC: 387E1444  addi r3, r30, 0x1444
	ctx.r[3].s64 = ctx.r[30].s64 + 5188;
	// 8318EC00: 4BFFFDD9  bl 0x8318e9d8
	ctx.lr = 0x8318EC04;
	sub_8318E9D8(ctx, base);
	// 8318EC04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EC08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EC0C: 480195B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318EC10 size=148
    let mut pc: u32 = 0x8318EC10;
    'dispatch: loop {
        match pc {
            0x8318EC10 => {
    //   block [0x8318EC10..0x8318ECA4)
	// 8318EC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318EC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318EC18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318EC1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318EC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318EC24: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318EC28: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8318EC2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318EC30: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318EC34: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8318EC38: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 8318EC3C: 41980050  blt cr6, 0x8318ec8c
	if ctx.cr[6].lt {
	pc = 0x8318EC8C; continue 'dispatch;
	}
	// 8318EC40: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 8318EC44: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8318EC48: 386B13D0  addi r3, r11, 0x13d0
	ctx.r[3].s64 = ctx.r[11].s64 + 5072;
	// 8318EC4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318EC50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318EC54: 419A0038  beq cr6, 0x8318ec8c
	if ctx.cr[6].eq {
	pc = 0x8318EC8C; continue 'dispatch;
	}
	// 8318EC58: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318EC5C: 4BFFFDDD  bl 0x8318ea38
	ctx.lr = 0x8318EC60;
	sub_8318EA38(ctx, base);
	// 8318EC60: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8318EC64: 409A0028  bne cr6, 0x8318ec8c
	if !ctx.cr[6].eq {
	pc = 0x8318EC8C; continue 'dispatch;
	}
	// 8318EC68: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318EC6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318EC70: 60840421  ori r4, r4, 0x421
	ctx.r[4].u64 = ctx.r[4].u64 | 1057;
	// 8318EC74: 4BFF8885  bl 0x831874f8
	ctx.lr = 0x8318EC78;
	sub_831874F8(ctx, base);
	// 8318EC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318EC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318EC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318EC84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318EC88: 4E800020  blr
	return;
	// 8318EC8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EC90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318EC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318EC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318EC9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318ECA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318ECA8 size=128
    let mut pc: u32 = 0x8318ECA8;
    'dispatch: loop {
        match pc {
            0x8318ECA8 => {
    //   block [0x8318ECA8..0x8318ED28)
	// 8318ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318ECAC: 480194C1  bl 0x831a816c
	ctx.lr = 0x8318ECB0;
	sub_831A8130(ctx, base);
	// 8318ECB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318ECB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318ECB8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318ECBC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318ECC0: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318ECC4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8318ECC8: 419A0058  beq cr6, 0x8318ed20
	if ctx.cr[6].eq {
	pc = 0x8318ED20; continue 'dispatch;
	}
	// 8318ECCC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8318ECD0: 4BFFFDF1  bl 0x8318eac0
	ctx.lr = 0x8318ECD4;
	sub_8318EAC0(ctx, base);
	// 8318ECD4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8318ECD8: 419A0048  beq cr6, 0x8318ed20
	if ctx.cr[6].eq {
	pc = 0x8318ED20; continue 'dispatch;
	}
	// 8318ECDC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318ECE0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318ECE4: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318ECE8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8318ECEC: 4198000C  blt cr6, 0x8318ecf8
	if ctx.cr[6].lt {
	pc = 0x8318ECF8; continue 'dispatch;
	}
	// 8318ECF0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8318ECF4: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318ECF8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318ECFC: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318ED00: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8318ED04: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8318ED08: 7D23E850  subf r9, r3, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8318ED0C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8318ED10: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8318ED14: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8318ED18: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8318ED1C: F97E0008  std r11, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8318ED20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318ED24: 48019498  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318ED28 size=108
    let mut pc: u32 = 0x8318ED28;
    'dispatch: loop {
        match pc {
            0x8318ED28 => {
    //   block [0x8318ED28..0x8318ED94)
	// 8318ED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318ED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318ED30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318ED34: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8318ED38: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8318ED3C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8318ED40: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318ED44: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 8318ED48: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8318ED4C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8318ED50: F9250000  std r9, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8318ED54: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318ED58: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318ED5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318ED60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318ED64: 419A001C  beq cr6, 0x8318ed80
	if ctx.cr[6].eq {
	pc = 0x8318ED80; continue 'dispatch;
	}
	// 8318ED68: 7D673214  add r11, r7, r6
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 8318ED6C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318ED70: 41980008  blt cr6, 0x8318ed78
	if ctx.cr[6].lt {
	pc = 0x8318ED78; continue 'dispatch;
	}
	// 8318ED74: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 8318ED78: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8318ED7C: 4BFFFF2D  bl 0x8318eca8
	ctx.lr = 0x8318ED80;
	sub_8318ECA8(ctx, base);
	// 8318ED80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318ED84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318ED88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318ED8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318ED90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318ED98 size=104
    let mut pc: u32 = 0x8318ED98;
    'dispatch: loop {
        match pc {
            0x8318ED98 => {
    //   block [0x8318ED98..0x8318EE00)
	// 8318ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318EDA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318EDA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318EDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318EDAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318EDB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318EDB4: 4BFFD53D  bl 0x8318c2f0
	ctx.lr = 0x8318EDB8;
	sub_8318C2F0(ctx, base);
	// 8318EDB8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318EDBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EDC0: 419A0014  beq cr6, 0x8318edd4
	if ctx.cr[6].eq {
	pc = 0x8318EDD4; continue 'dispatch;
	}
	// 8318EDC4: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318EDC8: 60840201  ori r4, r4, 0x201
	ctx.r[4].u64 = ctx.r[4].u64 | 513;
	// 8318EDCC: 4BFFD605  bl 0x8318c3d0
	ctx.lr = 0x8318EDD0;
	sub_8318C3D0(ctx, base);
	// 8318EDD0: 48000018  b 0x8318ede8
	pc = 0x8318EDE8; continue 'dispatch;
	// 8318EDD4: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8318EDD8: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8318EDDC: F95F0000  std r10, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8318EDE0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8318EDE4: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8318EDE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318EDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318EDF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318EDF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318EE00 size=116
    let mut pc: u32 = 0x8318EE00;
    'dispatch: loop {
        match pc {
            0x8318EE00 => {
    //   block [0x8318EE00..0x8318EE74)
	// 8318EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318EE04: 48019369  bl 0x831a816c
	ctx.lr = 0x8318EE08;
	sub_831A8130(ctx, base);
	// 8318EE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318EE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318EE10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318EE14: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318EE18: 4BFFD4D9  bl 0x8318c2f0
	ctx.lr = 0x8318EE1C;
	sub_8318C2F0(ctx, base);
	// 8318EE1C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318EE20: 419A001C  beq cr6, 0x8318ee3c
	if ctx.cr[6].eq {
	pc = 0x8318EE3C; continue 'dispatch;
	}
	// 8318EE24: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318EE28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EE2C: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 8318EE30: 4BFFD5A1  bl 0x8318c3d0
	ctx.lr = 0x8318EE34;
	sub_8318C3D0(ctx, base);
	// 8318EE34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EE38: 48019384  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318EE3C: 57AB2834  slwi r11, r29, 5
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318EE40: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8318EE44: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8318EE48: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8318EE4C: 396B0048  addi r11, r11, 0x48
	ctx.r[11].s64 = ctx.r[11].s64 + 72;
	// 8318EE50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318EE54: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318EE58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318EE5C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318EE60: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318EE64: 4200FFF0  bdnz 0x8318ee54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318EE54; continue 'dispatch;
	}
	// 8318EE68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EE6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EE70: 4801934C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318EE78 size=124
    let mut pc: u32 = 0x8318EE78;
    'dispatch: loop {
        match pc {
            0x8318EE78 => {
    //   block [0x8318EE78..0x8318EEF4)
	// 8318EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318EE80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318EE84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318EE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318EE8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318EE90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318EE94: 4BFFD45D  bl 0x8318c2f0
	ctx.lr = 0x8318EE98;
	sub_8318C2F0(ctx, base);
	// 8318EE98: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318EE9C: 419A0018  beq cr6, 0x8318eeb4
	if ctx.cr[6].eq {
	pc = 0x8318EEB4; continue 'dispatch;
	}
	// 8318EEA0: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318EEA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EEA8: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 8318EEAC: 4BFFD525  bl 0x8318c3d0
	ctx.lr = 0x8318EEB0;
	sub_8318C3D0(ctx, base);
	// 8318EEB0: 4800002C  b 0x8318eedc
	pc = 0x8318EEDC; continue 'dispatch;
	// 8318EEB4: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 8318EEB8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8318EEBC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8318EEC0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318EEC4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318EEC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318EECC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318EED0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8318EED4: 4200FFF0  bdnz 0x8318eec4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318EEC4; continue 'dispatch;
	}
	// 8318EED8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EEDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318EEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318EEE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318EEEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318EEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318EEF8 size=124
    let mut pc: u32 = 0x8318EEF8;
    'dispatch: loop {
        match pc {
            0x8318EEF8 => {
    //   block [0x8318EEF8..0x8318EF74)
	// 8318EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318EF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318EF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318EF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318EF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318EF10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318EF14: 4BFFD3DD  bl 0x8318c2f0
	ctx.lr = 0x8318EF18;
	sub_8318C2F0(ctx, base);
	// 8318EF18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318EF1C: 419A0018  beq cr6, 0x8318ef34
	if ctx.cr[6].eq {
	pc = 0x8318EF34; continue 'dispatch;
	}
	// 8318EF20: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8318EF24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EF28: 60840203  ori r4, r4, 0x203
	ctx.r[4].u64 = ctx.r[4].u64 | 515;
	// 8318EF2C: 4BFFD4A5  bl 0x8318c3d0
	ctx.lr = 0x8318EF30;
	sub_8318C3D0(ctx, base);
	// 8318EF30: 4800002C  b 0x8318ef5c
	pc = 0x8318EF5C; continue 'dispatch;
	// 8318EF34: 397F00A8  addi r11, r31, 0xa8
	ctx.r[11].s64 = ctx.r[31].s64 + 168;
	// 8318EF38: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8318EF3C: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8318EF40: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318EF44: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8318EF48: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8318EF4C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8318EF50: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8318EF54: 4200FFF0  bdnz 0x8318ef44
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318EF44; continue 'dispatch;
	}
	// 8318EF58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EF5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318EF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318EF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318EF68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318EF6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318EF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EF78 size=80
    let mut pc: u32 = 0x8318EF78;
    'dispatch: loop {
        match pc {
            0x8318EF78 => {
    //   block [0x8318EF78..0x8318EFC8)
	// 8318EF78: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318EF7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318EF80: 409A0060  bne cr6, 0x8318efe0
	if !ctx.cr[6].eq {
		sub_8318EFE0(ctx, base);
		return;
	}
	// 8318EF84: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8318EF88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318EF8C: 409A0054  bne cr6, 0x8318efe0
	if !ctx.cr[6].eq {
		sub_8318EFE0(ctx, base);
		return;
	}
	// 8318EF90: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 8318EF94: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8318EF98: 409A0048  bne cr6, 0x8318efe0
	if !ctx.cr[6].eq {
		sub_8318EFE0(ctx, base);
		return;
	}
	// 8318EF9C: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 8318EFA0: 2F0B00B9  cmpwi cr6, r11, 0xb9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 185, &mut ctx.xer);
	// 8318EFA4: 419A0034  beq cr6, 0x8318efd8
	if ctx.cr[6].eq {
		sub_8318EFD8(ctx, base);
		return;
	}
	// 8318EFA8: 2F0B00BA  cmpwi cr6, r11, 0xba
	ctx.cr[6].compare_i32(ctx.r[11].s32, 186, &mut ctx.xer);
	// 8318EFAC: 419A0024  beq cr6, 0x8318efd0
	if ctx.cr[6].eq {
		sub_8318EFD0(ctx, base);
		return;
	}
	// 8318EFB0: 2F0B00BB  cmpwi cr6, r11, 0xbb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 187, &mut ctx.xer);
	// 8318EFB4: 419A0014  beq cr6, 0x8318efc8
	if ctx.cr[6].eq {
		sub_8318EFC8(ctx, base);
		return;
	}
	// 8318EFB8: 2B0B00BC  cmplwi cr6, r11, 0xbc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 188 as u32, &mut ctx.xer);
	// 8318EFBC: 41980024  blt cr6, 0x8318efe0
	if ctx.cr[6].lt {
		sub_8318EFE0(ctx, base);
		return;
	}
	// 8318EFC0: 3C600004  lis r3, 4
	ctx.r[3].s64 = 262144;
	// 8318EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EFC8 size=8
    let mut pc: u32 = 0x8318EFC8;
    'dispatch: loop {
        match pc {
            0x8318EFC8 => {
    //   block [0x8318EFC8..0x8318EFD0)
	// 8318EFC8: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 8318EFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EFD0 size=8
    let mut pc: u32 = 0x8318EFD0;
    'dispatch: loop {
        match pc {
            0x8318EFD0 => {
    //   block [0x8318EFD0..0x8318EFD8)
	// 8318EFD0: 3C600001  lis r3, 1
	ctx.r[3].s64 = 65536;
	// 8318EFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EFD8 size=8
    let mut pc: u32 = 0x8318EFD8;
    'dispatch: loop {
        match pc {
            0x8318EFD8 => {
    //   block [0x8318EFD8..0x8318EFE0)
	// 8318EFD8: 3C600008  lis r3, 8
	ctx.r[3].s64 = 524288;
	// 8318EFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EFE0 size=8
    let mut pc: u32 = 0x8318EFE0;
    'dispatch: loop {
        match pc {
            0x8318EFE0 => {
    //   block [0x8318EFE0..0x8318EFE8)
	// 8318EFE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318EFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EFE8 size=16
    let mut pc: u32 = 0x8318EFE8;
    'dispatch: loop {
        match pc {
            0x8318EFE8 => {
    //   block [0x8318EFE8..0x8318EFF8)
	// 8318EFE8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318EFEC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8318EFF0: 388BD7E8  addi r4, r11, -0x2818
	ctx.r[4].s64 = ctx.r[11].s64 + -10264;
	// 8318EFF4: 4BFF9D84  b 0x83188d78
	sub_83188D78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318EFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318EFF8 size=4
    let mut pc: u32 = 0x8318EFF8;
    'dispatch: loop {
        match pc {
            0x8318EFF8 => {
    //   block [0x8318EFF8..0x8318EFFC)
	// 8318EFF8: 4BFF9DE8  b 0x83188de0
	sub_83188DE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F000 size=28
    let mut pc: u32 = 0x8318F000;
    'dispatch: loop {
        match pc {
            0x8318F000 => {
    //   block [0x8318F000..0x8318F01C)
	// 8318F000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F004: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318F008: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318F00C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318F010: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318F014: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8318F018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F020 size=20
    let mut pc: u32 = 0x8318F020;
    'dispatch: loop {
        match pc {
            0x8318F020 => {
    //   block [0x8318F020..0x8318F034)
	// 8318F020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F024: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318F028: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318F02C: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8318F030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F038 size=48
    let mut pc: u32 = 0x8318F038;
    'dispatch: loop {
        match pc {
            0x8318F038 => {
    //   block [0x8318F038..0x8318F068)
	// 8318F038: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8318F03C: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318F040: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 8318F044: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318F048: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318F04C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8318F050: 896B0002  lbz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8318F054: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318F058: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8318F05C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8318F060: 7D435B78  or r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8318F064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F068 size=112
    let mut pc: u32 = 0x8318F068;
    'dispatch: loop {
        match pc {
            0x8318F068 => {
    //   block [0x8318F068..0x8318F0D8)
	// 8318F068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F074: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F078: 4BFF9D91  bl 0x83188e08
	ctx.lr = 0x8318F07C;
	sub_83188E08(ctx, base);
	// 8318F07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F080: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8318F084: 409A0018  bne cr6, 0x8318f09c
	if !ctx.cr[6].eq {
	pc = 0x8318F09C; continue 'dispatch;
	}
	// 8318F088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F098: 4E800020  blr
	return;
	// 8318F09C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318F0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F0A4: 4BFF9E15  bl 0x83188eb8
	ctx.lr = 0x8318F0A8;
	sub_83188EB8(ctx, base);
	// 8318F0A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F0AC: 409A000C  bne cr6, 0x8318f0b8
	if !ctx.cr[6].eq {
	pc = 0x8318F0B8; continue 'dispatch;
	}
	// 8318F0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F0B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318F0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F0BC: 4BFF962D  bl 0x831886e8
	ctx.lr = 0x8318F0C0;
	sub_831886E8(ctx, base);
	// 8318F0C0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318F0C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F0D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F0D8 size=8
    let mut pc: u32 = 0x8318F0D8;
    'dispatch: loop {
        match pc {
            0x8318F0D8 => {
    //   block [0x8318F0D8..0x8318F0E0)
	// 8318F0D8: 38A00894  li r5, 0x894
	ctx.r[5].s64 = 2196;
	// 8318F0DC: 48019434  b 0x831a8510
	sub_831A8510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F0E0 size=100
    let mut pc: u32 = 0x8318F0E0;
    'dispatch: loop {
        match pc {
            0x8318F0E0 => {
    //   block [0x8318F0E0..0x8318F144)
	// 8318F0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F0E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F0EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F0F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F0F4: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 8318F0F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318F0FC: 409A001C  bne cr6, 0x8318f118
	if !ctx.cr[6].eq {
	pc = 0x8318F118; continue 'dispatch;
	}
	// 8318F100: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318F104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318F108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F114: 4E800020  blr
	return;
	// 8318F118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F11C: 4BFEB06D  bl 0x8317a188
	ctx.lr = 0x8318F120;
	sub_8317A188(ctx, base);
	// 8318F120: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F124: 4199FFDC  bgt cr6, 0x8318f100
	if ctx.cr[6].gt {
	pc = 0x8318F100; continue 'dispatch;
	}
	// 8318F128: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 8318F12C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8318F130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318F134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F13C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F148 size=104
    let mut pc: u32 = 0x8318F148;
    'dispatch: loop {
        match pc {
            0x8318F148 => {
    //   block [0x8318F148..0x8318F1B0)
	// 8318F148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F14C: 48019021  bl 0x831a816c
	ctx.lr = 0x8318F150;
	sub_831A8130(ctx, base);
	// 8318F150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F154: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8318F158: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8318F15C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318F160: 7F04F000  cmpw cr6, r4, r30
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8318F164: 41990034  bgt cr6, 0x8318f198
	if ctx.cr[6].gt {
	pc = 0x8318F198; continue 'dispatch;
	}
	// 8318F168: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318F16C: 57E4063E  clrlwi r4, r31, 0x18
	ctx.r[4].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 8318F170: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318F174: 4BFF95AD  bl 0x83188720
	ctx.lr = 0x8318F178;
	sub_83188720(ctx, base);
	// 8318F178: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F17C: 419A0010  beq cr6, 0x8318f18c
	if ctx.cr[6].eq {
	pc = 0x8318F18C; continue 'dispatch;
	}
	// 8318F180: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318F184: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318F188: 409A001C  bne cr6, 0x8318f1a4
	if !ctx.cr[6].eq {
	pc = 0x8318F1A4; continue 'dispatch;
	}
	// 8318F18C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8318F190: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8318F194: 4099FFD4  ble cr6, 0x8318f168
	if !ctx.cr[6].gt {
	pc = 0x8318F168; continue 'dispatch;
	}
	// 8318F198: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318F19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318F1A0: 4801901C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318F1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F1A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318F1AC: 48019010  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F1B0 size=60
    let mut pc: u32 = 0x8318F1B0;
    'dispatch: loop {
        match pc {
            0x8318F1B0 => {
    //   block [0x8318F1B0..0x8318F1EC)
	// 8318F1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F1BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318F1C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318F1C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318F1C8: 4E800421  bctrl
	ctx.lr = 0x8318F1CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318F1CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F1D0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318F1D4: 419A0008  beq cr6, 0x8318f1dc
	if ctx.cr[6].eq {
	pc = 0x8318F1DC; continue 'dispatch;
	}
	// 8318F1D8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318F1DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318F1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F1F0 size=64
    let mut pc: u32 = 0x8318F1F0;
    'dispatch: loop {
        match pc {
            0x8318F1F0 => {
    //   block [0x8318F1F0..0x8318F230)
	// 8318F1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F1F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F1FC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8318F200: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318F204: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8318F208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318F20C: 4E800421  bctrl
	ctx.lr = 0x8318F210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318F210: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F214: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318F218: 419A0008  beq cr6, 0x8318f220
	if ctx.cr[6].eq {
	pc = 0x8318F220; continue 'dispatch;
	}
	// 8318F21C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318F220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318F224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F230 size=28
    let mut pc: u32 = 0x8318F230;
    'dispatch: loop {
        match pc {
            0x8318F230 => {
    //   block [0x8318F230..0x8318F24C)
	// 8318F230: 816300A0  lwz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 8318F234: 91630934  stw r11, 0x934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2356 as u32), ctx.r[11].u32 ) };
	// 8318F238: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8318F23C: 91630938  stw r11, 0x938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2360 as u32), ctx.r[11].u32 ) };
	// 8318F240: 816300A8  lwz r11, 0xa8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 8318F244: 9163093C  stw r11, 0x93c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2364 as u32), ctx.r[11].u32 ) };
	// 8318F248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F250 size=20
    let mut pc: u32 = 0x8318F250;
    'dispatch: loop {
        match pc {
            0x8318F250 => {
    //   block [0x8318F250..0x8318F264)
	// 8318F250: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 8318F254: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318F258: 409A000C  bne cr6, 0x8318f264
	if !ctx.cr[6].eq {
		sub_8318F264(ctx, base);
		return;
	}
	// 8318F25C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318F260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F264 size=20
    let mut pc: u32 = 0x8318F264;
    'dispatch: loop {
        match pc {
            0x8318F264 => {
    //   block [0x8318F264..0x8318F278)
	// 8318F264: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 8318F268: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 8318F26C: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 8318F270: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8318F274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F278 size=20
    let mut pc: u32 = 0x8318F278;
    'dispatch: loop {
        match pc {
            0x8318F278 => {
    //   block [0x8318F278..0x8318F28C)
	// 8318F278: 3963007C  addi r11, r3, 0x7c
	ctx.r[11].s64 = ctx.r[3].s64 + 124;
	// 8318F27C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318F280: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318F284: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318F288: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F28C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F28C size=12
    let mut pc: u32 = 0x8318F28C;
    'dispatch: loop {
        match pc {
            0x8318F28C => {
    //   block [0x8318F28C..0x8318F298)
	// 8318F28C: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8318F290: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318F294: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318F298 size=8
    let mut pc: u32 = 0x8318F298;
    'dispatch: loop {
        match pc {
            0x8318F298 => {
    //   block [0x8318F298..0x8318F2A0)
	// 8318F298: 806B0074  lwz r3, 0x74(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 8318F29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F2A0 size=44
    let mut pc: u32 = 0x8318F2A0;
    'dispatch: loop {
        match pc {
            0x8318F2A0 => {
    //   block [0x8318F2A0..0x8318F2CC)
	// 8318F2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F2A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F2AC: 4BFFFD8D  bl 0x8318f038
	ctx.lr = 0x8318F2B0;
	sub_8318F038(ctx, base);
	// 8318F2B0: 3963FE41  addi r11, r3, -0x1bf
	ctx.r[11].s64 = ctx.r[3].s64 + -447;
	// 8318F2B4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318F2B8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318F2BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318F2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F2D0 size=60
    let mut pc: u32 = 0x8318F2D0;
    'dispatch: loop {
        match pc {
            0x8318F2D0 => {
    //   block [0x8318F2D0..0x8318F30C)
	// 8318F2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F2DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F2E4: 4BFFFDFD  bl 0x8318f0e0
	ctx.lr = 0x8318F2E8;
	sub_8318F0E0(ctx, base);
	// 8318F2E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318F2EC: 419A000C  beq cr6, 0x8318f2f8
	if ctx.cr[6].eq {
	pc = 0x8318F2F8; continue 'dispatch;
	}
	// 8318F2F0: 389F007C  addi r4, r31, 0x7c
	ctx.r[4].s64 = ctx.r[31].s64 + 124;
	// 8318F2F4: 4BFFFDE5  bl 0x8318f0d8
	ctx.lr = 0x8318F2F8;
	sub_8318F0D8(ctx, base);
	// 8318F2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318F2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F310 size=152
    let mut pc: u32 = 0x8318F310;
    'dispatch: loop {
        match pc {
            0x8318F310 => {
    //   block [0x8318F310..0x8318F3A8)
	// 8318F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318F31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F324: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F328: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318F32C: 388B8798  addi r4, r11, -0x7868
	ctx.r[4].s64 = ctx.r[11].s64 + -30824;
	// 8318F330: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318F334: 4BFFFE7D  bl 0x8318f1b0
	ctx.lr = 0x8318F338;
	sub_8318F1B0(ctx, base);
	// 8318F338: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F33C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F344: 388B87C0  addi r4, r11, -0x7840
	ctx.r[4].s64 = ctx.r[11].s64 + -30784;
	// 8318F348: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8318F34C: 4BFFFE65  bl 0x8318f1b0
	ctx.lr = 0x8318F350;
	sub_8318F1B0(ctx, base);
	// 8318F350: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F354: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F35C: 388B87E8  addi r4, r11, -0x7818
	ctx.r[4].s64 = ctx.r[11].s64 + -30744;
	// 8318F360: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8318F364: 4BFFFE4D  bl 0x8318f1b0
	ctx.lr = 0x8318F368;
	sub_8318F1B0(ctx, base);
	// 8318F368: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8318F36C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8318F370: 409A000C  bne cr6, 0x8318f37c
	if !ctx.cr[6].eq {
	pc = 0x8318F37C; continue 'dispatch;
	}
	// 8318F374: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8318F378: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318F37C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F384: 388B8810  addi r4, r11, -0x77f0
	ctx.r[4].s64 = ctx.r[11].s64 + -30704;
	// 8318F388: 4BFFFE29  bl 0x8318f1b0
	ctx.lr = 0x8318F38C;
	sub_8318F1B0(ctx, base);
	// 8318F38C: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8318F390: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F39C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318F3A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F3A8 size=212
    let mut pc: u32 = 0x8318F3A8;
    'dispatch: loop {
        match pc {
            0x8318F3A8 => {
    //   block [0x8318F3A8..0x8318F47C)
	// 8318F3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F3B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318F3B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F3BC: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F3C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318F3C4: 388B8838  addi r4, r11, -0x77c8
	ctx.r[4].s64 = ctx.r[11].s64 + -30664;
	// 8318F3C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F3CC: 4BFFFDE5  bl 0x8318f1b0
	ctx.lr = 0x8318F3D0;
	sub_8318F1B0(ctx, base);
	// 8318F3D0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F3D4: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F3D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F3DC: 388B8860  addi r4, r11, -0x77a0
	ctx.r[4].s64 = ctx.r[11].s64 + -30624;
	// 8318F3E0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318F3E4: 4BFFFDCD  bl 0x8318f1b0
	ctx.lr = 0x8318F3E8;
	sub_8318F1B0(ctx, base);
	// 8318F3E8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F3EC: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F3F4: 388B8888  addi r4, r11, -0x7778
	ctx.r[4].s64 = ctx.r[11].s64 + -30584;
	// 8318F3F8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318F3FC: 4BFFFDB5  bl 0x8318f1b0
	ctx.lr = 0x8318F400;
	sub_8318F1B0(ctx, base);
	// 8318F400: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F404: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F40C: 388B88B0  addi r4, r11, -0x7750
	ctx.r[4].s64 = ctx.r[11].s64 + -30544;
	// 8318F410: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318F414: 4BFFFD9D  bl 0x8318f1b0
	ctx.lr = 0x8318F418;
	sub_8318F1B0(ctx, base);
	// 8318F418: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F41C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F424: 388B8900  addi r4, r11, -0x7700
	ctx.r[4].s64 = ctx.r[11].s64 + -30464;
	// 8318F428: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8318F42C: 4BFFFD85  bl 0x8318f1b0
	ctx.lr = 0x8318F430;
	sub_8318F1B0(ctx, base);
	// 8318F430: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F434: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F43C: 388B8928  addi r4, r11, -0x76d8
	ctx.r[4].s64 = ctx.r[11].s64 + -30424;
	// 8318F440: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8318F444: 4BFFFD6D  bl 0x8318f1b0
	ctx.lr = 0x8318F448;
	sub_8318F1B0(ctx, base);
	// 8318F448: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F44C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F454: 388B8950  addi r4, r11, -0x76b0
	ctx.r[4].s64 = ctx.r[11].s64 + -30384;
	// 8318F458: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8318F45C: 4BFFFD55  bl 0x8318f1b0
	ctx.lr = 0x8318F460;
	sub_8318F1B0(ctx, base);
	// 8318F460: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8318F464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F470: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318F474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F480 size=140
    let mut pc: u32 = 0x8318F480;
    'dispatch: loop {
        match pc {
            0x8318F480 => {
    //   block [0x8318F480..0x8318F50C)
	// 8318F480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F484: 48018CE9  bl 0x831a816c
	ctx.lr = 0x8318F488;
	sub_831A8130(ctx, base);
	// 8318F488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F48C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318F490: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318F494: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318F498: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8318F49C: 419A0068  beq cr6, 0x8318f504
	if ctx.cr[6].eq {
	pc = 0x8318F504; continue 'dispatch;
	}
	// 8318F4A0: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F4A4: 38AB89A0  addi r5, r11, -0x7660
	ctx.r[5].s64 = ctx.r[11].s64 + -30304;
	// 8318F4A8: 4BFFFD49  bl 0x8318f1f0
	ctx.lr = 0x8318F4AC;
	sub_8318F1F0(ctx, base);
	// 8318F4AC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F4B0: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F4B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318F4B8: 38AB89C8  addi r5, r11, -0x7638
	ctx.r[5].s64 = ctx.r[11].s64 + -30264;
	// 8318F4BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F4C0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318F4C4: 4BFFFD2D  bl 0x8318f1f0
	ctx.lr = 0x8318F4C8;
	sub_8318F1F0(ctx, base);
	// 8318F4C8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F4CC: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F4D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318F4D4: 38AB89F0  addi r5, r11, -0x7610
	ctx.r[5].s64 = ctx.r[11].s64 + -30224;
	// 8318F4D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F4DC: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318F4E0: 4BFFFD11  bl 0x8318f1f0
	ctx.lr = 0x8318F4E4;
	sub_8318F1F0(ctx, base);
	// 8318F4E4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F4E8: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F4EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318F4F0: 38AB8A18  addi r5, r11, -0x75e8
	ctx.r[5].s64 = ctx.r[11].s64 + -30184;
	// 8318F4F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F4F8: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318F4FC: 4BFFFCF5  bl 0x8318f1f0
	ctx.lr = 0x8318F500;
	sub_8318F1F0(ctx, base);
	// 8318F500: 907D000C  stw r3, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8318F504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F508: 48018CB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F510 size=416
    let mut pc: u32 = 0x8318F510;
    'dispatch: loop {
        match pc {
            0x8318F510 => {
    //   block [0x8318F510..0x8318F6B0)
	// 8318F510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F514: 48018C4D  bl 0x831a8160
	ctx.lr = 0x8318F518;
	sub_831A8130(ctx, base);
	// 8318F518: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F51C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F520: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8318F524: 38AB8A40  addi r5, r11, -0x75c0
	ctx.r[5].s64 = ctx.r[11].s64 + -30144;
	// 8318F528: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318F52C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8318F530: 4BFFFCC1  bl 0x8318f1f0
	ctx.lr = 0x8318F534;
	sub_8318F1F0(ctx, base);
	// 8318F534: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F538: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F53C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F540: 38AB8A68  addi r5, r11, -0x7598
	ctx.r[5].s64 = ctx.r[11].s64 + -30104;
	// 8318F544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F548: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318F54C: 4BFFFCA5  bl 0x8318f1f0
	ctx.lr = 0x8318F550;
	sub_8318F1F0(ctx, base);
	// 8318F550: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F554: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 8318F558: 3B7F0008  addi r27, r31, 8
	ctx.r[27].s64 = ctx.r[31].s64 + 8;
	// 8318F55C: 57BA063E  clrlwi r26, r29, 0x18
	ctx.r[26].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8318F560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F564: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8318F568: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318F56C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8318F570: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8318F574: 4BFF951D  bl 0x83188a90
	ctx.lr = 0x8318F578;
	sub_83188A90(ctx, base);
	// 8318F578: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F57C: 409A0010  bne cr6, 0x8318f58c
	if !ctx.cr[6].eq {
	pc = 0x8318F58C; continue 'dispatch;
	}
	// 8318F580: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8318F584: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318F588: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318F58C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F590: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F594: 38AB8AB8  addi r5, r11, -0x7548
	ctx.r[5].s64 = ctx.r[11].s64 + -30024;
	// 8318F598: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F59C: 4BFFFC55  bl 0x8318f1f0
	ctx.lr = 0x8318F5A0;
	sub_8318F1F0(ctx, base);
	// 8318F5A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F5A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318F5A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8318F5AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F5B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318F5B4: 4BFF9195  bl 0x83188748
	ctx.lr = 0x8318F5B8;
	sub_83188748(ctx, base);
	// 8318F5B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F5BC: 409A0010  bne cr6, 0x8318f5cc
	if !ctx.cr[6].eq {
	pc = 0x8318F5CC; continue 'dispatch;
	}
	// 8318F5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F5C4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318F5C8: 48000008  b 0x8318f5d0
	pc = 0x8318F5D0; continue 'dispatch;
	// 8318F5CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318F5D0: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8318F5D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318F5D8: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8318F5DC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8318F5E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318F5E4: 419A00C4  beq cr6, 0x8318f6a8
	if ctx.cr[6].eq {
	pc = 0x8318F6A8; continue 'dispatch;
	}
	// 8318F5E8: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F5EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F5F0: 38AB8AE0  addi r5, r11, -0x7520
	ctx.r[5].s64 = ctx.r[11].s64 + -29984;
	// 8318F5F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F5F8: 4BFFFBF9  bl 0x8318f1f0
	ctx.lr = 0x8318F5FC;
	sub_8318F1F0(ctx, base);
	// 8318F5FC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F600: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F604: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F608: 38AB8B08  addi r5, r11, -0x74f8
	ctx.r[5].s64 = ctx.r[11].s64 + -29944;
	// 8318F60C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F610: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8318F614: 4BFFFBDD  bl 0x8318f1f0
	ctx.lr = 0x8318F618;
	sub_8318F1F0(ctx, base);
	// 8318F618: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F61C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F620: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F624: 38AB8B30  addi r5, r11, -0x74d0
	ctx.r[5].s64 = ctx.r[11].s64 + -29904;
	// 8318F628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F62C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8318F630: 4BFFFBC1  bl 0x8318f1f0
	ctx.lr = 0x8318F634;
	sub_8318F1F0(ctx, base);
	// 8318F634: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F638: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F63C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F640: 38AB8B58  addi r5, r11, -0x74a8
	ctx.r[5].s64 = ctx.r[11].s64 + -29864;
	// 8318F644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F648: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8318F64C: 4BFFFBA5  bl 0x8318f1f0
	ctx.lr = 0x8318F650;
	sub_8318F1F0(ctx, base);
	// 8318F650: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F654: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F658: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F65C: 38AB8B80  addi r5, r11, -0x7480
	ctx.r[5].s64 = ctx.r[11].s64 + -29824;
	// 8318F660: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F664: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8318F668: 4BFFFB89  bl 0x8318f1f0
	ctx.lr = 0x8318F66C;
	sub_8318F1F0(ctx, base);
	// 8318F66C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F670: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F674: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F678: 38AB8BA8  addi r5, r11, -0x7458
	ctx.r[5].s64 = ctx.r[11].s64 + -29784;
	// 8318F67C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F680: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8318F684: 4BFFFB6D  bl 0x8318f1f0
	ctx.lr = 0x8318F688;
	sub_8318F1F0(ctx, base);
	// 8318F688: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8318F68C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 8318F690: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318F694: 38AB8BD0  addi r5, r11, -0x7430
	ctx.r[5].s64 = ctx.r[11].s64 + -29744;
	// 8318F698: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F69C: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8318F6A0: 4BFFFB51  bl 0x8318f1f0
	ctx.lr = 0x8318F6A4;
	sub_8318F1F0(ctx, base);
	// 8318F6A4: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 8318F6A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318F6AC: 48018B04  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F6B0 size=340
    let mut pc: u32 = 0x8318F6B0;
    'dispatch: loop {
        match pc {
            0x8318F6B0 => {
    //   block [0x8318F6B0..0x8318F804)
	// 8318F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F6B4: 48018AB9  bl 0x831a816c
	ctx.lr = 0x8318F6B8;
	sub_831A8130(ctx, base);
	// 8318F6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F6BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8318F6C0: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8318F6C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318F6C8: 4BFF97F1  bl 0x83188eb8
	ctx.lr = 0x8318F6CC;
	sub_83188EB8(ctx, base);
	// 8318F6CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F6D0: 419A012C  beq cr6, 0x8318f7fc
	if ctx.cr[6].eq {
	pc = 0x8318F7FC; continue 'dispatch;
	}
	// 8318F6D4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318F6D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318F6DC: 419A0120  beq cr6, 0x8318f7fc
	if ctx.cr[6].eq {
	pc = 0x8318F7FC; continue 'dispatch;
	}
	// 8318F6E0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8318F6E4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8318F6E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F6EC: 4BFF9085  bl 0x83188770
	ctx.lr = 0x8318F6F0;
	sub_83188770(ctx, base);
	// 8318F6F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F6F4: 409A0018  bne cr6, 0x8318f70c
	if !ctx.cr[6].eq {
	pc = 0x8318F70C; continue 'dispatch;
	}
	// 8318F6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318F6FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F700: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8318F704: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8318F708: 4800000C  b 0x8318f714
	pc = 0x8318F714; continue 'dispatch;
	// 8318F70C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318F710: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318F714: 1D2A0064  mulli r9, r10, 0x64
	ctx.r[9].s64 = ctx.r[10].s64 * 100;
	// 8318F718: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318F71C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8318F720: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318F724: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F728: 7FA95A14  add r29, r9, r11
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8318F72C: 4BFF91AD  bl 0x831888d8
	ctx.lr = 0x8318F730;
	sub_831888D8(ctx, base);
	// 8318F730: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F734: 409A0010  bne cr6, 0x8318f744
	if !ctx.cr[6].eq {
	pc = 0x8318F744; continue 'dispatch;
	}
	// 8318F738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F73C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318F740: 48000008  b 0x8318f748
	pc = 0x8318F748; continue 'dispatch;
	// 8318F744: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318F748: 2F1D006E  cmpwi cr6, r29, 0x6e
	ctx.cr[6].compare_i32(ctx.r[29].s32, 110, &mut ctx.xer);
	// 8318F74C: 4098000C  bge cr6, 0x8318f758
	if !ctx.cr[6].lt {
	pc = 0x8318F758; continue 'dispatch;
	}
	// 8318F750: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8318F754: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318F758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318F75C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318F760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F764: 4BFFFBAD  bl 0x8318f310
	ctx.lr = 0x8318F768;
	sub_8318F310(ctx, base);
	// 8318F768: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8318F76C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F770: 4BFFFC39  bl 0x8318f3a8
	ctx.lr = 0x8318F774;
	sub_8318F3A8(ctx, base);
	// 8318F774: 38A000BD  li r5, 0xbd
	ctx.r[5].s64 = 189;
	// 8318F778: 388000BD  li r4, 0xbd
	ctx.r[4].s64 = 189;
	// 8318F77C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F780: 4BFFF9C9  bl 0x8318f148
	ctx.lr = 0x8318F784;
	sub_8318F148(ctx, base);
	// 8318F784: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F788: 38A000BF  li r5, 0xbf
	ctx.r[5].s64 = 191;
	// 8318F78C: 388000BF  li r4, 0xbf
	ctx.r[4].s64 = 191;
	// 8318F790: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F794: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8318F798: 4BFFF9B1  bl 0x8318f148
	ctx.lr = 0x8318F79C;
	sub_8318F148(ctx, base);
	// 8318F79C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F7A0: 38A000DF  li r5, 0xdf
	ctx.r[5].s64 = 223;
	// 8318F7A4: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 8318F7A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F7AC: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8318F7B0: 4BFFF999  bl 0x8318f148
	ctx.lr = 0x8318F7B4;
	sub_8318F148(ctx, base);
	// 8318F7B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F7B8: 38A000EF  li r5, 0xef
	ctx.r[5].s64 = 239;
	// 8318F7BC: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 8318F7C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F7C4: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8318F7C8: 4BFFF981  bl 0x8318f148
	ctx.lr = 0x8318F7CC;
	sub_8318F148(ctx, base);
	// 8318F7CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F7D0: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318F7D4: 38BF004C  addi r5, r31, 0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + 76;
	// 8318F7D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F7DC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8318F7E0: 4BFFFCA1  bl 0x8318f480
	ctx.lr = 0x8318F7E4;
	sub_8318F480(ctx, base);
	// 8318F7E4: 38BF005C  addi r5, r31, 0x5c
	ctx.r[5].s64 = ctx.r[31].s64 + 92;
	// 8318F7E8: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318F7EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F7F0: 4BFFFD21  bl 0x8318f510
	ctx.lr = 0x8318F7F4;
	sub_8318F510(ctx, base);
	// 8318F7F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318F7F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318F7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318F800: 480189BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F808 size=108
    let mut pc: u32 = 0x8318F808;
    'dispatch: loop {
        match pc {
            0x8318F808 => {
    //   block [0x8318F808..0x8318F874)
	// 8318F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318F814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F81C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F820: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 8318F824: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8318F828: 4BFF95E1  bl 0x83188e08
	ctx.lr = 0x8318F82C;
	sub_83188E08(ctx, base);
	// 8318F82C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318F830: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8318F834: 409A0014  bne cr6, 0x8318f848
	if !ctx.cr[6].eq {
	pc = 0x8318F848; continue 'dispatch;
	}
	// 8318F838: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318F83C: 60840232  ori r4, r4, 0x232
	ctx.r[4].u64 = ctx.r[4].u64 | 562;
	// 8318F840: 4BFF7CB9  bl 0x831874f8
	ctx.lr = 0x8318F844;
	sub_831874F8(ctx, base);
	// 8318F844: 48000018  b 0x8318f85c
	pc = 0x8318F85C; continue 'dispatch;
	// 8318F848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318F84C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F850: 4BFFFE61  bl 0x8318f6b0
	ctx.lr = 0x8318F854;
	sub_8318F6B0(ctx, base);
	// 8318F854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F858: 4BFF8E91  bl 0x831886e8
	ctx.lr = 0x8318F85C;
	sub_831886E8(ctx, base);
	// 8318F85C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F868: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318F86C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F878 size=140
    let mut pc: u32 = 0x8318F878;
    'dispatch: loop {
        match pc {
            0x8318F878 => {
    //   block [0x8318F878..0x8318F904)
	// 8318F878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F87C: 480188ED  bl 0x831a8168
	ctx.lr = 0x8318F880;
	sub_831A8130(ctx, base);
	// 8318F880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F888: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8318F88C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318F890: 817F0D2C  lwz r11, 0xd2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3372 as u32) ) } as u64;
	// 8318F894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318F898: 419A0010  beq cr6, 0x8318f8a8
	if ctx.cr[6].eq {
	pc = 0x8318F8A8; continue 'dispatch;
	}
	// 8318F89C: 807F0D30  lwz r3, 0xd30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3376 as u32) ) } as u64;
	// 8318F8A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318F8A4: 4E800421  bctrl
	ctx.lr = 0x8318F8A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318F8A8: 3BDF007C  addi r30, r31, 0x7c
	ctx.r[30].s64 = ctx.r[31].s64 + 124;
	// 8318F8AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318F8B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318F8B4: 419A0010  beq cr6, 0x8318f8c4
	if ctx.cr[6].eq {
	pc = 0x8318F8C4; continue 'dispatch;
	}
	// 8318F8B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318F8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318F8C0: 480188F8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8318F8C4: 2F1D0800  cmpwi cr6, r29, 0x800
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2048, &mut ctx.xer);
	// 8318F8C8: 41980008  blt cr6, 0x8318f8d0
	if ctx.cr[6].lt {
	pc = 0x8318F8D0; continue 'dispatch;
	}
	// 8318F8CC: 3BA00800  li r29, 0x800
	ctx.r[29].s64 = 2048;
	// 8318F8D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318F8D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8318F8D8: 387E0094  addi r3, r30, 0x94
	ctx.r[3].s64 = ctx.r[30].s64 + 148;
	// 8318F8DC: 4BFFF0BD  bl 0x8318e998
	ctx.lr = 0x8318F8E0;
	sub_8318E998(ctx, base);
	// 8318F8E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F8E4: 93BE0090  stw r29, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[29].u32 ) };
	// 8318F8E8: 4BFFFF21  bl 0x8318f808
	ctx.lr = 0x8318F8EC;
	sub_8318F808(ctx, base);
	// 8318F8EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F8F0: 4BFFF941  bl 0x8318f230
	ctx.lr = 0x8318F8F4;
	sub_8318F230(ctx, base);
	// 8318F8F4: 4BFFF9DD  bl 0x8318f2d0
	ctx.lr = 0x8318F8F8;
	sub_8318F2D0(ctx, base);
	// 8318F8F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318F8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318F900: 480188B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F908 size=92
    let mut pc: u32 = 0x8318F908;
    'dispatch: loop {
        match pc {
            0x8318F908 => {
    //   block [0x8318F908..0x8318F964)
	// 8318F908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318F910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318F914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318F918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F91C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318F920: 4BFFF7C1  bl 0x8318f0e0
	ctx.lr = 0x8318F924;
	sub_8318F0E0(ctx, base);
	// 8318F924: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8318F928: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8318F92C: 419A0020  beq cr6, 0x8318f94c
	if ctx.cr[6].eq {
	pc = 0x8318F94C; continue 'dispatch;
	}
	// 8318F930: 3BDF007C  addi r30, r31, 0x7c
	ctx.r[30].s64 = ctx.r[31].s64 + 124;
	// 8318F934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F938: 4BFFF7A1  bl 0x8318f0d8
	ctx.lr = 0x8318F93C;
	sub_8318F0D8(ctx, base);
	// 8318F93C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318F940: 4BFFFEC9  bl 0x8318f808
	ctx.lr = 0x8318F944;
	sub_8318F808(ctx, base);
	// 8318F944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F948: 4BFFF8E9  bl 0x8318f230
	ctx.lr = 0x8318F94C;
	sub_8318F230(ctx, base);
	// 8318F94C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318F950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318F954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318F958: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318F95C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318F960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318F968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318F968 size=160
    let mut pc: u32 = 0x8318F968;
    'dispatch: loop {
        match pc {
            0x8318F968 => {
    //   block [0x8318F968..0x8318FA08)
	// 8318F968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318F96C: 480187FD  bl 0x831a8168
	ctx.lr = 0x8318F970;
	sub_831A8130(ctx, base);
	// 8318F970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318F974: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8318F978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318F97C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8318F980: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8318F984: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318F988: 419A0010  beq cr6, 0x8318f998
	if ctx.cr[6].eq {
	pc = 0x8318F998; continue 'dispatch;
	}
	// 8318F98C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318F990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318F994: 48018824  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8318F998: 3905FFFA  addi r8, r5, -6
	ctx.r[8].s64 = ctx.r[5].s64 + -6;
	// 8318F99C: 38E60006  addi r7, r6, 6
	ctx.r[7].s64 = ctx.r[6].s64 + 6;
	// 8318F9A0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8318F9A4: 4BFFF8FD  bl 0x8318f2a0
	ctx.lr = 0x8318F9A8;
	sub_8318F2A0(ctx, base);
	// 8318F9A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F9AC: 409A001C  bne cr6, 0x8318f9c8
	if !ctx.cr[6].eq {
	pc = 0x8318F9C8; continue 'dispatch;
	}
	// 8318F9B0: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8318F9B4: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 8318F9B8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8318F9BC: 4BFFF8E5  bl 0x8318f2a0
	ctx.lr = 0x8318F9C0;
	sub_8318F2A0(ctx, base);
	// 8318F9C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F9C4: 419AFFC8  beq cr6, 0x8318f98c
	if ctx.cr[6].eq {
	pc = 0x8318F98C; continue 'dispatch;
	}
	// 8318F9C8: 3BE8FFF4  addi r31, r8, -0xc
	ctx.r[31].s64 = ctx.r[8].s64 + -12;
	// 8318F9CC: 3BC7000C  addi r30, r7, 0xc
	ctx.r[30].s64 = ctx.r[7].s64 + 12;
	// 8318F9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318F9D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318F9D8: 4BFFF691  bl 0x8318f068
	ctx.lr = 0x8318F9DC;
	sub_8318F068(ctx, base);
	// 8318F9DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318F9E0: 419AFFAC  beq cr6, 0x8318f98c
	if ctx.cr[6].eq {
	pc = 0x8318F98C; continue 'dispatch;
	}
	// 8318F9E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318F9E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8318F9EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8318F9F0: 4BFFFE89  bl 0x8318f878
	ctx.lr = 0x8318F9F4;
	sub_8318F878(ctx, base);
	// 8318F9F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318F9F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318F9FC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318FA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318FA04: 480187B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FA08 size=36
    let mut pc: u32 = 0x8318FA08;
    'dispatch: loop {
        match pc {
            0x8318FA08 => {
    //   block [0x8318FA08..0x8318FA2C)
	// 8318FA08: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 8318FA0C: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 8318FA10: 7C6907B4  extsw r9, r3
	ctx.r[9].s64 = ctx.r[3].s32 as i64;
	// 8318FA14: 7CC807B4  extsw r8, r6
	ctx.r[8].s64 = ctx.r[6].s32 as i64;
	// 8318FA18: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8318FA1C: 7D4941D2  mulld r10, r9, r8
	ctx.r[10].s64 = ctx.r[9].s64 * ctx.r[8].s64;
	// 8318FA20: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8318FA24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318FA28: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FA2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FA2C size=8
    let mut pc: u32 = 0x8318FA2C;
    'dispatch: loop {
        match pc {
            0x8318FA2C => {
    //   block [0x8318FA2C..0x8318FA34)
	// 8318FA2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318FA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318FA38 size=116
    let mut pc: u32 = 0x8318FA38;
    'dispatch: loop {
        match pc {
            0x8318FA38 => {
    //   block [0x8318FA38..0x8318FAAC)
	// 8318FA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318FA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318FA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318FA44: 4BFEEA45  bl 0x8317e488
	ctx.lr = 0x8318FA48;
	sub_8317E488(ctx, base);
	// 8318FA48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318FA4C: 419A0024  beq cr6, 0x8318fa70
	if ctx.cr[6].eq {
	pc = 0x8318FA70; continue 'dispatch;
	}
	// 8318FA50: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8318FA54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318FA58: 6084FF03  ori r4, r4, 0xff03
	ctx.r[4].u64 = ctx.r[4].u64 | 65283;
	// 8318FA5C: 48003935  bl 0x83193390
	ctx.lr = 0x8318FA60;
	sub_83193390(ctx, base);
	// 8318FA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318FA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318FA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318FA6C: 4E800020  blr
	return;
	// 8318FA70: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318FA74: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8318FA78: 386BBB50  addi r3, r11, -0x44b0
	ctx.r[3].s64 = ctx.r[11].s64 + -17584;
	// 8318FA7C: 38801530  li r4, 0x1530
	ctx.r[4].s64 = 5424;
	// 8318FA80: 48008C39  bl 0x831986b8
	ctx.lr = 0x8318FA84;
	sub_831986B8(ctx, base);
	// 8318FA84: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318FA88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318FA8C: 419A0010  beq cr6, 0x8318fa9c
	if ctx.cr[6].eq {
	pc = 0x8318FA9C; continue 'dispatch;
	}
	// 8318FA90: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8318FA94: 6084FF07  ori r4, r4, 0xff07
	ctx.r[4].u64 = ctx.r[4].u64 | 65287;
	// 8318FA98: 480038F9  bl 0x83193390
	ctx.lr = 0x8318FA9C;
	sub_83193390(ctx, base);
	// 8318FA9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318FAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318FAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318FAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FAB0 size=108
    let mut pc: u32 = 0x8318FAB0;
    'dispatch: loop {
        match pc {
            0x8318FAB0 => {
    //   block [0x8318FAB0..0x8318FB1C)
	// 8318FAB0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8318FAB8: 390BF8C0  addi r8, r11, -0x740
	ctx.r[8].s64 = ctx.r[11].s64 + -1856;
	// 8318FABC: 39400180  li r10, 0x180
	ctx.r[10].s64 = 384;
	// 8318FAC0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8318FAC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318FAC8: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8318FACC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318FAD0: 4200FFF8  bdnz 0x8318fac8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318FAC8; continue 'dispatch;
	}
	// 8318FAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318FAD8: 39680180  addi r11, r8, 0x180
	ctx.r[11].s64 = ctx.r[8].s64 + 384;
	// 8318FADC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8318FAE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318FAE4: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 8318FAE8: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8318FAEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318FAF0: 4198FFEC  blt cr6, 0x8318fadc
	if ctx.cr[6].lt {
	pc = 0x8318FADC; continue 'dispatch;
	}
	// 8318FAF4: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 8318FAF8: 39400180  li r10, 0x180
	ctx.r[10].s64 = 384;
	// 8318FAFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318FB00: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8318FB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318FB08: 4200FFF8  bdnz 0x8318fb00
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318FB00; continue 'dispatch;
	}
	// 8318FB0C: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FB10: 39680180  addi r11, r8, 0x180
	ctx.r[11].s64 = ctx.r[8].s64 + 384;
	// 8318FB14: 916A9948  stw r11, -0x66b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26296 as u32), ctx.r[11].u32 ) };
	// 8318FB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FB20 size=20
    let mut pc: u32 = 0x8318FB20;
    'dispatch: loop {
        match pc {
            0x8318FB20 => {
    //   block [0x8318FB20..0x8318FB34)
	// 8318FB20: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FB24: 396BFCC0  addi r11, r11, -0x340
	ctx.r[11].s64 = ctx.r[11].s64 + -832;
	// 8318FB28: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318FB2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318FB30: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FB34 size=60
    let mut pc: u32 = 0x8318FB34;
    'dispatch: loop {
        match pc {
            0x8318FB34 => {
    //   block [0x8318FB34..0x8318FB70)
	// 8318FB34: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318FB38: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 8318FB3C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8318FB40: 394B1288  addi r10, r11, 0x1288
	ctx.r[10].s64 = ctx.r[11].s64 + 4744;
	// 8318FB44: 7D694396  divwu r11, r9, r8
	ctx.r[11].u32 = ctx.r[9].u32 / ctx.r[8].u32;
	// 8318FB48: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8318FB4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318FB50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8318FB54: 61298100  ori r9, r9, 0x8100
	ctx.r[9].u64 = ctx.r[9].u64 | 33024;
	// 8318FB58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318FB5C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8318FB60: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8318FB64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318FB68: 409AFFF0  bne cr6, 0x8318fb58
	if !ctx.cr[6].eq {
	pc = 0x8318FB58; continue 'dispatch;
	}
	// 8318FB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FB70 size=76
    let mut pc: u32 = 0x8318FB70;
    'dispatch: loop {
        match pc {
            0x8318FB70 => {
    //   block [0x8318FB70..0x8318FBBC)
	// 8318FB70: 39630B00  addi r11, r3, 0xb00
	ctx.r[11].s64 = ctx.r[3].s64 + 2816;
	// 8318FB74: 81431140  lwz r10, 0x1140(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4416 as u32) ) } as u64;
	// 8318FB78: 39230100  addi r9, r3, 0x100
	ctx.r[9].s64 = ctx.r[3].s64 + 256;
	// 8318FB7C: 90631214  stw r3, 0x1214(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4628 as u32), ctx.r[3].u32 ) };
	// 8318FB80: 388B0180  addi r4, r11, 0x180
	ctx.r[4].s64 = ctx.r[11].s64 + 384;
	// 8318FB84: 39030180  addi r8, r3, 0x180
	ctx.r[8].s64 = ctx.r[3].s64 + 384;
	// 8318FB88: 38E30200  addi r7, r3, 0x200
	ctx.r[7].s64 = ctx.r[3].s64 + 512;
	// 8318FB8C: 38C30280  addi r6, r3, 0x280
	ctx.r[6].s64 = ctx.r[3].s64 + 640;
	// 8318FB90: 91431210  stw r10, 0x1210(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4624 as u32), ctx.r[10].u32 ) };
	// 8318FB94: 38A30080  addi r5, r3, 0x80
	ctx.r[5].s64 = ctx.r[3].s64 + 128;
	// 8318FB98: 91631218  stw r11, 0x1218(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4632 as u32), ctx.r[11].u32 ) };
	// 8318FB9C: 9083121C  stw r4, 0x121c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4636 as u32), ctx.r[4].u32 ) };
	// 8318FBA0: 9123148C  stw r9, 0x148c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5260 as u32), ctx.r[9].u32 ) };
	// 8318FBA4: 91031490  stw r8, 0x1490(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5264 as u32), ctx.r[8].u32 ) };
	// 8318FBA8: 90E31494  stw r7, 0x1494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5268 as u32), ctx.r[7].u32 ) };
	// 8318FBAC: 90C31498  stw r6, 0x1498(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5272 as u32), ctx.r[6].u32 ) };
	// 8318FBB0: 9063149C  stw r3, 0x149c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5276 as u32), ctx.r[3].u32 ) };
	// 8318FBB4: 90A314A0  stw r5, 0x14a0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5280 as u32), ctx.r[5].u32 ) };
	// 8318FBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FBC0 size=304
    let mut pc: u32 = 0x8318FBC0;
    'dispatch: loop {
        match pc {
            0x8318FBC0 => {
    //   block [0x8318FBC0..0x8318FCF0)
	// 8318FBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318FBC4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8318FBC8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8318FBCC: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 8318FBD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8318FBD4: 9161FFC4  stw r11, -0x3c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), ctx.r[11].u32 ) };
	// 8318FBD8: 9161FFC8  stw r11, -0x38(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[11].u32 ) };
	// 8318FBDC: 9161FFCC  stw r11, -0x34(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), ctx.r[11].u32 ) };
	// 8318FBE0: 9161FFD0  stw r11, -0x30(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[11].u32 ) };
	// 8318FBE4: 9161FFD4  stw r11, -0x2c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), ctx.r[11].u32 ) };
	// 8318FBE8: 9161FFD8  stw r11, -0x28(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[11].u32 ) };
	// 8318FBEC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318FBF0: 9161FFDC  stw r11, -0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), ctx.r[11].u32 ) };
	// 8318FBF4: 9161FFC0  stw r11, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[11].u32 ) };
	// 8318FBF8: 9161FFE0  stw r11, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u32 ) };
	// 8318FBFC: 9161FFE4  stw r11, -0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), ctx.r[11].u32 ) };
	// 8318FC00: 9161FFE8  stw r11, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[11].u32 ) };
	// 8318FC04: 9161FFEC  stw r11, -0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), ctx.r[11].u32 ) };
	// 8318FC08: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 8318FC0C: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 8318FC10: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318FC14: 90E30038  stw r7, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[7].u32 ) };
	// 8318FC18: 9103003C  stw r8, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[8].u32 ) };
	// 8318FC1C: 91030040  stw r8, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 8318FC20: 91030044  stw r8, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[8].u32 ) };
	// 8318FC24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318FC28: B1430050  sth r10, 0x50(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8318FC2C: B1430052  sth r10, 0x52(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(82 as u32), ctx.r[10].u16 ) };
	// 8318FC30: 99430055  stb r10, 0x55(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(85 as u32), ctx.r[10].u8 ) };
	// 8318FC34: 99430056  stb r10, 0x56(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(86 as u32), ctx.r[10].u8 ) };
	// 8318FC38: 99430057  stb r10, 0x57(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(87 as u32), ctx.r[10].u8 ) };
	// 8318FC3C: 99030059  stb r8, 0x59(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(89 as u32), ctx.r[8].u8 ) };
	// 8318FC40: 9923005D  stb r9, 0x5d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(93 as u32), ctx.r[9].u8 ) };
	// 8318FC44: 9943005E  stb r10, 0x5e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(94 as u32), ctx.r[10].u8 ) };
	// 8318FC48: 9943005F  stb r10, 0x5f(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(95 as u32), ctx.r[10].u8 ) };
	// 8318FC4C: 99430060  stb r10, 0x60(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[10].u8 ) };
	// 8318FC50: 99230062  stb r9, 0x62(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(98 as u32), ctx.r[9].u8 ) };
	// 8318FC54: 99230063  stb r9, 0x63(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(99 as u32), ctx.r[9].u8 ) };
	// 8318FC58: 99230064  stb r9, 0x64(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[9].u8 ) };
	// 8318FC5C: 8161FFC4  lwz r11, -0x3c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-60 as u32) ) } as u64;
	// 8318FC60: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318FC64: 8161FFC8  lwz r11, -0x38(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 8318FC68: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318FC6C: 8161FFCC  lwz r11, -0x34(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-52 as u32) ) } as u64;
	// 8318FC70: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318FC74: 8161FFD0  lwz r11, -0x30(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) } as u64;
	// 8318FC78: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318FC7C: 8161FFD4  lwz r11, -0x2c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-44 as u32) ) } as u64;
	// 8318FC80: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318FC84: 8161FFD8  lwz r11, -0x28(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) } as u64;
	// 8318FC88: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318FC8C: 8161FFDC  lwz r11, -0x24(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-36 as u32) ) } as u64;
	// 8318FC90: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318FC94: 8161FFE0  lwz r11, -0x20(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 8318FC98: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318FC9C: 8161FFE4  lwz r11, -0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-28 as u32) ) } as u64;
	// 8318FCA0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318FCA4: 8161FFE8  lwz r11, -0x18(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 8318FCA8: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8318FCAC: 8161FFEC  lwz r11, -0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-20 as u32) ) } as u64;
	// 8318FCB0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318FCB4: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8318FCB8: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8318FCBC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8318FCC0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8318FCC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318FCC8: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8318FCCC: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8318FCD0: 99630054  stb r11, 0x54(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8318FCD4: 99630058  stb r11, 0x58(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 8318FCD8: 9963005A  stb r11, 0x5a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 8318FCDC: 9963005B  stb r11, 0x5b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(91 as u32), ctx.r[11].u8 ) };
	// 8318FCE0: 9963005C  stb r11, 0x5c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u8 ) };
	// 8318FCE4: 99630061  stb r11, 0x61(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 8318FCE8: 91630068  stw r11, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8318FCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FCF0 size=48
    let mut pc: u32 = 0x8318FCF0;
    'dispatch: loop {
        match pc {
            0x8318FCF0 => {
    //   block [0x8318FCF0..0x8318FD20)
	// 8318FCF0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FCF4: 394BFCC0  addi r10, r11, -0x340
	ctx.r[10].s64 = ctx.r[11].s64 + -832;
	// 8318FCF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318FCFC: 806A0058  lwz r3, 0x58(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318FD00: 814A0054  lwz r10, 0x54(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318FD04: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318FD08: 40990028  ble cr6, 0x8318fd30
	if !ctx.cr[6].gt {
		sub_8318FD20(ctx, base);
		return;
	}
	// 8318FD0C: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8318FD10: 61298100  ori r9, r9, 0x8100
	ctx.r[9].u64 = ctx.r[9].u64 | 33024;
	// 8318FD14: 81031288  lwz r8, 0x1288(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4744 as u32) ) } as u64;
	// 8318FD18: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 8318FD1C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FD20 size=24
    let mut pc: u32 = 0x8318FD20;
    'dispatch: loop {
        match pc {
            0x8318FD20 => {
    //   block [0x8318FD20..0x8318FD38)
	// 8318FD20: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 8318FD24: 7C634A14  add r3, r3, r9
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 8318FD28: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8318FD2C: 4198FFE8  blt cr6, 0x8318fd14
	if ctx.cr[6].lt {
		sub_8318FCF0(ctx, base);
		return;
	}
	// 8318FD30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318FD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FD38 size=28
    let mut pc: u32 = 0x8318FD38;
    'dispatch: loop {
        match pc {
            0x8318FD38 => {
    //   block [0x8318FD38..0x8318FD54)
	// 8318FD38: 39630B00  addi r11, r3, 0xb00
	ctx.r[11].s64 = ctx.r[3].s64 + 2816;
	// 8318FD3C: 39430300  addi r10, r3, 0x300
	ctx.r[10].s64 = ctx.r[3].s64 + 768;
	// 8318FD40: 3923148C  addi r9, r3, 0x148c
	ctx.r[9].s64 = ctx.r[3].s64 + 5260;
	// 8318FD44: 916311C0  stw r11, 0x11c0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4544 as u32), ctx.r[11].u32 ) };
	// 8318FD48: 914311A4  stw r10, 0x11a4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4516 as u32), ctx.r[10].u32 ) };
	// 8318FD4C: 912311A8  stw r9, 0x11a8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4520 as u32), ctx.r[9].u32 ) };
	// 8318FD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FD58 size=20
    let mut pc: u32 = 0x8318FD58;
    'dispatch: loop {
        match pc {
            0x8318FD58 => {
    //   block [0x8318FD58..0x8318FD6C)
	// 8318FD58: 81631190  lwz r11, 0x1190(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4496 as u32) ) } as u64;
	// 8318FD5C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318FD60: 81631194  lwz r11, 0x1194(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4500 as u32) ) } as u64;
	// 8318FD64: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318FD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FD70 size=24
    let mut pc: u32 = 0x8318FD70;
    'dispatch: loop {
        match pc {
            0x8318FD70 => {
    //   block [0x8318FD70..0x8318FD88)
	// 8318FD70: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FD74: 394BFCC0  addi r10, r11, -0x340
	ctx.r[10].s64 = ctx.r[11].s64 + -832;
	// 8318FD78: 816A0058  lwz r11, 0x58(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318FD7C: 814A0054  lwz r10, 0x54(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318FD80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318FD84: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FD88 size=68
    let mut pc: u32 = 0x8318FD88;
    'dispatch: loop {
        match pc {
            0x8318FD88 => {
    //   block [0x8318FD88..0x8318FDCC)
	// 8318FD88: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318FD8C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8318FD90: 7D4A4B96  divwu r10, r10, r9
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 8318FD94: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8318FD98: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318FD9C: 61298100  ori r9, r9, 0x8100
	ctx.r[9].u64 = ctx.r[9].u64 | 33024;
	// 8318FDA0: 810B1288  lwz r8, 0x1288(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4744 as u32) ) } as u64;
	// 8318FDA4: 2F080002  cmpwi cr6, r8, 2
	ctx.cr[6].compare_i32(ctx.r[8].s32, 2, &mut ctx.xer);
	// 8318FDA8: 409A0010  bne cr6, 0x8318fdb8
	if !ctx.cr[6].eq {
	pc = 0x8318FDB8; continue 'dispatch;
	}
	// 8318FDAC: 390304A4  addi r8, r3, 0x4a4
	ctx.r[8].s64 = ctx.r[3].s64 + 1188;
	// 8318FDB0: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318FDB4: 7C88592E  stwx r4, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 8318FDB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318FDBC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8318FDC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318FDC4: 409AFFDC  bne cr6, 0x8318fda0
	if !ctx.cr[6].eq {
	pc = 0x8318FDA0; continue 'dispatch;
	}
	// 8318FDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FDD0 size=16
    let mut pc: u32 = 0x8318FDD0;
    'dispatch: loop {
        match pc {
            0x8318FDD0 => {
    //   block [0x8318FDD0..0x8318FDE0)
	// 8318FDD0: 908312D0  stw r4, 0x12d0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4816 as u32), ctx.r[4].u32 ) };
	// 8318FDD4: 90C312D8  stw r6, 0x12d8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4824 as u32), ctx.r[6].u32 ) };
	// 8318FDD8: 90A312D4  stw r5, 0x12d4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4820 as u32), ctx.r[5].u32 ) };
	// 8318FDDC: 4BF37D04  b 0x830c7ae0
	sub_830C7AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FDE0 size=16
    let mut pc: u32 = 0x8318FDE0;
    'dispatch: loop {
        match pc {
            0x8318FDE0 => {
    //   block [0x8318FDE0..0x8318FDF0)
	// 8318FDE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318FDE4: 409A000C  bne cr6, 0x8318fdf0
	if !ctx.cr[6].eq {
		sub_8318FDF0(ctx, base);
		return;
	}
	// 8318FDE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8318FDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FDF0 size=28
    let mut pc: u32 = 0x8318FDF0;
    'dispatch: loop {
        match pc {
            0x8318FDF0 => {
    //   block [0x8318FDF0..0x8318FE0C)
	// 8318FDF0: 81631288  lwz r11, 0x1288(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4744 as u32) ) } as u64;
	// 8318FDF4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8318FDF8: 409AFFF0  bne cr6, 0x8318fde8
	if !ctx.cr[6].eq {
		sub_8318FDE0(ctx, base);
		return;
	}
	// 8318FDFC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FE00: 906BF8B8  stw r3, -0x748(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-1864 as u32), ctx.r[3].u32 ) };
	// 8318FE04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318FE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318FE10 size=80
    let mut pc: u32 = 0x8318FE10;
    'dispatch: loop {
        match pc {
            0x8318FE10 => {
    //   block [0x8318FE10..0x8318FE60)
	// 8318FE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318FE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318FE18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318FE1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318FE20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318FE24: 4BFFFC8D  bl 0x8318fab0
	ctx.lr = 0x8318FE28;
	sub_8318FAB0(ctx, base);
	// 8318FE28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8318FE2C: 419A0020  beq cr6, 0x8318fe4c
	if ctx.cr[6].eq {
	pc = 0x8318FE4C; continue 'dispatch;
	}
	// 8318FE30: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FE34: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8318FE38: 388BF8C0  addi r4, r11, -0x740
	ctx.r[4].s64 = ctx.r[11].s64 + -1856;
	// 8318FE3C: 4800A64D  bl 0x8319a488
	ctx.lr = 0x8318FE40;
	sub_8319A488(ctx, base);
	// 8318FE40: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FE44: 397F0180  addi r11, r31, 0x180
	ctx.r[11].s64 = ctx.r[31].s64 + 384;
	// 8318FE48: 916A9948  stw r11, -0x66b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26296 as u32), ctx.r[11].u32 ) };
	// 8318FE4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318FE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318FE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318FE58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318FE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318FE60 size=148
    let mut pc: u32 = 0x8318FE60;
    'dispatch: loop {
        match pc {
            0x8318FE60 => {
    //   block [0x8318FE60..0x8318FEF4)
	// 8318FE60: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FE64: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FE68: 396BFCC0  addi r11, r11, -0x340
	ctx.r[11].s64 = ctx.r[11].s64 + -832;
	// 8318FE6C: 814A97E8  lwz r10, -0x6818(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26648 as u32) ) } as u64;
	// 8318FE70: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318FE74: 91431110  stw r10, 0x1110(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4368 as u32), ctx.r[10].u32 ) };
	// 8318FE78: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FE7C: 392B1200  addi r9, r11, 0x1200
	ctx.r[9].s64 = ctx.r[11].s64 + 4608;
	// 8318FE80: 814A97D8  lwz r10, -0x6828(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26664 as u32) ) } as u64;
	// 8318FE84: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 8318FE88: 91431114  stw r10, 0x1114(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4372 as u32), ctx.r[10].u32 ) };
	// 8318FE8C: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FE90: 814A97C0  lwz r10, -0x6840(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26688 as u32) ) } as u64;
	// 8318FE94: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318FE98: 91431118  stw r10, 0x1118(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4376 as u32), ctx.r[10].u32 ) };
	// 8318FE9C: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FEA0: 814A97E4  lwz r10, -0x681c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26652 as u32) ) } as u64;
	// 8318FEA4: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8318FEA8: 9143111C  stw r10, 0x111c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4380 as u32), ctx.r[10].u32 ) };
	// 8318FEAC: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FEB0: 814A9794  lwz r10, -0x686c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26732 as u32) ) } as u64;
	// 8318FEB4: 91431120  stw r10, 0x1120(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4384 as u32), ctx.r[10].u32 ) };
	// 8318FEB8: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FEBC: 814A9798  lwz r10, -0x6868(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26728 as u32) ) } as u64;
	// 8318FEC0: 91431124  stw r10, 0x1124(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4388 as u32), ctx.r[10].u32 ) };
	// 8318FEC4: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8318FEC8: 814A97E0  lwz r10, -0x6820(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26656 as u32) ) } as u64;
	// 8318FECC: 9123113C  stw r9, 0x113c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4412 as u32), ctx.r[9].u32 ) };
	// 8318FED0: 91431128  stw r10, 0x1128(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4392 as u32), ctx.r[10].u32 ) };
	// 8318FED4: 394B11E0  addi r10, r11, 0x11e0
	ctx.r[10].s64 = ctx.r[11].s64 + 4576;
	// 8318FED8: 396B1100  addi r11, r11, 0x1100
	ctx.r[11].s64 = ctx.r[11].s64 + 4352;
	// 8318FEDC: 91431138  stw r10, 0x1138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4408 as u32), ctx.r[10].u32 ) };
	// 8318FEE0: 91631130  stw r11, 0x1130(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4400 as u32), ctx.r[11].u32 ) };
	// 8318FEE4: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318FEE8: 816B9948  lwz r11, -0x66b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26296 as u32) ) } as u64;
	// 8318FEEC: 91631140  stw r11, 0x1140(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4416 as u32), ctx.r[11].u32 ) };
	// 8318FEF0: 4BFFFC80  b 0x8318fb70
	sub_8318FB70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318FEF8 size=108
    let mut pc: u32 = 0x8318FEF8;
    'dispatch: loop {
        match pc {
            0x8318FEF8 => {
    //   block [0x8318FEF8..0x8318FF64)
	// 8318FEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318FEFC: 4801826D  bl 0x831a8168
	ctx.lr = 0x8318FF00;
	sub_831A8130(ctx, base);
	// 8318FF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318FF04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318FF08: 3964007F  addi r11, r4, 0x7f
	ctx.r[11].s64 = ctx.r[4].s64 + 127;
	// 8318FF0C: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 8318FF10: 557D0030  rlwinm r29, r11, 0, 0, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8318FF14: 554558A8  rlwinm r5, r10, 0xb, 2, 0x14
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x001FFFFFu64;
	// 8318FF18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318FF1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318FF20: 48003169  bl 0x83193088
	ctx.lr = 0x8318FF24;
	sub_83193088(ctx, base);
	// 8318FF24: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318FF28: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8318FF2C: 388BBB00  addi r4, r11, -0x4500
	ctx.r[4].s64 = ctx.r[11].s64 + -17664;
	// 8318FF30: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318FF34: 3BEBFCC0  addi r31, r11, -0x340
	ctx.r[31].s64 = ctx.r[11].s64 + -832;
	// 8318FF38: 1D7E1580  mulli r11, r30, 0x1580
	ctx.r[11].s64 = ctx.r[30].s64 * 5504;
	// 8318FF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318FF40: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8318FF44: 4BFFEA55  bl 0x8318e998
	ctx.lr = 0x8318FF48;
	sub_8318E998(ctx, base);
	// 8318FF48: 397C0420  addi r11, r28, 0x420
	ctx.r[11].s64 = ctx.r[28].s64 + 1056;
	// 8318FF4C: 939F004C  stw r28, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 8318FF50: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318FF54: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8318FF58: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8318FF5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318FF60: 48018258  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318FF68 size=40
    let mut pc: u32 = 0x8318FF68;
    'dispatch: loop {
        match pc {
            0x8318FF68 => {
    //   block [0x8318FF68..0x8318FF90)
	// 8318FF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318FF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318FF70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318FF74: 4BF37B6D  bl 0x830c7ae0
	ctx.lr = 0x8318FF78;
	sub_830C7AE0(ctx, base);
	// 8318FF78: 4800A629  bl 0x8319a5a0
	ctx.lr = 0x8318FF7C;
	sub_8319A5A0(ctx, base);
	// 8318FF7C: 4BF37B65  bl 0x830c7ae0
	ctx.lr = 0x8318FF80;
	sub_830C7AE0(ctx, base);
	// 8318FF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318FF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318FF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318FF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318FF90 size=56
    let mut pc: u32 = 0x8318FF90;
    'dispatch: loop {
        match pc {
            0x8318FF90 => {
    //   block [0x8318FF90..0x8318FFC8)
	// 8318FF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318FF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318FF98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318FF9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318FFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318FFA4: 387F1178  addi r3, r31, 0x1178
	ctx.r[3].s64 = ctx.r[31].s64 + 4472;
	// 8318FFA8: 4800AFB1  bl 0x8319af58
	ctx.lr = 0x8318FFAC;
	sub_8319AF58(ctx, base);
	// 8318FFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318FFB0: 4BFFFD89  bl 0x8318fd38
	ctx.lr = 0x8318FFB4;
	sub_8318FD38(ctx, base);
	// 8318FFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318FFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318FFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318FFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318FFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318FFC8 size=116
    let mut pc: u32 = 0x8318FFC8;
    'dispatch: loop {
        match pc {
            0x8318FFC8 => {
    //   block [0x8318FFC8..0x8319003C)
	// 8318FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318FFD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318FFD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318FFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318FFDC: 4BFFFE05  bl 0x8318fde0
	ctx.lr = 0x8318FFE0;
	sub_8318FDE0(ctx, base);
	// 8318FFE0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318FFE4: 419A0028  beq cr6, 0x8319000c
	if ctx.cr[6].eq {
	pc = 0x8319000C; continue 'dispatch;
	}
	// 8318FFE8: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8318FFEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318FFF0: 60840201  ori r4, r4, 0x201
	ctx.r[4].u64 = ctx.r[4].u64 | 513;
	// 8318FFF4: 4800339D  bl 0x83193390
	ctx.lr = 0x8318FFF8;
	sub_83193390(ctx, base);
	// 8318FFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318FFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83190000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83190004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83190008: 4E800020  blr
	return;
	// 8319000C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190010: 4BF37AD1  bl 0x830c7ae0
	ctx.lr = 0x83190014;
	sub_830C7AE0(ctx, base);
	// 83190014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190018: 4800A5D1  bl 0x8319a5e8
	ctx.lr = 0x8319001C;
	sub_8319A5E8(ctx, base);
	// 8319001C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83190020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83190024: 917F1288  stw r11, 0x1288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4744 as u32), ctx.r[11].u32 ) };
	// 83190028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319002C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83190030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83190034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83190038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190040 size=144
    let mut pc: u32 = 0x83190040;
    'dispatch: loop {
        match pc {
            0x83190040 => {
    //   block [0x83190040..0x831900D0)
	// 83190040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83190044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83190048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319004C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83190050: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 83190054: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 83190058: 409A001C  bne cr6, 0x83190074
	if !ctx.cr[6].eq {
	pc = 0x83190074; continue 'dispatch;
	}
	// 8319005C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83190060: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83190064: 4BFFFD0D  bl 0x8318fd70
	ctx.lr = 0x83190068;
	sub_8318FD70(ctx, base);
	// 83190068: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319006C: 396BFCC0  addi r11, r11, -0x340
	ctx.r[11].s64 = ctx.r[11].s64 + -832;
	// 83190070: 48000038  b 0x831900a8
	pc = 0x831900A8; continue 'dispatch;
	// 83190074: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83190078: 4BFFFD69  bl 0x8318fde0
	ctx.lr = 0x8319007C;
	sub_8318FDE0(ctx, base);
	// 8319007C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190080: 419A0024  beq cr6, 0x831900a4
	if ctx.cr[6].eq {
	pc = 0x831900A4; continue 'dispatch;
	}
	// 83190084: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 83190088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319008C: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 83190090: 48003301  bl 0x83193390
	ctx.lr = 0x83190094;
	sub_83193390(ctx, base);
	// 83190094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83190098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319009C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831900A0: 4E800020  blr
	return;
	// 831900A4: 39661290  addi r11, r6, 0x1290
	ctx.r[11].s64 = ctx.r[6].s64 + 4752;
	// 831900A8: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831900AC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 831900B0: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 831900B4: 7CAA592E  stwx r5, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u32) };
	// 831900B8: 4BF37A29  bl 0x830c7ae0
	ctx.lr = 0x831900BC;
	sub_830C7AE0(ctx, base);
	// 831900BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831900C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831900C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831900C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831900CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831900D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831900D0 size=120
    let mut pc: u32 = 0x831900D0;
    'dispatch: loop {
        match pc {
            0x831900D0 => {
    //   block [0x831900D0..0x83190148)
	// 831900D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831900D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831900D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831900DC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831900E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831900E4: 409A0010  bne cr6, 0x831900f4
	if !ctx.cr[6].eq {
	pc = 0x831900F4; continue 'dispatch;
	}
	// 831900E8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831900EC: 396BFCC0  addi r11, r11, -0x340
	ctx.r[11].s64 = ctx.r[11].s64 + -832;
	// 831900F0: 48000038  b 0x83190128
	pc = 0x83190128; continue 'dispatch;
	// 831900F4: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 831900F8: 4BFFFCE9  bl 0x8318fde0
	ctx.lr = 0x831900FC;
	sub_8318FDE0(ctx, base);
	// 831900FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190100: 419A0024  beq cr6, 0x83190124
	if ctx.cr[6].eq {
	pc = 0x83190124; continue 'dispatch;
	}
	// 83190104: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 83190108: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319010C: 60840210  ori r4, r4, 0x210
	ctx.r[4].u64 = ctx.r[4].u64 | 528;
	// 83190110: 48003281  bl 0x83193390
	ctx.lr = 0x83190114;
	sub_83193390(ctx, base);
	// 83190114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83190118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319011C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83190120: 4E800020  blr
	return;
	// 83190124: 396A1290  addi r11, r10, 0x1290
	ctx.r[11].s64 = ctx.r[10].s64 + 4752;
	// 83190128: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8319012C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83190130: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83190134: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83190138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8319013C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83190140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83190144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190148 size=184
    let mut pc: u32 = 0x83190148;
    'dispatch: loop {
        match pc {
            0x83190148 => {
    //   block [0x83190148..0x83190200)
	// 83190148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319014C: 48018021  bl 0x831a816c
	ctx.lr = 0x83190150;
	sub_831A8130(ctx, base);
	// 83190150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190154: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 83190158: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8319015C: 3BEAFCC0  addi r31, r10, -0x340
	ctx.r[31].s64 = ctx.r[10].s64 + -832;
	// 83190160: 396BBAB0  addi r11, r11, -0x4550
	ctx.r[11].s64 = ctx.r[11].s64 + -17744;
	// 83190164: 395FFBFC  addi r10, r31, -0x404
	ctx.r[10].s64 = ctx.r[31].s64 + -1028;
	// 83190168: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8319016C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83190170: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83190174: 4BFFF8C5  bl 0x8318fa38
	ctx.lr = 0x83190178;
	sub_8318FA38(ctx, base);
	// 83190178: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8319017C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83190180: 419A0018  beq cr6, 0x83190198
	if ctx.cr[6].eq {
	pc = 0x83190198; continue 'dispatch;
	}
	// 83190184: 3D40FF03  lis r10, -0xfd
	ctx.r[10].s64 = -16580608;
	// 83190188: 6143FF05  ori r3, r10, 0xff05
	ctx.r[3].u64 = ctx.r[10].u64 | 65285;
	// 8319018C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83190190: 419A0068  beq cr6, 0x831901f8
	if ctx.cr[6].eq {
	pc = 0x831901F8; continue 'dispatch;
	}
	// 83190194: 48000000  b 0x83190194
	pc = 0x83190194; continue 'dispatch;
	// 83190198: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8319019C: 4BF37945  bl 0x830c7ae0
	ctx.lr = 0x831901A0;
	sub_830C7AE0(ctx, base);
	// 831901A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831901A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831901A8: 4BFFFD51  bl 0x8318fef8
	ctx.lr = 0x831901AC;
	sub_8318FEF8(ctx, base);
	// 831901AC: 83FF0050  lwz r31, 0x50(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831901B0: 480031D1  bl 0x83193380
	ctx.lr = 0x831901B4;
	sub_83193380(ctx, base);
	// 831901B4: 48002515  bl 0x831926c8
	ctx.lr = 0x831901B8;
	sub_831926C8(ctx, base);
	// 831901B8: 48000171  bl 0x83190328
	ctx.lr = 0x831901BC;
	sub_83190328(ctx, base);
	// 831901BC: 4BF37925  bl 0x830c7ae0
	ctx.lr = 0x831901C0;
	sub_830C7AE0(ctx, base);
	// 831901C0: 4800A3D9  bl 0x8319a598
	ctx.lr = 0x831901C4;
	sub_8319A598(ctx, base);
	// 831901C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831901C8: 387F1230  addi r3, r31, 0x1230
	ctx.r[3].s64 = ctx.r[31].s64 + 4656;
	// 831901CC: 4800A265  bl 0x8319a430
	ctx.lr = 0x831901D0;
	sub_8319A430(ctx, base);
	// 831901D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831901D4: 4800B3B5  bl 0x8319b588
	ctx.lr = 0x831901D8;
	sub_8319B588(ctx, base);
	// 831901D8: 4800AC61  bl 0x8319ae38
	ctx.lr = 0x831901DC;
	sub_8319AE38(ctx, base);
	// 831901DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831901E0: 4800AEB1  bl 0x8319b090
	ctx.lr = 0x831901E4;
	sub_8319B090(ctx, base);
	// 831901E4: 387F17E0  addi r3, r31, 0x17e0
	ctx.r[3].s64 = ctx.r[31].s64 + 6112;
	// 831901E8: 4BFFFC29  bl 0x8318fe10
	ctx.lr = 0x831901EC;
	sub_8318FE10(ctx, base);
	// 831901EC: 4BFFF935  bl 0x8318fb20
	ctx.lr = 0x831901F0;
	sub_8318FB20(ctx, base);
	// 831901F0: 4BF378F1  bl 0x830c7ae0
	ctx.lr = 0x831901F4;
	sub_830C7AE0(ctx, base);
	// 831901F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831901F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831901FC: 48017FC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190200 size=232
    let mut pc: u32 = 0x83190200;
    'dispatch: loop {
        match pc {
            0x83190200 => {
    //   block [0x83190200..0x831902E8)
	// 83190200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83190204: 48017F69  bl 0x831a816c
	ctx.lr = 0x83190208;
	sub_831A8130(ctx, base);
	// 83190208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319020C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83190210: 4BFFFC51  bl 0x8318fe60
	ctx.lr = 0x83190214;
	sub_8318FE60(ctx, base);
	// 83190214: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83190218: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8319021C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83190220: 388BFCC0  addi r4, r11, -0x340
	ctx.r[4].s64 = ctx.r[11].s64 + -832;
	// 83190224: 387F1290  addi r3, r31, 0x1290
	ctx.r[3].s64 = ctx.r[31].s64 + 4752;
	// 83190228: 93DF128C  stw r30, 0x128c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4748 as u32), ctx.r[30].u32 ) };
	// 8319022C: 4BFFE76D  bl 0x8318e998
	ctx.lr = 0x83190230;
	sub_8318E998(ctx, base);
	// 83190230: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83190234: 387F135C  addi r3, r31, 0x135c
	ctx.r[3].s64 = ctx.r[31].s64 + 4956;
	// 83190238: 93DF12D4  stw r30, 0x12d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4820 as u32), ctx.r[30].u32 ) };
	// 8319023C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83190240: 93DF12D8  stw r30, 0x12d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4824 as u32), ctx.r[30].u32 ) };
	// 83190244: 917F12D0  stw r11, 0x12d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4816 as u32), ctx.r[11].u32 ) };
	// 83190248: 4BFFE759  bl 0x8318e9a0
	ctx.lr = 0x8319024C;
	sub_8318E9A0(ctx, base);
	// 8319024C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190250: 4800B431  bl 0x8319b680
	ctx.lr = 0x83190254;
	sub_8319B680(ctx, base);
	// 83190254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190258: 4BFFFD39  bl 0x8318ff90
	ctx.lr = 0x8319025C;
	sub_8318FF90(ctx, base);
	// 8319025C: 387F12DC  addi r3, r31, 0x12dc
	ctx.r[3].s64 = ctx.r[31].s64 + 4828;
	// 83190260: 4BFFF961  bl 0x8318fbc0
	ctx.lr = 0x83190264;
	sub_8318FBC0(ctx, base);
	// 83190264: 93DF14A4  stw r30, 0x14a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5284 as u32), ctx.r[30].u32 ) };
	// 83190268: 93DF14A8  stw r30, 0x14a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5288 as u32), ctx.r[30].u32 ) };
	// 8319026C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83190270: 93DF147C  stw r30, 0x147c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5244 as u32), ctx.r[30].u32 ) };
	// 83190274: 93DF1480  stw r30, 0x1480(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5248 as u32), ctx.r[30].u32 ) };
	// 83190278: 93DF1484  stw r30, 0x1484(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5252 as u32), ctx.r[30].u32 ) };
	// 8319027C: 93DF1488  stw r30, 0x1488(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5256 as u32), ctx.r[30].u32 ) };
	// 83190280: 93DF14B8  stw r30, 0x14b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5304 as u32), ctx.r[30].u32 ) };
	// 83190284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83190288: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8319028C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83190290: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83190294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190298: 480001E1  bl 0x83190478
	ctx.lr = 0x8319029C;
	sub_83190478(ctx, base);
	// 8319029C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831902A0: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 831902A4: 4198FFE0  blt cr6, 0x83190284
	if ctx.cr[6].lt {
	pc = 0x83190284; continue 'dispatch;
	}
	// 831902A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831902AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831902B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831902B4: 480001ED  bl 0x831904a0
	ctx.lr = 0x831902B8;
	sub_831904A0(ctx, base);
	// 831902B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831902BC: 93DF1528  stw r30, 0x1528(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5416 as u32), ctx.r[30].u32 ) };
	// 831902C0: 4800A2E9  bl 0x8319a5a8
	ctx.lr = 0x831902C4;
	sub_8319A5A8(ctx, base);
	// 831902C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831902C8: 4BFEE1C1  bl 0x8317e488
	ctx.lr = 0x831902CC;
	sub_8317E488(ctx, base);
	// 831902CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831902D0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 831902D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831902D8: 917F14D4  stw r11, 0x14d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5332 as u32), ctx.r[11].u32 ) };
	// 831902DC: 915F1288  stw r10, 0x1288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4744 as u32), ctx.r[10].u32 ) };
	// 831902E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831902E4: 48017ED8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831902E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831902E8 size=60
    let mut pc: u32 = 0x831902E8;
    'dispatch: loop {
        match pc {
            0x831902E8 => {
    //   block [0x831902E8..0x83190324)
	// 831902E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831902EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831902F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831902F4: 4BFFF9FD  bl 0x8318fcf0
	ctx.lr = 0x831902F8;
	sub_8318FCF0(ctx, base);
	// 831902F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831902FC: 409A0014  bne cr6, 0x83190310
	if !ctx.cr[6].eq {
	pc = 0x83190310; continue 'dispatch;
	}
	// 83190300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83190304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83190308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319030C: 4E800020  blr
	return;
	// 83190310: 4BFFFEF1  bl 0x83190200
	ctx.lr = 0x83190314;
	sub_83190200(ctx, base);
	// 83190314: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83190318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319031C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83190320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190328 size=336
    let mut pc: u32 = 0x83190328;
    'dispatch: loop {
        match pc {
            0x83190328 => {
    //   block [0x83190328..0x83190478)
	// 83190328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319032C: 48017E31  bl 0x831a815c
	ctx.lr = 0x83190330;
	sub_831A8130(ctx, base);
	// 83190330: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190334: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83190338: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8319033C: 3BEBFD20  addi r31, r11, -0x2e0
	ctx.r[31].s64 = ctx.r[11].s64 + -736;
	// 83190340: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 83190344: 397F00A0  addi r11, r31, 0xa0
	ctx.r[11].s64 = ctx.r[31].s64 + 160;
	// 83190348: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8319034C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83190350: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83190354: 4200FFF8  bdnz 0x8319034c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8319034C; continue 'dispatch;
	}
	// 83190358: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 8319035C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83190360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190364: 48017E7D  bl 0x831a81e0
	ctx.lr = 0x83190368;
	sub_831A81E0(ctx, base);
	// 83190368: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 8319036C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83190370: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 83190374: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83190378: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8319037C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83190380: 4200FFF8  bdnz 0x83190378
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83190378; continue 'dispatch;
	}
	// 83190384: 397F0078  addi r11, r31, 0x78
	ctx.r[11].s64 = ctx.r[31].s64 + 120;
	// 83190388: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8319038C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 83190390: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83190394: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83190398: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8319039C: 4200FFF8  bdnz 0x83190394
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83190394; continue 'dispatch;
	}
	// 831903A0: 397F00C8  addi r11, r31, 0xc8
	ctx.r[11].s64 = ctx.r[31].s64 + 200;
	// 831903A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831903A8: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 831903AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831903B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831903B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831903B8: 4200FFF8  bdnz 0x831903b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831903B0; continue 'dispatch;
	}
	// 831903BC: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 831903C0: 3F208324  lis r25, -0x7cdc
	ctx.r[25].s64 = -2094792704;
	// 831903C4: 396B47C8  addi r11, r11, 0x47c8
	ctx.r[11].s64 = ctx.r[11].s64 + 18376;
	// 831903C8: 3F408324  lis r26, -0x7cdc
	ctx.r[26].s64 = -2094792704;
	// 831903CC: 3F608324  lis r27, -0x7cdc
	ctx.r[27].s64 = -2094792704;
	// 831903D0: 3F808324  lis r28, -0x7cdc
	ctx.r[28].s64 = -2094792704;
	// 831903D4: 3FA08324  lis r29, -0x7cdc
	ctx.r[29].s64 = -2094792704;
	// 831903D8: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 831903DC: 397947C8  addi r11, r25, 0x47c8
	ctx.r[11].s64 = ctx.r[25].s64 + 18376;
	// 831903E0: 3FC08324  lis r30, -0x7cdc
	ctx.r[30].s64 = -2094792704;
	// 831903E4: 3C608324  lis r3, -0x7cdc
	ctx.r[3].s64 = -2094792704;
	// 831903E8: 3C808324  lis r4, -0x7cdc
	ctx.r[4].s64 = -2094792704;
	// 831903EC: 3CA08324  lis r5, -0x7cdc
	ctx.r[5].s64 = -2094792704;
	// 831903F0: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 831903F4: 397A48F0  addi r11, r26, 0x48f0
	ctx.r[11].s64 = ctx.r[26].s64 + 18672;
	// 831903F8: 3CC08324  lis r6, -0x7cdc
	ctx.r[6].s64 = -2094792704;
	// 831903FC: 3CE08324  lis r7, -0x7cdc
	ctx.r[7].s64 = -2094792704;
	// 83190400: 3D008324  lis r8, -0x7cdc
	ctx.r[8].s64 = -2094792704;
	// 83190404: 3D208324  lis r9, -0x7cdc
	ctx.r[9].s64 = -2094792704;
	// 83190408: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 8319040C: 397B4598  addi r11, r27, 0x4598
	ctx.r[11].s64 = ctx.r[27].s64 + 17816;
	// 83190410: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 83190414: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83190418: 397C4598  addi r11, r28, 0x4598
	ctx.r[11].s64 = ctx.r[28].s64 + 17816;
	// 8319041C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83190420: 397D4598  addi r11, r29, 0x4598
	ctx.r[11].s64 = ctx.r[29].s64 + 17816;
	// 83190424: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83190428: 397E4598  addi r11, r30, 0x4598
	ctx.r[11].s64 = ctx.r[30].s64 + 17816;
	// 8319042C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83190430: 39634598  addi r11, r3, 0x4598
	ctx.r[11].s64 = ctx.r[3].s64 + 17816;
	// 83190434: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83190438: 39644598  addi r11, r4, 0x4598
	ctx.r[11].s64 = ctx.r[4].s64 + 17816;
	// 8319043C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83190440: 39654598  addi r11, r5, 0x4598
	ctx.r[11].s64 = ctx.r[5].s64 + 17816;
	// 83190444: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83190448: 39664598  addi r11, r6, 0x4598
	ctx.r[11].s64 = ctx.r[6].s64 + 17816;
	// 8319044C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83190450: 396749B8  addi r11, r7, 0x49b8
	ctx.r[11].s64 = ctx.r[7].s64 + 18872;
	// 83190454: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83190458: 396849B8  addi r11, r8, 0x49b8
	ctx.r[11].s64 = ctx.r[8].s64 + 18872;
	// 8319045C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83190460: 39694AD8  addi r11, r9, 0x4ad8
	ctx.r[11].s64 = ctx.r[9].s64 + 19160;
	// 83190464: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83190468: 396A4BF8  addi r11, r10, 0x4bf8
	ctx.r[11].s64 = ctx.r[10].s64 + 19448;
	// 8319046C: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 83190470: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83190474: 48017D38  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83190478 size=36
    let mut pc: u32 = 0x83190478;
    'dispatch: loop {
        match pc {
            0x83190478 => {
    //   block [0x83190478..0x8319049C)
	// 83190478: 396401BD  addi r11, r4, 0x1bd
	ctx.r[11].s64 = ctx.r[4].s64 + 445;
	// 8319047C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190480: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83190484: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83190488: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8319048C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83190490: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 83190494: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 83190498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831904A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831904A0 size=20
    let mut pc: u32 = 0x831904A0;
    'dispatch: loop {
        match pc {
            0x831904A0 => {
    //   block [0x831904A0..0x831904B4)
	// 831904A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831904A4: 9083150C  stw r4, 0x150c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5388 as u32), ctx.r[4].u32 ) };
	// 831904A8: 90A31510  stw r5, 0x1510(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5392 as u32), ctx.r[5].u32 ) };
	// 831904AC: 91631514  stw r11, 0x1514(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5396 as u32), ctx.r[11].u32 ) };
	// 831904B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831904B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831904B8 size=24
    let mut pc: u32 = 0x831904B8;
    'dispatch: loop {
        match pc {
            0x831904B8 => {
    //   block [0x831904B8..0x831904D0)
	// 831904B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831904BC: 419A000C  beq cr6, 0x831904c8
	if ctx.cr[6].eq {
	pc = 0x831904C8; continue 'dispatch;
	}
	// 831904C0: 8163150C  lwz r11, 0x150c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5388 as u32) ) } as u64;
	// 831904C4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831904C8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831904CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831904D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831904D0 size=12
    let mut pc: u32 = 0x831904D0;
    'dispatch: loop {
        match pc {
            0x831904D0 => {
    //   block [0x831904D0..0x831904DC)
	// 831904D0: 81631514  lwz r11, 0x1514(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5396 as u32) ) } as u64;
	// 831904D4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831904D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831904E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831904E0 size=140
    let mut pc: u32 = 0x831904E0;
    'dispatch: loop {
        match pc {
            0x831904E0 => {
    //   block [0x831904E0..0x8319056C)
	// 831904E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831904E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831904E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831904EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831904F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831904F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831904F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831904FC: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83190500: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83190504: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83190508: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319050C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190510: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83190514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190518: 4E800421  bctrl
	ctx.lr = 0x8319051C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319051C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190520: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83190524: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319052C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83190530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190534: 4E800421  bctrl
	ctx.lr = 0x83190538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190538: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8319053C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83190540: 4098000C  bge cr6, 0x8319054c
	if !ctx.cr[6].lt {
	pc = 0x8319054C; continue 'dispatch;
	}
	// 83190544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83190548: 4800000C  b 0x83190554
	pc = 0x83190554; continue 'dispatch;
	// 8319054C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190550: 480021D9  bl 0x83192728
	ctx.lr = 0x83190554;
	sub_83192728(ctx, base);
	// 83190554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83190558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8319055C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83190560: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83190564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83190568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190570 size=168
    let mut pc: u32 = 0x83190570;
    'dispatch: loop {
        match pc {
            0x83190570 => {
    //   block [0x83190570..0x83190618)
	// 83190570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83190574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83190578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8319057C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83190580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83190588: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8319058C: 807E147C  lwz r3, 0x147c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5244 as u32) ) } as u64;
	// 83190590: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190594: 409A006C  bne cr6, 0x83190600
	if !ctx.cr[6].eq {
	pc = 0x83190600; continue 'dispatch;
	}
	// 83190598: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8319059C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831905A0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831905A4: 48002235  bl 0x831927d8
	ctx.lr = 0x831905A8;
	sub_831927D8(ctx, base);
	// 831905A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831905AC: 419A0050  beq cr6, 0x831905fc
	if ctx.cr[6].eq {
	pc = 0x831905FC; continue 'dispatch;
	}
	// 831905B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831905B4: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 831905B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831905BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 831905C0: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 831905C4: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831905C8: 48002211  bl 0x831927d8
	ctx.lr = 0x831905CC;
	sub_831927D8(ctx, base);
	// 831905CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831905D0: 419A002C  beq cr6, 0x831905fc
	if ctx.cr[6].eq {
	pc = 0x831905FC; continue 'dispatch;
	}
	// 831905D4: 48002155  bl 0x83192728
	ctx.lr = 0x831905D8;
	sub_83192728(ctx, base);
	// 831905D8: 546B06F6  rlwinm r11, r3, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 831905DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831905E0: 419A000C  beq cr6, 0x831905ec
	if ctx.cr[6].eq {
	pc = 0x831905EC; continue 'dispatch;
	}
	// 831905E4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831905E8: 48000010  b 0x831905f8
	pc = 0x831905F8; continue 'dispatch;
	// 831905EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831905F0: 419A000C  beq cr6, 0x831905fc
	if ctx.cr[6].eq {
	pc = 0x831905FC; continue 'dispatch;
	}
	// 831905F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831905F8: 917E147C  stw r11, 0x147c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(5244 as u32), ctx.r[11].u32 ) };
	// 831905FC: 807E147C  lwz r3, 0x147c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5244 as u32) ) } as u64;
	// 83190600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83190604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83190608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8319060C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83190610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83190614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83190618 size=20
    let mut pc: u32 = 0x83190618;
    'dispatch: loop {
        match pc {
            0x83190618 => {
    //   block [0x83190618..0x8319062C)
	// 83190618: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8319061C: 396B9770  addi r11, r11, -0x6890
	ctx.r[11].s64 = ctx.r[11].s64 + -26768;
	// 83190620: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83190624: 9163112C  stw r11, 0x112c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4396 as u32), ctx.r[11].u32 ) };
	// 83190628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83190630 size=24
    let mut pc: u32 = 0x83190630;
    'dispatch: loop {
        match pc {
            0x83190630 => {
    //   block [0x83190630..0x83190648)
	// 83190630: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83190634: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83190638: 396B9780  addi r11, r11, -0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + -26752;
	// 8319063C: 38630900  addi r3, r3, 0x900
	ctx.r[3].s64 = ctx.r[3].s64 + 2304;
	// 83190640: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83190644: 48009E44  b 0x8319a488
	sub_8319A488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83190648 size=32
    let mut pc: u32 = 0x83190648;
    'dispatch: loop {
        match pc {
            0x83190648 => {
    //   block [0x83190648..0x83190668)
	// 83190648: 39630A00  addi r11, r3, 0xa00
	ctx.r[11].s64 = ctx.r[3].s64 + 2560;
	// 8319064C: 3D203F80  lis r9, 0x3f80
	ctx.r[9].s64 = 1065353216;
	// 83190650: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 83190654: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83190658: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8319065C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83190660: 4200FFF8  bdnz 0x83190658
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83190658; continue 'dispatch;
	}
	// 83190664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190668 size=476
    let mut pc: u32 = 0x83190668;
    'dispatch: loop {
        match pc {
            0x83190668 => {
    //   block [0x83190668..0x83190844)
	// 83190668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319066C: 48017AFD  bl 0x831a8168
	ctx.lr = 0x83190670;
	sub_831A8130(ctx, base);
	// 83190670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83190678: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8319067C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83190680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83190684: 3BBF14AC  addi r29, r31, 0x14ac
	ctx.r[29].s64 = ctx.r[31].s64 + 5292;
	// 83190688: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 8319068C: 917F14D8  stw r11, 0x14d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5336 as u32), ctx.r[11].u32 ) };
	// 83190690: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190694: 817F130C  lwz r11, 0x130c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4876 as u32) ) } as u64;
	// 83190698: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8319069C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831906A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831906A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 831906A8: 917F130C  stw r11, 0x130c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4876 as u32), ctx.r[11].u32 ) };
	// 831906AC: 939F151C  stw r28, 0x151c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5404 as u32), ctx.r[28].u32 ) };
	// 831906B0: 939F1520  stw r28, 0x1520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5408 as u32), ctx.r[28].u32 ) };
	// 831906B4: 939F1344  stw r28, 0x1344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4932 as u32), ctx.r[28].u32 ) };
	// 831906B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831906BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831906C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831906C4: 4E800421  bctrl
	ctx.lr = 0x831906C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831906C8: 817F14AC  lwz r11, 0x14ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5292 as u32) ) } as u64;
	// 831906CC: 556A003A  rlwinm r10, r11, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831906D0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831906D4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831906D8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831906DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831906E0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831906E4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831906E8: 419A0008  beq cr6, 0x831906f0
	if ctx.cr[6].eq {
	pc = 0x831906F0; continue 'dispatch;
	}
	// 831906EC: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831906F0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831906F4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 831906F8: 38EA0004  addi r7, r10, 4
	ctx.r[7].s64 = ctx.r[10].s64 + 4;
	// 831906FC: 41980044  blt cr6, 0x83190740
	if ctx.cr[6].lt {
	pc = 0x83190740; continue 'dispatch;
	}
	// 83190700: 396BFFF9  addi r11, r11, -7
	ctx.r[11].s64 = ctx.r[11].s64 + -7;
	// 83190704: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83190708: 419A0024  beq cr6, 0x8319072c
	if ctx.cr[6].eq {
	pc = 0x8319072C; continue 'dispatch;
	}
	// 8319070C: 214B0019  subfic r10, r11, 0x19
	ctx.xer.ca = ctx.r[11].u32 <= 25 as u32;
	ctx.r[10].s64 = (25 as i64) - ctx.r[11].s64;
	// 83190710: 7D0A5430  srw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83190714: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83190718: 7D095830  slw r9, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8319071C: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190720: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190724: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 83190728: 48000024  b 0x8319074c
	pc = 0x8319074C; continue 'dispatch;
	// 8319072C: 552AC9FE  srwi r10, r9, 7
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190730: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 83190734: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190738: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8319073C: 48000010  b 0x8319074c
	pc = 0x8319074C; continue 'dispatch;
	// 83190740: 552AC9FE  srwi r10, r9, 7
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190744: 396B0019  addi r11, r11, 0x19
	ctx.r[11].s64 = ctx.r[11].s64 + 25;
	// 83190748: 5529C80C  slwi r9, r9, 0x19
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(25);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319074C: 554606BE  clrlwi r6, r10, 0x1a
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 83190750: 7D250034  cntlzw r5, r9
	ctx.r[5].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83190754: 554AD1BE  srwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190758: 7CA50034  cntlzw r5, r5
	ctx.r[5].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 8319075C: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 83190760: 90DF1308  stw r6, 0x1308(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4872 as u32), ctx.r[6].u32 ) };
	// 83190764: 54A6DFFE  rlwinm r6, r5, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 83190768: 554506BE  clrlwi r5, r10, 0x1a
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 8319076C: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190770: 90BF1304  stw r5, 0x1304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4868 as u32), ctx.r[5].u32 ) };
	// 83190774: 554506BE  clrlwi r5, r10, 0x1a
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 83190778: 554AD1BE  srwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8319077C: 90BF1300  stw r5, 0x1300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4864 as u32), ctx.r[5].u32 ) };
	// 83190780: 554506FE  clrlwi r5, r10, 0x1b
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83190784: 554AD97E  srwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190788: 90BF12FC  stw r5, 0x12fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4860 as u32), ctx.r[5].u32 ) };
	// 8319078C: 915F12F8  stw r10, 0x12f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4856 as u32), ctx.r[10].u32 ) };
	// 83190790: 90DF13D4  stw r6, 0x13d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5076 as u32), ctx.r[6].u32 ) };
	// 83190794: 409A0014  bne cr6, 0x831907a8
	if !ctx.cr[6].eq {
	pc = 0x831907A8; continue 'dispatch;
	}
	// 83190798: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8319079C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 831907A0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 831907A4: 4800000C  b 0x831907b0
	pc = 0x831907B0; continue 'dispatch;
	// 831907A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831907AC: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831907B0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 831907B4: 2F0B001F  cmpwi cr6, r11, 0x1f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 31, &mut ctx.xer);
	// 831907B8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 831907BC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 831907C0: 915F13D8  stw r10, 0x13d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5080 as u32), ctx.r[10].u32 ) };
	// 831907C4: 409A0010  bne cr6, 0x831907d4
	if !ctx.cr[6].eq {
	pc = 0x831907D4; continue 'dispatch;
	}
	// 831907C8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 831907CC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 831907D0: 48000008  b 0x831907d8
	pc = 0x831907D8; continue 'dispatch;
	// 831907D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831907D8: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 831907DC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831907E0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831907E4: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831907E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831907EC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831907F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831907F4: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 831907F8: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 831907FC: 4BFA4995  bl 0x83135190
	ctx.lr = 0x83190800;
	sub_83135190(ctx, base);
	// 83190800: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190804: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83190808: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8319080C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83190810: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83190814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190818: 4E800421  bctrl
	ctx.lr = 0x8319081C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319081C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190820: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190824: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319082C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83190830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190834: 4E800421  bctrl
	ctx.lr = 0x83190838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190838: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8319083C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83190840: 48017978  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190848 size=1196
    let mut pc: u32 = 0x83190848;
    'dispatch: loop {
        match pc {
            0x83190848 => {
    //   block [0x83190848..0x83190CF4)
	// 83190848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319084C: 48017911  bl 0x831a815c
	ctx.lr = 0x83190850;
	sub_831A8130(ctx, base);
	// 83190850: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83190858: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8319085C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83190860: 3B9F14AC  addi r28, r31, 0x14ac
	ctx.r[28].s64 = ctx.r[31].s64 + 5292;
	// 83190864: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83190868: 3BDF12DC  addi r30, r31, 0x12dc
	ctx.r[30].s64 = ctx.r[31].s64 + 4828;
	// 8319086C: 917F14D8  stw r11, 0x14d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5336 as u32), ctx.r[11].u32 ) };
	// 83190870: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83190874: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190878: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8319087C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83190880: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83190884: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83190888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8319088C: 4E800421  bctrl
	ctx.lr = 0x83190890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190890: 817F14AC  lwz r11, 0x14ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5292 as u32) ) } as u64;
	// 83190894: 556A003A  rlwinm r10, r11, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83190898: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8319089C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831908A0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831908A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831908A8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831908AC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831908B0: 419A0008  beq cr6, 0x831908b8
	if ctx.cr[6].eq {
	pc = 0x831908B8; continue 'dispatch;
	}
	// 831908B4: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831908B8: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831908BC: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 831908C0: 3BAA0004  addi r29, r10, 4
	ctx.r[29].s64 = ctx.r[10].s64 + 4;
	// 831908C4: 4198004C  blt cr6, 0x83190910
	if ctx.cr[6].lt {
	pc = 0x83190910; continue 'dispatch;
	}
	// 831908C8: 394BFFEA  addi r10, r11, -0x16
	ctx.r[10].s64 = ctx.r[11].s64 + -22;
	// 831908CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831908D0: 419A0028  beq cr6, 0x831908f8
	if ctx.cr[6].eq {
	pc = 0x831908F8; continue 'dispatch;
	}
	// 831908D4: 216A000A  subfic r11, r10, 0xa
	ctx.xer.ca = ctx.r[10].u32 <= 10 as u32;
	ctx.r[11].s64 = (10 as i64) - ctx.r[10].s64;
	// 831908D8: 7CAB5C30  srw r11, r5, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831908DC: 7D694B78  or r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 831908E0: 552955BE  srwi r9, r9, 0x16
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(22);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831908E4: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 831908E8: 7CAB5030  slw r11, r5, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 831908EC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831908F0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 831908F4: 4800002C  b 0x83190920
	pc = 0x83190920; continue 'dispatch;
	// 831908F8: 552955BE  srwi r9, r9, 0x16
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(22);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831908FC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83190900: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83190904: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190908: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8319090C: 48000014  b 0x83190920
	pc = 0x83190920; continue 'dispatch;
	// 83190910: 552855BE  srwi r8, r9, 0x16
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(22);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83190914: 394B000A  addi r10, r11, 0xa
	ctx.r[10].s64 = ctx.r[11].s64 + 10;
	// 83190918: 552B502A  slwi r11, r9, 0xa
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319091C: 911E0014  stw r8, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 83190920: 2F0A001D  cmpwi cr6, r10, 0x1d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 29, &mut ctx.xer);
	// 83190924: 41980044  blt cr6, 0x83190968
	if ctx.cr[6].lt {
	pc = 0x83190968; continue 'dispatch;
	}
	// 83190928: 394AFFE3  addi r10, r10, -0x1d
	ctx.r[10].s64 = ctx.r[10].s64 + -29;
	// 8319092C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83190930: 419A0024  beq cr6, 0x83190954
	if ctx.cr[6].eq {
	pc = 0x83190954; continue 'dispatch;
	}
	// 83190934: 212A0003  subfic r9, r10, 3
	ctx.xer.ca = ctx.r[10].u32 <= 3 as u32;
	ctx.r[9].s64 = (3 as i64) - ctx.r[10].s64;
	// 83190938: 7CA94C30  srw r9, r5, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[5].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319093C: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 83190940: 7CAB5030  slw r11, r5, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83190944: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190948: 55291F7E  srwi r9, r9, 0x1d
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(29);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319094C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190950: 48000024  b 0x83190974
	pc = 0x83190974; continue 'dispatch;
	// 83190954: 55691F7E  srwi r9, r11, 0x1d
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(29);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83190958: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8319095C: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190960: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190964: 48000010  b 0x83190974
	pc = 0x83190974; continue 'dispatch;
	// 83190968: 55691F7E  srwi r9, r11, 0x1d
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(29);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319096C: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 83190970: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83190974: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 83190978: 913E0018  stw r9, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8319097C: 4198004C  blt cr6, 0x831909c8
	if ctx.cr[6].lt {
	pc = 0x831909C8; continue 'dispatch;
	}
	// 83190980: 390AFFF0  addi r8, r10, -0x10
	ctx.r[8].s64 = ctx.r[10].s64 + -16;
	// 83190984: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83190988: 419A0028  beq cr6, 0x831909b0
	if ctx.cr[6].eq {
	pc = 0x831909B0; continue 'dispatch;
	}
	// 8319098C: 21480010  subfic r10, r8, 0x10
	ctx.xer.ca = ctx.r[8].u32 <= 16 as u32;
	ctx.r[10].s64 = (16 as i64) - ctx.r[8].s64;
	// 83190990: 7CA74030  slw r7, r5, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[5].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83190994: 7CAA5430  srw r10, r5, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[5].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83190998: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8319099C: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831909A0: 917F13DC  stw r11, 0x13dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5084 as u32), ctx.r[11].u32 ) };
	// 831909A4: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831909A8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 831909AC: 4800002C  b 0x831909d8
	pc = 0x831909D8; continue 'dispatch;
	// 831909B0: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831909B4: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 831909B8: 917F13DC  stw r11, 0x13dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5084 as u32), ctx.r[11].u32 ) };
	// 831909BC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831909C0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 831909C4: 48000014  b 0x831909d8
	pc = 0x831909D8; continue 'dispatch;
	// 831909C8: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831909CC: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 831909D0: 5567801E  slwi r7, r11, 0x10
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831909D4: 913F13DC  stw r9, 0x13dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5084 as u32), ctx.r[9].u32 ) };
	// 831909D8: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 831909DC: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 831909E0: 419A002C  beq cr6, 0x83190a0c
	if ctx.cr[6].eq {
	pc = 0x83190A0C; continue 'dispatch;
	}
	// 831909E4: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 831909E8: 419A0024  beq cr6, 0x83190a0c
	if ctx.cr[6].eq {
	pc = 0x83190A0C; continue 'dispatch;
	}
	// 831909EC: 817F1520  lwz r11, 0x1520(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5408 as u32) ) } as u64;
	// 831909F0: 815F151C  lwz r10, 0x151c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5404 as u32) ) } as u64;
	// 831909F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831909F8: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831909FC: 3D4AFFFF  addis r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -65536;
	// 83190A00: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83190A04: 917F1520  stw r11, 0x1520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5408 as u32), ctx.r[11].u32 ) };
	// 83190A08: 4800001C  b 0x83190a24
	pc = 0x83190A24; continue 'dispatch;
	// 83190A0C: 817F151C  lwz r11, 0x151c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5404 as u32) ) } as u64;
	// 83190A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83190A14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83190A18: 915F1520  stw r10, 0x1520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5408 as u32), ctx.r[10].u32 ) };
	// 83190A1C: 556A801E  slwi r10, r11, 0x10
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190A20: 917F151C  stw r11, 0x151c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5404 as u32), ctx.r[11].u32 ) };
	// 83190A24: 915E0068  stw r10, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 83190A28: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 83190A2C: 419A000C  beq cr6, 0x83190a38
	if ctx.cr[6].eq {
	pc = 0x83190A38; continue 'dispatch;
	}
	// 83190A30: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 83190A34: 409A00A8  bne cr6, 0x83190adc
	if !ctx.cr[6].eq {
	pc = 0x83190ADC; continue 'dispatch;
	}
	// 83190A38: 7CEB0034  cntlzw r11, r7
	ctx.r[11].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 83190A3C: 2F08001F  cmpwi cr6, r8, 0x1f
	ctx.cr[6].compare_i32(ctx.r[8].s32, 31, &mut ctx.xer);
	// 83190A40: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83190A44: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83190A48: 917F1410  stw r11, 0x1410(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5136 as u32), ctx.r[11].u32 ) };
	// 83190A4C: 409A0168  bne cr6, 0x83190bb4
	if !ctx.cr[6].eq {
	pc = 0x83190BB4; continue 'dispatch;
	}
	// 83190A50: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83190A54: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83190A5C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190A60: 390A0003  addi r8, r10, 3
	ctx.r[8].s64 = ctx.r[10].s64 + 3;
	// 83190A64: 55671838  slwi r7, r11, 3
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83190A68: 55691F7E  srwi r9, r11, 0x1d
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(29);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83190A6C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83190A70: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 83190A74: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 83190A78: 214B001B  subfic r10, r11, 0x1b
	ctx.xer.ca = ctx.r[11].u32 <= 27 as u32;
	ctx.r[10].s64 = (27 as i64) - ctx.r[11].s64;
	// 83190A7C: 917F1414  stw r11, 0x1414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5140 as u32), ctx.r[11].u32 ) };
	// 83190A80: 915F1418  stw r10, 0x1418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5144 as u32), ctx.r[10].u32 ) };
	// 83190A84: 7CC95830  slw r9, r6, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[6].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83190A88: 913F141C  stw r9, 0x141c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5148 as u32), ctx.r[9].u32 ) };
	// 83190A8C: 409A0050  bne cr6, 0x83190adc
	if !ctx.cr[6].eq {
	pc = 0x83190ADC; continue 'dispatch;
	}
	// 83190A90: 7CEB0034  cntlzw r11, r7
	ctx.r[11].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 83190A94: 2F08001F  cmpwi cr6, r8, 0x1f
	ctx.cr[6].compare_i32(ctx.r[8].s32, 31, &mut ctx.xer);
	// 83190A98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83190A9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83190AA0: 917F1434  stw r11, 0x1434(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5172 as u32), ctx.r[11].u32 ) };
	// 83190AA4: 409A0158  bne cr6, 0x83190bfc
	if !ctx.cr[6].eq {
	pc = 0x83190BFC; continue 'dispatch;
	}
	// 83190AA8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83190AAC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83190AB4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190AB8: 390A0003  addi r8, r10, 3
	ctx.r[8].s64 = ctx.r[10].s64 + 3;
	// 83190ABC: 55671838  slwi r7, r11, 3
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83190AC0: 55691F7E  srwi r9, r11, 0x1d
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(29);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83190AC4: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 83190AC8: 214B001B  subfic r10, r11, 0x1b
	ctx.xer.ca = ctx.r[11].u32 <= 27 as u32;
	ctx.r[10].s64 = (27 as i64) - ctx.r[11].s64;
	// 83190ACC: 917F1438  stw r11, 0x1438(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5176 as u32), ctx.r[11].u32 ) };
	// 83190AD0: 915F143C  stw r10, 0x143c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5180 as u32), ctx.r[10].u32 ) };
	// 83190AD4: 7CC95830  slw r9, r6, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[6].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83190AD8: 913F1440  stw r9, 0x1440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5184 as u32), ctx.r[9].u32 ) };
	// 83190ADC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83190AE0: 817F12A8  lwz r11, 0x12a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4776 as u32) ) } as u64;
	// 83190AE4: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83190AE8: 392ABB58  addi r9, r10, -0x44a8
	ctx.r[9].s64 = ctx.r[10].s64 + -17576;
	// 83190AEC: 815F12A0  lwz r10, 0x12a0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4768 as u32) ) } as u64;
	// 83190AF0: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 83190AF4: 5546083C  slwi r6, r10, 1
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83190AF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83190AFC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83190B00: 7D23482E  lwzx r9, r3, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83190B04: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83190B08: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83190B0C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83190B10: 396BFD20  addi r11, r11, -0x2e0
	ctx.r[11].s64 = ctx.r[11].s64 + -736;
	// 83190B14: 913F13E0  stw r9, 0x13e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5088 as u32), ctx.r[9].u32 ) };
	// 83190B18: 7D265214  add r9, r6, r10
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 83190B1C: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83190B20: 3BCB00A0  addi r30, r11, 0xa0
	ctx.r[30].s64 = ctx.r[11].s64 + 160;
	// 83190B24: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 83190B28: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190B2C: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 83190B30: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83190B34: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83190B38: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 83190B3C: 386B0078  addi r3, r11, 0x78
	ctx.r[3].s64 = ctx.r[11].s64 + 120;
	// 83190B40: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83190B44: 3B4B0050  addi r26, r11, 0x50
	ctx.r[26].s64 = ctx.r[11].s64 + 80;
	// 83190B48: 7CCAF02E  lwzx r6, r10, r30
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83190B4C: 3B2B00C8  addi r25, r11, 0xc8
	ctx.r[25].s64 = ctx.r[11].s64 + 200;
	// 83190B50: 90DF13E4  stw r6, 0x13e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5092 as u32), ctx.r[6].u32 ) };
	// 83190B54: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83190B58: 917F13F0  stw r11, 0x13f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5104 as u32), ctx.r[11].u32 ) };
	// 83190B5C: 7D6A182E  lwzx r11, r10, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83190B60: 917F13F8  stw r11, 0x13f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5112 as u32), ctx.r[11].u32 ) };
	// 83190B64: 7D6AD02E  lwzx r11, r10, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83190B68: 917F13FC  stw r11, 0x13fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5116 as u32), ctx.r[11].u32 ) };
	// 83190B6C: 7D6AC82E  lwzx r11, r10, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83190B70: 815F13FC  lwz r10, 0x13fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5116 as u32) ) } as u64;
	// 83190B74: 917F1400  stw r11, 0x1400(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5120 as u32), ctx.r[11].u32 ) };
	// 83190B78: 915F13F4  stw r10, 0x13f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5108 as u32), ctx.r[10].u32 ) };
	// 83190B7C: 409800EC  bge cr6, 0x83190c68
	if !ctx.cr[6].lt {
	pc = 0x83190C68; continue 'dispatch;
	}
	// 83190B80: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190B84: 39680007  addi r11, r8, 7
	ctx.r[11].s64 = ctx.r[8].s64 + 7;
	// 83190B88: 80DF14B0  lwz r6, 0x14b0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5296 as u32) ) } as u64;
	// 83190B8C: 39080009  addi r8, r8, 9
	ctx.r[8].s64 = ctx.r[8].s64 + 9;
	// 83190B90: 396B0009  addi r11, r11, 9
	ctx.r[11].s64 = ctx.r[11].s64 + 9;
	// 83190B94: 2F080020  cmpwi cr6, r8, 0x20
	ctx.cr[6].compare_i32(ctx.r[8].s32, 32, &mut ctx.xer);
	// 83190B98: 419800AC  blt cr6, 0x83190c44
	if ctx.cr[6].lt {
	pc = 0x83190C44; continue 'dispatch;
	}
	// 83190B9C: 3908FFE0  addi r8, r8, -0x20
	ctx.r[8].s64 = ctx.r[8].s64 + -32;
	// 83190BA0: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83190BA4: 7CA74030  slw r7, r5, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[5].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83190BA8: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190BAC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190BB0: 48000098  b 0x83190c48
	pc = 0x83190C48; continue 'dispatch;
	// 83190BB4: 39480001  addi r10, r8, 1
	ctx.r[10].s64 = ctx.r[8].s64 + 1;
	// 83190BB8: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83190BBC: 2F0A001D  cmpwi cr6, r10, 0x1d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 29, &mut ctx.xer);
	// 83190BC0: 4198FEA0  blt cr6, 0x83190a60
	if ctx.cr[6].lt {
	pc = 0x83190A60; continue 'dispatch;
	}
	// 83190BC4: 390AFFE3  addi r8, r10, -0x1d
	ctx.r[8].s64 = ctx.r[10].s64 + -29;
	// 83190BC8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83190BCC: 419A0020  beq cr6, 0x83190bec
	if ctx.cr[6].eq {
	pc = 0x83190BEC; continue 'dispatch;
	}
	// 83190BD0: 21480003  subfic r10, r8, 3
	ctx.xer.ca = ctx.r[8].u32 <= 3 as u32;
	ctx.r[10].s64 = (3 as i64) - ctx.r[8].s64;
	// 83190BD4: 7CA74030  slw r7, r5, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[5].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83190BD8: 7CAA5430  srw r10, r5, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[5].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83190BDC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190BE0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190BE4: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83190BE8: 4BFFFE80  b 0x83190a68
	pc = 0x83190A68; continue 'dispatch;
	// 83190BEC: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83190BF0: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190BF4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190BF8: 4BFFFE70  b 0x83190a68
	pc = 0x83190A68; continue 'dispatch;
	// 83190BFC: 39480001  addi r10, r8, 1
	ctx.r[10].s64 = ctx.r[8].s64 + 1;
	// 83190C00: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83190C04: 2F0A001D  cmpwi cr6, r10, 0x1d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 29, &mut ctx.xer);
	// 83190C08: 4198FEB0  blt cr6, 0x83190ab8
	if ctx.cr[6].lt {
	pc = 0x83190AB8; continue 'dispatch;
	}
	// 83190C0C: 390AFFE3  addi r8, r10, -0x1d
	ctx.r[8].s64 = ctx.r[10].s64 + -29;
	// 83190C10: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83190C14: 419A0020  beq cr6, 0x83190c34
	if ctx.cr[6].eq {
	pc = 0x83190C34; continue 'dispatch;
	}
	// 83190C18: 21480003  subfic r10, r8, 3
	ctx.xer.ca = ctx.r[8].u32 <= 3 as u32;
	ctx.r[10].s64 = (3 as i64) - ctx.r[8].s64;
	// 83190C1C: 7CA74030  slw r7, r5, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[5].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83190C20: 7CAA5430  srw r10, r5, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[5].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83190C24: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190C28: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190C2C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83190C30: 4BFFFE90  b 0x83190ac0
	pc = 0x83190AC0; continue 'dispatch;
	// 83190C34: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83190C38: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190C3C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190C40: 4BFFFE80  b 0x83190ac0
	pc = 0x83190AC0; continue 'dispatch;
	// 83190C44: 54E7482C  slwi r7, r7, 9
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(9);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83190C48: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83190C4C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83190C50: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 83190C54: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 83190C58: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83190C5C: 4099008C  ble cr6, 0x83190ce8
	if !ctx.cr[6].gt {
	pc = 0x83190CE8; continue 'dispatch;
	}
	// 83190C60: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83190C64: 4198FF28  blt cr6, 0x83190b8c
	if ctx.cr[6].lt {
	pc = 0x83190B8C; continue 'dispatch;
	}
	// 83190C68: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 83190C6C: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83190C70: 4198000C  blt cr6, 0x83190c7c
	if ctx.cr[6].lt {
	pc = 0x83190C7C; continue 'dispatch;
	}
	// 83190C74: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83190C78: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83190C7C: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 83190C80: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190C84: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83190C88: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83190C8C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83190C90: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83190C94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83190C98: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83190C9C: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 83190CA0: 4BFA44F1  bl 0x83135190
	ctx.lr = 0x83190CA4;
	sub_83135190(ctx, base);
	// 83190CA4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190CA8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83190CAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83190CB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83190CB4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83190CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190CBC: 4E800421  bctrl
	ctx.lr = 0x83190CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190CC0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190CC4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190CC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190CCC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83190CD0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83190CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190CD8: 4E800421  bctrl
	ctx.lr = 0x83190CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190CDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83190CE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83190CE4: 480174C8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83190CE8: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 83190CEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83190CF0: 480174BC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190CF8 size=268
    let mut pc: u32 = 0x83190CF8;
    'dispatch: loop {
        match pc {
            0x83190CF8 => {
    //   block [0x83190CF8..0x83190E04)
	// 83190CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83190CFC: 48017459  bl 0x831a8154
	ctx.lr = 0x83190D00;
	sub_831A8130(ctx, base);
	// 83190D00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190D04: 3B45FFFC  addi r26, r5, -4
	ctx.r[26].s64 = ctx.r[5].s64 + -4;
	// 83190D08: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83190D0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83190D10: 3BC40004  addi r30, r4, 4
	ctx.r[30].s64 = ctx.r[4].s64 + 4;
	// 83190D14: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 83190D18: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83190D1C: 409900C0  ble cr6, 0x83190ddc
	if !ctx.cr[6].gt {
	pc = 0x83190DDC; continue 'dispatch;
	}
	// 83190D20: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83190D24: 3B000003  li r24, 3
	ctx.r[24].s64 = 3;
	// 83190D28: 3B2BBB74  addi r25, r11, -0x448c
	ctx.r[25].s64 = ctx.r[11].s64 + -17548;
	// 83190D2C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83190D30: 3B6BBB6C  addi r27, r11, -0x4494
	ctx.r[27].s64 = ctx.r[11].s64 + -17556;
	// 83190D34: 7F9FF214  add r28, r31, r30
	ctx.r[28].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83190D38: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 83190D3C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83190D40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83190D44: 4801C08D  bl 0x831acdd0
	ctx.lr = 0x83190D48;
	sub_831ACDD0(ctx, base);
	// 83190D48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190D4C: 409A0024  bne cr6, 0x83190d70
	if !ctx.cr[6].eq {
	pc = 0x83190D70; continue 'dispatch;
	}
	// 83190D50: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83190D54: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83190D58: 4801B8F9  bl 0x831ac650
	ctx.lr = 0x83190D5C;
	sub_831AC650(ctx, base);
	// 83190D5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190D60: 409A000C  bne cr6, 0x83190d6c
	if !ctx.cr[6].eq {
	pc = 0x83190D6C; continue 'dispatch;
	}
	// 83190D64: 92FD14B8  stw r23, 0x14b8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(5304 as u32), ctx.r[23].u32 ) };
	// 83190D68: 48000008  b 0x83190d70
	pc = 0x83190D70; continue 'dispatch;
	// 83190D6C: 931D14B8  stw r24, 0x14b8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(5304 as u32), ctx.r[24].u32 ) };
	// 83190D70: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 83190D74: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83190D78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83190D7C: 4801C055  bl 0x831acdd0
	ctx.lr = 0x83190D80;
	sub_831ACDD0(ctx, base);
	// 83190D80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190D84: 409A003C  bne cr6, 0x83190dc0
	if !ctx.cr[6].eq {
	pc = 0x83190DC0; continue 'dispatch;
	}
	// 83190D88: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83190D8C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 83190D90: 4801B8C1  bl 0x831ac650
	ctx.lr = 0x83190D94;
	sub_831AC650(ctx, base);
	// 83190D94: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83190D98: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83190D9C: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 83190DA0: 915D1480  stw r10, 0x1480(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(5248 as u32), ctx.r[10].u32 ) };
	// 83190DA4: 4801B8AD  bl 0x831ac650
	ctx.lr = 0x83190DA8;
	sub_831AC650(ctx, base);
	// 83190DA8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83190DAC: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83190DB0: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 83190DB4: 915D1484  stw r10, 0x1484(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(5252 as u32), ctx.r[10].u32 ) };
	// 83190DB8: 4801B899  bl 0x831ac650
	ctx.lr = 0x83190DBC;
	sub_831AC650(ctx, base);
	// 83190DBC: 907D1488  stw r3, 0x1488(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(5256 as u32), ctx.r[3].u32 ) };
	// 83190DC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83190DC4: 48001965  bl 0x83192728
	ctx.lr = 0x83190DC8;
	sub_83192728(ctx, base);
	// 83190DC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83190DCC: 409A0010  bne cr6, 0x83190ddc
	if !ctx.cr[6].eq {
	pc = 0x83190DDC; continue 'dispatch;
	}
	// 83190DD0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83190DD4: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 83190DD8: 4198FF5C  blt cr6, 0x83190d34
	if ctx.cr[6].lt {
	pc = 0x83190D34; continue 'dispatch;
	}
	// 83190DDC: 817D1480  lwz r11, 0x1480(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(5248 as u32) ) } as u64;
	// 83190DE0: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83190DE4: 409A0008  bne cr6, 0x83190dec
	if !ctx.cr[6].eq {
	pc = 0x83190DEC; continue 'dispatch;
	}
	// 83190DE8: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 83190DEC: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 83190DF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83190DF4: 419A0008  beq cr6, 0x83190dfc
	if ctx.cr[6].eq {
	pc = 0x83190DFC; continue 'dispatch;
	}
	// 83190DF8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83190DFC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83190E00: 480173A4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190E08 size=356
    let mut pc: u32 = 0x83190E08;
    'dispatch: loop {
        match pc {
            0x83190E08 => {
    //   block [0x83190E08..0x83190F6C)
	// 83190E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83190E0C: 48017361  bl 0x831a816c
	ctx.lr = 0x83190E10;
	sub_831A8130(ctx, base);
	// 83190E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190E14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83190E18: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83190E1C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83190E20: 615DFFFF  ori r29, r10, 0xffff
	ctx.r[29].u64 = ctx.r[10].u64 | 65535;
	// 83190E24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190E28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190E2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83190E30: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83190E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190E38: 4E800421  bctrl
	ctx.lr = 0x83190E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190E3C: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83190E40: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83190E44: 41980098  blt cr6, 0x83190edc
	if ctx.cr[6].lt {
	pc = 0x83190EDC; continue 'dispatch;
	}
	// 83190E48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83190E4C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83190E50: 48001989  bl 0x831927d8
	ctx.lr = 0x83190E54;
	sub_831927D8(ctx, base);
	// 83190E54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83190E58: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83190E5C: 409A00A8  bne cr6, 0x83190f04
	if !ctx.cr[6].eq {
	pc = 0x83190F04; continue 'dispatch;
	}
	// 83190E60: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83190E64: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83190E68: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190E6C: 388BFFFD  addi r4, r11, -3
	ctx.r[4].s64 = ctx.r[11].s64 + -3;
	// 83190E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83190E74: 4BFA431D  bl 0x83135190
	ctx.lr = 0x83190E78;
	sub_83135190(ctx, base);
	// 83190E78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190E7C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190E80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83190E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190E88: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83190E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190E90: 4E800421  bctrl
	ctx.lr = 0x83190E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190E94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190E98: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83190E9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190EA4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83190EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190EAC: 4E800421  bctrl
	ctx.lr = 0x83190EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190EB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190EB4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83190EB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83190EBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190EC4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83190EC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190ECC: 4E800421  bctrl
	ctx.lr = 0x83190ED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190ED0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83190ED4: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83190ED8: 4098FF70  bge cr6, 0x83190e48
	if !ctx.cr[6].lt {
	pc = 0x83190E48; continue 'dispatch;
	}
	// 83190EDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190EE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190EE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190EE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190EEC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83190EF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190EF4: 4E800421  bctrl
	ctx.lr = 0x83190EF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190EF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83190EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83190F00: 480172BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83190F04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83190F08: 48001821  bl 0x83192728
	ctx.lr = 0x83190F0C;
	sub_83192728(ctx, base);
	// 83190F0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83190F10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83190F14: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83190F18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190F1C: 7C8BF050  subf r4, r11, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83190F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83190F24: 4BFA426D  bl 0x83135190
	ctx.lr = 0x83190F28;
	sub_83135190(ctx, base);
	// 83190F28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190F2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190F30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83190F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190F38: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83190F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190F40: 4E800421  bctrl
	ctx.lr = 0x83190F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190F44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190F48: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83190F4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190F54: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83190F58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190F5C: 4E800421  bctrl
	ctx.lr = 0x83190F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190F60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83190F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83190F68: 48017254  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83190F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83190F70 size=376
    let mut pc: u32 = 0x83190F70;
    'dispatch: loop {
        match pc {
            0x83190F70 => {
    //   block [0x83190F70..0x831910E8)
	// 83190F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83190F74: 480171F5  bl 0x831a8168
	ctx.lr = 0x83190F78;
	sub_831A8130(ctx, base);
	// 83190F78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83190F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83190F80: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83190F84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83190F88: 615DFFFF  ori r29, r10, 0xffff
	ctx.r[29].u64 = ctx.r[10].u64 | 65535;
	// 83190F8C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83190F90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190F94: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83190F98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83190F9C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83190FA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190FA4: 4E800421  bctrl
	ctx.lr = 0x83190FA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83190FA8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83190FAC: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83190FB0: 41980098  blt cr6, 0x83191048
	if ctx.cr[6].lt {
	pc = 0x83191048; continue 'dispatch;
	}
	// 83190FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83190FB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83190FBC: 4800181D  bl 0x831927d8
	ctx.lr = 0x83190FC0;
	sub_831927D8(ctx, base);
	// 83190FC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83190FC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83190FC8: 409A00AC  bne cr6, 0x83191074
	if !ctx.cr[6].eq {
	pc = 0x83191074; continue 'dispatch;
	}
	// 83190FCC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83190FD0: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83190FD4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190FD8: 388BFFFD  addi r4, r11, -3
	ctx.r[4].s64 = ctx.r[11].s64 + -3;
	// 83190FDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83190FE0: 4BFA41B1  bl 0x83135190
	ctx.lr = 0x83190FE4;
	sub_83135190(ctx, base);
	// 83190FE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83190FE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83190FEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83190FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83190FF4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83190FF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83190FFC: 4E800421  bctrl
	ctx.lr = 0x83191000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191000: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191004: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83191008: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8319100C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191010: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83191014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191018: 4E800421  bctrl
	ctx.lr = 0x8319101C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319101C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191020: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83191024: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83191028: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8319102C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191030: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83191034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191038: 4E800421  bctrl
	ctx.lr = 0x8319103C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319103C: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83191040: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83191044: 4098FF70  bge cr6, 0x83190fb4
	if !ctx.cr[6].lt {
	pc = 0x83190FB4; continue 'dispatch;
	}
	// 83191048: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319104C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83191050: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83191054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191058: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8319105C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191060: 4E800421  bctrl
	ctx.lr = 0x83191064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191064: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83191068: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8319106C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83191070: 48017148  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83191074: 897E0003  lbz r11, 3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(3 as u32) ) } as u64;
	// 83191078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319107C: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 83191080: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83191084: 480016A5  bl 0x83192728
	ctx.lr = 0x83191088;
	sub_83192728(ctx, base);
	// 83191088: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8319108C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83191090: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83191094: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83191098: 7C8BF050  subf r4, r11, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8319109C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831910A0: 4BFA40F1  bl 0x83135190
	ctx.lr = 0x831910A4;
	sub_83135190(ctx, base);
	// 831910A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831910A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831910AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831910B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831910B4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831910B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831910BC: 4E800421  bctrl
	ctx.lr = 0x831910C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831910C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831910C4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831910C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831910CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831910D0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831910D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831910D8: 4E800421  bctrl
	ctx.lr = 0x831910DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831910DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831910E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831910E4: 480170D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831910E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831910E8 size=108
    let mut pc: u32 = 0x831910E8;
    'dispatch: loop {
        match pc {
            0x831910E8 => {
    //   block [0x831910E8..0x83191154)
	// 831910E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831910EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831910F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831910F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831910F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831910FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83191100: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83191104: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83191108: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319110C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83191110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191114: 4E800421  bctrl
	ctx.lr = 0x83191118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191118: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319111C: 7FCA0034  cntlzw r10, r30
	ctx.r[10].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 83191120: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83191124: 5544DFFE  rlwinm r4, r10, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83191128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319112C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83191130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191134: 4E800421  bctrl
	ctx.lr = 0x83191138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191138: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8319113C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83191140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83191144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83191148: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8319114C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83191150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83191158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83191158 size=76
    let mut pc: u32 = 0x83191158;
    'dispatch: loop {
        match pc {
            0x83191158 => {
    //   block [0x83191158..0x831911A4)
	// 83191158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319115C: 4801700D  bl 0x831a8168
	ctx.lr = 0x83191160;
	sub_831A8130(ctx, base);
	// 83191160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83191164: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83191168: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8319116C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83191170: 4BFFFF79  bl 0x831910e8
	ctx.lr = 0x83191174;
	sub_831910E8(ctx, base);
	// 83191174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83191178: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8319117C: 4198000C  blt cr6, 0x83191188
	if ctx.cr[6].lt {
	pc = 0x83191188; continue 'dispatch;
	}
	// 83191180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83191184: 48017034  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83191188: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8319118C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83191190: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83191194: 4BFFFF55  bl 0x831910e8
	ctx.lr = 0x83191198;
	sub_831910E8(ctx, base);
	// 83191198: 7C63FA14  add r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8319119C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831911A0: 48017018  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831911A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831911A8 size=3332
    let mut pc: u32 = 0x831911A8;
    'dispatch: loop {
        match pc {
            0x831911A8 => {
    //   block [0x831911A8..0x83191EAC)
	// 831911A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831911AC: 48016FA9  bl 0x831a8154
	ctx.lr = 0x831911B0;
	sub_831A8130(ctx, base);
	// 831911B0: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 831911B4: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831911B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831911BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831911C0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 831911C4: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 831911C8: 917C14D8  stw r11, 0x14d8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5336 as u32), ctx.r[11].u32 ) };
	// 831911CC: 937C1480  stw r27, 0x1480(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5248 as u32), ctx.r[27].u32 ) };
	// 831911D0: 937C14B8  stw r27, 0x14b8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5304 as u32), ctx.r[27].u32 ) };
	// 831911D4: 4BFFF445  bl 0x83190618
	ctx.lr = 0x831911D8;
	sub_83190618(ctx, base);
	// 831911D8: 3B3C12DC  addi r25, r28, 0x12dc
	ctx.r[25].s64 = ctx.r[28].s64 + 4828;
	// 831911DC: 3AFC14AC  addi r23, r28, 0x14ac
	ctx.r[23].s64 = ctx.r[28].s64 + 5292;
	// 831911E0: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 831911E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831911E8: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 831911EC: 81790034  lwz r11, 0x34(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(52 as u32) ) } as u64;
	// 831911F0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831911F4: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 831911F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831911FC: 91790034  stw r11, 0x34(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83191200: 937C151C  stw r27, 0x151c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5404 as u32), ctx.r[27].u32 ) };
	// 83191204: 937C1520  stw r27, 0x1520(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5408 as u32), ctx.r[27].u32 ) };
	// 83191208: 93790068  stw r27, 0x68(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8319120C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191210: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83191214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191218: 4E800421  bctrl
	ctx.lr = 0x8319121C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319121C: 817C14AC  lwz r11, 0x14ac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5292 as u32) ) } as u64;
	// 83191220: 556A003A  rlwinm r10, r11, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83191224: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83191228: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8319122C: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191230: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 83191234: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83191238: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319123C: 7D2B5030  slw r11, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191240: 409A0008  bne cr6, 0x83191248
	if !ctx.cr[6].eq {
	pc = 0x83191248; continue 'dispatch;
	}
	// 83191244: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83191248: 83E80000  lwz r31, 0(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319124C: 2F0A0014  cmpwi cr6, r10, 0x14
	ctx.cr[6].compare_i32(ctx.r[10].s32, 20, &mut ctx.xer);
	// 83191250: 3B480004  addi r26, r8, 4
	ctx.r[26].s64 = ctx.r[8].s64 + 4;
	// 83191254: 4198004C  blt cr6, 0x831912a0
	if ctx.cr[6].lt {
	pc = 0x831912A0; continue 'dispatch;
	}
	// 83191258: 394AFFEC  addi r10, r10, -0x14
	ctx.r[10].s64 = ctx.r[10].s64 + -20;
	// 8319125C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83191260: 419A0028  beq cr6, 0x83191288
	if ctx.cr[6].eq {
	pc = 0x83191288; continue 'dispatch;
	}
	// 83191264: 212A000C  subfic r9, r10, 0xc
	ctx.xer.ca = ctx.r[10].u32 <= 12 as u32;
	ctx.r[9].s64 = (12 as i64) - ctx.r[10].s64;
	// 83191268: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319126C: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 83191270: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191274: 5529653E  srwi r9, r9, 0x14
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191278: 91390000  stw r9, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8319127C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191280: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191284: 4800002C  b 0x831912b0
	pc = 0x831912B0; continue 'dispatch;
	// 83191288: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319128C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83191290: 91390000  stw r9, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83191294: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191298: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8319129C: 48000014  b 0x831912b0
	pc = 0x831912B0; continue 'dispatch;
	// 831912A0: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831912A4: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 831912A8: 556B6026  slwi r11, r11, 0xc
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(12);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831912AC: 91390000  stw r9, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831912B0: 2F0A0014  cmpwi cr6, r10, 0x14
	ctx.cr[6].compare_i32(ctx.r[10].s32, 20, &mut ctx.xer);
	// 831912B4: 4198004C  blt cr6, 0x83191300
	if ctx.cr[6].lt {
	pc = 0x83191300; continue 'dispatch;
	}
	// 831912B8: 394AFFEC  addi r10, r10, -0x14
	ctx.r[10].s64 = ctx.r[10].s64 + -20;
	// 831912BC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831912C0: 419A0028  beq cr6, 0x831912e8
	if ctx.cr[6].eq {
	pc = 0x831912E8; continue 'dispatch;
	}
	// 831912C4: 212A000C  subfic r9, r10, 0xc
	ctx.xer.ca = ctx.r[10].u32 <= 12 as u32;
	ctx.r[9].s64 = (12 as i64) - ctx.r[10].s64;
	// 831912C8: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831912CC: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 831912D0: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 831912D4: 5529653E  srwi r9, r9, 0x14
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831912D8: 91390004  stw r9, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831912DC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831912E0: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831912E4: 4800002C  b 0x83191310
	pc = 0x83191310; continue 'dispatch;
	// 831912E8: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831912EC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831912F0: 91390004  stw r9, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831912F4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831912F8: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831912FC: 48000014  b 0x83191310
	pc = 0x83191310; continue 'dispatch;
	// 83191300: 5569653E  srwi r9, r11, 0x14
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(20);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191304: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 83191308: 556B6026  slwi r11, r11, 0xc
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(12);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319130C: 91390004  stw r9, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83191310: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 83191314: 4198004C  blt cr6, 0x83191360
	if ctx.cr[6].lt {
	pc = 0x83191360; continue 'dispatch;
	}
	// 83191318: 394AFFE4  addi r10, r10, -0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + -28;
	// 8319131C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83191320: 419A0028  beq cr6, 0x83191348
	if ctx.cr[6].eq {
	pc = 0x83191348; continue 'dispatch;
	}
	// 83191324: 212A0004  subfic r9, r10, 4
	ctx.xer.ca = ctx.r[10].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[10].s64;
	// 83191328: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319132C: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 83191330: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191334: 5529273E  srwi r9, r9, 0x1c
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(28);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191338: 913C13C4  stw r9, 0x13c4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5060 as u32), ctx.r[9].u32 ) };
	// 8319133C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191340: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191344: 4800002C  b 0x83191370
	pc = 0x83191370; continue 'dispatch;
	// 83191348: 5569273E  srwi r9, r11, 0x1c
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319134C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83191350: 913C13C4  stw r9, 0x13c4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5060 as u32), ctx.r[9].u32 ) };
	// 83191354: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191358: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8319135C: 48000014  b 0x83191370
	pc = 0x83191370; continue 'dispatch;
	// 83191360: 5569273E  srwi r9, r11, 0x1c
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191364: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83191368: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319136C: 913C13C4  stw r9, 0x13c4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5060 as u32), ctx.r[9].u32 ) };
	// 83191370: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 83191374: 41980044  blt cr6, 0x831913b8
	if ctx.cr[6].lt {
	pc = 0x831913B8; continue 'dispatch;
	}
	// 83191378: 394AFFE4  addi r10, r10, -0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + -28;
	// 8319137C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83191380: 419A0024  beq cr6, 0x831913a4
	if ctx.cr[6].eq {
	pc = 0x831913A4; continue 'dispatch;
	}
	// 83191384: 212A0004  subfic r9, r10, 4
	ctx.xer.ca = ctx.r[10].u32 <= 4 as u32;
	ctx.r[9].s64 = (4 as i64) - ctx.r[10].s64;
	// 83191388: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319138C: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 83191390: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191394: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191398: 5529273E  srwi r9, r9, 0x1c
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(28);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319139C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831913A0: 48000024  b 0x831913c4
	pc = 0x831913C4; continue 'dispatch;
	// 831913A4: 5569273E  srwi r9, r11, 0x1c
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831913A8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831913AC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831913B0: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831913B4: 48000010  b 0x831913c4
	pc = 0x831913C4; continue 'dispatch;
	// 831913B8: 5569273E  srwi r9, r11, 0x1c
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831913BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831913C0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831913C4: 2F0A000E  cmpwi cr6, r10, 0xe
	ctx.cr[6].compare_i32(ctx.r[10].s32, 14, &mut ctx.xer);
	// 831913C8: 91390010  stw r9, 0x10(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 831913CC: 4198004C  blt cr6, 0x83191418
	if ctx.cr[6].lt {
	pc = 0x83191418; continue 'dispatch;
	}
	// 831913D0: 394AFFF2  addi r10, r10, -0xe
	ctx.r[10].s64 = ctx.r[10].s64 + -14;
	// 831913D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831913D8: 419A0028  beq cr6, 0x83191400
	if ctx.cr[6].eq {
	pc = 0x83191400; continue 'dispatch;
	}
	// 831913DC: 212A0012  subfic r9, r10, 0x12
	ctx.xer.ca = ctx.r[10].u32 <= 18 as u32;
	ctx.r[9].s64 = (18 as i64) - ctx.r[10].s64;
	// 831913E0: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831913E4: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 831913E8: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 831913EC: 552993BE  srwi r9, r9, 0xe
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831913F0: 913C13C8  stw r9, 0x13c8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5064 as u32), ctx.r[9].u32 ) };
	// 831913F4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831913F8: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831913FC: 4800002C  b 0x83191428
	pc = 0x83191428; continue 'dispatch;
	// 83191400: 556993BE  srwi r9, r11, 0xe
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191404: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83191408: 913C13C8  stw r9, 0x13c8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5064 as u32), ctx.r[9].u32 ) };
	// 8319140C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191410: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191414: 48000014  b 0x83191428
	pc = 0x83191428; continue 'dispatch;
	// 83191418: 556993BE  srwi r9, r11, 0xe
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319141C: 394A0012  addi r10, r10, 0x12
	ctx.r[10].s64 = ctx.r[10].s64 + 18;
	// 83191420: 556B901A  slwi r11, r11, 0x12
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(18);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83191424: 913C13C8  stw r9, 0x13c8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5064 as u32), ctx.r[9].u32 ) };
	// 83191428: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8319142C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 83191430: 41980018  blt cr6, 0x83191448
	if ctx.cr[6].lt {
	pc = 0x83191448; continue 'dispatch;
	}
	// 83191434: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 83191438: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8319143C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191440: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191444: 48000008  b 0x8319144c
	pc = 0x8319144C; continue 'dispatch;
	// 83191448: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319144C: 2F0A0016  cmpwi cr6, r10, 0x16
	ctx.cr[6].compare_i32(ctx.r[10].s32, 22, &mut ctx.xer);
	// 83191450: 4198004C  blt cr6, 0x8319149c
	if ctx.cr[6].lt {
	pc = 0x8319149C; continue 'dispatch;
	}
	// 83191454: 394AFFEA  addi r10, r10, -0x16
	ctx.r[10].s64 = ctx.r[10].s64 + -22;
	// 83191458: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8319145C: 419A0028  beq cr6, 0x83191484
	if ctx.cr[6].eq {
	pc = 0x83191484; continue 'dispatch;
	}
	// 83191460: 212A000A  subfic r9, r10, 0xa
	ctx.xer.ca = ctx.r[10].u32 <= 10 as u32;
	ctx.r[9].s64 = (10 as i64) - ctx.r[10].s64;
	// 83191464: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83191468: 7D295B78  or r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 8319146C: 7FEB5030  slw r11, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191470: 552955BE  srwi r9, r9, 0x16
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(22);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191474: 913C13CC  stw r9, 0x13cc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5068 as u32), ctx.r[9].u32 ) };
	// 83191478: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319147C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191480: 4800002C  b 0x831914ac
	pc = 0x831914AC; continue 'dispatch;
	// 83191484: 556955BE  srwi r9, r11, 0x16
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(22);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191488: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8319148C: 913C13CC  stw r9, 0x13cc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5068 as u32), ctx.r[9].u32 ) };
	// 83191490: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191494: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191498: 48000014  b 0x831914ac
	pc = 0x831914AC; continue 'dispatch;
	// 8319149C: 556955BE  srwi r9, r11, 0x16
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(22);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831914A0: 394A000A  addi r10, r10, 0xa
	ctx.r[10].s64 = ctx.r[10].s64 + 10;
	// 831914A4: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831914A8: 913C13CC  stw r9, 0x13cc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5068 as u32), ctx.r[9].u32 ) };
	// 831914AC: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831914B0: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 831914B4: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 831914B8: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 831914BC: 913C13D0  stw r9, 0x13d0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5072 as u32), ctx.r[9].u32 ) };
	// 831914C0: 409A0018  bne cr6, 0x831914d8
	if !ctx.cr[6].eq {
	pc = 0x831914D8; continue 'dispatch;
	}
	// 831914C4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831914C8: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831914CC: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831914D0: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831914D4: 4800000C  b 0x831914e0
	pc = 0x831914E0; continue 'dispatch;
	// 831914D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831914DC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831914E0: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831914E4: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 831914E8: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 831914EC: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 831914F0: 409A0018  bne cr6, 0x83191508
	if !ctx.cr[6].eq {
	pc = 0x83191508; continue 'dispatch;
	}
	// 831914F4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 831914F8: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831914FC: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 83191500: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191504: 4800000C  b 0x83191510
	pc = 0x83191510; continue 'dispatch;
	// 83191508: 3BAA0001  addi r29, r10, 1
	ctx.r[29].s64 = ctx.r[10].s64 + 1;
	// 8319150C: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83191510: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83191514: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83191518: C3EBD910  lfs f31, -0x26f0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9968 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8319151C: 419A0454  beq cr6, 0x83191970
	if ctx.cr[6].eq {
	pc = 0x83191970; continue 'dispatch;
	}
	// 83191520: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 83191524: 2F1D0018  cmpwi cr6, r29, 0x18
	ctx.cr[6].compare_i32(ctx.r[29].s32, 24, &mut ctx.xer);
	// 83191528: 41980044  blt cr6, 0x8319156c
	if ctx.cr[6].lt {
	pc = 0x8319156C; continue 'dispatch;
	}
	// 8319152C: 397DFFE8  addi r11, r29, -0x18
	ctx.r[11].s64 = ctx.r[29].s64 + -24;
	// 83191530: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191534: 419A0024  beq cr6, 0x83191558
	if ctx.cr[6].eq {
	pc = 0x83191558; continue 'dispatch;
	}
	// 83191538: 214B0008  subfic r10, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[10].s64 = (8 as i64) - ctx.r[11].s64;
	// 8319153C: 7FEA5430  srw r10, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191540: 7D49F378  or r9, r10, r30
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	// 83191544: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191548: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8319154C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191550: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191554: 48000024  b 0x83191578
	pc = 0x83191578; continue 'dispatch;
	// 83191558: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8319155C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191560: 57C9463E  srwi r9, r30, 0x18
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191564: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191568: 48000010  b 0x83191578
	pc = 0x83191578; continue 'dispatch;
	// 8319156C: 57C9463E  srwi r9, r30, 0x18
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191570: 397D0008  addi r11, r29, 8
	ctx.r[11].s64 = ctx.r[29].s64 + 8;
	// 83191574: 57CA402E  slwi r10, r30, 8
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191578: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 8319157C: 80FC112C  lwz r7, 0x112c(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191580: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191584: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 83191588: 7D2740AE  lbzx r9, r7, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8319158C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191590: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 83191594: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191598: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8319159C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831915A0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 831915A4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831915A8: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 831915AC: 41980044  blt cr6, 0x831915f0
	if ctx.cr[6].lt {
	pc = 0x831915F0; continue 'dispatch;
	}
	// 831915B0: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 831915B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831915B8: 419A0024  beq cr6, 0x831915dc
	if ctx.cr[6].eq {
	pc = 0x831915DC; continue 'dispatch;
	}
	// 831915BC: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 831915C0: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831915C4: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831915C8: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831915CC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831915D0: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831915D4: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831915D8: 48000024  b 0x831915fc
	pc = 0x831915FC; continue 'dispatch;
	// 831915DC: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831915E0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 831915E4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831915E8: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831915EC: 48000010  b 0x831915fc
	pc = 0x831915FC; continue 'dispatch;
	// 831915F0: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831915F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831915F8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831915FC: 79270020  clrldi r7, r9, 0x20
	ctx.r[7].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191600: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191604: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191608: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8319160C: F8E10058  std r7, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u64 ) };
	// 83191610: 89290001  lbz r9, 1(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 83191614: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191618: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 8319161C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191620: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83191624: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191628: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8319162C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191630: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191634: 41980044  blt cr6, 0x83191678
	if ctx.cr[6].lt {
	pc = 0x83191678; continue 'dispatch;
	}
	// 83191638: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 8319163C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191640: 419A0024  beq cr6, 0x83191664
	if ctx.cr[6].eq {
	pc = 0x83191664; continue 'dispatch;
	}
	// 83191644: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191648: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319164C: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191650: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191654: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191658: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319165C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191660: 48000024  b 0x83191684
	pc = 0x83191684; continue 'dispatch;
	// 83191664: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191668: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8319166C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191670: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191674: 48000010  b 0x83191684
	pc = 0x83191684; continue 'dispatch;
	// 83191678: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319167C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191680: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191684: 79270020  clrldi r7, r9, 0x20
	ctx.r[7].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191688: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 8319168C: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191690: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83191694: F8E10060  std r7, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u64 ) };
	// 83191698: 89290002  lbz r9, 2(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 8319169C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 831916A0: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 831916A4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831916A8: C8010060  lfd f0, 0x60(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 831916AC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831916B0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 831916B4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831916B8: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 831916BC: 41980044  blt cr6, 0x83191700
	if ctx.cr[6].lt {
	pc = 0x83191700; continue 'dispatch;
	}
	// 831916C0: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 831916C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831916C8: 419A0024  beq cr6, 0x831916ec
	if ctx.cr[6].eq {
	pc = 0x831916EC; continue 'dispatch;
	}
	// 831916CC: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 831916D0: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831916D4: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831916D8: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831916DC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831916E0: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831916E4: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831916E8: 48000024  b 0x8319170c
	pc = 0x8319170C; continue 'dispatch;
	// 831916EC: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831916F0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 831916F4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831916F8: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831916FC: 48000010  b 0x8319170c
	pc = 0x8319170C; continue 'dispatch;
	// 83191700: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191704: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191708: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8319170C: 79270020  clrldi r7, r9, 0x20
	ctx.r[7].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191710: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191714: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191718: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8319171C: F8E10068  std r7, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u64 ) };
	// 83191720: 89290003  lbz r9, 3(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 83191724: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191728: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 8319172C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191730: C8010068  lfd f0, 0x68(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83191734: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191738: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8319173C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191740: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191744: 41980044  blt cr6, 0x83191788
	if ctx.cr[6].lt {
	pc = 0x83191788; continue 'dispatch;
	}
	// 83191748: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 8319174C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191750: 419A0024  beq cr6, 0x83191774
	if ctx.cr[6].eq {
	pc = 0x83191774; continue 'dispatch;
	}
	// 83191754: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191758: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319175C: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191760: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191764: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191768: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319176C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191770: 48000024  b 0x83191794
	pc = 0x83191794; continue 'dispatch;
	// 83191774: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191778: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8319177C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191780: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191784: 48000010  b 0x83191794
	pc = 0x83191794; continue 'dispatch;
	// 83191788: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319178C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191790: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191794: 79270020  clrldi r7, r9, 0x20
	ctx.r[7].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191798: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 8319179C: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 831917A0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 831917A4: F8E10070  std r7, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u64 ) };
	// 831917A8: 89290004  lbz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 831917AC: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 831917B0: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 831917B4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831917B8: C8010070  lfd f0, 0x70(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 831917BC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831917C0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 831917C4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831917C8: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 831917CC: 41980044  blt cr6, 0x83191810
	if ctx.cr[6].lt {
	pc = 0x83191810; continue 'dispatch;
	}
	// 831917D0: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 831917D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831917D8: 419A0024  beq cr6, 0x831917fc
	if ctx.cr[6].eq {
	pc = 0x831917FC; continue 'dispatch;
	}
	// 831917DC: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 831917E0: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831917E4: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 831917E8: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831917EC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831917F0: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831917F4: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831917F8: 48000024  b 0x8319181c
	pc = 0x8319181C; continue 'dispatch;
	// 831917FC: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191800: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191804: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191808: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8319180C: 48000010  b 0x8319181c
	pc = 0x8319181C; continue 'dispatch;
	// 83191810: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191814: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191818: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8319181C: 79270020  clrldi r7, r9, 0x20
	ctx.r[7].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191820: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191824: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191828: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8319182C: F8E10078  std r7, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[7].u64 ) };
	// 83191830: 89290005  lbz r9, 5(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 83191834: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191838: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 8319183C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191840: C8010078  lfd f0, 0x78(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 83191844: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191848: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8319184C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191850: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191854: 41980044  blt cr6, 0x83191898
	if ctx.cr[6].lt {
	pc = 0x83191898; continue 'dispatch;
	}
	// 83191858: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 8319185C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191860: 419A0024  beq cr6, 0x83191884
	if ctx.cr[6].eq {
	pc = 0x83191884; continue 'dispatch;
	}
	// 83191864: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191868: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8319186C: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191870: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191874: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191878: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319187C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191880: 48000024  b 0x831918a4
	pc = 0x831918A4; continue 'dispatch;
	// 83191884: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191888: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8319188C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191890: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191894: 48000010  b 0x831918a4
	pc = 0x831918A4; continue 'dispatch;
	// 83191898: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8319189C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831918A0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831918A4: 79270020  clrldi r7, r9, 0x20
	ctx.r[7].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 831918A8: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 831918AC: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 831918B0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 831918B4: F8E10080  std r7, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[7].u64 ) };
	// 831918B8: 89290006  lbz r9, 6(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 831918BC: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 831918C0: 39290240  addi r9, r9, 0x240
	ctx.r[9].s64 = ctx.r[9].s64 + 576;
	// 831918C4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831918C8: C8010080  lfd f0, 0x80(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	// 831918CC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 831918D0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 831918D4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831918D8: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 831918DC: 41980044  blt cr6, 0x83191920
	if ctx.cr[6].lt {
	pc = 0x83191920; continue 'dispatch;
	}
	// 831918E0: 3BABFFE8  addi r29, r11, -0x18
	ctx.r[29].s64 = ctx.r[11].s64 + -24;
	// 831918E4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831918E8: 419A0024  beq cr6, 0x8319190c
	if ctx.cr[6].eq {
	pc = 0x8319190C; continue 'dispatch;
	}
	// 831918EC: 217D0008  subfic r11, r29, 8
	ctx.xer.ca = ctx.r[29].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[29].s64;
	// 831918F0: 7FFEE830  slw r30, r31, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[31].u32) << ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 831918F4: 7FEB5C30  srw r11, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 831918F8: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831918FC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191900: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83191904: 5569463E  srwi r9, r11, 0x18
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191908: 48000024  b 0x8319192c
	pc = 0x8319192C; continue 'dispatch;
	// 8319190C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 83191910: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191914: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191918: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8319191C: 48000010  b 0x8319192c
	pc = 0x8319192C; continue 'dispatch;
	// 83191920: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191924: 3BAB0008  addi r29, r11, 8
	ctx.r[29].s64 = ctx.r[11].s64 + 8;
	// 83191928: 555E402E  slwi r30, r10, 8
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8319192C: 792A0020  clrldi r10, r9, 0x20
	ctx.r[10].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191930: 817C112C  lwz r11, 0x112c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191934: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 83191938: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 8319193C: F9410088  std r10, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u64 ) };
	// 83191940: C8010088  lfd f0, 0x88(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	// 83191944: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191948: 2F080040  cmpwi cr6, r8, 0x40
	ctx.cr[6].compare_i32(ctx.r[8].s32, 64, &mut ctx.xer);
	// 8319194C: 896B0007  lbz r11, 7(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 83191950: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83191954: 396B0240  addi r11, r11, 0x240
	ctx.r[11].s64 = ctx.r[11].s64 + 576;
	// 83191958: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8319195C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191960: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191964: 7C0BE52E  stfsx f0, r11, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191968: 4198FBBC  blt cr6, 0x83191524
	if ctx.cr[6].lt {
	pc = 0x83191524; continue 'dispatch;
	}
	// 8319196C: 4800000C  b 0x83191978
	pc = 0x83191978; continue 'dispatch;
	// 83191970: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83191974: 4BFFECBD  bl 0x83190630
	ctx.lr = 0x83191978;
	sub_83190630(ctx, base);
	// 83191978: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 8319197C: 2F1D001F  cmpwi cr6, r29, 0x1f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 31, &mut ctx.xer);
	// 83191980: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83191984: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83191988: 409A0018  bne cr6, 0x831919a0
	if !ctx.cr[6].eq {
	pc = 0x831919A0; continue 'dispatch;
	}
	// 8319198C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83191990: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191994: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 83191998: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8319199C: 4800000C  b 0x831919a8
	pc = 0x831919A8; continue 'dispatch;
	// 831919A0: 391D0001  addi r8, r29, 1
	ctx.r[8].s64 = ctx.r[29].s64 + 1;
	// 831919A4: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831919A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831919AC: 419A043C  beq cr6, 0x83191de8
	if ctx.cr[6].eq {
	pc = 0x83191DE8; continue 'dispatch;
	}
	// 831919B0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 831919B4: 2F080018  cmpwi cr6, r8, 0x18
	ctx.cr[6].compare_i32(ctx.r[8].s32, 24, &mut ctx.xer);
	// 831919B8: 4198003C  blt cr6, 0x831919f4
	if ctx.cr[6].lt {
	pc = 0x831919F4; continue 'dispatch;
	}
	// 831919BC: 3948FFE8  addi r10, r8, -0x18
	ctx.r[10].s64 = ctx.r[8].s64 + -24;
	// 831919C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831919C4: 419A0020  beq cr6, 0x831919e4
	if ctx.cr[6].eq {
	pc = 0x831919E4; continue 'dispatch;
	}
	// 831919C8: 212A0008  subfic r9, r10, 8
	ctx.xer.ca = ctx.r[10].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[10].s64;
	// 831919CC: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 831919D0: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 831919D4: 7FE95030  slw r9, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 831919D8: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831919DC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831919E0: 4800001C  b 0x831919fc
	pc = 0x831919FC; continue 'dispatch;
	// 831919E4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 831919E8: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831919EC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 831919F0: 4800000C  b 0x831919fc
	pc = 0x831919FC; continue 'dispatch;
	// 831919F4: 39480008  addi r10, r8, 8
	ctx.r[10].s64 = ctx.r[8].s64 + 8;
	// 831919F8: 5569402E  slwi r9, r11, 8
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831919FC: 5567463E  srwi r7, r11, 0x18
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(24);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83191A00: 811C112C  lwz r8, 0x112c(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191A04: 2F0A0018  cmpwi cr6, r10, 0x18
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24, &mut ctx.xer);
	// 83191A08: 78EB0020  clrldi r11, r7, 0x20
	ctx.r[11].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 83191A0C: F9610088  std r11, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u64 ) };
	// 83191A10: C8010088  lfd f0, 0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	// 83191A14: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191A18: 7D6830AE  lbzx r11, r8, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 83191A1C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83191A20: 396B0280  addi r11, r11, 0x280
	ctx.r[11].s64 = ctx.r[11].s64 + 640;
	// 83191A24: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83191A28: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191A2C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191A30: 7C0BE52E  stfsx f0, r11, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191A34: 4198003C  blt cr6, 0x83191a70
	if ctx.cr[6].lt {
	pc = 0x83191A70; continue 'dispatch;
	}
	// 83191A38: 396AFFE8  addi r11, r10, -0x18
	ctx.r[11].s64 = ctx.r[10].s64 + -24;
	// 83191A3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191A40: 419A0020  beq cr6, 0x83191a60
	if ctx.cr[6].eq {
	pc = 0x83191A60; continue 'dispatch;
	}
	// 83191A44: 214B0008  subfic r10, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[10].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191A48: 7FEA5430  srw r10, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83191A4C: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83191A50: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191A54: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191A58: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191A5C: 4800001C  b 0x83191a78
	pc = 0x83191A78; continue 'dispatch;
	// 83191A60: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191A64: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191A68: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191A6C: 4800000C  b 0x83191a78
	pc = 0x83191A78; continue 'dispatch;
	// 83191A70: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 83191A74: 552A402E  slwi r10, r9, 8
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191A78: 5528463E  srwi r8, r9, 0x18
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83191A7C: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191A80: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191A84: 79080020  clrldi r8, r8, 0x20
	ctx.r[8].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 83191A88: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83191A8C: F9010080  std r8, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[8].u64 ) };
	// 83191A90: 89290001  lbz r9, 1(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 83191A94: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191A98: 39290280  addi r9, r9, 0x280
	ctx.r[9].s64 = ctx.r[9].s64 + 640;
	// 83191A9C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191AA0: C8010080  lfd f0, 0x80(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	// 83191AA4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191AA8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191AAC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191AB0: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191AB4: 41980044  blt cr6, 0x83191af8
	if ctx.cr[6].lt {
	pc = 0x83191AF8; continue 'dispatch;
	}
	// 83191AB8: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 83191ABC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191AC0: 419A0024  beq cr6, 0x83191ae4
	if ctx.cr[6].eq {
	pc = 0x83191AE4; continue 'dispatch;
	}
	// 83191AC4: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191AC8: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83191ACC: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191AD0: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191AD4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191AD8: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191ADC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191AE0: 48000024  b 0x83191b04
	pc = 0x83191B04; continue 'dispatch;
	// 83191AE4: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191AE8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191AEC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191AF0: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191AF4: 48000010  b 0x83191b04
	pc = 0x83191B04; continue 'dispatch;
	// 83191AF8: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191AFC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191B00: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191B04: 79280020  clrldi r8, r9, 0x20
	ctx.r[8].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191B08: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191B0C: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191B10: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83191B14: F9010078  std r8, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u64 ) };
	// 83191B18: 89290002  lbz r9, 2(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 83191B1C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191B20: 39290280  addi r9, r9, 0x280
	ctx.r[9].s64 = ctx.r[9].s64 + 640;
	// 83191B24: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191B28: C8010078  lfd f0, 0x78(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 83191B2C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191B30: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191B34: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191B38: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191B3C: 41980044  blt cr6, 0x83191b80
	if ctx.cr[6].lt {
	pc = 0x83191B80; continue 'dispatch;
	}
	// 83191B40: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 83191B44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191B48: 419A0024  beq cr6, 0x83191b6c
	if ctx.cr[6].eq {
	pc = 0x83191B6C; continue 'dispatch;
	}
	// 83191B4C: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191B50: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83191B54: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191B58: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191B5C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191B60: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191B64: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191B68: 48000024  b 0x83191b8c
	pc = 0x83191B8C; continue 'dispatch;
	// 83191B6C: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191B70: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191B74: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191B78: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191B7C: 48000010  b 0x83191b8c
	pc = 0x83191B8C; continue 'dispatch;
	// 83191B80: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191B84: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191B88: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191B8C: 79280020  clrldi r8, r9, 0x20
	ctx.r[8].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191B90: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191B94: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191B98: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83191B9C: F9010070  std r8, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u64 ) };
	// 83191BA0: 89290003  lbz r9, 3(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 83191BA4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191BA8: 39290280  addi r9, r9, 0x280
	ctx.r[9].s64 = ctx.r[9].s64 + 640;
	// 83191BAC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191BB0: C8010070  lfd f0, 0x70(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 83191BB4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191BB8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191BBC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191BC0: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191BC4: 41980044  blt cr6, 0x83191c08
	if ctx.cr[6].lt {
	pc = 0x83191C08; continue 'dispatch;
	}
	// 83191BC8: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 83191BCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191BD0: 419A0024  beq cr6, 0x83191bf4
	if ctx.cr[6].eq {
	pc = 0x83191BF4; continue 'dispatch;
	}
	// 83191BD4: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191BD8: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83191BDC: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191BE0: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191BE4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191BE8: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191BEC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191BF0: 48000024  b 0x83191c14
	pc = 0x83191C14; continue 'dispatch;
	// 83191BF4: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191BF8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191BFC: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191C00: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191C04: 48000010  b 0x83191c14
	pc = 0x83191C14; continue 'dispatch;
	// 83191C08: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191C0C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191C10: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191C14: 79280020  clrldi r8, r9, 0x20
	ctx.r[8].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191C18: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191C1C: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191C20: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83191C24: F9010068  std r8, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u64 ) };
	// 83191C28: 89290004  lbz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83191C2C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191C30: 39290280  addi r9, r9, 0x280
	ctx.r[9].s64 = ctx.r[9].s64 + 640;
	// 83191C34: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191C38: C8010068  lfd f0, 0x68(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83191C3C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191C40: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191C44: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191C48: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191C4C: 41980044  blt cr6, 0x83191c90
	if ctx.cr[6].lt {
	pc = 0x83191C90; continue 'dispatch;
	}
	// 83191C50: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 83191C54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191C58: 419A0024  beq cr6, 0x83191c7c
	if ctx.cr[6].eq {
	pc = 0x83191C7C; continue 'dispatch;
	}
	// 83191C5C: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191C60: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83191C64: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191C68: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191C6C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191C70: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191C74: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191C78: 48000024  b 0x83191c9c
	pc = 0x83191C9C; continue 'dispatch;
	// 83191C7C: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191C80: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191C84: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191C88: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191C8C: 48000010  b 0x83191c9c
	pc = 0x83191C9C; continue 'dispatch;
	// 83191C90: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191C94: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191C98: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191C9C: 79280020  clrldi r8, r9, 0x20
	ctx.r[8].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191CA0: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191CA4: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191CA8: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83191CAC: F9010060  std r8, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 83191CB0: 89290005  lbz r9, 5(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 83191CB4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191CB8: 39290280  addi r9, r9, 0x280
	ctx.r[9].s64 = ctx.r[9].s64 + 640;
	// 83191CBC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191CC0: C8010060  lfd f0, 0x60(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83191CC4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191CC8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191CCC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191CD0: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191CD4: 41980044  blt cr6, 0x83191d18
	if ctx.cr[6].lt {
	pc = 0x83191D18; continue 'dispatch;
	}
	// 83191CD8: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 83191CDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83191CE0: 419A0024  beq cr6, 0x83191d04
	if ctx.cr[6].eq {
	pc = 0x83191D04; continue 'dispatch;
	}
	// 83191CE4: 212B0008  subfic r9, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[11].s64;
	// 83191CE8: 7FE94C30  srw r9, r31, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[31].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 83191CEC: 7D295378  or r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83191CF0: 7FEA5830  slw r10, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[31].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191CF4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191CF8: 5529463E  srwi r9, r9, 0x18
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191CFC: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191D00: 48000024  b 0x83191d24
	pc = 0x83191D24; continue 'dispatch;
	// 83191D04: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191D08: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83191D0C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191D10: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191D14: 48000010  b 0x83191d24
	pc = 0x83191D24; continue 'dispatch;
	// 83191D18: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191D1C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83191D20: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191D24: 79280020  clrldi r8, r9, 0x20
	ctx.r[8].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191D28: 813C112C  lwz r9, 0x112c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191D2C: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83191D30: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83191D34: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 83191D38: 89290006  lbz r9, 6(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 83191D3C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83191D40: 39290280  addi r9, r9, 0x280
	ctx.r[9].s64 = ctx.r[9].s64 + 640;
	// 83191D44: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191D48: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83191D4C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191D50: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191D54: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191D58: 7C09E52E  stfsx f0, r9, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191D5C: 4198003C  blt cr6, 0x83191d98
	if ctx.cr[6].lt {
	pc = 0x83191D98; continue 'dispatch;
	}
	// 83191D60: 390BFFE8  addi r8, r11, -0x18
	ctx.r[8].s64 = ctx.r[11].s64 + -24;
	// 83191D64: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83191D68: 419A0020  beq cr6, 0x83191d88
	if ctx.cr[6].eq {
	pc = 0x83191D88; continue 'dispatch;
	}
	// 83191D6C: 21680008  subfic r11, r8, 8
	ctx.xer.ca = ctx.r[8].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[8].s64;
	// 83191D70: 7FEB5C30  srw r11, r31, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 83191D74: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83191D78: 7FEB4030  slw r11, r31, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[31].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 83191D7C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191D80: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191D84: 4800001C  b 0x83191da0
	pc = 0x83191DA0; continue 'dispatch;
	// 83191D88: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83191D8C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191D90: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83191D94: 4800000C  b 0x83191da0
	pc = 0x83191DA0; continue 'dispatch;
	// 83191D98: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 83191D9C: 554B402E  slwi r11, r10, 8
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83191DA0: 5549463E  srwi r9, r10, 0x18
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83191DA4: 815C112C  lwz r10, 0x112c(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4396 as u32) ) } as u64;
	// 83191DA8: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 83191DAC: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 83191DB0: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 83191DB4: 2F060040  cmpwi cr6, r6, 0x40
	ctx.cr[6].compare_i32(ctx.r[6].s32, 64, &mut ctx.xer);
	// 83191DB8: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 83191DBC: 894A0007  lbz r10, 7(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(7 as u32) ) } as u64;
	// 83191DC0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83191DC4: 394A0280  addi r10, r10, 0x280
	ctx.r[10].s64 = ctx.r[10].s64 + 640;
	// 83191DC8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191DCC: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83191DD0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83191DD4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83191DD8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 83191DDC: 7C0AE52E  stfsx f0, r10, r28
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), tmp.u32) };
	// 83191DE0: 4198FBD4  blt cr6, 0x831919b4
	if ctx.cr[6].lt {
	pc = 0x831919B4; continue 'dispatch;
	}
	// 83191DE4: 4800000C  b 0x83191df0
	pc = 0x83191DF0; continue 'dispatch;
	// 83191DE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83191DEC: 4BFFE85D  bl 0x83190648
	ctx.lr = 0x83191DF0;
	sub_83190648(ctx, base);
	// 83191DF0: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191DF4: 39280007  addi r9, r8, 7
	ctx.r[9].s64 = ctx.r[8].s64 + 7;
	// 83191DF8: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 83191DFC: 38C10088  addi r6, r1, 0x88
	ctx.r[6].s64 = ctx.r[1].s64 + 136;
	// 83191E00: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 83191E04: 390B000F  addi r8, r11, 0xf
	ctx.r[8].s64 = ctx.r[11].s64 + 15;
	// 83191E08: 7D4B2670  srawi r11, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 83191E0C: 7D0A2670  srawi r10, r8, 4
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[8].s32 >> 4) as i64;
	// 83191E10: 7D281E70  srawi r8, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 83191E14: 7D2A59D6  mullw r9, r10, r11
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83191E18: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83191E1C: 9159000C  stw r10, 0xc(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83191E20: 817C13C8  lwz r11, 0x13c8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5064 as u32) ) } as u64;
	// 83191E24: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 83191E28: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83191E2C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83191E30: 915C1464  stw r10, 0x1464(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(5220 as u32), ctx.r[10].u32 ) };
	// 83191E34: 91790048  stw r11, 0x48(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83191E38: 817C13CC  lwz r11, 0x13cc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5068 as u32) ) } as u64;
	// 83191E3C: 9179004C  stw r11, 0x4c(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83191E40: 817C13C4  lwz r11, 0x13c4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5060 as u32) ) } as u64;
	// 83191E44: 99790059  stb r11, 0x59(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 83191E48: 817C13D0  lwz r11, 0x13d0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5072 as u32) ) } as u64;
	// 83191E4C: 9979005A  stb r11, 0x5a(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 83191E50: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191E54: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 83191E58: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 83191E5C: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 83191E60: 4BFA3331  bl 0x83135190
	ctx.lr = 0x83191E64;
	sub_83135190(ctx, base);
	// 83191E64: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191E68: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83191E6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83191E70: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83191E74: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83191E78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191E7C: 4E800421  bctrl
	ctx.lr = 0x83191E80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191E80: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191E84: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 83191E88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83191E8C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83191E90: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83191E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191E98: 4E800421  bctrl
	ctx.lr = 0x83191E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191E9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83191EA0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83191EA4: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 83191EA8: 480162FC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83191EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83191EB0 size=196
    let mut pc: u32 = 0x83191EB0;
    'dispatch: loop {
        match pc {
            0x83191EB0 => {
    //   block [0x83191EB0..0x83191F74)
	// 83191EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83191EB4: 480162B9  bl 0x831a816c
	ctx.lr = 0x83191EB8;
	sub_831A8130(ctx, base);
	// 83191EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83191EBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83191EC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83191EC4: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83191EC8: 3BDD14AC  addi r30, r29, 0x14ac
	ctx.r[30].s64 = ctx.r[29].s64 + 5292;
	// 83191ECC: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83191ED0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191ED4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83191ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191EDC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83191EE0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83191EE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191EE8: 4E800421  bctrl
	ctx.lr = 0x83191EEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191EEC: 817D14AC  lwz r11, 0x14ac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(5292 as u32) ) } as u64;
	// 83191EF0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83191EF4: 556A003A  rlwinm r10, r11, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83191EF8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83191EFC: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83191F00: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 83191F04: 550A1838  slwi r10, r8, 3
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191F08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83191F0C: 394A0007  addi r10, r10, 7
	ctx.r[10].s64 = ctx.r[10].s64 + 7;
	// 83191F10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83191F14: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 83191F18: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83191F1C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83191F20: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83191F24: 4BFA326D  bl 0x83135190
	ctx.lr = 0x83191F28;
	sub_83135190(ctx, base);
	// 83191F28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191F2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83191F30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83191F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191F38: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83191F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191F40: 4E800421  bctrl
	ctx.lr = 0x83191F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191F44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83191F48: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83191F4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83191F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191F54: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83191F58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83191F5C: 4E800421  bctrl
	ctx.lr = 0x83191F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83191F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83191F64: 4BFFEEA5  bl 0x83190e08
	ctx.lr = 0x83191F68;
	sub_83190E08(ctx, base);
	// 83191F68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83191F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83191F70: 4801624C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83191F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83191F78 size=428
    let mut pc: u32 = 0x83191F78;
    'dispatch: loop {
        match pc {
            0x83191F78 => {
    //   block [0x83191F78..0x83192124)
	// 83191F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83191F7C: 480161E1  bl 0x831a815c
	ctx.lr = 0x83191F80;
	sub_831A8130(ctx, base);
	// 83191F80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83191F84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83191F88: 3BE5FFFD  addi r31, r5, -3
	ctx.r[31].s64 = ctx.r[5].s64 + -3;
	// 83191F8C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83191F90: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83191F94: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83191F98: 839D14D8  lwz r28, 0x14d8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(5336 as u32) ) } as u64;
	// 83191F9C: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 83191FA0: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83191FA4: 40990020  ble cr6, 0x83191fc4
	if !ctx.cr[6].gt {
	pc = 0x83191FC4; continue 'dispatch;
	}
	// 83191FA8: 7C7EDA14  add r3, r30, r27
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 83191FAC: 4800077D  bl 0x83192728
	ctx.lr = 0x83191FB0;
	sub_83192728(ctx, base);
	// 83191FB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83191FB4: 409A0010  bne cr6, 0x83191fc4
	if !ctx.cr[6].eq {
	pc = 0x83191FC4; continue 'dispatch;
	}
	// 83191FB8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83191FBC: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83191FC0: 4198FFE8  blt cr6, 0x83191fa8
	if ctx.cr[6].lt {
	pc = 0x83191FA8; continue 'dispatch;
	}
	// 83191FC4: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83191FC8: 409A0008  bne cr6, 0x83191fd0
	if !ctx.cr[6].eq {
	pc = 0x83191FD0; continue 'dispatch;
	}
	// 83191FCC: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 83191FD0: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 83191FD4: 409A0018  bne cr6, 0x83191fec
	if !ctx.cr[6].eq {
	pc = 0x83191FEC; continue 'dispatch;
	}
	// 83191FD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83191FDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83191FE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83191FE4: 4BFFED15  bl 0x83190cf8
	ctx.lr = 0x83191FE8;
	sub_83190CF8(ctx, base);
	// 83191FE8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83191FEC: 397C01BD  addi r11, r28, 0x1bd
	ctx.r[11].s64 = ctx.r[28].s64 + 445;
	// 83191FF0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83191FF4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83191FF8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83191FFC: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83192000: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83192004: 419A00D8  beq cr6, 0x831920dc
	if ctx.cr[6].eq {
	pc = 0x831920DC; continue 'dispatch;
	}
	// 83192008: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319200C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83192010: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83192014: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83192018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319201C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83192020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83192024: 4E800421  bctrl
	ctx.lr = 0x83192028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83192028: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8319202C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83192030: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83192034: 480164DD  bl 0x831a8510
	ctx.lr = 0x83192038;
	sub_831A8510(ctx, base);
	// 83192038: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319203C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83192040: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83192044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83192048: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8319204C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83192050: 4E800421  bctrl
	ctx.lr = 0x83192054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83192054: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83192058: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8319205C: 40980054  bge cr6, 0x831920b0
	if !ctx.cr[6].lt {
	pc = 0x831920B0; continue 'dispatch;
	}
	// 83192060: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192064: 7CABF050  subf r5, r11, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83192068: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8319206C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83192070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83192074: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 83192078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8319207C: 4E800421  bctrl
	ctx.lr = 0x83192080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83192080: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83192084: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83192088: 7C8BDA14  add r4, r11, r27
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8319208C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83192090: 48016481  bl 0x831a8510
	ctx.lr = 0x83192094;
	sub_831A8510(ctx, base);
	// 83192094: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192098: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8319209C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831920A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831920A4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831920A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831920AC: 4E800421  bctrl
	ctx.lr = 0x831920B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831920B0: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831920B4: 7D7C5A14  add r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 831920B8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831920BC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 831920C0: 814B14E0  lwz r10, 0x14e0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5344 as u32) ) } as u64;
	// 831920C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831920C8: 419A0014  beq cr6, 0x831920dc
	if ctx.cr[6].eq {
	pc = 0x831920DC; continue 'dispatch;
	}
	// 831920CC: 806B14E4  lwz r3, 0x14e4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5348 as u32) ) } as u64;
	// 831920D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831920D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831920D8: 4E800421  bctrl
	ctx.lr = 0x831920DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831920DC: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 831920E0: 409A002C  bne cr6, 0x8319210c
	if !ctx.cr[6].eq {
	pc = 0x8319210C; continue 'dispatch;
	}
	// 831920E4: 807D150C  lwz r3, 0x150c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(5388 as u32) ) } as u64;
	// 831920E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831920EC: 419A0020  beq cr6, 0x8319210c
	if ctx.cr[6].eq {
	pc = 0x8319210C; continue 'dispatch;
	}
	// 831920F0: 80BD1510  lwz r5, 0x1510(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(5392 as u32) ) } as u64;
	// 831920F4: 7F1E2800  cmpw cr6, r30, r5
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[5].s32, &mut ctx.xer);
	// 831920F8: 40980008  bge cr6, 0x83192100
	if !ctx.cr[6].lt {
	pc = 0x83192100; continue 'dispatch;
	}
	// 831920FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83192100: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83192104: 90BD1514  stw r5, 0x1514(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(5396 as u32), ctx.r[5].u32 ) };
	// 83192108: 48016409  bl 0x831a8510
	ctx.lr = 0x8319210C;
	sub_831A8510(ctx, base);
	// 8319210C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83192110: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83192114: 409A0008  bne cr6, 0x8319211c
	if !ctx.cr[6].eq {
	pc = 0x8319211C; continue 'dispatch;
	}
	// 83192118: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8319211C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83192120: 4801608C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192128 size=212
    let mut pc: u32 = 0x83192128;
    'dispatch: loop {
        match pc {
            0x83192128 => {
    //   block [0x83192128..0x831921FC)
	// 83192128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8319212C: 48016041  bl 0x831a816c
	ctx.lr = 0x83192130;
	sub_831A8130(ctx, base);
	// 83192130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83192134: 812314A4  lwz r9, 0x14a4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5284 as u32) ) } as u64;
	// 83192138: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8319213C: 81431294  lwz r10, 0x1294(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4756 as u32) ) } as u64;
	// 83192140: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83192144: 3963135C  addi r11, r3, 0x135c
	ctx.r[11].s64 = ctx.r[3].s64 + 4956;
	// 83192148: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8319214C: 419A0044  beq cr6, 0x83192190
	if ctx.cr[6].eq {
	pc = 0x83192190; continue 'dispatch;
	}
	// 83192150: 812314A8  lwz r9, 0x14a8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5288 as u32) ) } as u64;
	// 83192154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83192158: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8319215C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83192160: 910314A4  stw r8, 0x14a4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5284 as u32), ctx.r[8].u32 ) };
	// 83192164: 912314A8  stw r9, 0x14a8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(5288 as u32), ctx.r[9].u32 ) };
	// 83192168: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8319216C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83192170: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 83192174: 409A0010  bne cr6, 0x83192184
	if !ctx.cr[6].eq {
	pc = 0x83192184; continue 'dispatch;
	}
	// 83192178: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8319217C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83192180: 4801603C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83192184: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83192188: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8319218C: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83192190: 7D4B0034  cntlzw r11, r10
	ctx.r[11].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83192194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83192198: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8319219C: 3BCBFFFD  addi r30, r11, -3
	ctx.r[30].s64 = ctx.r[11].s64 + -3;
	// 831921A0: 4BFFEC69  bl 0x83190e08
	ctx.lr = 0x831921A4;
	sub_83190E08(ctx, base);
	// 831921A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831921A8: 419A0048  beq cr6, 0x831921f0
	if ctx.cr[6].eq {
	pc = 0x831921F0; continue 'dispatch;
	}
	// 831921AC: 7C6BE838  and r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[29].u64;
	// 831921B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831921B4: 409A0038  bne cr6, 0x831921ec
	if !ctx.cr[6].eq {
	pc = 0x831921EC; continue 'dispatch;
	}
	// 831921B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 831921BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831921C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831921C4: 4BFFEF25  bl 0x831910e8
	ctx.lr = 0x831921C8;
	sub_831910E8(ctx, base);
	// 831921C8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831921CC: 409A0024  bne cr6, 0x831921f0
	if !ctx.cr[6].eq {
	pc = 0x831921F0; continue 'dispatch;
	}
	// 831921D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831921D4: 4BFFEC35  bl 0x83190e08
	ctx.lr = 0x831921D8;
	sub_83190E08(ctx, base);
	// 831921D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831921DC: 409AFFD0  bne cr6, 0x831921ac
	if !ctx.cr[6].eq {
	pc = 0x831921AC; continue 'dispatch;
	}
	// 831921E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831921E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831921E8: 48015FD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831921EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831921F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831921F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831921F8: 48015FC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192200 size=212
    let mut pc: u32 = 0x83192200;
    'dispatch: loop {
        match pc {
            0x83192200 => {
    //   block [0x83192200..0x831922D4)
	// 83192200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83192204: 48015F61  bl 0x831a8164
	ctx.lr = 0x83192208;
	sub_831A8130(ctx, base);
	// 83192208: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319220C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83192210: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83192214: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83192218: 3BBE14AC  addi r29, r30, 0x14ac
	ctx.r[29].s64 = ctx.r[30].s64 + 5292;
	// 8319221C: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83192220: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83192228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319222C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83192230: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83192234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83192238: 4E800421  bctrl
	ctx.lr = 0x8319223C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319223C: 809E14AC  lwz r4, 0x14ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5292 as u32) ) } as u64;
	// 83192240: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192244: 80BE14B0  lwz r5, 0x14b0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5296 as u32) ) } as u64;
	// 83192248: 548B003A  rlwinm r11, r4, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 8319224C: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 83192250: 3B8B0008  addi r28, r11, 8
	ctx.r[28].s64 = ctx.r[11].s64 + 8;
	// 83192254: 555B1838  slwi r27, r10, 3
	ctx.r[27].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83192258: 4BFFFD21  bl 0x83191f78
	ctx.lr = 0x8319225C;
	sub_83191F78(ctx, base);
	// 8319225C: 397B0007  addi r11, r27, 7
	ctx.r[11].s64 = ctx.r[27].s64 + 7;
	// 83192260: 815E14AC  lwz r10, 0x14ac(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5292 as u32) ) } as u64;
	// 83192264: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83192268: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 8319226C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83192270: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83192274: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83192278: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8319227C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83192280: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83192284: 4BFA2F0D  bl 0x83135190
	ctx.lr = 0x83192288;
	sub_83135190(ctx, base);
	// 83192288: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319228C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83192290: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83192294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83192298: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8319229C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831922A0: 4E800421  bctrl
	ctx.lr = 0x831922A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831922A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831922A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831922AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831922B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831922B4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831922B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831922BC: 4E800421  bctrl
	ctx.lr = 0x831922C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831922C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831922C4: 4BFFEB45  bl 0x83190e08
	ctx.lr = 0x831922C8;
	sub_83190E08(ctx, base);
	// 831922C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831922CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831922D0: 48015EE4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831922D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831922D8 size=692
    let mut pc: u32 = 0x831922D8;
    'dispatch: loop {
        match pc {
            0x831922D8 => {
    //   block [0x831922D8..0x831924EC)
	// 831922D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831922DC: 48015E8D  bl 0x831a8168
	ctx.lr = 0x831922E0;
	sub_831A8130(ctx, base);
	// 831922E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831922E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831922E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831922EC: 4BFFDAF5  bl 0x8318fde0
	ctx.lr = 0x831922F0;
	sub_8318FDE0(ctx, base);
	// 831922F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831922F4: 419A001C  beq cr6, 0x83192310
	if ctx.cr[6].eq {
	pc = 0x83192310; continue 'dispatch;
	}
	// 831922F8: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 831922FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192300: 6084020C  ori r4, r4, 0x20c
	ctx.r[4].u64 = ctx.r[4].u64 | 524;
	// 83192304: 4800108D  bl 0x83193390
	ctx.lr = 0x83192308;
	sub_83193390(ctx, base);
	// 83192308: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8319230C: 48015EAC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83192310: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83192314: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83192318: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8319231C: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83192320: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83192324: 93BE1514  stw r29, 0x1514(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(5396 as u32), ctx.r[29].u32 ) };
	// 83192328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319232C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192330: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83192334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83192338: 4E800421  bctrl
	ctx.lr = 0x8319233C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8319233C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192340: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83192344: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83192348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8319234C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83192350: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83192354: 4E800421  bctrl
	ctx.lr = 0x83192358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83192358: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8319235C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83192360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192364: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83192368: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8319236C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83192370: 4BFFE201  bl 0x83190570
	ctx.lr = 0x83192374;
	sub_83190570(ctx, base);
	// 83192374: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83192378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319237C: 409A0014  bne cr6, 0x83192390
	if !ctx.cr[6].eq {
	pc = 0x83192390; continue 'dispatch;
	}
	// 83192380: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83192384: 4BFDC135  bl 0x8316e4b8
	ctx.lr = 0x83192388;
	sub_8316E4B8(ctx, base);
	// 83192388: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8319238C: 48015E2C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83192390: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83192394: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83192398: 4BFFFD91  bl 0x83192128
	ctx.lr = 0x8319239C;
	sub_83192128(ctx, base);
	// 8319239C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831923A0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831923A4: 409A01C4  bne cr6, 0x83192568
	if !ctx.cr[6].eq {
	pc = 0x83192568; continue 'dispatch;
	}
	// 831923A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831923AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831923B0: 4BFFE131  bl 0x831904e0
	ctx.lr = 0x831923B4;
	sub_831904E0(ctx, base);
	// 831923B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831923B8: 419A01C8  beq cr6, 0x83192580
	if ctx.cr[6].eq {
	pc = 0x83192580; continue 'dispatch;
	}
	// 831923BC: 546B07BE  clrlwi r11, r3, 0x1e
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	// 831923C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831923C4: 409A01BC  bne cr6, 0x83192580
	if !ctx.cr[6].eq {
	pc = 0x83192580; continue 'dispatch;
	}
	// 831923C8: 546B0630  rlwinm r11, r3, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 831923CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831923D0: 409A01AC  bne cr6, 0x8319257c
	if !ctx.cr[6].eq {
	pc = 0x8319257C; continue 'dispatch;
	}
	// 831923D4: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 831923D8: 2B0B003C  cmplwi cr6, r11, 0x3c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 60 as u32, &mut ctx.xer);
	// 831923DC: 41990170  bgt cr6, 0x8319254c
	if ctx.cr[6].gt {
	pc = 0x8319254C; continue 'dispatch;
	}
	// 831923E0: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 831923E4: 398C23F8  addi r12, r12, 0x23f8
	ctx.r[12].s64 = ctx.r[12].s64 + 9208;
	// 831923E8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 831923EC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 831923F0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 831923F4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83192514; continue 'dispatch;
		},
		1 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		2 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		3 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		4 => {
	pc = 0x83192500; continue 'dispatch;
		},
		5 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		6 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		7 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		8 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		9 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		10 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		11 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		12 => {
	pc = 0x83192528; continue 'dispatch;
		},
		13 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		14 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		15 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		16 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		17 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		18 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		19 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		20 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		21 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		22 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		23 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		24 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		25 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		26 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		27 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		28 => {
	pc = 0x8319253C; continue 'dispatch;
		},
		29 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		30 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		31 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		32 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		33 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		34 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		35 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		36 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		37 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		38 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		39 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		40 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		41 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		42 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		43 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		44 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		45 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		46 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		47 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		48 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		49 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		50 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		51 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		52 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		53 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		54 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		55 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		56 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		57 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		58 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		59 => {
	pc = 0x8319254C; continue 'dispatch;
		},
		60 => {
	pc = 0x831924EC; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 831923F8: 83192514  lwz r24, 0x2514(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9492 as u32) ) } as u64;
	// 831923FC: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192400: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192404: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192408: 83192500  lwz r24, 0x2500(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9472 as u32) ) } as u64;
	// 8319240C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192410: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192414: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192418: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319241C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192420: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192424: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192428: 83192528  lwz r24, 0x2528(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9512 as u32) ) } as u64;
	// 8319242C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192430: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192434: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192438: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319243C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192440: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192444: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192448: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319244C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192450: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192454: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192458: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319245C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192460: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192464: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192468: 8319253C  lwz r24, 0x253c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9532 as u32) ) } as u64;
	// 8319246C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192470: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192474: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192478: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319247C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192480: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192484: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192488: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319248C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192490: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192494: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 83192498: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 8319249C: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924A0: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924A4: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924A8: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924AC: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924B0: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924B4: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924B8: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924BC: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924C0: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924C4: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924C8: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924CC: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924D0: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924D4: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924D8: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924DC: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924E0: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924E4: 8319254C  lwz r24, 0x254c(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9548 as u32) ) } as u64;
	// 831924E8: 831924EC  lwz r24, 0x24ec(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(9452 as u32) ) } as u64;
            }
            0x831924EC => {
    //   block [0x831924EC..0x83192500)
	// 831924EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831924F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831924F4: 3BA00040  li r29, 0x40
	ctx.r[29].s64 = 64;
	// 831924F8: 4BFFECB1  bl 0x831911a8
	ctx.lr = 0x831924FC;
	sub_831911A8(ctx, base);
	// 831924FC: 48000050  b 0x8319254c
	pc = 0x8319254C; continue 'dispatch;
            }
            0x83192500 => {
    //   block [0x83192500..0x83192514)
	// 83192500: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83192504: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192508: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 8319250C: 4BFFE15D  bl 0x83190668
	ctx.lr = 0x83192510;
	sub_83190668(ctx, base);
	// 83192510: 4800003C  b 0x8319254c
	pc = 0x8319254C; continue 'dispatch;
            }
            0x83192514 => {
    //   block [0x83192514..0x83192528)
	// 83192514: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83192518: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8319251C: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 83192520: 4BFFE329  bl 0x83190848
	ctx.lr = 0x83192524;
	sub_83190848(ctx, base);
	// 83192524: 48000028  b 0x8319254c
	pc = 0x8319254C; continue 'dispatch;
            }
            0x83192528 => {
    //   block [0x83192528..0x8319253C)
	// 83192528: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8319252C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83192530: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192534: 4BFFF97D  bl 0x83191eb0
	ctx.lr = 0x83192538;
	sub_83191EB0(ctx, base);
	// 83192538: 48000014  b 0x8319254c
	pc = 0x8319254C; continue 'dispatch;
            }
            0x8319253C => {
    //   block [0x8319253C..0x8319254C)
	// 8319253C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83192540: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83192544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192548: 4BFFFCB9  bl 0x83192200
	ctx.lr = 0x8319254C;
	sub_83192200(ctx, base);
	pc = 0x8319254C; continue 'dispatch;
            }
            0x8319254C => {
    //   block [0x8319254C..0x8319258C)
	// 8319254C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83192550: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83192554: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192558: 4BFFFBD1  bl 0x83192128
	ctx.lr = 0x8319255C;
	sub_83192128(ctx, base);
	// 8319255C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83192560: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83192564: 419AFE44  beq cr6, 0x831923a8
	if ctx.cr[6].eq {
	pc = 0x831923A8; continue 'dispatch;
	}
	// 83192568: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8319256C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83192570: 48000E21  bl 0x83193390
	ctx.lr = 0x83192574;
	sub_83193390(ctx, base);
	// 83192574: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83192578: 48015C40  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8319257C: 3B80FFFE  li r28, -2
	ctx.r[28].s64 = -2;
	// 83192580: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83192584: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83192588: 48015C30  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83192590 size=148
    let mut pc: u32 = 0x83192590;
    'dispatch: loop {
        match pc {
            0x83192590 => {
    //   block [0x83192590..0x83192624)
	// 83192590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83192594: 48015BD5  bl 0x831a8168
	ctx.lr = 0x83192598;
	sub_831A8130(ctx, base);
	// 83192598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8319259C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831925A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831925A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831925A8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831925AC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831925B0: 4BFA14E9  bl 0x83133a98
	ctx.lr = 0x831925B4;
	sub_83133A98(ctx, base);
	// 831925B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831925B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831925BC: 409A0010  bne cr6, 0x831925cc
	if !ctx.cr[6].eq {
	pc = 0x831925CC; continue 'dispatch;
	}
	// 831925C0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831925C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831925C8: 48015BF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831925CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831925D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831925D4: 4BFFFD05  bl 0x831922d8
	ctx.lr = 0x831925D8;
	sub_831922D8(ctx, base);
	// 831925D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831925DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831925E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831925E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831925E8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831925EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831925F0: 4E800421  bctrl
	ctx.lr = 0x831925F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831925F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831925F8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831925FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83192600: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83192604: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83192608: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319260C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83192610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83192614: 4E800421  bctrl
	ctx.lr = 0x83192618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83192618: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8319261C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83192620: 48015B98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192628 size=24
    let mut pc: u32 = 0x83192628;
    'dispatch: loop {
        match pc {
            0x83192628 => {
    //   block [0x83192628..0x83192640)
	// 83192628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8319262C: 614B0100  ori r11, r10, 0x100
	ctx.r[11].u64 = ctx.r[10].u64 | 256;
	// 83192630: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 83192634: 409A000C  bne cr6, 0x83192640
	if !ctx.cr[6].eq {
		sub_83192640(ctx, base);
		return;
	}
	// 83192638: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8319263C: 48000074  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192640 size=16
    let mut pc: u32 = 0x83192640;
    'dispatch: loop {
        match pc {
            0x83192640 => {
    //   block [0x83192640..0x83192650)
	// 83192640: 2B0B0101  cmplwi cr6, r11, 0x101
	ctx.cr[6].compare_u32(ctx.r[11].u32, 257 as u32, &mut ctx.xer);
	// 83192644: 409A000C  bne cr6, 0x83192650
	if !ctx.cr[6].eq {
		sub_83192650(ctx, base);
		return;
	}
	// 83192648: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8319264C: 48000064  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192650 size=20
    let mut pc: u32 = 0x83192650;
    'dispatch: loop {
        match pc {
            0x83192650 => {
    //   block [0x83192650..0x83192664)
	// 83192650: 392BFEFE  addi r9, r11, -0x102
	ctx.r[9].s64 = ctx.r[11].s64 + -258;
	// 83192654: 2B0900AD  cmplwi cr6, r9, 0xad
	ctx.cr[6].compare_u32(ctx.r[9].u32, 173 as u32, &mut ctx.xer);
	// 83192658: 4199000C  bgt cr6, 0x83192664
	if ctx.cr[6].gt {
		sub_83192664(ctx, base);
		return;
	}
	// 8319265C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83192660: 48000050  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192664 size=16
    let mut pc: u32 = 0x83192664;
    'dispatch: loop {
        match pc {
            0x83192664 => {
    //   block [0x83192664..0x83192674)
	// 83192664: 2B0B01B2  cmplwi cr6, r11, 0x1b2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 434 as u32, &mut ctx.xer);
	// 83192668: 409A000C  bne cr6, 0x83192674
	if !ctx.cr[6].eq {
		sub_83192674(ctx, base);
		return;
	}
	// 8319266C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 83192670: 48000040  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192674 size=16
    let mut pc: u32 = 0x83192674;
    'dispatch: loop {
        match pc {
            0x83192674 => {
    //   block [0x83192674..0x83192684)
	// 83192674: 2B0B01B3  cmplwi cr6, r11, 0x1b3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 435 as u32, &mut ctx.xer);
	// 83192678: 409A000C  bne cr6, 0x83192684
	if !ctx.cr[6].eq {
		sub_83192684(ctx, base);
		return;
	}
	// 8319267C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 83192680: 48000030  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192684(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192684 size=16
    let mut pc: u32 = 0x83192684;
    'dispatch: loop {
        match pc {
            0x83192684 => {
    //   block [0x83192684..0x83192694)
	// 83192684: 2B0B01B5  cmplwi cr6, r11, 0x1b5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 437 as u32, &mut ctx.xer);
	// 83192688: 409A000C  bne cr6, 0x83192694
	if !ctx.cr[6].eq {
		sub_83192694(ctx, base);
		return;
	}
	// 8319268C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 83192690: 48000020  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192694 size=16
    let mut pc: u32 = 0x83192694;
    'dispatch: loop {
        match pc {
            0x83192694 => {
    //   block [0x83192694..0x831926A4)
	// 83192694: 2B0B01B7  cmplwi cr6, r11, 0x1b7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 439 as u32, &mut ctx.xer);
	// 83192698: 409A000C  bne cr6, 0x831926a4
	if !ctx.cr[6].eq {
		sub_831926A4(ctx, base);
		return;
	}
	// 8319269C: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 831926A0: 48000010  b 0x831926b0
	sub_831926A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831926A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831926A4 size=32
    let mut pc: u32 = 0x831926A4;
    'dispatch: loop {
        match pc {
            0x831926A4 => {
    //   block [0x831926A4..0x831926C4)
	// 831926A4: 396BFE48  addi r11, r11, -0x1b8
	ctx.r[11].s64 = ctx.r[11].s64 + -440;
	// 831926A8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831926AC: 556BF738  rlwinm r11, r11, 0x1e, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 831926B0: 7D6A19AE  stbx r11, r10, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u8) };
	// 831926B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831926B8: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 831926BC: 4198FF70  blt cr6, 0x8319262c
	if ctx.cr[6].lt {
		sub_83192628(ctx, base);
		return;
	}
	// 831926C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831926C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831926C8 size=96
    let mut pc: u32 = 0x831926C8;
    'dispatch: loop {
        match pc {
            0x831926C8 => {
    //   block [0x831926C8..0x83192728)
	// 831926C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831926CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831926D0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831926D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831926D8: 4BFFFF51  bl 0x83192628
	ctx.lr = 0x831926DC;
	sub_83192628(ctx, base);
	// 831926DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831926E0: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831926E4: 390B0100  addi r8, r11, 0x100
	ctx.r[8].s64 = ctx.r[11].s64 + 256;
	// 831926E8: 394ABB80  addi r10, r10, -0x4480
	ctx.r[10].s64 = ctx.r[10].s64 + -17536;
	// 831926EC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831926F0: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831926F4: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831926F8: 40820014  bne 0x8319270c
	if !ctx.cr[0].eq {
	pc = 0x8319270C; continue 'dispatch;
	}
	// 831926FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83192700: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83192704: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83192708: 409AFFE4  bne cr6, 0x831926ec
	if !ctx.cr[6].eq {
	pc = 0x831926EC; continue 'dispatch;
	}
	// 8319270C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83192710: 419A0008  beq cr6, 0x83192718
	if ctx.cr[6].eq {
	pc = 0x83192718; continue 'dispatch;
	}
	// 83192714: 48000000  b 0x83192714
	pc = 0x83192714; continue 'dispatch;
	// 83192718: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8319271C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83192720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83192724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192728 size=44
    let mut pc: u32 = 0x83192728;
    'dispatch: loop {
        match pc {
            0x83192728 => {
    //   block [0x83192728..0x83192754)
	// 83192728: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8319272C: 89430001  lbz r10, 1(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83192730: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83192734: 89230002  lbz r9, 2(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83192738: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8319273C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83192740: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 83192744: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83192748: 419A000C  beq cr6, 0x83192754
	if ctx.cr[6].eq {
		sub_83192754(ctx, base);
		return;
	}
	// 8319274C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192754 size=20
    let mut pc: u32 = 0x83192754;
    'dispatch: loop {
        match pc {
            0x83192754 => {
    //   block [0x83192754..0x83192768)
	// 83192754: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83192758: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 8319275C: 396BBB80  addi r11, r11, -0x4480
	ctx.r[11].s64 = ctx.r[11].s64 + -17536;
	// 83192760: 7C6A58AE  lbzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83192764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192768 size=100
    let mut pc: u32 = 0x83192768;
    'dispatch: loop {
        match pc {
            0x83192768 => {
    //   block [0x83192768..0x831927CC)
	// 83192768: 3940FF00  li r10, -0x100
	ctx.r[10].s64 = -256;
	// 8319276C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 83192770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83192774: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83192778: 4099004C  ble cr6, 0x831927c4
	if !ctx.cr[6].gt {
	pc = 0x831927C4; continue 'dispatch;
	}
	// 8319277C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83192780: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83192784: 38CBBB80  addi r6, r11, -0x4480
	ctx.r[6].s64 = ctx.r[11].s64 + -17536;
	// 83192788: 3CE00100  lis r7, 0x100
	ctx.r[7].s64 = 16777216;
	// 8319278C: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83192790: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83192794: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83192798: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8319279C: 409A0018  bne cr6, 0x831927b4
	if !ctx.cr[6].eq {
	pc = 0x831927B4; continue 'dispatch;
	}
	// 831927A0: 556B463E  srwi r11, r11, 0x18
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(24);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831927A4: 7D6B30AE  lbzx r11, r11, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 831927A8: 7D6B2838  and r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[5].u64;
	// 831927AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831927B0: 409A001C  bne cr6, 0x831927cc
	if !ctx.cr[6].eq {
		sub_831927CC(ctx, base);
		return;
	}
	// 831927B4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 831927B8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 831927BC: 7F082000  cmpw cr6, r8, r4
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831927C0: 4198FFCC  blt cr6, 0x8319278c
	if ctx.cr[6].lt {
	pc = 0x8319278C; continue 'dispatch;
	}
	// 831927C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831927C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831927CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831927CC size=8
    let mut pc: u32 = 0x831927CC;
    'dispatch: loop {
        match pc {
            0x831927CC => {
    //   block [0x831927CC..0x831927D4)
	// 831927CC: 7C681850  subf r3, r8, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[8].s64;
	// 831927D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831927D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831927D8 size=92
    let mut pc: u32 = 0x831927D8;
    'dispatch: loop {
        match pc {
            0x831927D8 => {
    //   block [0x831927D8..0x83192834)
	// 831927D8: 3920FF00  li r9, -0x100
	ctx.r[9].s64 = -256;
	// 831927DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831927E0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831927E4: 40990048  ble cr6, 0x8319282c
	if !ctx.cr[6].gt {
	pc = 0x8319282C; continue 'dispatch;
	}
	// 831927E8: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831927EC: 38EABB80  addi r7, r10, -0x4480
	ctx.r[7].s64 = ctx.r[10].s64 + -17536;
	// 831927F0: 7D4B18AE  lbzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831927F4: 7D0B1A14  add r8, r11, r3
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831927F8: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 831927FC: 7C06422C  dcbt r6, r8
	// 83192800: 2B090100  cmplwi cr6, r9, 0x100
	ctx.cr[6].compare_u32(ctx.r[9].u32, 256 as u32, &mut ctx.xer);
	// 83192804: 409A0014  bne cr6, 0x83192818
	if !ctx.cr[6].eq {
	pc = 0x83192818; continue 'dispatch;
	}
	// 83192808: 7D0A38AE  lbzx r8, r10, r7
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8319280C: 7D082838  and r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[5].u64;
	// 83192810: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83192814: 409A0020  bne cr6, 0x83192834
	if !ctx.cr[6].eq {
		sub_83192834(ctx, base);
		return;
	}
	// 83192818: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8319281C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83192820: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83192824: 5549402E  slwi r9, r10, 8
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83192828: 4198FFC8  blt cr6, 0x831927f0
	if ctx.cr[6].lt {
	pc = 0x831927F0; continue 'dispatch;
	}
	// 8319282C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83192830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192834(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192834 size=12
    let mut pc: u32 = 0x83192834;
    'dispatch: loop {
        match pc {
            0x83192834 => {
    //   block [0x83192834..0x83192840)
	// 83192834: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83192838: 386BFFFD  addi r3, r11, -3
	ctx.r[3].s64 = ctx.r[11].s64 + -3;
	// 8319283C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192840 size=32
    let mut pc: u32 = 0x83192840;
    'dispatch: loop {
        match pc {
            0x83192840 => {
    //   block [0x83192840..0x83192860)
	// 83192840: 2F250000  cmpdi cr6, r5, 0
	ctx.cr[6].compare_i64(ctx.r[5].s64, 0, &mut ctx.xer);
	// 83192844: 409A0028  bne cr6, 0x8319286c
	if !ctx.cr[6].eq {
		sub_8319286C(ctx, base);
		return;
	}
	// 83192848: 7C6B2278  xor r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 ^ ctx.r[4].u64;
	// 8319284C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83192850: 41980010  blt cr6, 0x83192860
	if ctx.cr[6].lt {
		sub_83192860(ctx, base);
		return;
	}
	// 83192854: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83192858: 78630040  clrldi r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8319285C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83192860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83192860 size=12
    let mut pc: u32 = 0x83192860;
    'dispatch: loop {
        match pc {
            0x83192860 => {
    //   block [0x83192860..0x8319286C)
	// 83192860: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83192864: 7863FFE6  rldicr r3, r3, 0x3f, 0x3f
	ctx.r[3].u64 = (ctx.r[3].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 83192868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8319286C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8319286C size=84
    let mut pc: u32 = 0x8319286C;
    'dispatch: loop {
        match pc {
            0x8319286C => {
    //   block [0x8319286C..0x831928C0)
	// 8319286C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83192870: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 83192874: 4098000C  bge cr6, 0x83192880
	if !ctx.cr[6].lt {
	pc = 0x83192880; continue 'dispatch;
	}
	// 83192878: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 8319287C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83192880: 2F240000  cmpdi cr6, r4, 0
	ctx.cr[6].compare_i64(ctx.r[4].s64, 0, &mut ctx.xer);
	// 83192884: 4098000C  bge cr6, 0x83192890
	if !ctx.cr[6].lt {
	pc = 0x83192890; continue 'dispatch;
	}
	// 83192888: 7C8400D0  neg r4, r4
	ctx.r[4].s64 = -ctx.r[4].s64;
	// 8319288C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 83192890: 2F250000  cmpdi cr6, r5, 0
	ctx.cr[6].compare_i64(ctx.r[5].s64, 0, &mut ctx.xer);
	// 83192894: 4098000C  bge cr6, 0x831928a0
	if !ctx.cr[6].lt {
	pc = 0x831928A0; continue 'dispatch;
	}
	// 83192898: 7CA500D0  neg r5, r5
	ctx.r[5].s64 = -ctx.r[5].s64;
	// 8319289C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 831928A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831928A4: 78AB0FE0  rldicl r11, r5, 1, 0x3f
	ctx.r[11].u64 = ctx.r[5].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 831928A8: 7D4321D2  mulld r10, r3, r4
	ctx.r[10].s64 = ctx.r[3].s64 * ctx.r[4].s64;
	// 831928AC: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 831928B0: 7D6B0E74  sradi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 1) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 1;
	// 831928B4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831928B8: 7C6B2BD2  divd r3, r11, r5
	ctx.r[3].s64 = ctx.r[11].s64 / ctx.r[5].s64;
	// 831928BC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831928C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831928C0 size=8
    let mut pc: u32 = 0x831928C0;
    'dispatch: loop {
        match pc {
            0x831928C0 => {
    //   block [0x831928C0..0x831928C8)
	// 831928C0: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 831928C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


