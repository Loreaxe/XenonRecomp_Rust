pub fn sub_82534A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534A68 size=20
    let mut pc: u32 = 0x82534A68;
    'dispatch: loop {
        match pc {
            0x82534A68 => {
    //   block [0x82534A68..0x82534A7C)
	// 82534A68: 7D4B0074  cntlzd r11, r10
	ctx.r[11].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 82534A6C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82534A70: 7D4A5836  sld r10, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82534A74: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 82534A78: 48000008  b 0x82534a80
	sub_82534A7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534A7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534A7C size=12
    let mut pc: u32 = 0x82534A7C;
    'dispatch: loop {
        match pc {
            0x82534A7C => {
    //   block [0x82534A7C..0x82534A88)
	// 82534A7C: 7D4A3378  or r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 82534A80: 78A7F842  rldicl r7, r5, 0x3f, 1
	ctx.r[7].u64 = ctx.r[5].u64 & 0x0000000000000001u64;
	// 82534A84: 48000028  b 0x82534aac
	sub_82534A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534A88 size=132
    let mut pc: u32 = 0x82534A88;
    'dispatch: loop {
        match pc {
            0x82534A88 => {
    //   block [0x82534A88..0x82534B0C)
	// 82534A88: 7F2A2840  cmpld cr6, r10, r5
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[5].u64, &mut ctx.xer);
	// 82534A8C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82534A90: 40980008  bge cr6, 0x82534a98
	if !ctx.cr[6].lt {
	pc = 0x82534A98; continue 'dispatch;
	}
	// 82534A94: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82534A98: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82534A9C: 7D490074  cntlzd r9, r10
	ctx.r[9].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 82534AA0: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82534AA4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82534AA8: 7D4A4836  sld r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82534AAC: 7F2B4000  cmpd cr6, r11, r8
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[8].s64, &mut ctx.xer);
	// 82534AB0: 4199FFD8  bgt cr6, 0x82534a88
	if ctx.cr[6].gt {
	pc = 0x82534A88; continue 'dispatch;
	}
	// 82534AB4: 409A0020  bne cr6, 0x82534ad4
	if !ctx.cr[6].eq {
	pc = 0x82534AD4; continue 'dispatch;
	}
	// 82534AB8: 7F2A2840  cmpld cr6, r10, r5
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[5].u64, &mut ctx.xer);
	// 82534ABC: 41980018  blt cr6, 0x82534ad4
	if ctx.cr[6].lt {
	pc = 0x82534AD4; continue 'dispatch;
	}
	// 82534AC0: 7D455050  subf r10, r5, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82534AC4: 7D490074  cntlzd r9, r10
	ctx.r[9].u64 = if ctx.r[10].u64 == 0 { 64 } else { ctx.r[10].u64.leading_zeros() as u64 };
	// 82534AC8: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82534ACC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82534AD0: 7D4A4836  sld r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) << ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82534AD4: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 82534AD8: 419A002C  beq cr6, 0x82534b04
	if ctx.cr[6].eq {
	pc = 0x82534B04; continue 'dispatch;
	}
	// 82534ADC: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82534AE0: 41990010  bgt cr6, 0x82534af0
	if ctx.cr[6].gt {
	pc = 0x82534AF0; continue 'dispatch;
	}
	// 82534AE4: 212B0001  subfic r9, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[9].s64 = (1 as i64) - ctx.r[11].s64;
	// 82534AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82534AEC: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82534AF0: 796BA2C6  sldi r11, r11, 0x34
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(52);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 82534AF4: 794AAB02  rldicl r10, r10, 0x35, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000000007FFu64;
	// 82534AF8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82534AFC: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82534B00: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82534B04: C821FFF0  lfd f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82534B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534B10 size=60
    let mut pc: u32 = 0x82534B10;
    'dispatch: loop {
        match pc {
            0x82534B10 => {
    //   block [0x82534B10..0x82534B4C)
	// 82534B10: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82534B14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82534B18: FDA00A10  fabs f13, f1
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82534B1C: C98B6FE0  lfd f12, 0x6fe0(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(28640 as u32) ) };
	// 82534B20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82534B24: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82534B28: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 82534B2C: FD606850  fneg f11, f13
	ctx.f[11].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82534B30: C9AB2000  lfd f13, 0x2000(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8192 as u32) ) };
	// 82534B34: FD410028  fsub f10, f1, f0
	ctx.f[10].f64 = ctx.f[1].f64 - ctx.f[0].f64;
	// 82534B38: FDA06828  fsub f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82534B3C: FC0A682E  fsel f0, f10, f0, f13
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82534B40: FC0C082E  fsel f0, f12, f0, f1
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[1].f64 };
	// 82534B44: FC2B006E  fsel f1, f11, f1, f0
	ctx.f[1].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[0].f64 };
	// 82534B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534B50 size=80
    let mut pc: u32 = 0x82534B50;
    'dispatch: loop {
        match pc {
            0x82534B50 => {
    //   block [0x82534B50..0x82534BA0)
	// 82534B50: F861FFF8  std r3, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[3].u64 ) };
	// 82534B54: 5466077E  clrlwi r6, r3, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 82534B58: 7C00222C  dcbt 0, r4
	// 82534B5C: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534B60: 20C60008  subfic r6, r6, 8
	ctx.xer.ca = ctx.r[6].u32 <= 8 as u32;
	ctx.r[6].s64 = (8 as i64) - ctx.r[6].s64;
	// 82534B64: 41820050  beq 0x82534bb4
	if ctx.cr[0].eq {
		sub_82534BA0(ctx, base);
		return;
	}
	// 82534B68: 7C053040  cmplw r5, r6
	ctx.cr[0].compare_u32(ctx.r[6].u32, ctx.r[0].u32, &mut ctx.xer);
	// 82534B6C: 40810064  ble 0x82534bd0
	if !ctx.cr[0].gt {
		sub_82534BA0(ctx, base);
		return;
	}
	// 82534B70: 28060004  cmplwi r6, 4
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534B74: 4182002C  beq 0x82534ba0
	if ctx.cr[0].eq {
		sub_82534BA0(ctx, base);
		return;
	}
	// 82534B78: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82534B7C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82534B80: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82534B84: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534B88: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534B8C: 9CC30001  stbu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[6].u8) };
	ctx.r[3].u32 = ea;
	// 82534B90: 4200FFF8  bdnz 0x82534b88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534B88; continue 'dispatch;
	}
	// 82534B94: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82534B98: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82534B9C: 48000018  b 0x82534bb4
	sub_82534BA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534BA0 size=140
    let mut pc: u32 = 0x82534BA0;
    'dispatch: loop {
        match pc {
            0x82534BA0 => {
    //   block [0x82534BA0..0x82534C2C)
	// 82534BA0: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82534BA4: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82534BA8: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82534BAC: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82534BB0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82534BB4: 5486077E  clrlwi r6, r4, 0x1d
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 82534BB8: 2B060004  cmplwi cr6, r6, 4
	ctx.cr[6].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 82534BBC: 28860000  cmplwi cr1, r6, 0
	ctx.cr[1].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82534BC0: 2B850080  cmplwi cr7, r5, 0x80
	ctx.cr[7].compare_u32(ctx.r[5].u32, 128 as u32, &mut ctx.xer);
	// 82534BC4: 419A01D4  beq cr6, 0x82534d98
	if ctx.cr[6].eq {
		sub_82534D98(ctx, base);
		return;
	}
	// 82534BC8: 40860300  bne cr1, 0x82534ec8
	if !ctx.cr[1].eq {
		sub_82534EC8(ctx, base);
		return;
	}
	// 82534BCC: 409C00A0  bge cr7, 0x82534c6c
	if !ctx.cr[7].lt {
		sub_82534C6C(ctx, base);
		return;
	}
	// 82534BD0: 7C0019EC  dcbtst 0, r3
	// 82534BD4: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 82534BD8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82534BDC: 54A7EF3E  rlwinm r7, r5, 0x1d, 0x1c, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x00000007u64;
	// 82534BE0: 54A6077E  clrlwi r6, r5, 0x1d
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x00000007u64;
	// 82534BE4: 28870000  cmplwi cr1, r7, 0
	ctx.cr[1].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82534BE8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82534BEC: 41860014  beq cr1, 0x82534c00
	if ctx.cr[1].eq {
	pc = 0x82534C00; continue 'dispatch;
	}
	// 82534BF0: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82534BF4: E8E40009  ldu r7, 8(r4)
	ea = ctx.r[4].u32.wrapping_add(8 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ea) };
	ctx.r[4].u32 = ea;
	// 82534BF8: F8E30009  stdu r7, 8(r3)
	ea = ctx.r[3].u32.wrapping_add(8 as u32);
	unsafe { crate::rt::store_u64(base as *mut u8, ea, ctx.r[7].u64) };
	ctx.r[3].u32 = ea;
	// 82534BFC: 4200FFF8  bdnz 0x82534bf4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534BF4; continue 'dispatch;
	}
	// 82534C00: 28860004  cmplwi cr1, r6, 4
	ctx.cr[1].compare_u32(ctx.r[6].u32, 4 as u32, &mut ctx.xer);
	// 82534C04: 419A0020  beq cr6, 0x82534c24
	if ctx.cr[6].eq {
	pc = 0x82534C24; continue 'dispatch;
	}
	// 82534C08: 41860024  beq cr1, 0x82534c2c
	if ctx.cr[1].eq {
		sub_82534C2C(ctx, base);
		return;
	}
	// 82534C0C: 38630007  addi r3, r3, 7
	ctx.r[3].s64 = ctx.r[3].s64 + 7;
	// 82534C10: 38840007  addi r4, r4, 7
	ctx.r[4].s64 = ctx.r[4].s64 + 7;
	// 82534C14: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534C18: 8CE40001  lbzu r7, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534C1C: 9CE30001  stbu r7, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[7].u8) };
	ctx.r[3].u32 = ea;
	// 82534C20: 4200FFF8  bdnz 0x82534c18
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534C18; continue 'dispatch;
	}
	// 82534C24: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82534C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534C2C size=28
    let mut pc: u32 = 0x82534C2C;
    'dispatch: loop {
        match pc {
            0x82534C2C => {
    //   block [0x82534C2C..0x82534C48)
	// 82534C2C: 546607BE  clrlwi r6, r3, 0x1e
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x00000003u64;
	// 82534C30: 80A40008  lwz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82534C34: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534C38: 40820010  bne 0x82534c48
	if !ctx.cr[0].eq {
		sub_82534C48(ctx, base);
		return;
	}
	// 82534C3C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82534C40: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82534C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534C48 size=36
    let mut pc: u32 = 0x82534C48;
    'dispatch: loop {
        match pc {
            0x82534C48 => {
    //   block [0x82534C48..0x82534C6C)
	// 82534C48: 89040008  lbz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82534C4C: 88E40009  lbz r7, 9(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(9 as u32) ) } as u64;
	// 82534C50: 88C4000A  lbz r6, 0xa(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(10 as u32) ) } as u64;
	// 82534C54: 99030008  stb r8, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82534C58: 98E30009  stb r7, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[7].u8 ) };
	// 82534C5C: 98C3000A  stb r6, 0xa(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[6].u8 ) };
	// 82534C60: 98A3000B  stb r5, 0xb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(11 as u32), ctx.r[5].u8 ) };
	// 82534C64: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82534C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534C6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534C6C size=276
    let mut pc: u32 = 0x82534C6C;
    'dispatch: loop {
        match pc {
            0x82534C6C => {
    //   block [0x82534C6C..0x82534D80)
	// 82534C6C: 5466067E  clrlwi r6, r3, 0x19
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000007Fu64;
	// 82534C70: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82534C74: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 82534C78: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534C7C: 20C60080  subfic r6, r6, 0x80
	ctx.xer.ca = ctx.r[6].u32 <= 128 as u32;
	ctx.r[6].s64 = (128 as i64) - ctx.r[6].s64;
	// 82534C80: 4182001C  beq 0x82534c9c
	if ctx.cr[0].eq {
	pc = 0x82534C9C; continue 'dispatch;
	}
	// 82534C84: 54C7E8FE  srwi r7, r6, 3
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shr(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82534C88: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82534C8C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82534C90: E8E40009  ldu r7, 8(r4)
	ea = ctx.r[4].u32.wrapping_add(8 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ea) };
	ctx.r[4].u32 = ea;
	// 82534C94: F8E30009  stdu r7, 8(r3)
	ea = ctx.r[3].u32.wrapping_add(8 as u32);
	unsafe { crate::rt::store_u64(base as *mut u8, ea, ctx.r[7].u64) };
	ctx.r[3].u32 = ea;
	// 82534C98: 4200FFF8  bdnz 0x82534c90
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534C90; continue 'dispatch;
	}
	// 82534C9C: 54A6C9FE  srwi r6, r5, 7
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82534CA0: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534CA4: 4182FF38  beq 0x82534bdc
	if ctx.cr[0].eq {
		sub_82534BA0(ctx, base);
		return;
	}
	// 82534CA8: 3945007F  addi r10, r5, 0x7f
	ctx.r[10].s64 = ctx.r[5].s64 + 127;
	// 82534CAC: 54A8067E  clrlwi r8, r5, 0x19
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 82534CB0: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82534CB4: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82534CB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82534CBC: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82534CC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82534CC4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82534CC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82534CCC: 7C09222C  dcbt r9, r4
	// 82534CD0: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 82534CD4: 4200FFF8  bdnz 0x82534ccc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534CCC; continue 'dispatch;
	}
	// 82534CD8: 7D842A14  add r12, r4, r5
	ctx.r[12].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82534CDC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82534CE0: 7D696050  subf r11, r9, r12
	ctx.r[11].s64 = ctx.r[12].s64 - ctx.r[9].s64;
	// 82534CE4: 7D832A14  add r12, r3, r5
	ctx.r[12].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 82534CE8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534CEC: E8C40008  ld r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82534CF0: E8E40010  ld r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	// 82534CF4: E9040018  ld r8, 0x18(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	// 82534CF8: F8C30008  std r6, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82534CFC: E8C40020  ld r6, 0x20(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	// 82534D00: F8E30010  std r7, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u64 ) };
	// 82534D04: E8E40028  ld r7, 0x28(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	// 82534D08: F9030018  std r8, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u64 ) };
	// 82534D0C: E9040030  ld r8, 0x30(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	// 82534D10: F8C30020  std r6, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u64 ) };
	// 82534D14: E8C40038  ld r6, 0x38(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	// 82534D18: F8E30028  std r7, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[7].u64 ) };
	// 82534D1C: E8E40040  ld r7, 0x40(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	// 82534D20: F9030030  std r8, 0x30(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u64 ) };
	// 82534D24: E9040048  ld r8, 0x48(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) };
	// 82534D28: F8C30038  std r6, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[6].u64 ) };
	// 82534D2C: E8C40050  ld r6, 0x50(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) };
	// 82534D30: F8E30040  std r7, 0x40(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[7].u64 ) };
	// 82534D34: E8E40058  ld r7, 0x58(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) };
	// 82534D38: F9030048  std r8, 0x48(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[8].u64 ) };
	// 82534D3C: E9040060  ld r8, 0x60(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) };
	// 82534D40: F8C30050  std r6, 0x50(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[6].u64 ) };
	// 82534D44: E8C40068  ld r6, 0x68(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) };
	// 82534D48: F8E30058  std r7, 0x58(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[7].u64 ) };
	// 82534D4C: E8E40070  ld r7, 0x70(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) };
	// 82534D50: F9030060  std r8, 0x60(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 82534D54: E9040078  ld r8, 0x78(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) };
	// 82534D58: F8C30068  std r6, 0x68(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[6].u64 ) };
	// 82534D5C: E8C40081  ldu r6, 0x80(r4)
	ea = ctx.r[4].u32.wrapping_add(128 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u64(base as *const u8, ea) };
	ctx.r[4].u32 = ea;
	// 82534D60: F8E30070  std r7, 0x70(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[7].u64 ) };
	// 82534D64: F9030078  std r8, 0x78(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[8].u64 ) };
	// 82534D68: F8C30081  stdu r6, 0x80(r3)
	ea = ctx.r[3].u32.wrapping_add(128 as u32);
	unsafe { crate::rt::store_u64(base as *mut u8, ea, ctx.r[6].u64) };
	ctx.r[3].u32 = ea;
	// 82534D6C: 7C045840  cmplw r4, r11
	ctx.cr[0].compare_u32(ctx.r[11].u32, ctx.r[0].u32, &mut ctx.xer);
	// 82534D70: 40800010  bge 0x82534d80
	if !ctx.cr[0].lt {
		sub_82534D80(ctx, base);
		return;
	}
	// 82534D74: 7C09222C  dcbt r9, r4
	// 82534D78: 4200FF74  bdnz 0x82534cec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534CEC; continue 'dispatch;
	}
	// 82534D7C: 4BFFFE60  b 0x82534bdc
	sub_82534BA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534D80 size=24
    let mut pc: u32 = 0x82534D80;
    'dispatch: loop {
        match pc {
            0x82534D80 => {
    //   block [0x82534D80..0x82534D98)
	// 82534D80: 41860010  beq cr1, 0x82534d90
	if ctx.cr[1].eq {
	pc = 0x82534D90; continue 'dispatch;
	}
	// 82534D84: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82534D88: 7C0861EC  dcbtst r8, r12
	// 82534D8C: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82534D90: 4200FF5C  bdnz 0x82534cec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			sub_82534C6C(ctx, base);
		return;
	}
	// 82534D94: 4BFFFE48  b 0x82534bdc
	sub_82534BA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534D98 size=88
    let mut pc: u32 = 0x82534D98;
    'dispatch: loop {
        match pc {
            0x82534D98 => {
    //   block [0x82534D98..0x82534DF0)
	// 82534D98: 3884FFFC  addi r4, r4, -4
	ctx.r[4].s64 = ctx.r[4].s64 + -4;
	// 82534D9C: 409C0054  bge cr7, 0x82534df0
	if !ctx.cr[7].lt {
		sub_82534DF0(ctx, base);
		return;
	}
	// 82534DA0: 7C0019EC  dcbtst 0, r3
	// 82534DA4: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82534DA8: 54A7F6FE  rlwinm r7, r5, 0x1e, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82534DAC: 54A607BE  clrlwi r6, r5, 0x1e
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82534DB0: 28870000  cmplwi cr1, r7, 0
	ctx.cr[1].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82534DB4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82534DB8: 41860014  beq cr1, 0x82534dcc
	if ctx.cr[1].eq {
	pc = 0x82534DCC; continue 'dispatch;
	}
	// 82534DBC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82534DC0: 84E40004  lwzu r7, 4(r4)
	ea = ctx.r[4].u32.wrapping_add(4 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534DC4: 94E30004  stwu r7, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[3].u32 = ea;
	// 82534DC8: 4200FFF8  bdnz 0x82534dc0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534DC0; continue 'dispatch;
	}
	// 82534DCC: 419A001C  beq cr6, 0x82534de8
	if ctx.cr[6].eq {
	pc = 0x82534DE8; continue 'dispatch;
	}
	// 82534DD0: 38630003  addi r3, r3, 3
	ctx.r[3].s64 = ctx.r[3].s64 + 3;
	// 82534DD4: 38840003  addi r4, r4, 3
	ctx.r[4].s64 = ctx.r[4].s64 + 3;
	// 82534DD8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534DDC: 8CE40001  lbzu r7, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534DE0: 9CE30001  stbu r7, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[7].u8) };
	ctx.r[3].u32 = ea;
	// 82534DE4: 4200FFF8  bdnz 0x82534ddc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534DDC; continue 'dispatch;
	}
	// 82534DE8: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82534DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534DF0 size=192
    let mut pc: u32 = 0x82534DF0;
    'dispatch: loop {
        match pc {
            0x82534DF0 => {
    //   block [0x82534DF0..0x82534EB0)
	// 82534DF0: 5466067E  clrlwi r6, r3, 0x19
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000007Fu64;
	// 82534DF4: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82534DF8: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534DFC: 20C60080  subfic r6, r6, 0x80
	ctx.xer.ca = ctx.r[6].u32 <= 128 as u32;
	ctx.r[6].s64 = (128 as i64) - ctx.r[6].s64;
	// 82534E00: 4182001C  beq 0x82534e1c
	if ctx.cr[0].eq {
	pc = 0x82534E1C; continue 'dispatch;
	}
	// 82534E04: 54C7F0BE  srwi r7, r6, 2
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82534E08: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82534E0C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82534E10: 84E40004  lwzu r7, 4(r4)
	ea = ctx.r[4].u32.wrapping_add(4 as u32);
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534E14: 94E30004  stwu r7, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[3].u32 = ea;
	// 82534E18: 4200FFF8  bdnz 0x82534e10
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534E10; continue 'dispatch;
	}
	// 82534E1C: 54A6C9FE  srwi r6, r5, 7
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82534E20: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534E24: 4182FF84  beq 0x82534da8
	if ctx.cr[0].eq {
		sub_82534D98(ctx, base);
		return;
	}
	// 82534E28: 3945007F  addi r10, r5, 0x7f
	ctx.r[10].s64 = ctx.r[5].s64 + 127;
	// 82534E2C: 54A8067E  clrlwi r8, r5, 0x19
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 82534E30: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82534E34: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82534E38: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82534E3C: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82534E40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82534E44: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82534E48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82534E4C: 7C09222C  dcbt r9, r4
	// 82534E50: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 82534E54: 4200FFF8  bdnz 0x82534e4c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534E4C; continue 'dispatch;
	}
	// 82534E58: 7D842A14  add r12, r4, r5
	ctx.r[12].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82534E5C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82534E60: 7D696050  subf r11, r9, r12
	ctx.r[11].s64 = ctx.r[12].s64 - ctx.r[9].s64;
	// 82534E64: 7D832A14  add r12, r3, r5
	ctx.r[12].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 82534E68: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534E6C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82534E70: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 82534E74: 80040004  lwz r0, 4(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82534E78: 80E40008  lwz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82534E7C: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82534E80: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534E84: 90030004  stw r0, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82534E88: 84040010  lwzu r0, 0x10(r4)
	ea = ctx.r[4].u32.wrapping_add(16 as u32);
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534E8C: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82534E90: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82534E94: 94030010  stwu r0, 0x10(r3)
	ea = ctx.r[3].u32.wrapping_add(16 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[0].u32) };
	ctx.r[3].u32 = ea;
	// 82534E98: 4082FFD8  bne 0x82534e70
	if !ctx.cr[0].eq {
	pc = 0x82534E70; continue 'dispatch;
	}
	// 82534E9C: 7C045840  cmplw r4, r11
	ctx.cr[0].compare_u32(ctx.r[11].u32, ctx.r[0].u32, &mut ctx.xer);
	// 82534EA0: 40800010  bge 0x82534eb0
	if !ctx.cr[0].lt {
		sub_82534EB0(ctx, base);
		return;
	}
	// 82534EA4: 7C09222C  dcbt r9, r4
	// 82534EA8: 4200FFC4  bdnz 0x82534e6c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534E6C; continue 'dispatch;
	}
	// 82534EAC: 4BFFFEFC  b 0x82534da8
	sub_82534D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534EB0 size=24
    let mut pc: u32 = 0x82534EB0;
    'dispatch: loop {
        match pc {
            0x82534EB0 => {
    //   block [0x82534EB0..0x82534EC8)
	// 82534EB0: 41860010  beq cr1, 0x82534ec0
	if ctx.cr[1].eq {
	pc = 0x82534EC0; continue 'dispatch;
	}
	// 82534EB4: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82534EB8: 7C0861EC  dcbtst r8, r12
	// 82534EBC: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82534EC0: 4200FFAC  bdnz 0x82534e6c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			sub_82534DF0(ctx, base);
		return;
	}
	// 82534EC4: 4BFFFEE4  b 0x82534da8
	sub_82534D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534EC8 size=52
    let mut pc: u32 = 0x82534EC8;
    'dispatch: loop {
        match pc {
            0x82534EC8 => {
    //   block [0x82534EC8..0x82534EFC)
	// 82534EC8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82534ECC: 409C0030  bge cr7, 0x82534efc
	if !ctx.cr[7].lt {
		sub_82534EFC(ctx, base);
		return;
	}
	// 82534ED0: 7C0019EC  dcbtst 0, r3
	// 82534ED4: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82534ED8: 54A6067E  clrlwi r6, r5, 0x19
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 82534EDC: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534EE0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534EE4: 41820010  beq 0x82534ef4
	if ctx.cr[0].eq {
	pc = 0x82534EF4; continue 'dispatch;
	}
	// 82534EE8: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534EEC: 9CC30001  stbu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[6].u8) };
	ctx.r[3].u32 = ea;
	// 82534EF0: 4200FFF8  bdnz 0x82534ee8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534EE8; continue 'dispatch;
	}
	// 82534EF4: E861FFF8  ld r3, -8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82534EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82534EFC size=196
    let mut pc: u32 = 0x82534EFC;
    'dispatch: loop {
        match pc {
            0x82534EFC => {
    //   block [0x82534EFC..0x82534FC0)
	// 82534EFC: 5466067E  clrlwi r6, r3, 0x19
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0x0000007Fu64;
	// 82534F00: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82534F04: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534F08: 20C60080  subfic r6, r6, 0x80
	ctx.xer.ca = ctx.r[6].u32 <= 128 as u32;
	ctx.r[6].s64 = (128 as i64) - ctx.r[6].s64;
	// 82534F0C: 41820018  beq 0x82534f24
	if ctx.cr[0].eq {
	pc = 0x82534F24; continue 'dispatch;
	}
	// 82534F10: 7CA62850  subf r5, r6, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82534F14: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534F18: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 82534F1C: 9CC30001  stbu r6, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	unsafe { crate::rt::store_u8(base as *mut u8, ea, ctx.r[6].u8) };
	ctx.r[3].u32 = ea;
	// 82534F20: 4200FFF8  bdnz 0x82534f18
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534F18; continue 'dispatch;
	}
	// 82534F24: 54A6C9FE  srwi r6, r5, 7
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shr(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82534F28: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534F2C: 4182FFAC  beq 0x82534ed8
	if ctx.cr[0].eq {
		sub_82534EC8(ctx, base);
		return;
	}
	// 82534F30: 3945007F  addi r10, r5, 0x7f
	ctx.r[10].s64 = ctx.r[5].s64 + 127;
	// 82534F34: 54A8067E  clrlwi r8, r5, 0x19
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x0000007Fu64;
	// 82534F38: 554AC9FE  srwi r10, r10, 7
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82534F3C: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82534F40: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82534F44: 554A077E  clrlwi r10, r10, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82534F48: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82534F4C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82534F50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82534F54: 7C09222C  dcbt r9, r4
	// 82534F58: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 82534F5C: 4200FFF8  bdnz 0x82534f54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534F54; continue 'dispatch;
	}
	// 82534F60: 7D842A14  add r12, r4, r5
	ctx.r[12].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82534F64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82534F68: 7D696050  subf r11, r9, r12
	ctx.r[11].s64 = ctx.r[12].s64 - ctx.r[9].s64;
	// 82534F6C: 7D832A14  add r12, r3, r5
	ctx.r[12].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 82534F70: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82534F74: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82534F78: 88E40004  lbz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82534F7C: 89040003  lbz r8, 3(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 82534F80: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 82534F84: 5107442E  rlwimi r7, r8, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 82534F88: 89240002  lbz r9, 2(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 82534F8C: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82534F90: 5127821E  rlwimi r7, r9, 0x10, 8, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x0000000000FF0000) | (ctx.r[7].u64 & 0xFFFFFFFFFF00FFFF);
	// 82534F94: 89440001  lbz r10, 1(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 82534F98: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82534F9C: 5147C00E  rlwimi r7, r10, 0x18, 0, 7
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[7].u64 & 0xFFFFFFFF00FFFFFF);
	// 82534FA0: 90E30001  stw r7, 1(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[7].u32 ) };
	// 82534FA4: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82534FA8: 4082FFD0  bne 0x82534f78
	if !ctx.cr[0].eq {
	pc = 0x82534F78; continue 'dispatch;
	}
	// 82534FAC: 7C045840  cmplw r4, r11
	ctx.cr[0].compare_u32(ctx.r[11].u32, ctx.r[0].u32, &mut ctx.xer);
	// 82534FB0: 40800010  bge 0x82534fc0
	if !ctx.cr[0].lt {
		sub_82534FC0(ctx, base);
		return;
	}
	// 82534FB4: 7C09222C  dcbt r9, r4
	// 82534FB8: 4200FFBC  bdnz 0x82534f74
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82534F74; continue 'dispatch;
	}
	// 82534FBC: 4BFFFF1C  b 0x82534ed8
	sub_82534EC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534FC0 size=24
    let mut pc: u32 = 0x82534FC0;
    'dispatch: loop {
        match pc {
            0x82534FC0 => {
    //   block [0x82534FC0..0x82534FD8)
	// 82534FC0: 41860010  beq cr1, 0x82534fd0
	if ctx.cr[1].eq {
	pc = 0x82534FD0; continue 'dispatch;
	}
	// 82534FC4: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82534FC8: 7C0861EC  dcbtst r8, r12
	// 82534FCC: 28880000  cmplwi cr1, r8, 0
	ctx.cr[1].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82534FD0: 4200FFA4  bdnz 0x82534f74
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			sub_82534EFC(ctx, base);
		return;
	}
	// 82534FD4: 4BFFFF04  b 0x82534ed8
	sub_82534EC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534FD8 size=16
    let mut pc: u32 = 0x82534FD8;
    'dispatch: loop {
        match pc {
            0x82534FD8 => {
    //   block [0x82534FD8..0x82534FE8)
	// 82534FD8: 38050001  addi r0, r5, 1
	ctx.r[0].s64 = ctx.r[5].s64 + 1;
	// 82534FDC: 60660000  ori r6, r3, 0
	ctx.r[6].u64 = ctx.r[3].u64 | 0;
	// 82534FE0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82534FE4: 48000018  b 0x82534ffc
	sub_82534FE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82534FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82534FE8 size=80
    let mut pc: u32 = 0x82534FE8;
    'dispatch: loop {
        match pc {
            0x82534FE8 => {
    //   block [0x82534FE8..0x82535038)
	// 82534FE8: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82534FEC: 88040000  lbz r0, 0(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82534FF0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82534FF4: 98060000  stb r0, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[0].u8 ) };
	// 82534FF8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82534FFC: 70C00003  andi. r0, r6, 3
	ctx.r[0].u64 = ctx.r[6].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535000: 4002FFE8  bdnzf eq, 0x82534fe8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x82534FE8; continue 'dispatch;
	}
	// 82535004: 54A0F0BF  rlwinm. r0, r5, 0x1e, 2, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535008: 41C20024  beq- 0x8253502c
	if ctx.cr[0].eq {
	pc = 0x8253502C; continue 'dispatch;
	}
	// 8253500C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82535010: 70800003  andi. r0, r4, 3
	ctx.r[0].u64 = ctx.r[4].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535014: 40C2003C  bne- 0x82535050
	if !ctx.cr[0].eq {
		sub_82535050(ctx, base);
		return;
	}
	// 82535018: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253501C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82535020: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82535024: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82535028: 4320FFF0  bdnz+ 0x82535018
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82535018; continue 'dispatch;
	}
	// 8253502C: 70A00003  andi. r0, r5, 3
	ctx.r[0].u64 = ctx.r[5].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535030: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82535034: 4DE20020  beqlr+
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535038 size=24
    let mut pc: u32 = 0x82535038;
    'dispatch: loop {
        match pc {
            0x82535038 => {
    //   block [0x82535038..0x82535050)
	// 82535038: 88040000  lbz r0, 0(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253503C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82535040: 98060000  stb r0, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[0].u8 ) };
	// 82535044: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82535048: 4320FFF0  bdnz+ 0x82535038
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82535038; continue 'dispatch;
	}
	// 8253504C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535050 size=48
    let mut pc: u32 = 0x82535050;
    'dispatch: loop {
        match pc {
            0x82535050 => {
    //   block [0x82535050..0x82535080)
	// 82535050: 88E40003  lbz r7, 3(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 82535054: 89040002  lbz r8, 2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 82535058: 5107442E  rlwimi r7, r8, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 8253505C: 89240001  lbz r9, 1(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 82535060: 5127821E  rlwimi r7, r9, 0x10, 8, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x0000000000FF0000) | (ctx.r[7].u64 & 0xFFFFFFFFFF00FFFF);
	// 82535064: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535068: 5147C00E  rlwimi r7, r10, 0x18, 0, 7
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[7].u64 & 0xFFFFFFFF00FFFFFF);
	// 8253506C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82535070: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82535074: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82535078: 4200FFD8  bdnz 0x82535050
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82535050; continue 'dispatch;
	}
	// 8253507C: 4BFFFFB0  b 0x8253502c
	sub_82534FE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535080 size=80
    let mut pc: u32 = 0x82535080;
    'dispatch: loop {
        match pc {
            0x82535080 => {
    //   block [0x82535080..0x825350D0)
	// 82535080: F9C1FF68  std r14, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.r[14].u64 ) };
	// 82535084: F9E1FF70  std r15, -0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.r[15].u64 ) };
	// 82535088: FA01FF78  std r16, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.r[16].u64 ) };
	// 8253508C: FA21FF80  std r17, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.r[17].u64 ) };
	// 82535090: FA41FF88  std r18, -0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.r[18].u64 ) };
	// 82535094: FA61FF90  std r19, -0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.r[19].u64 ) };
	// 82535098: FA81FF98  std r20, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.r[20].u64 ) };
	// 8253509C: FAA1FFA0  std r21, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.r[21].u64 ) };
	// 825350A0: FAC1FFA8  std r22, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.r[22].u64 ) };
	// 825350A4: FAE1FFB0  std r23, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.r[23].u64 ) };
	// 825350A8: FB01FFB8  std r24, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.r[24].u64 ) };
	// 825350AC: FB21FFC0  std r25, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[25].u64 ) };
	// 825350B0: FB41FFC8  std r26, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[26].u64 ) };
	// 825350B4: FB61FFD0  std r27, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[27].u64 ) };
	// 825350B8: FB81FFD8  std r28, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[28].u64 ) };
	// 825350BC: FBA1FFE0  std r29, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[29].u64 ) };
	// 825350C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825350C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825350C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825350CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825350D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825350D0 size=84
    let mut pc: u32 = 0x825350D0;
    'dispatch: loop {
        match pc {
            0x825350D0 => {
    //   block [0x825350D0..0x82535124)
	// 825350D0: E9C1FF68  ld r14, -0x98(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-152 as u32) ) };
	// 825350D4: E9E1FF70  ld r15, -0x90(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 825350D8: EA01FF78  ld r16, -0x88(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 825350DC: EA21FF80  ld r17, -0x80(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-128 as u32) ) };
	// 825350E0: EA41FF88  ld r18, -0x78(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-120 as u32) ) };
	// 825350E4: EA61FF90  ld r19, -0x70(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-112 as u32) ) };
	// 825350E8: EA81FF98  ld r20, -0x68(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 825350EC: EAA1FFA0  ld r21, -0x60(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 825350F0: EAC1FFA8  ld r22, -0x58(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 825350F4: EAE1FFB0  ld r23, -0x50(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 825350F8: EB01FFB8  ld r24, -0x48(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 825350FC: EB21FFC0  ld r25, -0x40(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82535100: EB41FFC8  ld r26, -0x38(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82535104: EB61FFD0  ld r27, -0x30(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82535108: EB81FFD8  ld r28, -0x28(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8253510C: EBA1FFE0  ld r29, -0x20(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82535110: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82535114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253511C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535124 size=12
    let mut pc: u32 = 0x82535124;
    'dispatch: loop {
        match pc {
            0x82535124 => {
    //   block [0x82535124..0x82535130)
	// 82535124: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82535128: 7CAC2B78  mr r12, r5
	ctx.r[12].u64 = ctx.r[5].u64;
	// 8253512C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535130 size=148
    let mut pc: u32 = 0x82535130;
    'dispatch: loop {
        match pc {
            0x82535130 => {
    //   block [0x82535130..0x825351C4)
	// 82535130: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82535134: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82535138: 3D408254  lis r10, -0x7dac
	ctx.r[10].s64 = -2108424192;
	// 8253513C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82535140: 394AD178  addi r10, r10, -0x2e88
	ctx.r[10].s64 = ctx.r[10].s64 + -11912;
	// 82535144: 396BEFA4  addi r11, r11, -0x105c
	ctx.r[11].s64 = ctx.r[11].s64 + -4188;
	// 82535148: 3FC08254  lis r30, -0x7dac
	ctx.r[30].s64 = -2108424192;
	// 8253514C: 3FE08254  lis r31, -0x7dac
	ctx.r[31].s64 = -2108424192;
	// 82535150: 3C608254  lis r3, -0x7dac
	ctx.r[3].s64 = -2108424192;
	// 82535154: 3C808254  lis r4, -0x7dac
	ctx.r[4].s64 = -2108424192;
	// 82535158: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253515C: 395EC528  addi r10, r30, -0x3ad8
	ctx.r[10].s64 = ctx.r[30].s64 + -15064;
	// 82535160: 3CA08254  lis r5, -0x7dac
	ctx.r[5].s64 = -2108424192;
	// 82535164: 3CC08254  lis r6, -0x7dac
	ctx.r[6].s64 = -2108424192;
	// 82535168: 3CE08254  lis r7, -0x7dac
	ctx.r[7].s64 = -2108424192;
	// 8253516C: 3D008254  lis r8, -0x7dac
	ctx.r[8].s64 = -2108424192;
	// 82535170: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82535174: 395FC518  addi r10, r31, -0x3ae8
	ctx.r[10].s64 = ctx.r[31].s64 + -15080;
	// 82535178: 3D208254  lis r9, -0x7dac
	ctx.r[9].s64 = -2108424192;
	// 8253517C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82535180: 3943C520  addi r10, r3, -0x3ae0
	ctx.r[10].s64 = ctx.r[3].s64 + -15072;
	// 82535184: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82535188: 3944C498  addi r10, r4, -0x3b68
	ctx.r[10].s64 = ctx.r[4].s64 + -15208;
	// 8253518C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82535190: 3945D178  addi r10, r5, -0x2e88
	ctx.r[10].s64 = ctx.r[5].s64 + -11912;
	// 82535194: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82535198: 3946D128  addi r10, r6, -0x2ed8
	ctx.r[10].s64 = ctx.r[6].s64 + -11992;
	// 8253519C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 825351A0: 3947C4B8  addi r10, r7, -0x3b48
	ctx.r[10].s64 = ctx.r[7].s64 + -15176;
	// 825351A4: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 825351A8: 3948C3E0  addi r10, r8, -0x3c20
	ctx.r[10].s64 = ctx.r[8].s64 + -15392;
	// 825351AC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825351B0: 3949C340  addi r10, r9, -0x3cc0
	ctx.r[10].s64 = ctx.r[9].s64 + -15552;
	// 825351B4: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 825351B8: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825351BC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825351C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825351C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825351C8 size=4
    let mut pc: u32 = 0x825351C8;
    'dispatch: loop {
        match pc {
            0x825351C8 => {
    //   block [0x825351C8..0x825351CC)
	// 825351C8: 4BFFFF68  b 0x82535130
	sub_82535130(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825351D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825351D0 size=16
    let mut pc: u32 = 0x825351D0;
    'dispatch: loop {
        match pc {
            0x825351D0 => {
    //   block [0x825351D0..0x825351E0)
	// 825351D0: 38050001  addi r0, r5, 1
	ctx.r[0].s64 = ctx.r[5].s64 + 1;
	// 825351D4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 825351D8: 60660000  ori r6, r3, 0
	ctx.r[6].u64 = ctx.r[3].u64 | 0;
	// 825351DC: 48000010  b 0x825351ec
	sub_825351E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825351E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825351E0 size=120
    let mut pc: u32 = 0x825351E0;
    'dispatch: loop {
        match pc {
            0x825351E0 => {
    //   block [0x825351E0..0x82535258)
	// 825351E0: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 825351E4: 98860000  stb r4, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 825351E8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 825351EC: 70C00003  andi. r0, r6, 3
	ctx.r[0].u64 = ctx.r[6].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825351F0: 4002FFF0  bdnzf eq, 0x825351e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x825351E0; continue 'dispatch;
	}
	// 825351F4: 5084442E  rlwimi r4, r4, 8, 0x10, 0x17
	ctx.r[4].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF00FF);
	// 825351F8: 54A0E13F  rlwinm. r0, r5, 0x1c, 4, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825351FC: 5084801E  rlwimi r4, r4, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82535200: 41E20020  beq+ 0x82535220
	if ctx.cr[0].eq {
	pc = 0x82535220; continue 'dispatch;
	}
	// 82535204: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82535208: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8253520C: 90860004  stw r4, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82535210: 90860008  stw r4, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82535214: 9086000C  stw r4, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82535218: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 8253521C: 4320FFEC  bdnz+ 0x82535208
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82535208; continue 'dispatch;
	}
	// 82535220: 54A0F7BF  rlwinm. r0, r5, 0x1e, 0x1e, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535224: 41C20028  beq- 0x8253524c
	if ctx.cr[0].eq {
	pc = 0x8253524C; continue 'dispatch;
	}
	// 82535228: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8253522C: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82535230: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82535234: 43400018  bdz- 0x8253524c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 {
	pc = 0x8253524C; continue 'dispatch;
	}
	// 82535238: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8253523C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82535240: 4340000C  bdz- 0x8253524c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 {
	pc = 0x8253524C; continue 'dispatch;
	}
	// 82535244: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82535248: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8253524C: 70A00003  andi. r0, r5, 3
	ctx.r[0].u64 = ctx.r[5].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535250: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82535254: 4DE20020  beqlr+
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535258 size=8
    let mut pc: u32 = 0x82535258;
    'dispatch: loop {
        match pc {
            0x82535258 => {
    //   block [0x82535258..0x82535260)
	// 82535258: 98860000  stb r4, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8253525C: 4F400020  bdzlr-
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535260 size=8
    let mut pc: u32 = 0x82535260;
    'dispatch: loop {
        match pc {
            0x82535260 => {
    //   block [0x82535260..0x82535268)
	// 82535260: 98860001  stb r4, 1(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 82535264: 4F400020  bdzlr-
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 == 0 { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535268 size=8
    let mut pc: u32 = 0x82535268;
    'dispatch: loop {
        match pc {
            0x82535268 => {
    //   block [0x82535268..0x82535270)
	// 82535268: 98860002  stb r4, 2(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[4].u8 ) };
	// 8253526C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535270 size=228
    let mut pc: u32 = 0x82535270;
    'dispatch: loop {
        match pc {
            0x82535270 => {
    //   block [0x82535270..0x82535354)
	// 82535270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253527C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535280: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82535284: 419A000C  beq cr6, 0x82535290
	if ctx.cr[6].eq {
	pc = 0x82535290; continue 'dispatch;
	}
	// 82535288: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8253528C: 409A0034  bne cr6, 0x825352c0
	if !ctx.cr[6].eq {
	pc = 0x825352C0; continue 'dispatch;
	}
	// 82535290: 480056A9  bl 0x8253a938
	ctx.lr = 0x82535294;
	sub_8253A938(ctx, base);
	// 82535294: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82535298: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253529C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825352A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825352A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825352A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825352AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825352B0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825352B4: 4800554D  bl 0x8253a800
	ctx.lr = 0x825352B8;
	sub_8253A800(ctx, base);
	// 825352B8: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 825352BC: 48000084  b 0x82535340
	pc = 0x82535340; continue 'dispatch;
	// 825352C0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 825352C4: 409A0038  bne cr6, 0x825352fc
	if !ctx.cr[6].eq {
	pc = 0x825352FC; continue 'dispatch;
	}
	// 825352C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825352CC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825352D0: 48005669  bl 0x8253a938
	ctx.lr = 0x825352D4;
	sub_8253A938(ctx, base);
	// 825352D4: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 825352D8: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825352DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825352E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825352E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825352E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825352EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825352F0: 48005511  bl 0x8253a800
	ctx.lr = 0x825352F4;
	sub_8253A800(ctx, base);
	// 825352F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825352F8: 48000048  b 0x82535340
	pc = 0x82535340; continue 'dispatch;
	// 825352FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82535300: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535304: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82535308: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253530C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82535310: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82535314: 4182000C  beq 0x82535320
	if ctx.cr[0].eq {
	pc = 0x82535320; continue 'dispatch;
	}
	// 82535318: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8253531C: 4082FFE4  bne 0x82535300
	if !ctx.cr[0].eq {
	pc = 0x82535300; continue 'dispatch;
	}
	// 82535320: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82535324: 409A0018  bne cr6, 0x8253533c
	if !ctx.cr[6].eq {
	pc = 0x8253533C; continue 'dispatch;
	}
	// 82535328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253532C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82535330: 48005609  bl 0x8253a938
	ctx.lr = 0x82535334;
	sub_8253A938(ctx, base);
	// 82535334: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 82535338: 4BFFFFA0  b 0x825352d8
	pc = 0x825352D8; continue 'dispatch;
	// 8253533C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82535344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253534C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535358 size=12
    let mut pc: u32 = 0x82535358;
    'dispatch: loop {
        match pc {
            0x82535358 => {
    //   block [0x82535358..0x82535364)
	// 82535358: 3963FFBF  addi r11, r3, -0x41
	ctx.r[11].s64 = ctx.r[3].s64 + -65;
	// 8253535C: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82535360: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535364 size=8
    let mut pc: u32 = 0x82535364;
    'dispatch: loop {
        match pc {
            0x82535364 => {
    //   block [0x82535364..0x8253536C)
	// 82535364: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82535368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535370 size=8
    let mut pc: u32 = 0x82535370;
    'dispatch: loop {
        match pc {
            0x82535370 => {
    //   block [0x82535370..0x82535378)
	// 82535370: 7C032000  cmpw r3, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 82535374: 4DC20020  beqlr-
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535378 size=8
    let mut pc: u32 = 0x82535378;
    'dispatch: loop {
        match pc {
            0x82535378 => {
    //   block [0x82535378..0x82535380)
	// 82535378: 40E00008  bge+ 0x82535380
	if !ctx.cr[0].lt {
		sub_82535380(ctx, base);
		return;
	}
	// 8253537C: 4BFFF7D4  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535380 size=20
    let mut pc: u32 = 0x82535380;
    'dispatch: loop {
        match pc {
            0x82535380 => {
    //   block [0x82535380..0x82535394)
	// 82535380: 38050001  addi r0, r5, 1
	ctx.r[0].s64 = ctx.r[5].s64 + 1;
	// 82535384: 7C632A14  add r3, r3, r5
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[5].u64;
	// 82535388: 7C842A14  add r4, r4, r5
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 8253538C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82535390: 48000018  b 0x825353a8
	sub_82535394(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535394 size=80
    let mut pc: u32 = 0x82535394;
    'dispatch: loop {
        match pc {
            0x82535394 => {
    //   block [0x82535394..0x825353E4)
	// 82535394: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82535398: 8804FFFF  lbz r0, -1(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8253539C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 825353A0: 9803FFFF  stb r0, -1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(-1 as u32), ctx.r[0].u8 ) };
	// 825353A4: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 825353A8: 70600003  andi. r0, r3, 3
	ctx.r[0].u64 = ctx.r[3].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825353AC: 4002FFE8  bdnzf eq, 0x82535394
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 && !ctx.cr[0].eq {
	pc = 0x82535394; continue 'dispatch;
	}
	// 825353B0: 54A0F0BF  rlwinm. r0, r5, 0x1e, 2, 0x1f
	ctx.r[0].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825353B4: 41C20024  beq- 0x825353d8
	if ctx.cr[0].eq {
	pc = 0x825353D8; continue 'dispatch;
	}
	// 825353B8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 825353BC: 70800003  andi. r0, r4, 3
	ctx.r[0].u64 = ctx.r[4].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825353C0: 40C2003C  bne- 0x825353fc
	if !ctx.cr[0].eq {
		sub_825353FC(ctx, base);
		return;
	}
	// 825353C4: 80E4FFFC  lwz r7, -4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 825353C8: 3884FFFC  addi r4, r4, -4
	ctx.r[4].s64 = ctx.r[4].s64 + -4;
	// 825353CC: 90E3FFFC  stw r7, -4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 825353D0: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 825353D4: 4320FFF0  bdnz+ 0x825353c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825353C4; continue 'dispatch;
	}
	// 825353D8: 70A00003  andi. r0, r5, 3
	ctx.r[0].u64 = ctx.r[5].u64 & 3;
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825353DC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 825353E0: 4DE20020  beqlr+
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825353E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825353E4 size=24
    let mut pc: u32 = 0x825353E4;
    'dispatch: loop {
        match pc {
            0x825353E4 => {
    //   block [0x825353E4..0x825353FC)
	// 825353E4: 8804FFFF  lbz r0, -1(r4)
	ctx.r[0].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-1 as u32) ) } as u64;
	// 825353E8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 825353EC: 9803FFFF  stb r0, -1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(-1 as u32), ctx.r[0].u8 ) };
	// 825353F0: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 825353F4: 4320FFF0  bdnz+ 0x825353e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825353E4; continue 'dispatch;
	}
	// 825353F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825353FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825353FC size=48
    let mut pc: u32 = 0x825353FC;
    'dispatch: loop {
        match pc {
            0x825353FC => {
    //   block [0x825353FC..0x8253542C)
	// 825353FC: 88E4FFFF  lbz r7, -1(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82535400: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82535404: 8904FFFE  lbz r8, -2(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82535408: 5107442E  rlwimi r7, r8, 8, 0x10, 0x17
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF00FF);
	// 8253540C: 8924FFFD  lbz r9, -3(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-3 as u32) ) } as u64;
	// 82535410: 5127821E  rlwimi r7, r9, 0x10, 8, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x0000000000FF0000) | (ctx.r[7].u64 & 0xFFFFFFFFFF00FFFF);
	// 82535414: 8944FFFC  lbz r10, -4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82535418: 5147C00E  rlwimi r7, r10, 0x18, 0, 7
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[7].u64 & 0xFFFFFFFF00FFFFFF);
	// 8253541C: 3884FFFC  addi r4, r4, -4
	ctx.r[4].s64 = ctx.r[4].s64 + -4;
	// 82535420: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82535424: 4200FFD8  bdnz 0x825353fc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825353FC; continue 'dispatch;
	}
	// 82535428: 4BFFFFB0  b 0x825353d8
	sub_82535394(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535430 size=368
    let mut pc: u32 = 0x82535430;
    'dispatch: loop {
        match pc {
            0x82535430 => {
    //   block [0x82535430..0x825355A0)
	// 82535430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253543C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535440: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82535444: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82535448: 409A002C  bne cr6, 0x82535474
	if !ctx.cr[6].eq {
	pc = 0x82535474; continue 'dispatch;
	}
	// 8253544C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82535450: 409A002C  bne cr6, 0x8253547c
	if !ctx.cr[6].eq {
	pc = 0x8253547C; continue 'dispatch;
	}
	// 82535454: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82535458: 409A002C  bne cr6, 0x82535484
	if !ctx.cr[6].eq {
	pc = 0x82535484; continue 'dispatch;
	}
	// 8253545C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82535464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253546C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535470: 4E800020  blr
	return;
	// 82535474: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82535478: 419A000C  beq cr6, 0x82535484
	if ctx.cr[6].eq {
	pc = 0x82535484; continue 'dispatch;
	}
	// 8253547C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82535480: 409A0034  bne cr6, 0x825354b4
	if !ctx.cr[6].eq {
	pc = 0x825354B4; continue 'dispatch;
	}
	// 82535484: 480054B5  bl 0x8253a938
	ctx.lr = 0x82535488;
	sub_8253A938(ctx, base);
	// 82535488: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253548C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82535490: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82535494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82535498: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253549C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825354A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825354A4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825354A8: 48005359  bl 0x8253a800
	ctx.lr = 0x825354AC;
	sub_8253A800(ctx, base);
	// 825354AC: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 825354B0: 4BFFFFB0  b 0x82535460
	pc = 0x82535460; continue 'dispatch;
	// 825354B4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 825354B8: 409A0010  bne cr6, 0x825354c8
	if !ctx.cr[6].eq {
	pc = 0x825354C8; continue 'dispatch;
	}
	// 825354BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825354C0: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825354C4: 4BFFFF98  b 0x8253545c
	pc = 0x8253545C; continue 'dispatch;
	// 825354C8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 825354CC: 409A0038  bne cr6, 0x82535504
	if !ctx.cr[6].eq {
	pc = 0x82535504; continue 'dispatch;
	}
	// 825354D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825354D4: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825354D8: 48005461  bl 0x8253a938
	ctx.lr = 0x825354DC;
	sub_8253A938(ctx, base);
	// 825354DC: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 825354E0: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825354E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825354E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825354EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825354F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825354F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825354F8: 48005309  bl 0x8253a800
	ctx.lr = 0x825354FC;
	sub_8253A800(ctx, base);
	// 825354FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535500: 4BFFFF60  b 0x82535460
	pc = 0x82535460; continue 'dispatch;
	// 82535504: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82535508: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8253550C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82535510: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 82535514: 409A0028  bne cr6, 0x8253553c
	if !ctx.cr[6].eq {
	pc = 0x8253553C; continue 'dispatch;
	}
	// 82535518: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253551C: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82535520: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535524: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82535528: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253552C: 41820044  beq 0x82535570
	if ctx.cr[0].eq {
	pc = 0x82535570; continue 'dispatch;
	}
	// 82535530: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82535534: 4082FFE4  bne 0x82535518
	if !ctx.cr[0].eq {
	pc = 0x82535518; continue 'dispatch;
	}
	// 82535538: 48000038  b 0x82535570
	pc = 0x82535570; continue 'dispatch;
	// 8253553C: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535540: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82535544: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535548: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253554C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82535550: 41820014  beq 0x82535564
	if ctx.cr[0].eq {
	pc = 0x82535564; continue 'dispatch;
	}
	// 82535554: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82535558: 4182000C  beq 0x82535564
	if ctx.cr[0].eq {
	pc = 0x82535564; continue 'dispatch;
	}
	// 8253555C: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82535560: 4082FFDC  bne 0x8253553c
	if !ctx.cr[0].eq {
	pc = 0x8253553C; continue 'dispatch;
	}
	// 82535564: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82535568: 409A0008  bne cr6, 0x82535570
	if !ctx.cr[6].eq {
	pc = 0x82535570; continue 'dispatch;
	}
	// 8253556C: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82535570: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82535574: 409AFEE8  bne cr6, 0x8253545c
	if !ctx.cr[6].eq {
	pc = 0x8253545C; continue 'dispatch;
	}
	// 82535578: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 8253557C: 409A0014  bne cr6, 0x82535590
	if !ctx.cr[6].eq {
	pc = 0x82535590; continue 'dispatch;
	}
	// 82535580: 7D692214  add r11, r9, r4
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82535584: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82535588: 98EBFFFF  stb r7, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[7].u8 ) };
	// 8253558C: 4BFFFED4  b 0x82535460
	pc = 0x82535460; continue 'dispatch;
	// 82535590: 98E90000  stb r7, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82535594: 480053A5  bl 0x8253a938
	ctx.lr = 0x82535598;
	sub_8253A938(ctx, base);
	// 82535598: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 8253559C: 4BFFFF44  b 0x825354e0
	pc = 0x825354E0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825355A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825355A0 size=276
    let mut pc: u32 = 0x825355A0;
    'dispatch: loop {
        match pc {
            0x825355A0 => {
    //   block [0x825355A0..0x825356B4)
	// 825355A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825355A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825355A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825355AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825355B0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 825355B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825355B8: 419A000C  beq cr6, 0x825355c4
	if ctx.cr[6].eq {
	pc = 0x825355C4; continue 'dispatch;
	}
	// 825355BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825355C0: 409A0034  bne cr6, 0x825355f4
	if !ctx.cr[6].eq {
	pc = 0x825355F4; continue 'dispatch;
	}
	// 825355C4: 48005375  bl 0x8253a938
	ctx.lr = 0x825355C8;
	sub_8253A938(ctx, base);
	// 825355C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825355CC: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 825355D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825355D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825355D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825355DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825355E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825355E4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825355E8: 48005219  bl 0x8253a800
	ctx.lr = 0x825355EC;
	sub_8253A800(ctx, base);
	// 825355EC: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 825355F0: 480000B0  b 0x825356a0
	pc = 0x825356A0; continue 'dispatch;
	// 825355F4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 825355F8: 409A0010  bne cr6, 0x82535608
	if !ctx.cr[6].eq {
	pc = 0x82535608; continue 'dispatch;
	}
	// 825355FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82535600: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82535604: 4BFFFFC0  b 0x825355c4
	pc = 0x825355C4; continue 'dispatch;
	// 82535608: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253560C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535610: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82535614: 419A0010  beq cr6, 0x82535624
	if ctx.cr[6].eq {
	pc = 0x82535624; continue 'dispatch;
	}
	// 82535618: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253561C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82535620: 4082FFEC  bne 0x8253560c
	if !ctx.cr[0].eq {
	pc = 0x8253560C; continue 'dispatch;
	}
	// 82535624: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82535628: 409A0038  bne cr6, 0x82535660
	if !ctx.cr[6].eq {
	pc = 0x82535660; continue 'dispatch;
	}
	// 8253562C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82535630: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82535634: 48005305  bl 0x8253a938
	ctx.lr = 0x82535638;
	sub_8253A938(ctx, base);
	// 82535638: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 8253563C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82535640: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82535644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82535648: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253564C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535650: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535654: 480051AD  bl 0x8253a800
	ctx.lr = 0x82535658;
	sub_8253A800(ctx, base);
	// 82535658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253565C: 48000044  b 0x825356a0
	pc = 0x825356A0; continue 'dispatch;
	// 82535660: 89250000  lbz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535664: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82535668: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253566C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82535670: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82535674: 4182000C  beq 0x82535680
	if ctx.cr[0].eq {
	pc = 0x82535680; continue 'dispatch;
	}
	// 82535678: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253567C: 4082FFE4  bne 0x82535660
	if !ctx.cr[0].eq {
	pc = 0x82535660; continue 'dispatch;
	}
	// 82535680: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82535684: 409A0018  bne cr6, 0x8253569c
	if !ctx.cr[6].eq {
	pc = 0x8253569C; continue 'dispatch;
	}
	// 82535688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253568C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82535690: 480052A9  bl 0x8253a938
	ctx.lr = 0x82535694;
	sub_8253A938(ctx, base);
	// 82535694: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 82535698: 4BFFFFA4  b 0x8253563c
	pc = 0x8253563C; continue 'dispatch;
	// 8253569C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825356A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825356A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825356A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825356AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825356B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825356B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825356B8 size=404
    let mut pc: u32 = 0x825356B8;
    'dispatch: loop {
        match pc {
            0x825356B8 => {
    //   block [0x825356B8..0x8253584C)
	// 825356B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825356BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825356C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825356C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825356C8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 825356CC: 409A002C  bne cr6, 0x825356f8
	if !ctx.cr[6].eq {
	pc = 0x825356F8; continue 'dispatch;
	}
	// 825356D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825356D4: 409A002C  bne cr6, 0x82535700
	if !ctx.cr[6].eq {
	pc = 0x82535700; continue 'dispatch;
	}
	// 825356D8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825356DC: 409A002C  bne cr6, 0x82535708
	if !ctx.cr[6].eq {
	pc = 0x82535708; continue 'dispatch;
	}
	// 825356E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825356E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825356E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825356EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825356F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825356F4: 4E800020  blr
	return;
	// 825356F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825356FC: 419A000C  beq cr6, 0x82535708
	if ctx.cr[6].eq {
	pc = 0x82535708; continue 'dispatch;
	}
	// 82535700: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82535704: 409A0034  bne cr6, 0x82535738
	if !ctx.cr[6].eq {
	pc = 0x82535738; continue 'dispatch;
	}
	// 82535708: 48005231  bl 0x8253a938
	ctx.lr = 0x8253570C;
	sub_8253A938(ctx, base);
	// 8253570C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82535710: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82535714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82535718: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253571C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82535720: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535728: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253572C: 480050D5  bl 0x8253a800
	ctx.lr = 0x82535730;
	sub_8253A800(ctx, base);
	// 82535730: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 82535734: 4BFFFFB0  b 0x825356e4
	pc = 0x825356E4; continue 'dispatch;
	// 82535738: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8253573C: 419A0018  beq cr6, 0x82535754
	if ctx.cr[6].eq {
	pc = 0x82535754; continue 'dispatch;
	}
	// 82535740: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82535744: 409A0010  bne cr6, 0x82535754
	if !ctx.cr[6].eq {
	pc = 0x82535754; continue 'dispatch;
	}
	// 82535748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253574C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82535750: 4BFFFFB8  b 0x82535708
	pc = 0x82535708; continue 'dispatch;
	// 82535754: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82535758: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8253575C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535760: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82535764: 419A0010  beq cr6, 0x82535774
	if ctx.cr[6].eq {
	pc = 0x82535774; continue 'dispatch;
	}
	// 82535768: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253576C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82535770: 4082FFEC  bne 0x8253575c
	if !ctx.cr[0].eq {
	pc = 0x8253575C; continue 'dispatch;
	}
	// 82535774: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82535778: 409A0038  bne cr6, 0x825357b0
	if !ctx.cr[6].eq {
	pc = 0x825357B0; continue 'dispatch;
	}
	// 8253577C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82535780: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82535784: 480051B5  bl 0x8253a938
	ctx.lr = 0x82535788;
	sub_8253A938(ctx, base);
	// 82535788: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 8253578C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82535790: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82535794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82535798: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253579C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825357A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825357A4: 4800505D  bl 0x8253a800
	ctx.lr = 0x825357A8;
	sub_8253A800(ctx, base);
	// 825357A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825357AC: 4BFFFF38  b 0x825356e4
	pc = 0x825356E4; continue 'dispatch;
	// 825357B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825357B4: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 825357B8: 409A0028  bne cr6, 0x825357e0
	if !ctx.cr[6].eq {
	pc = 0x825357E0; continue 'dispatch;
	}
	// 825357BC: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825357C0: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 825357C4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825357C8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825357CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825357D0: 4182004C  beq 0x8253581c
	if ctx.cr[0].eq {
	pc = 0x8253581C; continue 'dispatch;
	}
	// 825357D4: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825357D8: 4082FFE4  bne 0x825357bc
	if !ctx.cr[0].eq {
	pc = 0x825357BC; continue 'dispatch;
	}
	// 825357DC: 48000040  b 0x8253581c
	pc = 0x8253581C; continue 'dispatch;
	// 825357E0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 825357E4: 419A0034  beq cr6, 0x82535818
	if ctx.cr[6].eq {
	pc = 0x82535818; continue 'dispatch;
	}
	// 825357E8: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825357EC: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 825357F0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825357F4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825357F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825357FC: 41820014  beq 0x82535810
	if ctx.cr[0].eq {
	pc = 0x82535810; continue 'dispatch;
	}
	// 82535800: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82535804: 4182000C  beq 0x82535810
	if ctx.cr[0].eq {
	pc = 0x82535810; continue 'dispatch;
	}
	// 82535808: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8253580C: 4082FFDC  bne 0x825357e8
	if !ctx.cr[0].eq {
	pc = 0x825357E8; continue 'dispatch;
	}
	// 82535810: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82535814: 409A0008  bne cr6, 0x8253581c
	if !ctx.cr[6].eq {
	pc = 0x8253581C; continue 'dispatch;
	}
	// 82535818: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8253581C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82535820: 409AFEC0  bne cr6, 0x825356e0
	if !ctx.cr[6].eq {
	pc = 0x825356E0; continue 'dispatch;
	}
	// 82535824: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 82535828: 409A0014  bne cr6, 0x8253583c
	if !ctx.cr[6].eq {
	pc = 0x8253583C; continue 'dispatch;
	}
	// 8253582C: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82535830: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82535834: 990BFFFF  stb r8, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[8].u8 ) };
	// 82535838: 4BFFFEAC  b 0x825356e4
	pc = 0x825356E4; continue 'dispatch;
	// 8253583C: 99030000  stb r8, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82535840: 480050F9  bl 0x8253a938
	ctx.lr = 0x82535844;
	sub_8253A938(ctx, base);
	// 82535844: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 82535848: 4BFFFF44  b 0x8253578c
	pc = 0x8253578C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535850 size=8
    let mut pc: u32 = 0x82535850;
    'dispatch: loop {
        match pc {
            0x82535850 => {
    //   block [0x82535850..0x82535858)
	// 82535850: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535854: 481D80F8  b 0x8270d94c
	sub_8270D94C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535858 size=72
    let mut pc: u32 = 0x82535858;
    'dispatch: loop {
        match pc {
            0x82535858 => {
    //   block [0x82535858..0x825358A0)
	// 82535858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253585C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82535864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253586C: 480041FD  bl 0x82539a68
	ctx.lr = 0x82535870;
	sub_82539A68(ctx, base);
	// 82535870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535874: 480041AD  bl 0x82539a20
	ctx.lr = 0x82535878;
	sub_82539A20(ctx, base);
	// 82535878: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253587C: 386000FF  li r3, 0xff
	ctx.r[3].s64 = 255;
	// 82535880: 816BE7D0  lwz r11, -0x1830(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6192 as u32) ) } as u64;
	// 82535884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535888: 4E800421  bctrl
	ctx.lr = 0x8253588C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253588C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82535890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253589C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825358A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825358A0 size=8
    let mut pc: u32 = 0x825358A0;
    'dispatch: loop {
        match pc {
            0x825358A0 => {
    //   block [0x825358A0..0x825358A8)
	// 825358A0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 825358A4: 48007B7C  b 0x8253d420
	sub_8253D420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825358A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825358A8 size=8
    let mut pc: u32 = 0x825358A8;
    'dispatch: loop {
        match pc {
            0x825358A8 => {
    //   block [0x825358A8..0x825358B0)
	// 825358A8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 825358AC: 48007A14  b 0x8253d2c0
	sub_8253D2C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825358B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825358B0 size=88
    let mut pc: u32 = 0x825358B0;
    'dispatch: loop {
        match pc {
            0x825358B0 => {
    //   block [0x825358B0..0x82535908)
	// 825358B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825358B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825358B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825358BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825358C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825358C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825358C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825358CC: 4800001C  b 0x825358e8
	pc = 0x825358E8; continue 'dispatch;
	// 825358D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825358D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825358D8: 4182000C  beq 0x825358e4
	if ctx.cr[0].eq {
	pc = 0x825358E4; continue 'dispatch;
	}
	// 825358DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825358E0: 4E800421  bctrl
	ctx.lr = 0x825358E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825358E4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825358E8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825358EC: 4198FFE4  blt cr6, 0x825358d0
	if ctx.cr[6].lt {
	pc = 0x825358D0; continue 'dispatch;
	}
	// 825358F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825358F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825358F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825358FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82535900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535908 size=8
    let mut pc: u32 = 0x82535908;
    'dispatch: loop {
        match pc {
            0x82535908 => {
    //   block [0x82535908..0x82535910)
	// 82535908: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8253590C: 820DA350  lwz r16, -0x5cb0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535910 size=256
    let mut pc: u32 = 0x82535910;
    'dispatch: loop {
        match pc {
            0x82535910 => {
    //   block [0x82535910..0x82535A10)
	// 82535910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535914: 4BFFF7A5  bl 0x825350b8
	ctx.lr = 0x82535918;
	sub_82535080(ctx, base);
	// 82535918: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8253591C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82535924: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82535928: 939F00A4  stw r28, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[28].u32 ) };
	// 8253592C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82535930: 48007AF1  bl 0x8253d420
	ctx.lr = 0x82535934;
	sub_8253D420(ctx, base);
	// 82535934: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82535938: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8253593C: 816B2D40  lwz r11, 0x2d40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11584 as u32) ) } as u64;
	// 82535940: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82535944: 419A00A4  beq cr6, 0x825359e8
	if ctx.cr[6].eq {
	pc = 0x825359E8; continue 'dispatch;
	}
	// 82535948: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8253594C: 816A2D3C  lwz r11, 0x2d3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(11580 as u32) ) } as u64;
	// 82535950: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82535954: 409A000C  bne cr6, 0x82535960
	if !ctx.cr[6].eq {
	pc = 0x82535960; continue 'dispatch;
	}
	// 82535958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253595C: 481D7FF1  bl 0x8270d94c
	ctx.lr = 0x82535960;
	// extern call 0x8270D94C → crate::xboxkrnl::KeBugCheck
	crate::xboxkrnl::KeBugCheck(ctx, base);
	// 82535960: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82535964: 916A2D3C  stw r11, 0x2d3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(11580 as u32), ctx.r[11].u32 ) };
	// 82535968: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8253596C: 9B8A2D38  stb r28, 0x2d38(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(11576 as u32), ctx.r[28].u8 ) };
	// 82535970: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82535974: 409A0060  bne cr6, 0x825359d4
	if !ctx.cr[6].eq {
	pc = 0x825359D4; continue 'dispatch;
	}
	// 82535978: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8253597C: 83AB3DF0  lwz r29, 0x3df0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15856 as u32) ) } as u64;
	// 82535980: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82535984: 83CB3DEC  lwz r30, 0x3dec(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15852 as u32) ) } as u64;
	// 82535988: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8253598C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535990: 41820030  beq 0x825359c0
	if ctx.cr[0].eq {
	pc = 0x825359C0; continue 'dispatch;
	}
	// 82535994: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82535998: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8253599C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825359A0: 41980020  blt cr6, 0x825359c0
	if ctx.cr[6].lt {
	pc = 0x825359C0; continue 'dispatch;
	}
	// 825359A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825359A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825359AC: 4182000C  beq 0x825359b8
	if ctx.cr[0].eq {
	pc = 0x825359B8; continue 'dispatch;
	}
	// 825359B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825359B4: 4E800421  bctrl
	ctx.lr = 0x825359B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825359B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825359BC: 4BFFFFD8  b 0x82535994
	pc = 0x82535994; continue 'dispatch;
	// 825359C0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825359C4: 388BB4AC  addi r4, r11, -0x4b54
	ctx.r[4].s64 = ctx.r[11].s64 + -19284;
	// 825359C8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825359CC: 386BB4A4  addi r3, r11, -0x4b5c
	ctx.r[3].s64 = ctx.r[11].s64 + -19292;
	// 825359D0: 4BFFFEE1  bl 0x825358b0
	ctx.lr = 0x825359D4;
	sub_825358B0(ctx, base);
	// 825359D4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825359D8: 388BB4B4  addi r4, r11, -0x4b4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19276;
	// 825359DC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825359E0: 386BB4B0  addi r3, r11, -0x4b50
	ctx.r[3].s64 = ctx.r[11].s64 + -19280;
	// 825359E4: 4BFFFECD  bl 0x825358b0
	ctx.lr = 0x825359E8;
	sub_825358B0(ctx, base);
	// 825359E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825359EC: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 825359F0: 48000041  bl 0x82535a30
	ctx.lr = 0x825359F4;
	sub_82535A10(ctx, base);
	// 825359F4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 825359F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825359FC: 409A000C  bne cr6, 0x82535a08
	if !ctx.cr[6].eq {
	pc = 0x82535A08; continue 'dispatch;
	}
	// 82535A00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535A04: 481D7F49  bl 0x8270d94c
	ctx.lr = 0x82535A08;
	// extern call 0x8270D94C → crate::xboxkrnl::KeBugCheck
	crate::xboxkrnl::KeBugCheck(ctx, base);
	// 82535A08: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82535A0C: 4BFFF6FC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535A10 size=96
    let mut pc: u32 = 0x82535A10;
    'dispatch: loop {
        match pc {
            0x82535A10 => {
    //   block [0x82535A10..0x82535A70)
	// 82535A10: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82535A14: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82535A18: FB81FFF0  std r28, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[28].u64 ) };
	// 82535A1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535A20: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82535A24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535A28: 839F00A4  lwz r28, 0xa4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82535A2C: 4800001C  b 0x82535a48
	pc = 0x82535A48; continue 'dispatch;
	// 82535A30: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82535A34: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82535A38: FB81FFF0  std r28, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[28].u64 ) };
	// 82535A3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535A40: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82535A44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535A48: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82535A4C: 419A000C  beq cr6, 0x82535a58
	if ctx.cr[6].eq {
	pc = 0x82535A58; continue 'dispatch;
	}
	// 82535A50: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82535A54: 4800786D  bl 0x8253d2c0
	ctx.lr = 0x82535A58;
	sub_8253D2C0(ctx, base);
	// 82535A58: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82535A5C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82535A60: EB81FFF0  ld r28, -0x10(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535A64: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82535A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535A70 size=12
    let mut pc: u32 = 0x82535A70;
    'dispatch: loop {
        match pc {
            0x82535A70 => {
    //   block [0x82535A70..0x82535A7C)
	// 82535A70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82535A74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535A78: 4BFFFE98  b 0x82535910
	sub_82535910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535A80 size=12
    let mut pc: u32 = 0x82535A80;
    'dispatch: loop {
        match pc {
            0x82535A80 => {
    //   block [0x82535A80..0x82535A8C)
	// 82535A80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82535A84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82535A88: 4BFFFE88  b 0x82535910
	sub_82535910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535A90 size=16
    let mut pc: u32 = 0x82535A90;
    'dispatch: loop {
        match pc {
            0x82535A90 => {
    //   block [0x82535A90..0x82535AA0)
	// 82535A90: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82535A94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535A98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535A9C: 4BFFFE74  b 0x82535910
	sub_82535910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535AA0 size=92
    let mut pc: u32 = 0x82535AA0;
    'dispatch: loop {
        match pc {
            0x82535AA0 => {
    //   block [0x82535AA0..0x82535AFC)
	// 82535AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535AB0: 48000C39  bl 0x825366e8
	ctx.lr = 0x82535AB4;
	sub_825366E8(ctx, base);
	// 82535AB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535AB8: 48007A99  bl 0x8253d550
	ctx.lr = 0x82535ABC;
	sub_8253D550(ctx, base);
	// 82535ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535AC0: 48004D31  bl 0x8253a7f0
	ctx.lr = 0x82535AC4;
	sub_8253A7F0(ctx, base);
	// 82535AC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535AC8: 48003FD1  bl 0x82539a98
	ctx.lr = 0x82535ACC;
	sub_82539A98(ctx, base);
	// 82535ACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535AD0: 4BD7D4B1  bl 0x822b2f80
	ctx.lr = 0x82535AD4;
	sub_822B2F80(ctx, base);
	// 82535AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535AD8: 48007A61  bl 0x8253d538
	ctx.lr = 0x82535ADC;
	sub_8253D538(ctx, base);
	// 82535ADC: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82535AE0: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82535AE4: 396B5A80  addi r11, r11, 0x5a80
	ctx.r[11].s64 = ctx.r[11].s64 + 23168;
	// 82535AE8: 916AE7D0  stw r11, -0x1830(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6192 as u32), ctx.r[11].u32 ) };
	// 82535AEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82535AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535B00 size=4
    let mut pc: u32 = 0x82535B00;
    'dispatch: loop {
        match pc {
            0x82535B00 => {
    //   block [0x82535B00..0x82535B04)
	// 82535B00: 481D822C  b 0x8270dd2c
	sub_8270DD2C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535B08 size=88
    let mut pc: u32 = 0x82535B08;
    'dispatch: loop {
        match pc {
            0x82535B08 => {
    //   block [0x82535B08..0x82535B60)
	// 82535B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82535B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82535B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535B1C: 3FC08282  lis r30, -0x7d7e
	ctx.r[30].s64 = -2105409536;
	// 82535B20: 807EE7D8  lwz r3, -0x1828(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6184 as u32) ) } as u64;
	// 82535B24: 481D8219  bl 0x8270dd3c
	ctx.lr = 0x82535B28;
	// extern call 0x8270DD3C → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 82535B28: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82535B2C: 40820018  bne 0x82535b44
	if !ctx.cr[0].eq {
	pc = 0x82535B44; continue 'dispatch;
	}
	// 82535B30: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82535B34: 807EE7D8  lwz r3, -0x1828(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6184 as u32) ) } as u64;
	// 82535B38: 83EB2D54  lwz r31, 0x2d54(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11604 as u32) ) } as u64;
	// 82535B3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82535B40: 481D820D  bl 0x8270dd4c
	ctx.lr = 0x82535B44;
	// extern call 0x8270DD4C → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 82535B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535B48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82535B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535B54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82535B58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535B60 size=120
    let mut pc: u32 = 0x82535B60;
    'dispatch: loop {
        match pc {
            0x82535B60 => {
    //   block [0x82535B60..0x82535BD8)
	// 82535B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82535B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82535B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535B74: 3FE08282  lis r31, -0x7d7e
	ctx.r[31].s64 = -2105409536;
	// 82535B78: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82535B7C: 807FE7D4  lwz r3, -0x182c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535B80: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82535B84: 419A001C  beq cr6, 0x82535ba0
	if ctx.cr[6].eq {
	pc = 0x82535BA0; continue 'dispatch;
	}
	// 82535B88: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82535B8C: 816B2D5C  lwz r11, 0x2d5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11612 as u32) ) } as u64;
	// 82535B90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535B94: 4E800421  bctrl
	ctx.lr = 0x82535B98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535B98: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82535B9C: 917FE7D4  stw r11, -0x182c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-6188 as u32), ctx.r[11].u32 ) };
	// 82535BA0: 3FE08282  lis r31, -0x7d7e
	ctx.r[31].s64 = -2105409536;
	// 82535BA4: 807FE7D8  lwz r3, -0x1828(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-6184 as u32) ) } as u64;
	// 82535BA8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82535BAC: 419A0010  beq cr6, 0x82535bbc
	if ctx.cr[6].eq {
	pc = 0x82535BBC; continue 'dispatch;
	}
	// 82535BB0: 481D81AD  bl 0x8270dd5c
	ctx.lr = 0x82535BB4;
	// extern call 0x8270DD5C → crate::xboxkrnl::KeTlsFree
	crate::xboxkrnl::KeTlsFree(ctx, base);
	// 82535BB4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82535BB8: 917FE7D8  stw r11, -0x1828(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-6184 as u32), ctx.r[11].u32 ) };
	// 82535BBC: 48007695  bl 0x8253d250
	ctx.lr = 0x82535BC0;
	sub_8253D250(ctx, base);
	// 82535BC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82535BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535BCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82535BD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535BD8 size=236
    let mut pc: u32 = 0x82535BD8;
    'dispatch: loop {
        match pc {
            0x82535BD8 => {
    //   block [0x82535BD8..0x82535CC4)
	// 82535BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535BDC: 4BFFF4DD  bl 0x825350b8
	ctx.lr = 0x82535BE0;
	sub_82535080(ctx, base);
	// 82535BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535BE4: 4BE8AA9D  bl 0x823c0680
	ctx.lr = 0x82535BE8;
	sub_823C0680(ctx, base);
	// 82535BE8: 3FC08282  lis r30, -0x7d7e
	ctx.r[30].s64 = -2105409536;
	// 82535BEC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82535BF0: 83FEE7D4  lwz r31, -0x182c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535BF4: 4BFFFF15  bl 0x82535b08
	ctx.lr = 0x82535BF8;
	sub_82535B08(ctx, base);
	// 82535BF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82535BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535C04: 4E800421  bctrl
	ctx.lr = 0x82535C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535C08: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82535C0C: 40820098  bne 0x82535ca4
	if !ctx.cr[0].eq {
	pc = 0x82535CA4; continue 'dispatch;
	}
	// 82535C10: 3FA0829A  lis r29, -0x7d66
	ctx.r[29].s64 = -2103836672;
	// 82535C14: 807EE7D4  lwz r3, -0x182c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535C18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82535C1C: 817D2D58  lwz r11, 0x2d58(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(11608 as u32) ) } as u64;
	// 82535C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535C24: 4E800421  bctrl
	ctx.lr = 0x82535C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535C28: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535C2C: 41820084  beq 0x82535cb0
	if ctx.cr[0].eq {
	pc = 0x82535CB0; continue 'dispatch;
	}
	// 82535C30: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 82535C34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82535C38: 48003D81  bl 0x825399b8
	ctx.lr = 0x82535C3C;
	sub_825399B8(ctx, base);
	// 82535C3C: 817D2D58  lwz r11, 0x2d58(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(11608 as u32) ) } as u64;
	// 82535C40: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82535C44: 807EE7D4  lwz r3, -0x182c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535C48: 4182004C  beq 0x82535c94
	if ctx.cr[0].eq {
	pc = 0x82535C94; continue 'dispatch;
	}
	// 82535C4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82535C50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535C54: 4E800421  bctrl
	ctx.lr = 0x82535C58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535C58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535C5C: 4182002C  beq 0x82535c88
	if ctx.cr[0].eq {
	pc = 0x82535C88; continue 'dispatch;
	}
	// 82535C60: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82535C64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82535C68: 396BF0F0  addi r11, r11, -0xf10
	ctx.r[11].s64 = ctx.r[11].s64 + -3856;
	// 82535C6C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82535C70: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82535C74: 480103CD  bl 0x82546040
	ctx.lr = 0x82535C78;
	sub_82546040(ctx, base);
	// 82535C78: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82535C7C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82535C80: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82535C84: 4800002C  b 0x82535cb0
	pc = 0x82535CB0; continue 'dispatch;
	// 82535C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535C8C: 4BFFE0AD  bl 0x82533d38
	ctx.lr = 0x82535C90;
	sub_82533D38(ctx, base);
	// 82535C90: 4800001C  b 0x82535cac
	pc = 0x82535CAC; continue 'dispatch;
	// 82535C94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535C98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535C9C: 4E800421  bctrl
	ctx.lr = 0x82535CA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535CA0: 48000010  b 0x82535cb0
	pc = 0x82535CB0; continue 'dispatch;
	// 82535CA4: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82535CA8: 409A0008  bne cr6, 0x82535cb0
	if !ctx.cr[6].eq {
	pc = 0x82535CB0; continue 'dispatch;
	}
	// 82535CAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82535CB0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82535CB4: 4BE8CADD  bl 0x823c2790
	ctx.lr = 0x82535CB8;
	sub_823C2790(ctx, base);
	// 82535CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82535CC0: 4BFFF448  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535CC8 size=60
    let mut pc: u32 = 0x82535CC8;
    'dispatch: loop {
        match pc {
            0x82535CC8 => {
    //   block [0x82535CC8..0x82535D04)
	// 82535CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82535CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535CD8: 4BFFFF01  bl 0x82535bd8
	ctx.lr = 0x82535CDC;
	sub_82535BD8(ctx, base);
	// 82535CDC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82535CE0: 4082000C  bne 0x82535cec
	if !ctx.cr[0].eq {
	pc = 0x82535CEC; continue 'dispatch;
	}
	// 82535CE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82535CE8: 4BFFFB71  bl 0x82535858
	ctx.lr = 0x82535CEC;
	sub_82535858(ctx, base);
	// 82535CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82535CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535D08 size=176
    let mut pc: u32 = 0x82535D08;
    'dispatch: loop {
        match pc {
            0x82535D08 => {
    //   block [0x82535D08..0x82535DB8)
	// 82535D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82535D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82535D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535D18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82535D1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82535D20: 419A0084  beq cr6, 0x82535da4
	if ctx.cr[6].eq {
	pc = 0x82535DA4; continue 'dispatch;
	}
	// 82535D24: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82535D28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535D2C: 41820008  beq 0x82535d34
	if ctx.cr[0].eq {
	pc = 0x82535D34; continue 'dispatch;
	}
	// 82535D30: 4BFFE009  bl 0x82533d38
	ctx.lr = 0x82535D34;
	sub_82533D38(ctx, base);
	// 82535D34: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82535D38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535D3C: 41820008  beq 0x82535d44
	if ctx.cr[0].eq {
	pc = 0x82535D44; continue 'dispatch;
	}
	// 82535D40: 4BFFDFF9  bl 0x82533d38
	ctx.lr = 0x82535D44;
	sub_82533D38(ctx, base);
	// 82535D44: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82535D48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535D4C: 41820008  beq 0x82535d54
	if ctx.cr[0].eq {
	pc = 0x82535D54; continue 'dispatch;
	}
	// 82535D50: 4BFFDFE9  bl 0x82533d38
	ctx.lr = 0x82535D54;
	sub_82533D38(ctx, base);
	// 82535D54: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82535D58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535D5C: 41820008  beq 0x82535d64
	if ctx.cr[0].eq {
	pc = 0x82535D64; continue 'dispatch;
	}
	// 82535D60: 4BFFDFD9  bl 0x82533d38
	ctx.lr = 0x82535D64;
	sub_82533D38(ctx, base);
	// 82535D64: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82535D68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535D6C: 41820008  beq 0x82535d74
	if ctx.cr[0].eq {
	pc = 0x82535D74; continue 'dispatch;
	}
	// 82535D70: 4BFFDFC9  bl 0x82533d38
	ctx.lr = 0x82535D74;
	sub_82533D38(ctx, base);
	// 82535D74: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82535D78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535D7C: 41820008  beq 0x82535d84
	if ctx.cr[0].eq {
	pc = 0x82535D84; continue 'dispatch;
	}
	// 82535D80: 4BFFDFB9  bl 0x82533d38
	ctx.lr = 0x82535D84;
	sub_82533D38(ctx, base);
	// 82535D84: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82535D88: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82535D8C: 396BF0F0  addi r11, r11, -0xf10
	ctx.r[11].s64 = ctx.r[11].s64 + -3856;
	// 82535D90: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82535D94: 419A0008  beq cr6, 0x82535d9c
	if ctx.cr[6].eq {
	pc = 0x82535D9C; continue 'dispatch;
	}
	// 82535D98: 4BFFDFA1  bl 0x82533d38
	ctx.lr = 0x82535D9C;
	sub_82533D38(ctx, base);
	// 82535D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82535DA0: 4BFFDF99  bl 0x82533d38
	ctx.lr = 0x82535DA4;
	sub_82533D38(ctx, base);
	// 82535DA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82535DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82535DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82535DB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82535DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535DB8 size=152
    let mut pc: u32 = 0x82535DB8;
    'dispatch: loop {
        match pc {
            0x82535DB8 => {
    //   block [0x82535DB8..0x82535E50)
	// 82535DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535DBC: 4BFFF301  bl 0x825350bc
	ctx.lr = 0x82535DC0;
	sub_82535080(ctx, base);
	// 82535DC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535DC4: 3FE08282  lis r31, -0x7d7e
	ctx.r[31].s64 = -2105409536;
	// 82535DC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82535DCC: 3FA08282  lis r29, -0x7d7e
	ctx.r[29].s64 = -2105409536;
	// 82535DD0: 817FE7D4  lwz r11, -0x182c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535DD4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82535DD8: 419A005C  beq cr6, 0x82535e34
	if ctx.cr[6].eq {
	pc = 0x82535E34; continue 'dispatch;
	}
	// 82535DDC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82535DE0: 409A0034  bne cr6, 0x82535e14
	if !ctx.cr[6].eq {
	pc = 0x82535E14; continue 'dispatch;
	}
	// 82535DE4: 807DE7D8  lwz r3, -0x1828(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-6184 as u32) ) } as u64;
	// 82535DE8: 481D7F55  bl 0x8270dd3c
	ctx.lr = 0x82535DEC;
	// extern call 0x8270DD3C → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 82535DEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82535DF0: 41820024  beq 0x82535e14
	if ctx.cr[0].eq {
	pc = 0x82535E14; continue 'dispatch;
	}
	// 82535DF4: 807DE7D8  lwz r3, -0x1828(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-6184 as u32) ) } as u64;
	// 82535DF8: 83DFE7D4  lwz r30, -0x182c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535DFC: 481D7F41  bl 0x8270dd3c
	ctx.lr = 0x82535E00;
	// extern call 0x8270DD3C → crate::xboxkrnl::KeTlsGetValue
	crate::xboxkrnl::KeTlsGetValue(ctx, base);
	// 82535E00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82535E04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82535E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535E0C: 4E800421  bctrl
	ctx.lr = 0x82535E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535E10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82535E14: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82535E18: 807FE7D4  lwz r3, -0x182c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535E1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535E20: 816B2D58  lwz r11, 0x2d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11608 as u32) ) } as u64;
	// 82535E24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535E28: 4E800421  bctrl
	ctx.lr = 0x82535E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535E2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82535E30: 4BFFFED9  bl 0x82535d08
	ctx.lr = 0x82535E34;
	sub_82535D08(ctx, base);
	// 82535E34: 807DE7D8  lwz r3, -0x1828(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-6184 as u32) ) } as u64;
	// 82535E38: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82535E3C: 419A000C  beq cr6, 0x82535e48
	if ctx.cr[6].eq {
	pc = 0x82535E48; continue 'dispatch;
	}
	// 82535E40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82535E44: 481D7F09  bl 0x8270dd4c
	ctx.lr = 0x82535E48;
	// extern call 0x8270DD4C → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 82535E48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82535E4C: 4BFFF2C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535E50 size=304
    let mut pc: u32 = 0x82535E50;
    'dispatch: loop {
        match pc {
            0x82535E50 => {
    //   block [0x82535E50..0x82535F80)
	// 82535E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82535E54: 4BFFF269  bl 0x825350bc
	ctx.lr = 0x82535E58;
	sub_82535080(ctx, base);
	// 82535E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535E5C: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82535E60: 3FC0829A  lis r30, -0x7d66
	ctx.r[30].s64 = -2103836672;
	// 82535E64: 396B5B00  addi r11, r11, 0x5b00
	ctx.r[11].s64 = ctx.r[11].s64 + 23296;
	// 82535E68: 3D008271  lis r8, -0x7d8f
	ctx.r[8].s64 = -2106523648;
	// 82535E6C: 3FE0829A  lis r31, -0x7d66
	ctx.r[31].s64 = -2103836672;
	// 82535E70: 3D208271  lis r9, -0x7d8f
	ctx.r[9].s64 = -2106523648;
	// 82535E74: 3FA0829A  lis r29, -0x7d66
	ctx.r[29].s64 = -2103836672;
	// 82535E78: 917E2D50  stw r11, 0x2d50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(11600 as u32), ctx.r[11].u32 ) };
	// 82535E7C: 3968DD3C  addi r11, r8, -0x22c4
	ctx.r[11].s64 = ctx.r[8].s64 + -8900;
	// 82535E80: 3D408271  lis r10, -0x7d8f
	ctx.r[10].s64 = -2106523648;
	// 82535E84: 917F2D54  stw r11, 0x2d54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11604 as u32), ctx.r[11].u32 ) };
	// 82535E88: 3969DD4C  addi r11, r9, -0x22b4
	ctx.r[11].s64 = ctx.r[9].s64 + -8884;
	// 82535E8C: 3D20829A  lis r9, -0x7d66
	ctx.r[9].s64 = -2103836672;
	// 82535E90: 917D2D58  stw r11, 0x2d58(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(11608 as u32), ctx.r[11].u32 ) };
	// 82535E94: 396ADD5C  addi r11, r10, -0x22a4
	ctx.r[11].s64 = ctx.r[10].s64 + -8868;
	// 82535E98: 91692D5C  stw r11, 0x2d5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(11612 as u32), ctx.r[11].u32 ) };
	// 82535E9C: 481D7E91  bl 0x8270dd2c
	ctx.lr = 0x82535EA0;
	// extern call 0x8270DD2C → crate::xboxkrnl::KeTlsAlloc
	crate::xboxkrnl::KeTlsAlloc(ctx, base);
	// 82535EA0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82535EA4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82535EA8: 906BE7D8  stw r3, -0x1828(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-6184 as u32), ctx.r[3].u32 ) };
	// 82535EAC: 419A00C8  beq cr6, 0x82535f74
	if ctx.cr[6].eq {
	pc = 0x82535F74; continue 'dispatch;
	}
	// 82535EB0: 809F2D54  lwz r4, 0x2d54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11604 as u32) ) } as u64;
	// 82535EB4: 481D7E99  bl 0x8270dd4c
	ctx.lr = 0x82535EB8;
	// extern call 0x8270DD4C → crate::xboxkrnl::KeTlsSetValue
	crate::xboxkrnl::KeTlsSetValue(ctx, base);
	// 82535EB8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535EBC: 418200B8  beq 0x82535f74
	if ctx.cr[0].eq {
	pc = 0x82535F74; continue 'dispatch;
	}
	// 82535EC0: 4BFFFBE1  bl 0x82535aa0
	ctx.lr = 0x82535EC4;
	sub_82535AA0(ctx, base);
	// 82535EC4: 4800730D  bl 0x8253d1d0
	ctx.lr = 0x82535EC8;
	sub_8253D1D0(ctx, base);
	// 82535EC8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535ECC: 418200A4  beq 0x82535f70
	if ctx.cr[0].eq {
	pc = 0x82535F70; continue 'dispatch;
	}
	// 82535ED0: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82535ED4: 386B5D08  addi r3, r11, 0x5d08
	ctx.r[3].s64 = ctx.r[11].s64 + 23816;
	// 82535ED8: 817E2D50  lwz r11, 0x2d50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(11600 as u32) ) } as u64;
	// 82535EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535EE0: 4E800421  bctrl
	ctx.lr = 0x82535EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535EE4: 3FC08282  lis r30, -0x7d7e
	ctx.r[30].s64 = -2105409536;
	// 82535EE8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82535EEC: 907EE7D4  stw r3, -0x182c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-6188 as u32), ctx.r[3].u32 ) };
	// 82535EF0: 419A0080  beq cr6, 0x82535f70
	if ctx.cr[6].eq {
	pc = 0x82535F70; continue 'dispatch;
	}
	// 82535EF4: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 82535EF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82535EFC: 48003ABD  bl 0x825399b8
	ctx.lr = 0x82535F00;
	sub_825399B8(ctx, base);
	// 82535F00: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82535F04: 4182006C  beq 0x82535f70
	if ctx.cr[0].eq {
	pc = 0x82535F70; continue 'dispatch;
	}
	// 82535F08: 807EE7D4  lwz r3, -0x182c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6188 as u32) ) } as u64;
	// 82535F0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82535F10: 817D2D58  lwz r11, 0x2d58(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(11608 as u32) ) } as u64;
	// 82535F14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82535F18: 4E800421  bctrl
	ctx.lr = 0x82535F1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82535F1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82535F20: 41820050  beq 0x82535f70
	if ctx.cr[0].eq {
	pc = 0x82535F70; continue 'dispatch;
	}
	// 82535F24: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82535F28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82535F2C: 396BF0F0  addi r11, r11, -0xf10
	ctx.r[11].s64 = ctx.r[11].s64 + -3856;
	// 82535F30: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82535F34: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82535F38: 48010109  bl 0x82546040
	ctx.lr = 0x82535F3C;
	sub_82546040(ctx, base);
	// 82535F3C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82535F40: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82535F44: 386B2D44  addi r3, r11, 0x2d44
	ctx.r[3].s64 = ctx.r[11].s64 + 11588;
	// 82535F48: 3D608254  lis r11, -0x7dac
	ctx.r[11].s64 = -2108424192;
	// 82535F4C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82535F50: 396BD640  addi r11, r11, -0x29c0
	ctx.r[11].s64 = ctx.r[11].s64 + -10688;
	// 82535F54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82535F58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82535F5C: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82535F60: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82535F64: 4BE97CA5  bl 0x823cdc08
	ctx.lr = 0x82535F68;
	sub_823CDC08(ctx, base);
	// 82535F68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82535F6C: 4800000C  b 0x82535f78
	pc = 0x82535F78; continue 'dispatch;
	// 82535F70: 4BFFFBF1  bl 0x82535b60
	ctx.lr = 0x82535F74;
	sub_82535B60(ctx, base);
	// 82535F74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82535F78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82535F7C: 4BFFF190  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82535F80 size=40
    let mut pc: u32 = 0x82535F80;
    'dispatch: loop {
        match pc {
            0x82535F80 => {
    //   block [0x82535F80..0x82535FA8)
	// 82535F80: FBE1FFB8  std r31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.r[31].u64 ) };
	// 82535F84: 7FE802A6  mflr r31
	ctx.r[31].u64 = ctx.lr;
	// 82535F88: 9421FFB0  stwu r1, -0x50(r1)
	ea = ctx.r[1].u32.wrapping_add(-80 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82535F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82535F90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82535F94: 481D7DD9  bl 0x8270dd6c
	ctx.lr = 0x82535F98;
	// extern call 0x8270DD6C → crate::xboxkrnl::RtlUnwind
	crate::xboxkrnl::RtlUnwind(ctx, base);
	// 82535F98: 7FE803A6  mtlr r31
	ctx.lr = ctx.r[31].u64;
	// 82535F9C: EBE10008  ld r31, 8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(8 as u32) ) };
	// 82535FA0: 38210050  addi r1, r1, 0x50
	ctx.r[1].s64 = ctx.r[1].s64 + 80;
	// 82535FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535FB0 size=76
    let mut pc: u32 = 0x82535FB0;
    'dispatch: loop {
        match pc {
            0x82535FB0 => {
    //   block [0x82535FB0..0x82535FFC)
	// 82535FB0: D9CCFF70  stfd f14, -0x90(r12)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-144 as u32), ctx.f[14].u64 ) };
	// 82535FB4: D9ECFF78  stfd f15, -0x88(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-136 as u32), ctx.f[15].u64 ) };
	// 82535FB8: DA0CFF80  stfd f16, -0x80(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-128 as u32), ctx.f[16].u64 ) };
	// 82535FBC: DA2CFF88  stfd f17, -0x78(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-120 as u32), ctx.f[17].u64 ) };
	// 82535FC0: DA4CFF90  stfd f18, -0x70(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-112 as u32), ctx.f[18].u64 ) };
	// 82535FC4: DA6CFF98  stfd f19, -0x68(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-104 as u32), ctx.f[19].u64 ) };
	// 82535FC8: DA8CFFA0  stfd f20, -0x60(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-96 as u32), ctx.f[20].u64 ) };
	// 82535FCC: DAACFFA8  stfd f21, -0x58(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-88 as u32), ctx.f[21].u64 ) };
	// 82535FD0: DACCFFB0  stfd f22, -0x50(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-80 as u32), ctx.f[22].u64 ) };
	// 82535FD4: DAECFFB8  stfd f23, -0x48(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-72 as u32), ctx.f[23].u64 ) };
	// 82535FD8: DB0CFFC0  stfd f24, -0x40(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-64 as u32), ctx.f[24].u64 ) };
	// 82535FDC: DB2CFFC8  stfd f25, -0x38(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-56 as u32), ctx.f[25].u64 ) };
	// 82535FE0: DB4CFFD0  stfd f26, -0x30(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-48 as u32), ctx.f[26].u64 ) };
	// 82535FE4: DB6CFFD8  stfd f27, -0x28(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-40 as u32), ctx.f[27].u64 ) };
	// 82535FE8: DB8CFFE0  stfd f28, -0x20(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-32 as u32), ctx.f[28].u64 ) };
	// 82535FEC: DBACFFE8  stfd f29, -0x18(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-24 as u32), ctx.f[29].u64 ) };
	// 82535FF0: DBCCFFF0  stfd f30, -0x10(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-16 as u32), ctx.f[30].u64 ) };
	// 82535FF4: DBECFFF8  stfd f31, -8(r12)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[12].u32.wrapping_add(-8 as u32), ctx.f[31].u64 ) };
	// 82535FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82535FFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82535FFC size=76
    let mut pc: u32 = 0x82535FFC;
    'dispatch: loop {
        match pc {
            0x82535FFC => {
    //   block [0x82535FFC..0x82536048)
	// 82535FFC: C9CCFF70  lfd f14, -0x90(r12)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[14].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-144 as u32) ) };
	// 82536000: C9ECFF78  lfd f15, -0x88(r12)
	ctx.f[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-136 as u32) ) };
	// 82536004: CA0CFF80  lfd f16, -0x80(r12)
	ctx.f[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-128 as u32) ) };
	// 82536008: CA2CFF88  lfd f17, -0x78(r12)
	ctx.f[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-120 as u32) ) };
	// 8253600C: CA4CFF90  lfd f18, -0x70(r12)
	ctx.f[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-112 as u32) ) };
	// 82536010: CA6CFF98  lfd f19, -0x68(r12)
	ctx.f[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-104 as u32) ) };
	// 82536014: CA8CFFA0  lfd f20, -0x60(r12)
	ctx.f[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-96 as u32) ) };
	// 82536018: CAACFFA8  lfd f21, -0x58(r12)
	ctx.f[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-88 as u32) ) };
	// 8253601C: CACCFFB0  lfd f22, -0x50(r12)
	ctx.f[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-80 as u32) ) };
	// 82536020: CAECFFB8  lfd f23, -0x48(r12)
	ctx.f[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-72 as u32) ) };
	// 82536024: CB0CFFC0  lfd f24, -0x40(r12)
	ctx.f[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-64 as u32) ) };
	// 82536028: CB2CFFC8  lfd f25, -0x38(r12)
	ctx.f[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-56 as u32) ) };
	// 8253602C: CB4CFFD0  lfd f26, -0x30(r12)
	ctx.f[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-48 as u32) ) };
	// 82536030: CB6CFFD8  lfd f27, -0x28(r12)
	ctx.f[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-40 as u32) ) };
	// 82536034: CB8CFFE0  lfd f28, -0x20(r12)
	ctx.f[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-32 as u32) ) };
	// 82536038: CBACFFE8  lfd f29, -0x18(r12)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-24 as u32) ) };
	// 8253603C: CBCCFFF0  lfd f30, -0x10(r12)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-16 as u32) ) };
	// 82536040: CBECFFF8  lfd f31, -8(r12)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(-8 as u32) ) };
	// 82536044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536050 size=40
    let mut pc: u32 = 0x82536050;
    'dispatch: loop {
        match pc {
            0x82536050 => {
    //   block [0x82536050..0x82536078)
	// 82536050: 7C650074  cntlzd r5, r3
	ctx.r[5].u64 = if ctx.r[3].u64 == 0 { 64 } else { ctx.r[3].u64.leading_zeros() as u64 };
	// 82536054: 7C632836  sld r3, r3, r5
	if (ctx.r[5].u8 & 0x40) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = (ctx.r[3].u64) << ((ctx.r[5].u8 & 0x3F) as u32);
	}
	// 82536058: 2C230000  cmpdi r3, 0
	ctx.cr[0].compare_i64(ctx.r[0].s64, 0, &mut ctx.xer);
	// 8253605C: 41820010  beq 0x8253606c
	if ctx.cr[0].eq {
	pc = 0x8253606C; continue 'dispatch;
	}
	// 82536060: 20A5043E  subfic r5, r5, 0x43e
	ctx.xer.ca = ctx.r[5].u32 <= 1086 as u32;
	ctx.r[5].s64 = (1086 as i64) - ctx.r[5].s64;
	// 82536064: 7863AB02  rldicl r3, r3, 0x35, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 & 0x00000000000007FFu64;
	// 82536068: 78A3A04E  rldimi r3, r5, 0x34, 1
	ctx.r[3].u64 = ((ctx.r[5].u64).rotate_left(52) & 0x7FF0000000000000) | (ctx.r[3].u64 & 0x800FFFFFFFFFFFFF);
	// 8253606C: F861FFF8  std r3, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[3].u64 ) };
	// 82536070: C821FFF8  lfd f1, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82536074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536078 size=480
    let mut pc: u32 = 0x82536078;
    'dispatch: loop {
        match pc {
            0x82536078 => {
    //   block [0x82536078..0x82536258)
	// 82536078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253607C: 4BFFF041  bl 0x825350bc
	ctx.lr = 0x82536080;
	sub_82535080(ctx, base);
	// 82536080: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82536084: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82536088: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253608C: 3D60C007  lis r11, -0x3ff9
	ctx.r[11].s64 = -1073283072;
	// 82536090: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82536094: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82536098: DBE100A0  stfd f31, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.f[31].u64 ) };
	// 8253609C: 617DFEFF  ori r29, r11, 0xfeff
	ctx.r[29].u64 = ctx.r[11].u64 | 65279;
	// 825360A0: 386000F8  li r3, 0xf8
	ctx.r[3].s64 = 248;
	// 825360A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825360A8: 48005669  bl 0x8253b710
	ctx.lr = 0x825360AC;
	sub_8253B710(ctx, base);
	// 825360AC: A16100A0  lhz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 825360B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825360B4: 556B0476  rlwinm r11, r11, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825360B8: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 825360BC: 409A007C  bne cr6, 0x82536138
	if !ctx.cr[6].eq {
	pc = 0x82536138; continue 'dispatch;
	}
	// 825360C0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825360C4: 480049C5  bl 0x8253aa88
	ctx.lr = 0x825360C8;
	sub_8253AA88(ctx, base);
	// 825360C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825360CC: 40810038  ble 0x82536104
	if !ctx.cr[0].gt {
	pc = 0x82536104; continue 'dispatch;
	}
	// 825360D0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 825360D4: 40990164  ble cr6, 0x82536238
	if !ctx.cr[6].gt {
	pc = 0x82536238; continue 'dispatch;
	}
	// 825360D8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 825360DC: 409A0028  bne cr6, 0x82536104
	if !ctx.cr[6].eq {
	pc = 0x82536104; continue 'dispatch;
	}
	// 825360E0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 825360E4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825360E8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825360EC: 38600019  li r3, 0x19
	ctx.r[3].s64 = 25;
	// 825360F0: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825360F4: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825360F8: FC40069C  fcfid f2, f0
	ctx.f[2].f64 = (ctx.f[0].s64 as f64);
	// 825360FC: 480053CD  bl 0x8253b4c8
	ctx.lr = 0x82536100;
	sub_8253B4C8(ctx, base);
	// 82536100: 48000148  b 0x82536248
	pc = 0x82536248; continue 'dispatch;
	// 82536104: 7FEA07B4  extsw r10, r31
	ctx.r[10].s64 = ctx.r[31].s32 as i64;
	// 82536108: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253610C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82536110: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82536114: C80B2000  lfd f0, 0x2000(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8192 as u32) ) };
	// 82536118: FC7F002A  fadd f3, f31, f0
	ctx.f[3].f64 = ctx.f[31].f64 + ctx.f[0].f64;
	// 8253611C: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82536120: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82536124: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82536128: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 8253612C: FC40069C  fcfid f2, f0
	ctx.f[2].f64 = (ctx.f[0].s64 as f64);
	// 82536130: 480054F1  bl 0x8253b620
	ctx.lr = 0x82536134;
	sub_8253B620(ctx, base);
	// 82536134: 48000114  b 0x82536248
	pc = 0x82536248; continue 'dispatch;
	// 82536138: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253613C: CBCB2008  lfd f30, 0x2008(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 82536140: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82536144: 419A00F4  beq cr6, 0x82536238
	if ctx.cr[6].eq {
	pc = 0x82536238; continue 'dispatch;
	}
	// 82536148: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253614C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82536150: 480049C1  bl 0x8253ab10
	ctx.lr = 0x82536154;
	sub_8253AB10(ctx, base);
	// 82536154: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82536158: 4098001C  bge cr6, 0x82536174
	if !ctx.cr[6].lt {
	pc = 0x82536174; continue 'dispatch;
	}
	// 8253615C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82536160: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82536164: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82536168: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8253616C: 41980084  blt cr6, 0x825361f0
	if ctx.cr[6].lt {
	pc = 0x825361F0; continue 'dispatch;
	}
	// 82536170: 4800001C  b 0x8253618c
	pc = 0x8253618C; continue 'dispatch;
	// 82536174: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82536178: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8253617C: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82536180: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82536184: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82536188: 41990010  bgt cr6, 0x82536198
	if ctx.cr[6].gt {
	pc = 0x82536198; continue 'dispatch;
	}
	// 8253618C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82536190: 2F040A00  cmpwi cr6, r4, 0xa00
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2560, &mut ctx.xer);
	// 82536194: 4099002C  ble cr6, 0x825361c0
	if !ctx.cr[6].gt {
	pc = 0x825361C0; continue 'dispatch;
	}
	// 82536198: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253619C: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 825361A0: C82BEE88  lfd f1, -0x1178(r11)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4472 as u32) ) };
	// 825361A4: 4BFFE05D  bl 0x82534200
	ctx.lr = 0x825361A8;
	sub_82534200(ctx, base);
	// 825361A8: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 825361AC: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 825361B0: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 825361B4: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825361B8: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825361BC: 4BFFFF64  b 0x82536120
	pc = 0x82536120; continue 'dispatch;
	// 825361C0: 2F040400  cmpwi cr6, r4, 0x400
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1024, &mut ctx.xer);
	// 825361C4: 40990024  ble cr6, 0x825361e8
	if !ctx.cr[6].gt {
	pc = 0x825361E8; continue 'dispatch;
	}
	// 825361C8: 3884FA00  addi r4, r4, -0x600
	ctx.r[4].s64 = ctx.r[4].s64 + -1536;
	// 825361CC: 4800487D  bl 0x8253aa48
	ctx.lr = 0x825361D0;
	sub_8253AA48(ctx, base);
	// 825361D0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 825361D4: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 825361D8: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 825361DC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825361E0: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825361E4: 4BFFFF3C  b 0x82536120
	pc = 0x82536120; continue 'dispatch;
	// 825361E8: 2F04F603  cmpwi cr6, r4, -0x9fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2557, &mut ctx.xer);
	// 825361EC: 4098001C  bge cr6, 0x82536208
	if !ctx.cr[6].lt {
	pc = 0x82536208; continue 'dispatch;
	}
	// 825361F0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 825361F4: FC6107B2  fmul f3, f1, f30
	ctx.f[3].f64 = ctx.f[1].f64 * ctx.f[30].f64;
	// 825361F8: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 825361FC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82536200: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82536204: 4BFFFF1C  b 0x82536120
	pc = 0x82536120; continue 'dispatch;
	// 82536208: 2F04FC03  cmpwi cr6, r4, -0x3fd
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1021, &mut ctx.xer);
	// 8253620C: 40980024  bge cr6, 0x82536230
	if !ctx.cr[6].lt {
	pc = 0x82536230; continue 'dispatch;
	}
	// 82536210: 38840600  addi r4, r4, 0x600
	ctx.r[4].s64 = ctx.r[4].s64 + 1536;
	// 82536214: 48004835  bl 0x8253aa48
	ctx.lr = 0x82536218;
	sub_8253AA48(ctx, base);
	// 82536218: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 8253621C: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 82536220: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 82536224: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82536228: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8253622C: 4BFFFEF4  b 0x82536120
	pc = 0x82536120; continue 'dispatch;
	// 82536230: 48004819  bl 0x8253aa48
	ctx.lr = 0x82536234;
	sub_8253AA48(ctx, base);
	// 82536234: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82536238: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8253623C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536240: 480054D1  bl 0x8253b710
	ctx.lr = 0x82536244;
	sub_8253B710(ctx, base);
	// 82536244: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82536248: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8253624C: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82536250: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82536254: 4BFFEEB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82536258 size=184
    let mut pc: u32 = 0x82536258;
    'dispatch: loop {
        match pc {
            0x82536258 => {
    //   block [0x82536258..0x82536310)
	// 82536258: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253625C: FD800A10  fabs f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82536260: D981FFF8  stfd f12, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[12].u64 ) };
	// 82536264: 396BE7E0  addi r11, r11, -0x1820
	ctx.r[11].s64 = ctx.r[11].s64 + -6176;
	// 82536268: C00B001C  lfs f0, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8253626C: C12B0020  lfs f9, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82536270: FD21482E  fsel f9, f1, f0, f9
	ctx.f[9].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[9].f64 };
	// 82536274: C80B0008  lfd f0, 8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82536278: FC000332  fmul f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[12].f64;
	// 8253627C: C9AB0028  lfd f13, 0x28(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82536280: C96B0030  lfd f11, 0x30(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82536284: C94B0070  lfd f10, 0x70(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82536288: FC00065C  fctid f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64 as i64 };
	// 8253628C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82536290: FDAD603C  fnmsub f13, f13, f0, f12
	ctx.f[13].f64 = -(ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64);
	// 82536294: FD00065E  fctidz f8, f0
	ctx.f[8].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82536298: D901FFF0  stfd f8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[8].u64 ) };
	// 8253629C: FDAB683C  fnmsub f13, f11, f0, f13
	ctx.f[13].f64 = -(ctx.f[11].f64 * ctx.f[0].f64 - ctx.f[13].f64);
	// 825362A0: C96B0068  lfd f11, 0x68(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 825362A4: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 825362A8: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362AC: C96B0060  lfd f11, 0x60(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 825362B0: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362B4: C96B0058  lfd f11, 0x58(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 825362B8: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362BC: C96B0050  lfd f11, 0x50(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 825362C0: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825362C4: 794A07E0  clrldi r10, r10, 0x3f
	ctx.r[10].u64 = ctx.r[10].u64 & 0x0000000000000001u64;
	// 825362C8: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 825362CC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825362D0: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362D4: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 825362D8: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362DC: C96B0040  lfd f11, 0x40(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 825362E0: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362E4: C96B0038  lfd f11, 0x38(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 825362E8: FD4A583A  fmadd f10, f10, f0, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362EC: C96A2000  lfd f11, 0x2000(r10)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8192 as u32) ) };
	// 825362F0: FC0A583A  fmadd f0, f10, f0, f11
	ctx.f[0].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 825362F4: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 825362F8: 419A0008  beq cr6, 0x82536300
	if ctx.cr[6].eq {
	pc = 0x82536300; continue 'dispatch;
	}
	// 825362FC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82536300: E941FFF8  ld r10, -8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82536304: FDA00272  fmul f13, f0, f9
	ctx.f[13].f64 = ctx.f[0].f64 * ctx.f[9].f64;
	// 82536308: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 8253630C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536310 size=24
    let mut pc: u32 = 0x82536310;
    'dispatch: loop {
        match pc {
            0x82536310 => {
    //   block [0x82536310..0x82536328)
	// 82536310: C80B0010  lfd f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82536314: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536318: FD8C0028  fsub f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[0].f64;
	// 8253631C: C80BEE90  lfd f0, -0x1170(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4464 as u32) ) };
	// 82536320: FC2C682E  fsel f1, f12, f0, f13
	ctx.f[1].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82536324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82536328 size=192
    let mut pc: u32 = 0x82536328;
    'dispatch: loop {
        match pc {
            0x82536328 => {
    //   block [0x82536328..0x825363E8)
	// 82536328: FD800A10  fabs f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 8253632C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536330: 396BE7E0  addi r11, r11, -0x1820
	ctx.r[11].s64 = ctx.r[11].s64 + -6176;
	// 82536334: C80B0000  lfd f0, 0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82536338: C1AB0024  lfs f13, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8253633C: C94B0028  lfd f10, 0x28(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82536340: FD60602A  fadd f11, f0, f12
	ctx.f[11].f64 = ctx.f[0].f64 + ctx.f[12].f64;
	// 82536344: C80B0008  lfd f0, 8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82536348: C92B0030  lfd f9, 0x30(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 8253634C: C90B0070  lfd f8, 0x70(r11)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82536350: FC0002F2  fmul f0, f0, f11
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[11].f64;
	// 82536354: FC00065C  fctid f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64 as i64 };
	// 82536358: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8253635C: FCE00090  fmr f7, f0
	ctx.f[7].f64 = ctx.f[0].f64;
	// 82536360: FC006828  fsub f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82536364: FDA03E5E  fctidz f13, f7
	ctx.f[13].s64 = if ctx.f[7].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[7].f64.trunc() as i64 };
	// 82536368: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 8253636C: FDAA603C  fnmsub f13, f10, f0, f12
	ctx.f[13].f64 = -(ctx.f[10].f64 * ctx.f[0].f64 - ctx.f[12].f64);
	// 82536370: C94B0068  lfd f10, 0x68(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 82536374: FDA9683C  fnmsub f13, f9, f0, f13
	ctx.f[13].f64 = -(ctx.f[9].f64 * ctx.f[0].f64 - ctx.f[13].f64);
	// 82536378: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 8253637C: FD28503A  fmadd f9, f8, f0, f10
	ctx.f[9].f64 = ctx.f[8].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 82536380: C94B0060  lfd f10, 0x60(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 82536384: FD29503A  fmadd f9, f9, f0, f10
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 82536388: C94B0058  lfd f10, 0x58(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 8253638C: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82536390: FD29503A  fmadd f9, f9, f0, f10
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 82536394: C94B0050  lfd f10, 0x50(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82536398: 794A07E0  clrldi r10, r10, 0x3f
	ctx.r[10].u64 = ctx.r[10].u64 & 0x0000000000000001u64;
	// 8253639C: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 825363A0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825363A4: FD29503A  fmadd f9, f9, f0, f10
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 825363A8: C94B0048  lfd f10, 0x48(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 825363AC: FD29503A  fmadd f9, f9, f0, f10
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 825363B0: C94B0040  lfd f10, 0x40(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 825363B4: FD29503A  fmadd f9, f9, f0, f10
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 825363B8: C94B0038  lfd f10, 0x38(r11)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 825363BC: FD29503A  fmadd f9, f9, f0, f10
	ctx.f[9].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 825363C0: C94A2000  lfd f10, 0x2000(r10)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8192 as u32) ) };
	// 825363C4: FC09503A  fmadd f0, f9, f0, f10
	ctx.f[0].f64 = ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64;
	// 825363C8: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 825363CC: 419A0008  beq cr6, 0x825363d4
	if ctx.cr[6].eq {
	pc = 0x825363D4; continue 'dispatch;
	}
	// 825363D0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825363D4: C1AB0018  lfs f13, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825363D8: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 825363DC: 409A000C  bne cr6, 0x825363e8
	if !ctx.cr[6].eq {
		sub_825363E8(ctx, base);
		return;
	}
	// 825363E0: C02B001C  lfs f1, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825363E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825363E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825363E8 size=24
    let mut pc: u32 = 0x825363E8;
    'dispatch: loop {
        match pc {
            0x825363E8 => {
    //   block [0x825363E8..0x82536400)
	// 825363E8: C9AB0010  lfd f13, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 825363EC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 825363F0: FD8B6828  fsub f12, f11, f13
	ctx.f[12].f64 = ctx.f[11].f64 - ctx.f[13].f64;
	// 825363F4: C9ABEE90  lfd f13, -0x1170(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4464 as u32) ) };
	// 825363F8: FC2C036E  fsel f1, f12, f13, f0
	ctx.f[1].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 825363FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82536400 size=60
    let mut pc: u32 = 0x82536400;
    'dispatch: loop {
        match pc {
            0x82536400 => {
    //   block [0x82536400..0x8253643C)
	// 82536400: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82536404: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536408: 396BE858  addi r11, r11, -0x17a8
	ctx.r[11].s64 = ctx.r[11].s64 + -6056;
	// 8253640C: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82536410: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82536414: 40990028  ble cr6, 0x8253643c
	if !ctx.cr[6].gt {
		sub_8253643C(ctx, base);
		return;
	}
	// 82536418: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8253641C: 21440001  subfic r10, r4, 1
	ctx.xer.ca = ctx.r[4].u32 <= 1 as u32;
	ctx.r[10].s64 = (1 as i64) - ctx.r[4].s64;
	// 82536420: FDAC6828  fsub f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 82536424: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82536428: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 8253642C: FDA0002C  fsqrt f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64).sqrt();
	// 82536430: FDAD02F2  fmul f13, f13, f11
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[11].f64;
	// 82536434: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82536438: 48000020  b 0x82536458
	sub_82536458(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253643C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8253643C size=28
    let mut pc: u32 = 0x8253643C;
    'dispatch: loop {
        match pc {
            0x8253643C => {
    //   block [0x8253643C..0x82536458)
	// 8253643C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82536440: FC0D0372  fmul f0, f13, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82536444: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82536448: 409A0010  bne cr6, 0x82536458
	if !ctx.cr[6].eq {
		sub_82536458(ctx, base);
		return;
	}
	// 8253644C: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82536450: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 82536454: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536458 size=128
    let mut pc: u32 = 0x82536458;
    'dispatch: loop {
        match pc {
            0x82536458 => {
    //   block [0x82536458..0x825364D8)
	// 82536458: C96B0078  lfd f11, 0x78(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	// 8253645C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82536460: FD4B002A  fadd f10, f11, f0
	ctx.f[10].f64 = ctx.f[11].f64 + ctx.f[0].f64;
	// 82536464: C98B0050  lfd f12, 0x50(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82536468: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 8253646C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82536470: FD6C583A  fmadd f11, f12, f0, f11
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82536474: C98B0070  lfd f12, 0x70(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82536478: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 8253647C: C98B0040  lfd f12, 0x40(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82536480: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536484: C98B0068  lfd f12, 0x68(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 82536488: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 8253648C: C98B0038  lfd f12, 0x38(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82536490: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536494: C98B0060  lfd f12, 0x60(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 82536498: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 8253649C: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 825364A0: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 825364A4: C98B0058  lfd f12, 0x58(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 825364A8: FD8A603A  fmadd f12, f10, f0, f12
	ctx.f[12].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 825364AC: FC0B0032  fmul f0, f11, f0
	ctx.f[0].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 825364B0: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 825364B4: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 825364B8: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 825364BC: 409A001C  bne cr6, 0x825364d8
	if !ctx.cr[6].eq {
		sub_825364D8(ctx, base);
		return;
	}
	// 825364C0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 825364C4: 7DAA5CAE  lfdx f13, r10, r11
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 825364C8: FC0D002A  fadd f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 + ctx.f[0].f64;
	// 825364CC: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825364D0: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 825364D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825364D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825364D8 size=32
    let mut pc: u32 = 0x825364D8;
    'dispatch: loop {
        match pc {
            0x825364D8 => {
    //   block [0x825364D8..0x825364F8)
	// 825364D8: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 825364DC: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 825364E0: 7DAA4CAE  lfdx f13, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 825364E4: 7D8A5CAE  lfdx f12, r10, r11
	ctx.f[12].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	// 825364E8: FDAD0028  fsub f13, f13, f0
	ctx.f[13].f64 = ctx.f[13].f64 - ctx.f[0].f64;
	// 825364EC: FC0C002A  fadd f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 + ctx.f[0].f64;
	// 825364F0: FC21036E  fsel f1, f1, f13, f0
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 825364F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825364F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825364F8 size=60
    let mut pc: u32 = 0x825364F8;
    'dispatch: loop {
        match pc {
            0x825364F8 => {
    //   block [0x825364F8..0x82536534)
	// 825364F8: FDA00A10  fabs f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 825364FC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536500: 396BE858  addi r11, r11, -0x17a8
	ctx.r[11].s64 = ctx.r[11].s64 + -6056;
	// 82536504: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82536508: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8253650C: 40990028  ble cr6, 0x82536534
	if !ctx.cr[6].gt {
		sub_82536534(ctx, base);
		return;
	}
	// 82536510: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82536514: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82536518: FDAC6828  fsub f13, f12, f13
	ctx.f[13].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 8253651C: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82536520: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82536524: FDA0002C  fsqrt f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64).sqrt();
	// 82536528: FDAD02F2  fmul f13, f13, f11
	ctx.f[13].f64 = ctx.f[13].f64 * ctx.f[11].f64;
	// 8253652C: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82536530: 48000018  b 0x82536548
	sub_82536548(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82536534 size=20
    let mut pc: u32 = 0x82536534;
    'dispatch: loop {
        match pc {
            0x82536534 => {
    //   block [0x82536534..0x82536548)
	// 82536534: C18B0000  lfs f12, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82536538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8253653C: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82536540: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 82536544: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536548 size=120
    let mut pc: u32 = 0x82536548;
    'dispatch: loop {
        match pc {
            0x82536548 => {
    //   block [0x82536548..0x825365C0)
	// 82536548: C96B0078  lfd f11, 0x78(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	// 8253654C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82536550: FD4B002A  fadd f10, f11, f0
	ctx.f[10].f64 = ctx.f[11].f64 + ctx.f[0].f64;
	// 82536554: C98B0050  lfd f12, 0x50(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82536558: C96B0048  lfd f11, 0x48(r11)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 8253655C: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82536560: FD6C583A  fmadd f11, f12, f0, f11
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64;
	// 82536564: C98B0070  lfd f12, 0x70(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	// 82536568: 7D2A4CAE  lfdx f9, r10, r9
	ctx.f[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 8253656C: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536570: C98B0040  lfd f12, 0x40(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82536574: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536578: C98B0068  lfd f12, 0x68(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	// 8253657C: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536580: C98B0038  lfd f12, 0x38(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82536584: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536588: C98B0060  lfd f12, 0x60(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	// 8253658C: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536590: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82536594: FD6B603A  fmadd f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 82536598: C98B0058  lfd f12, 0x58(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	// 8253659C: FD8A603A  fmadd f12, f10, f0, f12
	ctx.f[12].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 825365A0: FC0B0032  fmul f0, f11, f0
	ctx.f[0].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 825365A4: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 825365A8: FC006024  fdiv f0, f0, f12
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[12].f64;
	// 825365AC: FC00682A  fadd f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 825365B0: FC00482A  fadd f0, f0, f9
	ctx.f[0].f64 = ctx.f[0].f64 + ctx.f[9].f64;
	// 825365B4: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 825365B8: FC21682E  fsel f1, f1, f0, f13
	ctx.f[1].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 825365BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825365C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825365C0 size=8
    let mut pc: u32 = 0x825365C0;
    'dispatch: loop {
        match pc {
            0x825365C0 => {
    //   block [0x825365C0..0x825365C8)
	// 825365C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825365C4: 4BFFFE3C  b 0x82536400
	sub_82536400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825365C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825365C8 size=288
    let mut pc: u32 = 0x825365C8;
    'dispatch: loop {
        match pc {
            0x825365C8 => {
    //   block [0x825365C8..0x825366E8)
	// 825365C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825365CC: 4BFFEADD  bl 0x825350a8
	ctx.lr = 0x825365D0;
	sub_82535080(ctx, base);
	// 825365D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825365D4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825365D8: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 825365DC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 825365E0: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 825365E4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825365E8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825365EC: 7F0B2214  add r24, r11, r4
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825365F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825365F4: 409A0040  bne cr6, 0x82536634
	if !ctx.cr[6].eq {
	pc = 0x82536634; continue 'dispatch;
	}
	// 825365F8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 825365FC: 419A0038  beq cr6, 0x82536634
	if ctx.cr[6].eq {
	pc = 0x82536634; continue 'dispatch;
	}
	// 82536600: 48004339  bl 0x8253a938
	ctx.lr = 0x82536604;
	sub_8253A938(ctx, base);
	// 82536604: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82536608: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253660C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82536610: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82536614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82536618: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253661C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82536620: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82536624: 480041DD  bl 0x8253a800
	ctx.lr = 0x82536628;
	sub_8253A800(ctx, base);
	// 82536628: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253662C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82536630: 4BFFEAC8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82536634: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82536638: 419AFFC8  beq cr6, 0x82536600
	if ctx.cr[6].eq {
	pc = 0x82536600; continue 'dispatch;
	}
	// 8253663C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82536640: 419AFFC0  beq cr6, 0x82536600
	if ctx.cr[6].eq {
	pc = 0x82536600; continue 'dispatch;
	}
	// 82536644: 7F04C040  cmplw cr6, r4, r24
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82536648: 4199FFE0  bgt cr6, 0x82536628
	if ctx.cr[6].gt {
	pc = 0x82536628; continue 'dispatch;
	}
	// 8253664C: 54BCF87F  rlwinm. r28, r5, 0x1f, 1, 0x1f
	ctx.r[28].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82536650: 4182006C  beq 0x825366bc
	if ctx.cr[0].eq {
	pc = 0x825366BC; continue 'dispatch;
	}
	// 82536654: 54BB07FF  clrlwi. r27, r5, 0x1f
	ctx.r[27].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82536658: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8253665C: 40820008  bne 0x82536664
	if !ctx.cr[0].eq {
	pc = 0x82536664; continue 'dispatch;
	}
	// 82536660: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82536664: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82536668: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8253666C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82536670: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82536674: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 82536678: 4E800421  bctrl
	ctx.lr = 0x8253667C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253667C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536680: 41820034  beq 0x825366b4
	if ctx.cr[0].eq {
	pc = 0x825366B4; continue 'dispatch;
	}
	// 82536684: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82536688: 40980018  bge cr6, 0x825366a0
	if !ctx.cr[6].lt {
	pc = 0x825366A0; continue 'dispatch;
	}
	// 8253668C: 7F1EF850  subf r24, r30, r31
	ctx.r[24].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82536690: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82536694: 409A0010  bne cr6, 0x825366a4
	if !ctx.cr[6].eq {
	pc = 0x825366A4; continue 'dispatch;
	}
	// 82536698: 38BCFFFF  addi r5, r28, -1
	ctx.r[5].s64 = ctx.r[28].s64 + -1;
	// 8253669C: 4800000C  b 0x825366a8
	pc = 0x825366A8; continue 'dispatch;
	// 825366A0: 7FBFF214  add r29, r31, r30
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 825366A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825366A8: 7F1DC040  cmplw cr6, r29, r24
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[24].u32, &mut ctx.xer);
	// 825366AC: 4099FFA0  ble cr6, 0x8253664c
	if !ctx.cr[6].gt {
	pc = 0x8253664C; continue 'dispatch;
	}
	// 825366B0: 4BFFFF78  b 0x82536628
	pc = 0x82536628; continue 'dispatch;
	// 825366B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825366B8: 4BFFFF74  b 0x8253662c
	pc = 0x8253662C; continue 'dispatch;
	// 825366BC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 825366C0: 419AFF68  beq cr6, 0x82536628
	if ctx.cr[6].eq {
	pc = 0x82536628; continue 'dispatch;
	}
	// 825366C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825366C8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825366CC: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 825366D0: 4E800421  bctrl
	ctx.lr = 0x825366D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825366D4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825366D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825366DC: 4082FF50  bne 0x8253662c
	if !ctx.cr[0].eq {
	pc = 0x8253662C; continue 'dispatch;
	}
	// 825366E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825366E4: 4BFFFF48  b 0x8253662c
	pc = 0x8253662C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825366E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825366E8 size=12
    let mut pc: u32 = 0x825366E8;
    'dispatch: loop {
        match pc {
            0x825366E8 => {
    //   block [0x825366E8..0x825366F4)
	// 825366E8: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 825366EC: 906B2D60  stw r3, 0x2d60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(11616 as u32), ctx.r[3].u32 ) };
	// 825366F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825366F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825366F8 size=68
    let mut pc: u32 = 0x825366F8;
    'dispatch: loop {
        match pc {
            0x825366F8 => {
    //   block [0x825366F8..0x8253673C)
	// 825366F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825366FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536704: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82536708: 816B2D60  lwz r11, 0x2d60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11616 as u32) ) } as u64;
	// 8253670C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82536710: 41820018  beq 0x82536728
	if ctx.cr[0].eq {
	pc = 0x82536728; continue 'dispatch;
	}
	// 82536714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82536718: 4E800421  bctrl
	ctx.lr = 0x8253671C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253671C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536720: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82536724: 40820008  bne 0x8253672c
	if !ctx.cr[0].eq {
	pc = 0x8253672C; continue 'dispatch;
	}
	// 82536728: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253672C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82536730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536740 size=132
    let mut pc: u32 = 0x82536740;
    'dispatch: loop {
        match pc {
            0x82536740 => {
    //   block [0x82536740..0x825367C4)
	// 82536740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8253674C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82536750: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536754: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82536758: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8253675C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82536760: 388B6FF4  addi r4, r11, 0x6ff4
	ctx.r[4].s64 = ctx.r[11].s64 + 28660;
	// 82536764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82536768: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8253676C: 4BFFE3E5  bl 0x82534b50
	ctx.lr = 0x82536770;
	sub_82534B50(ctx, base);
	// 82536770: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82536774: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82536778: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8253677C: 419A001C  beq cr6, 0x82536798
	if ctx.cr[6].eq {
	pc = 0x82536798; continue 'dispatch;
	}
	// 82536780: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536784: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536788: 41820010  beq 0x82536798
	if ctx.cr[0].eq {
	pc = 0x82536798; continue 'dispatch;
	}
	// 8253678C: 3D600199  lis r11, 0x199
	ctx.r[11].s64 = 26804224;
	// 82536790: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 82536794: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82536798: 38C10064  addi r6, r1, 0x64
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	// 8253679C: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825367A0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825367A4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825367A8: 4BE8B569  bl 0x823c1d10
	ctx.lr = 0x825367AC;
	sub_823C1D10(ctx, base);
	// 825367AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825367B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825367B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825367B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825367BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825367C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825367C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825367C8 size=88
    let mut pc: u32 = 0x825367C8;
    'dispatch: loop {
        match pc {
            0x825367C8 => {
    //   block [0x825367C8..0x82536820)
	// 825367C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825367CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825367D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825367D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825367D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825367DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825367E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825367E4: 396B7014  addi r11, r11, 0x7014
	ctx.r[11].s64 = ctx.r[11].s64 + 28692;
	// 825367E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825367EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825367F0: 48006EC1  bl 0x8253d6b0
	ctx.lr = 0x825367F4;
	sub_8253D6B0(ctx, base);
	// 825367F4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825367F8: 4182000C  beq 0x82536804
	if ctx.cr[0].eq {
	pc = 0x82536804; continue 'dispatch;
	}
	// 825367FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82536800: 4BFFC3B9  bl 0x82532bb8
	ctx.lr = 0x82536804;
	sub_82532BB8(ctx, base);
	// 82536804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82536808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253680C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82536818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253681C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536820 size=176
    let mut pc: u32 = 0x82536820;
    'dispatch: loop {
        match pc {
            0x82536820 => {
    //   block [0x82536820..0x825368D0)
	// 82536820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253682C: 9421F500  stwu r1, -0xb00(r1)
	ea = ctx.r[1].u32.wrapping_add(-2816 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536830: 3FE08282  lis r31, -0x7d7e
	ctx.r[31].s64 = -2105409536;
	// 82536834: 817FE8DC  lwz r11, -0x1724(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-5924 as u32) ) } as u64;
	// 82536838: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253683C: 4182000C  beq 0x82536848
	if ctx.cr[0].eq {
	pc = 0x82536848; continue 'dispatch;
	}
	// 82536840: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82536844: 480031DD  bl 0x82539a20
	ctx.lr = 0x82536848;
	sub_82539A20(ctx, base);
	// 82536848: 48006F31  bl 0x8253d778
	ctx.lr = 0x8253684C;
	sub_8253D778(ctx, base);
	// 8253684C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82536850: 4182000C  beq 0x8253685c
	if ctx.cr[0].eq {
	pc = 0x8253685C; continue 'dispatch;
	}
	// 82536854: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 82536858: 48006F39  bl 0x8253d790
	ctx.lr = 0x8253685C;
	sub_8253D790(ctx, base);
	// 8253685C: 817FE8DC  lwz r11, -0x1724(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-5924 as u32) ) } as u64;
	// 82536860: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536864: 41820064  beq 0x825368c8
	if ctx.cr[0].eq {
	pc = 0x825368C8; continue 'dispatch;
	}
	// 82536868: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 8253686C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82536870: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82536874: 4BFFE95D  bl 0x825351d0
	ctx.lr = 0x82536878;
	sub_825351D0(ctx, base);
	// 82536878: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8253687C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82536880: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82536884: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82536888: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8253688C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82536890: 4200FFF8  bdnz 0x82536888
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82536888; continue 'dispatch;
	}
	// 82536894: 3D604000  lis r11, 0x4000
	ctx.r[11].s64 = 1073741824;
	// 82536898: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253689C: 616B0015  ori r11, r11, 0x15
	ctx.r[11].u64 = ctx.r[11].u64 | 21;
	// 825368A0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825368A4: 81610AF8  lwz r11, 0xaf8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2808 as u32) ) } as u64;
	// 825368A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825368AC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825368B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825368B4: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 825368B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825368BC: 4BE8B2DD  bl 0x823c1b98
	ctx.lr = 0x825368C0;
	sub_823C1B98(ctx, base);
	// 825368C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825368C4: 4BE8B3BD  bl 0x823c1c80
	ctx.lr = 0x825368C8;
	sub_823C1C80(ctx, base);
	// 825368C8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 825368CC: 4BFFF1B5  bl 0x82535a80
	ctx.lr = 0x825368D0;
	sub_82535A80(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825368D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825368D0 size=32
    let mut pc: u32 = 0x825368D0;
    'dispatch: loop {
        match pc {
            0x825368D0 => {
    //   block [0x825368D0..0x825368F0)
	// 825368D0: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 825368D4: 7C692038  and r9, r3, r4
	ctx.r[9].u64 = ctx.r[3].u64 & ctx.r[4].u64;
	// 825368D8: 816AE8DC  lwz r11, -0x1724(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5924 as u32) ) } as u64;
	// 825368DC: 7D682078  andc r8, r11, r4
	ctx.r[8].u64 = ctx.r[11].u64 & !ctx.r[4].u64;
	// 825368E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825368E4: 7D0B4B78  or r11, r8, r9
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 825368E8: 916AE8DC  stw r11, -0x1724(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5924 as u32), ctx.r[11].u32 ) };
	// 825368EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825368F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825368F0 size=8
    let mut pc: u32 = 0x825368F0;
    'dispatch: loop {
        match pc {
            0x825368F0 => {
    //   block [0x825368F0..0x825368F8)
	// 825368F0: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 825368F4: 820DA368  lwz r16, -0x5c98(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825368F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825368F8 size=472
    let mut pc: u32 = 0x825368F8;
    'dispatch: loop {
        match pc {
            0x825368F8 => {
    //   block [0x825368F8..0x82536AD0)
	// 825368F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825368FC: 4BFFE7B9  bl 0x825350b4
	ctx.lr = 0x82536900;
	sub_82535080(ctx, base);
	// 82536900: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82536904: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536908: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8253690C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82536910: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82536914: 7F6B0034  cntlzw r11, r27
	ctx.r[11].u64 = if ctx.r[27].u32 == 0 { 32 } else { ctx.r[27].u32.leading_zeros() as u64 };
	// 82536918: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8253691C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82536920: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536924: 40820034  bne 0x82536958
	if !ctx.cr[0].eq {
	pc = 0x82536958; continue 'dispatch;
	}
	// 82536928: 48004011  bl 0x8253a938
	ctx.lr = 0x8253692C;
	sub_8253A938(ctx, base);
	// 8253692C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82536930: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82536934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82536938: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253693C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82536940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82536944: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82536948: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253694C: 48003EB5  bl 0x8253a800
	ctx.lr = 0x82536950;
	sub_8253A800(ctx, base);
	// 82536950: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82536954: 48000174  b 0x82536ac8
	pc = 0x82536AC8; continue 'dispatch;
	// 82536958: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 8253695C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82536960: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82536964: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536968: 4182FFC0  beq 0x82536928
	if ctx.cr[0].eq {
	pc = 0x82536928; continue 'dispatch;
	}
	// 8253696C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536970: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536974: 408200CC  bne 0x82536a40
	if !ctx.cr[0].eq {
	pc = 0x82536A40; continue 'dispatch;
	}
	// 82536978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253697C: 48007245  bl 0x8253dbc0
	ctx.lr = 0x82536980;
	sub_8253DBC0(ctx, base);
	// 82536980: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82536984: 419A004C  beq cr6, 0x825369d0
	if ctx.cr[6].eq {
	pc = 0x825369D0; continue 'dispatch;
	}
	// 82536988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253698C: 48007235  bl 0x8253dbc0
	ctx.lr = 0x82536990;
	sub_8253DBC0(ctx, base);
	// 82536990: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 82536994: 419A003C  beq cr6, 0x825369d0
	if ctx.cr[6].eq {
	pc = 0x825369D0; continue 'dispatch;
	}
	// 82536998: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253699C: 48007225  bl 0x8253dbc0
	ctx.lr = 0x825369A0;
	sub_8253DBC0(ctx, base);
	// 825369A0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 825369A4: 3BAB3CE0  addi r29, r11, 0x3ce0
	ctx.r[29].s64 = ctx.r[11].s64 + 15584;
	// 825369A8: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 825369AC: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 825369B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825369B4: 4800720D  bl 0x8253dbc0
	ctx.lr = 0x825369B8;
	sub_8253DBC0(ctx, base);
	// 825369B8: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 825369BC: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 825369C0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825369C4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 825369C8: 3B8BF188  addi r28, r11, -0xe78
	ctx.r[28].s64 = ctx.r[11].s64 + -3704;
	// 825369CC: 48000018  b 0x825369e4
	pc = 0x825369E4; continue 'dispatch;
	// 825369D0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 825369D4: 3B8BF188  addi r28, r11, -0xe78
	ctx.r[28].s64 = ctx.r[11].s64 + -3704;
	// 825369D8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825369DC: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 825369E0: 3BAB3CE0  addi r29, r11, 0x3ce0
	ctx.r[29].s64 = ctx.r[11].s64 + 15584;
	// 825369E4: 896A0028  lbz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 825369E8: 556B003D  rlwinm. r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825369EC: 4082FF3C  bne 0x82536928
	if !ctx.cr[0].eq {
	pc = 0x82536928; continue 'dispatch;
	}
	// 825369F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825369F4: 480071CD  bl 0x8253dbc0
	ctx.lr = 0x825369F8;
	sub_8253DBC0(ctx, base);
	// 825369F8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825369FC: 419A0038  beq cr6, 0x82536a34
	if ctx.cr[6].eq {
	pc = 0x82536A34; continue 'dispatch;
	}
	// 82536A00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536A04: 480071BD  bl 0x8253dbc0
	ctx.lr = 0x82536A08;
	sub_8253DBC0(ctx, base);
	// 82536A08: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 82536A0C: 419A0028  beq cr6, 0x82536a34
	if ctx.cr[6].eq {
	pc = 0x82536A34; continue 'dispatch;
	}
	// 82536A10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536A14: 480071AD  bl 0x8253dbc0
	ctx.lr = 0x82536A18;
	sub_8253DBC0(ctx, base);
	// 82536A18: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 82536A1C: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82536A20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536A24: 4800719D  bl 0x8253dbc0
	ctx.lr = 0x82536A28;
	sub_8253DBC0(ctx, base);
	// 82536A28: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82536A2C: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 82536A30: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82536A34: 897C0028  lbz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82536A38: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536A3C: 4082FEEC  bne 0x82536928
	if !ctx.cr[0].eq {
	pc = 0x82536928; continue 'dispatch;
	}
	// 82536A40: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82536A44: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82536A48: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536A4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82536A50: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82536A54: 409AFFF4  bne cr6, 0x82536a48
	if !ctx.cr[6].eq {
	pc = 0x82536A48; continue 'dispatch;
	}
	// 82536A58: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82536A5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536A60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82536A64: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82536A68: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82536A6C: 4800022D  bl 0x82536c98
	ctx.lr = 0x82536A70;
	sub_82536C98(ctx, base);
	// 82536A70: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536A78: 48004D41  bl 0x8253b7b8
	ctx.lr = 0x82536A7C;
	sub_8253B7B8(ctx, base);
	// 82536A7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82536A80: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82536A84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82536A88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82536A8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82536A90: 48002311  bl 0x82538da0
	ctx.lr = 0x82536A94;
	sub_82538DA0(ctx, base);
	// 82536A94: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82536A98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82536A9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82536AA0: 48004E01  bl 0x8253b8a0
	ctx.lr = 0x82536AA4;
	sub_8253B8A0(ctx, base);
	// 82536AA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536AA8: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 82536AAC: 48000045  bl 0x82536af0
	ctx.lr = 0x82536AB0;
	sub_82536AD0(ctx, base);
	// 82536AB0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82536AB4: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82536AB8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82536ABC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82536AC0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82536AC4: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82536AC8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82536ACC: 4BFFE638  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536AD0 size=88
    let mut pc: u32 = 0x82536AD0;
    'dispatch: loop {
        match pc {
            0x82536AD0 => {
    //   block [0x82536AD0..0x82536B28)
	// 82536AD0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82536AD4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82536AD8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82536ADC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536AE0: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82536AE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536AE8: 83DF00AC  lwz r30, 0xac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82536AEC: 4800001C  b 0x82536b08
	pc = 0x82536B08; continue 'dispatch;
	// 82536AF0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82536AF4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82536AF8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82536AFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536B00: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82536B04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536B08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82536B0C: 4800024D  bl 0x82536d58
	ctx.lr = 0x82536B10;
	sub_82536D58(ctx, base);
	// 82536B10: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536B14: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82536B18: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82536B1C: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82536B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536B28 size=12
    let mut pc: u32 = 0x82536B28;
    'dispatch: loop {
        match pc {
            0x82536B28 => {
    //   block [0x82536B28..0x82536B34)
	// 82536B28: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536B2C: 386BE8E0  addi r3, r11, -0x1720
	ctx.r[3].s64 = ctx.r[11].s64 + -5920;
	// 82536B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536B38 size=288
    let mut pc: u32 = 0x82536B38;
    'dispatch: loop {
        match pc {
            0x82536B38 => {
    //   block [0x82536B38..0x82536C58)
	// 82536B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536B40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82536B44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82536B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536B4C: 3FE08313  lis r31, -0x7ced
	ctx.r[31].s64 = -2095906816;
	// 82536B50: 807F3DE8  lwz r3, 0x3de8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(15848 as u32) ) } as u64;
	// 82536B54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82536B58: 409A000C  bne cr6, 0x82536b64
	if !ctx.cr[6].eq {
	pc = 0x82536B64; continue 'dispatch;
	}
	// 82536B5C: 38600200  li r3, 0x200
	ctx.r[3].s64 = 512;
	// 82536B60: 48000010  b 0x82536b70
	pc = 0x82536B70; continue 'dispatch;
	// 82536B64: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 82536B68: 4098000C  bge cr6, 0x82536b74
	if !ctx.cr[6].lt {
	pc = 0x82536B74; continue 'dispatch;
	}
	// 82536B6C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82536B70: 907F3DE8  stw r3, 0x3de8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(15848 as u32), ctx.r[3].u32 ) };
	// 82536B74: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82536B78: 48002E41  bl 0x825399b8
	ctx.lr = 0x82536B7C;
	sub_825399B8(ctx, base);
	// 82536B7C: 3FC08313  lis r30, -0x7ced
	ctx.r[30].s64 = -2095906816;
	// 82536B80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82536B84: 907E3DE4  stw r3, 0x3de4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(15844 as u32), ctx.r[3].u32 ) };
	// 82536B88: 4082002C  bne 0x82536bb4
	if !ctx.cr[0].eq {
	pc = 0x82536BB4; continue 'dispatch;
	}
	// 82536B8C: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82536B90: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82536B94: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82536B98: 917F3DE8  stw r11, 0x3de8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(15848 as u32), ctx.r[11].u32 ) };
	// 82536B9C: 48002E1D  bl 0x825399b8
	ctx.lr = 0x82536BA0;
	sub_825399B8(ctx, base);
	// 82536BA0: 907E3DE4  stw r3, 0x3de4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(15844 as u32), ctx.r[3].u32 ) };
	// 82536BA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82536BA8: 4082000C  bne 0x82536bb4
	if !ctx.cr[0].eq {
	pc = 0x82536BB4; continue 'dispatch;
	}
	// 82536BAC: 3860001A  li r3, 0x1a
	ctx.r[3].s64 = 26;
	// 82536BB0: 48000090  b 0x82536c40
	pc = 0x82536C40; continue 'dispatch;
	// 82536BB4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82536BBC: 392BE8E0  addi r9, r11, -0x1720
	ctx.r[9].s64 = ctx.r[11].s64 + -5920;
	// 82536BC0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82536BC4: 48000008  b 0x82536bcc
	pc = 0x82536BCC; continue 'dispatch;
	// 82536BC8: 807E3DE4  lwz r3, 0x3de4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(15844 as u32) ) } as u64;
	// 82536BCC: 7D6A192E  stwx r11, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82536BD0: 39090280  addi r8, r9, 0x280
	ctx.r[8].s64 = ctx.r[9].s64 + 640;
	// 82536BD4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82536BD8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82536BDC: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82536BE0: 4198FFE8  blt cr6, 0x82536bc8
	if ctx.cr[6].lt {
	pc = 0x82536BC8; continue 'dispatch;
	}
	// 82536BE4: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82536BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82536BEC: 39090010  addi r8, r9, 0x10
	ctx.r[8].s64 = ctx.r[9].s64 + 16;
	// 82536BF0: 38EA3CE0  addi r7, r10, 0x3ce0
	ctx.r[7].s64 = ctx.r[10].s64 + 15584;
	// 82536BF4: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82536BF8: 55663572  rlwinm r6, r11, 6, 0x15, 0x19
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82536BFC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82536C00: 7D4A382E  lwzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82536C04: 7D4A302E  lwzx r10, r10, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82536C08: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82536C0C: 419A0014  beq cr6, 0x82536c20
	if ctx.cr[6].eq {
	pc = 0x82536C20; continue 'dispatch;
	}
	// 82536C10: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 82536C14: 419A000C  beq cr6, 0x82536c20
	if ctx.cr[6].eq {
	pc = 0x82536C20; continue 'dispatch;
	}
	// 82536C18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82536C1C: 409A000C  bne cr6, 0x82536c28
	if !ctx.cr[6].eq {
	pc = 0x82536C28; continue 'dispatch;
	}
	// 82536C20: 3940FFFE  li r10, -2
	ctx.r[10].s64 = -2;
	// 82536C24: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82536C28: 39080020  addi r8, r8, 0x20
	ctx.r[8].s64 = ctx.r[8].s64 + 32;
	// 82536C2C: 39490070  addi r10, r9, 0x70
	ctx.r[10].s64 = ctx.r[9].s64 + 112;
	// 82536C30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82536C34: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82536C38: 4198FFBC  blt cr6, 0x82536bf4
	if ctx.cr[6].lt {
	pc = 0x82536BF4; continue 'dispatch;
	}
	// 82536C3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82536C40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82536C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536C4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82536C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82536C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536C58 size=64
    let mut pc: u32 = 0x82536C58;
    'dispatch: loop {
        match pc {
            0x82536C58 => {
    //   block [0x82536C58..0x82536C98)
	// 82536C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536C64: 4800055D  bl 0x825371c0
	ctx.lr = 0x82536C68;
	sub_825371C0(ctx, base);
	// 82536C68: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82536C6C: 896B2D38  lbz r11, 0x2d38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11576 as u32) ) } as u64;
	// 82536C70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82536C74: 41820008  beq 0x82536c7c
	if ctx.cr[0].eq {
	pc = 0x82536C7C; continue 'dispatch;
	}
	// 82536C78: 48006FA9  bl 0x8253dc20
	ctx.lr = 0x82536C7C;
	sub_8253DC20(ctx, base);
	// 82536C7C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82536C80: 806B3DE4  lwz r3, 0x3de4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15844 as u32) ) } as u64;
	// 82536C84: 4BFFD0B5  bl 0x82533d38
	ctx.lr = 0x82536C88;
	sub_82533D38(ctx, base);
	// 82536C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82536C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536C98 size=108
    let mut pc: u32 = 0x82536C98;
    'dispatch: loop {
        match pc {
            0x82536C98 => {
    //   block [0x82536C98..0x82536D04)
	// 82536C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536CA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82536CA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536CA8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82536CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82536CB0: 396BE8E0  addi r11, r11, -0x1720
	ctx.r[11].s64 = ctx.r[11].s64 + -5920;
	// 82536CB4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82536CB8: 41980030  blt cr6, 0x82536ce8
	if ctx.cr[6].lt {
	pc = 0x82536CE8; continue 'dispatch;
	}
	// 82536CBC: 394B0260  addi r10, r11, 0x260
	ctx.r[10].s64 = ctx.r[11].s64 + 608;
	// 82536CC0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82536CC4: 41990024  bgt cr6, 0x82536ce8
	if ctx.cr[6].gt {
	pc = 0x82536CE8; continue 'dispatch;
	}
	// 82536CC8: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82536CCC: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82536CD0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82536CD4: 4800674D  bl 0x8253d420
	ctx.lr = 0x82536CD8;
	sub_8253D420(ctx, base);
	// 82536CD8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536CDC: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 82536CE0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82536CE4: 4800000C  b 0x82536cf0
	pc = 0x82536CF0; continue 'dispatch;
	// 82536CE8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82536CEC: 481D6571  bl 0x8270d25c
	ctx.lr = 0x82536CF0;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82536CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82536CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82536D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536D08 size=80
    let mut pc: u32 = 0x82536D08;
    'dispatch: loop {
        match pc {
            0x82536D08 => {
    //   block [0x82536D08..0x82536D58)
	// 82536D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82536D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536D18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82536D1C: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 82536D20: 4098001C  bge cr6, 0x82536d3c
	if !ctx.cr[6].lt {
	pc = 0x82536D3C; continue 'dispatch;
	}
	// 82536D24: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82536D28: 480066F9  bl 0x8253d420
	ctx.lr = 0x82536D2C;
	sub_8253D420(ctx, base);
	// 82536D2C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536D30: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 82536D34: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82536D38: 4800000C  b 0x82536d44
	pc = 0x82536D44; continue 'dispatch;
	// 82536D3C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82536D40: 481D651D  bl 0x8270d25c
	ctx.lr = 0x82536D44;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82536D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82536D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82536D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536D58 size=60
    let mut pc: u32 = 0x82536D58;
    'dispatch: loop {
        match pc {
            0x82536D58 => {
    //   block [0x82536D58..0x82536D94)
	// 82536D58: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82536D5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82536D60: 394AE8E0  addi r10, r10, -0x1720
	ctx.r[10].s64 = ctx.r[10].s64 + -5920;
	// 82536D64: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82536D68: 4198002C  blt cr6, 0x82536d94
	if ctx.cr[6].lt {
		sub_82536D94(ctx, base);
		return;
	}
	// 82536D6C: 392A0260  addi r9, r10, 0x260
	ctx.r[9].s64 = ctx.r[10].s64 + 608;
	// 82536D70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82536D74: 41990020  bgt cr6, 0x82536d94
	if ctx.cr[6].gt {
		sub_82536D94(ctx, base);
		return;
	}
	// 82536D78: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536D7C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82536D80: 5529045E  rlwinm r9, r9, 0, 0x11, 0xf
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82536D84: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 82536D88: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82536D8C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82536D90: 48006530  b 0x8253d2c0
	sub_8253D2C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536D94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536D94 size=8
    let mut pc: u32 = 0x82536D94;
    'dispatch: loop {
        match pc {
            0x82536D94 => {
    //   block [0x82536D94..0x82536D9C)
	// 82536D94: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82536D98: 481D64D4  b 0x8270d26c
	sub_8270D26C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536DA0 size=28
    let mut pc: u32 = 0x82536DA0;
    'dispatch: loop {
        match pc {
            0x82536DA0 => {
    //   block [0x82536DA0..0x82536DBC)
	// 82536DA0: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 82536DA4: 40980018  bge cr6, 0x82536dbc
	if !ctx.cr[6].lt {
		sub_82536DBC(ctx, base);
		return;
	}
	// 82536DA8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536DAC: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82536DB0: 556B045E  rlwinm r11, r11, 0, 0x11, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82536DB4: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82536DB8: 48006508  b 0x8253d2c0
	sub_8253D2C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536DBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536DBC size=8
    let mut pc: u32 = 0x82536DBC;
    'dispatch: loop {
        match pc {
            0x82536DBC => {
    //   block [0x82536DBC..0x82536DC4)
	// 82536DBC: 38640020  addi r3, r4, 0x20
	ctx.r[3].s64 = ctx.r[4].s64 + 32;
	// 82536DC0: 481D64AC  b 0x8270d26c
	sub_8270D26C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536DC8 size=148
    let mut pc: u32 = 0x82536DC8;
    'dispatch: loop {
        match pc {
            0x82536DC8 => {
    //   block [0x82536DC8..0x82536E5C)
	// 82536DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536DCC: 4BFFE2ED  bl 0x825350b8
	ctx.lr = 0x82536DD0;
	sub_82535080(ctx, base);
	// 82536DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82536DD8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82536DDC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536DE0: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82536DE4: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82536DE8: 409A0058  bne cr6, 0x82536e40
	if !ctx.cr[6].eq {
	pc = 0x82536E40; continue 'dispatch;
	}
	// 82536DEC: 716B0108  andi. r11, r11, 0x108
	ctx.r[11].u64 = ctx.r[11].u64 & 264;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536DF0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536DF4: 4182004C  beq 0x82536e40
	if ctx.cr[0].eq {
	pc = 0x82536E40; continue 'dispatch;
	}
	// 82536DF8: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82536DFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536E00: 7FDD5851  subf. r30, r29, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82536E04: 4081003C  ble 0x82536e40
	if !ctx.cr[0].gt {
	pc = 0x82536E40; continue 'dispatch;
	}
	// 82536E08: 48006DB9  bl 0x8253dbc0
	ctx.lr = 0x82536E0C;
	sub_8253DBC0(ctx, base);
	// 82536E0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82536E10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82536E14: 4800714D  bl 0x8253df60
	ctx.lr = 0x82536E18;
	sub_8253DF60(ctx, base);
	// 82536E18: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536E1C: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82536E20: 409A0014  bne cr6, 0x82536e34
	if !ctx.cr[6].eq {
	pc = 0x82536E34; continue 'dispatch;
	}
	// 82536E24: 556A0631  rlwinm. r10, r11, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82536E28: 41820018  beq 0x82536e40
	if ctx.cr[0].eq {
	pc = 0x82536E40; continue 'dispatch;
	}
	// 82536E2C: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82536E30: 4800000C  b 0x82536e3c
	pc = 0x82536E3C; continue 'dispatch;
	// 82536E34: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82536E38: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82536E3C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82536E40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82536E44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82536E48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82536E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82536E50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82536E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82536E58: 4BFFE2B0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82536E60 size=120
    let mut pc: u32 = 0x82536E60;
    'dispatch: loop {
        match pc {
            0x82536E60 => {
    //   block [0x82536E60..0x82536ED8)
	// 82536E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82536E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82536E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82536E74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82536E78: 409A000C  bne cr6, 0x82536e84
	if !ctx.cr[6].eq {
	pc = 0x82536E84; continue 'dispatch;
	}
	// 82536E7C: 48000065  bl 0x82536ee0
	ctx.lr = 0x82536E80;
	sub_82536EE0(ctx, base);
	// 82536E80: 48000044  b 0x82536ec4
	pc = 0x82536EC4; continue 'dispatch;
	// 82536E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82536E88: 4BFFFF41  bl 0x82536dc8
	ctx.lr = 0x82536E8C;
	sub_82536DC8(ctx, base);
	// 82536E8C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536E90: 4182000C  beq 0x82536e9c
	if ctx.cr[0].eq {
	pc = 0x82536E9C; continue 'dispatch;
	}
	// 82536E94: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82536E98: 4800002C  b 0x82536ec4
	pc = 0x82536EC4; continue 'dispatch;
	// 82536E9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536EA0: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536EA4: 4182001C  beq 0x82536ec0
	if ctx.cr[0].eq {
	pc = 0x82536EC0; continue 'dispatch;
	}
	// 82536EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82536EAC: 48006D15  bl 0x8253dbc0
	ctx.lr = 0x82536EB0;
	sub_8253DBC0(ctx, base);
	// 82536EB0: 48007249  bl 0x8253e0f8
	ctx.lr = 0x82536EB4;
	sub_8253E0F8(ctx, base);
	// 82536EB4: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 82536EB8: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82536EBC: 48000008  b 0x82536ec4
	pc = 0x82536EC4; continue 'dispatch;
	// 82536EC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82536EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82536EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82536ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82536ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82536ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82536ED8 size=8
    let mut pc: u32 = 0x82536ED8;
    'dispatch: loop {
        match pc {
            0x82536ED8 => {
    //   block [0x82536ED8..0x82536EE0)
	// 82536ED8: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 82536EDC: 820DA380  lwz r16, -0x5c80(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82536EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82536EE0 size=296
    let mut pc: u32 = 0x82536EE0;
    'dispatch: loop {
        match pc {
            0x82536EE0 => {
    //   block [0x82536EE0..0x82537008)
	// 82536EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82536EE4: 4BFFE1D1  bl 0x825350b4
	ctx.lr = 0x82536EE8;
	sub_82535080(ctx, base);
	// 82536EE8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82536EEC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82536EF0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82536EF4: 937F00A4  stw r27, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[27].u32 ) };
	// 82536EF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82536EFC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82536F00: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82536F04: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82536F08: 48006519  bl 0x8253d420
	ctx.lr = 0x82536F0C;
	sub_8253D420(ctx, base);
	// 82536F0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536F10: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82536F14: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82536F18: 3BCB3DE4  addi r30, r11, 0x3de4
	ctx.r[30].s64 = ctx.r[11].s64 + 15844;
	// 82536F1C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82536F20: 394B3DE8  addi r10, r11, 0x3de8
	ctx.r[10].s64 = ctx.r[11].s64 + 15848;
	// 82536F24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536F28: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536F2C: 7F1C4800  cmpw cr6, r28, r9
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82536F30: 409800B0  bge cr6, 0x82536fe0
	if !ctx.cr[6].lt {
	pc = 0x82536FE0; continue 'dispatch;
	}
	// 82536F34: 579D103A  slwi r29, r28, 2
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82536F38: 7D3D582E  lwzx r9, r29, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82536F3C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82536F40: 419A0090  beq cr6, 0x82536fd0
	if ctx.cr[6].eq {
	pc = 0x82536FD0; continue 'dispatch;
	}
	// 82536F44: 5524003E  slwi r4, r9, 0
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82536F48: 8124000C  lwz r9, 0xc(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536F4C: 71290083  andi. r9, r9, 0x83
	ctx.r[9].u64 = ctx.r[9].u64 & 131;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82536F50: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536F54: 4182007C  beq 0x82536fd0
	if ctx.cr[0].eq {
	pc = 0x82536FD0; continue 'dispatch;
	}
	// 82536F58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82536F5C: 4BFFFDAD  bl 0x82536d08
	ctx.lr = 0x82536F60;
	sub_82536D08(ctx, base);
	// 82536F60: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536F64: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82536F68: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82536F6C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82536F70: 716A0083  andi. r10, r11, 0x83
	ctx.r[10].u64 = ctx.r[11].u64 & 131;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82536F74: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82536F78: 4182004C  beq 0x82536fc4
	if ctx.cr[0].eq {
	pc = 0x82536FC4; continue 'dispatch;
	}
	// 82536F7C: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 82536F80: 409A0020  bne cr6, 0x82536fa0
	if !ctx.cr[6].eq {
	pc = 0x82536FA0; continue 'dispatch;
	}
	// 82536F84: 4BFFFEDD  bl 0x82536e60
	ctx.lr = 0x82536F88;
	sub_82536E60(ctx, base);
	// 82536F88: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82536F8C: 419A0038  beq cr6, 0x82536fc4
	if ctx.cr[6].eq {
	pc = 0x82536FC4; continue 'dispatch;
	}
	// 82536F90: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82536F94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82536F98: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82536F9C: 48000028  b 0x82536fc4
	pc = 0x82536FC4; continue 'dispatch;
	// 82536FA0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82536FA4: 409A0020  bne cr6, 0x82536fc4
	if !ctx.cr[6].eq {
	pc = 0x82536FC4; continue 'dispatch;
	}
	// 82536FA8: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82536FAC: 41820018  beq 0x82536fc4
	if ctx.cr[0].eq {
	pc = 0x82536FC4; continue 'dispatch;
	}
	// 82536FB0: 4BFFFEB1  bl 0x82536e60
	ctx.lr = 0x82536FB4;
	sub_82536E60(ctx, base);
	// 82536FB4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82536FB8: 409A000C  bne cr6, 0x82536fc4
	if !ctx.cr[6].eq {
	pc = 0x82536FC4; continue 'dispatch;
	}
	// 82536FBC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82536FC0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82536FC4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536FC8: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 82536FCC: 4800008D  bl 0x82537058
	ctx.lr = 0x82536FD0;
	sub_8253702C(ctx, base);
	// 82536FD0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536FD4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82536FD8: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82536FDC: 4BFFFF4C  b 0x82536f28
	pc = 0x82536F28; continue 'dispatch;
	// 82536FE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82536FE4: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 82536FE8: 48000021  bl 0x82537008
	ctx.lr = 0x82536FEC;
	sub_82537008(ctx, base);
	// 82536FEC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82536FF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82536FF4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82536FF8: 419A0008  beq cr6, 0x82537000
	if ctx.cr[6].eq {
	pc = 0x82537000; continue 'dispatch;
	}
	// 82536FFC: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82537000: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82537004: 4BFFE100  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537008 size=36
    let mut pc: u32 = 0x82537008;
    'dispatch: loop {
        match pc {
            0x82537008 => {
    //   block [0x82537008..0x8253702C)
	// 82537008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537014: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82537018: 480062A9  bl 0x8253d2c0
	ctx.lr = 0x8253701C;
	sub_8253D2C0(ctx, base);
	// 8253701C: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253702C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253702C size=148
    let mut pc: u32 = 0x8253702C;
    'dispatch: loop {
        match pc {
            0x8253702C => {
    //   block [0x8253702C..0x825370C0)
	// 8253702C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82537030: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82537034: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82537038: FB81FFE8  std r28, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[28].u64 ) };
	// 8253703C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537040: 9181FFE0  stw r12, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[12].u32 ) };
	// 82537044: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537048: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8253704C: 3BCB3DE4  addi r30, r11, 0x3de4
	ctx.r[30].s64 = ctx.r[11].s64 + 15844;
	// 82537050: 839F0050  lwz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82537054: 48000020  b 0x82537074
	pc = 0x82537074; continue 'dispatch;
	// 82537058: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8253705C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82537060: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82537064: FB81FFE8  std r28, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[28].u64 ) };
	// 82537068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253706C: 9181FFE0  stw r12, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[12].u32 ) };
	// 82537070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537074: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82537078: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8253707C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537080: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82537084: 4BFFFD1D  bl 0x82536da0
	ctx.lr = 0x82537088;
	sub_82536DA0(ctx, base);
	// 82537088: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8253708C: 3BCB3DE4  addi r30, r11, 0x3de4
	ctx.r[30].s64 = ctx.r[11].s64 + 15844;
	// 82537090: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82537094: 394B3DE8  addi r10, r11, 0x3de8
	ctx.r[10].s64 = ctx.r[11].s64 + 15848;
	// 82537098: 837F00A4  lwz r27, 0xa4(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8253709C: 839F0050  lwz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825370A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825370A4: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 825370A8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825370AC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825370B0: EB81FFE8  ld r28, -0x18(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825370B4: 8181FFE0  lwz r12, -0x20(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 825370B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825370BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825370C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825370C0 size=8
    let mut pc: u32 = 0x825370C0;
    'dispatch: loop {
        match pc {
            0x825370C0 => {
    //   block [0x825370C0..0x825370C8)
	// 825370C0: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 825370C4: 820DA3A8  lwz r16, -0x5c58(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23640 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825370C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825370C8 size=156
    let mut pc: u32 = 0x825370C8;
    'dispatch: loop {
        match pc {
            0x825370C8 => {
    //   block [0x825370C8..0x82537164)
	// 825370C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825370CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825370D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825370D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825370D8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 825370DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825370E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825370E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 825370E8: 4BFFFA41  bl 0x82536b28
	ctx.lr = 0x825370EC;
	sub_82536B28(ctx, base);
	// 825370EC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 825370F0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825370F4: 419A0054  beq cr6, 0x82537148
	if ctx.cr[6].eq {
	pc = 0x82537148; continue 'dispatch;
	}
	// 825370F8: 4BFFFA31  bl 0x82536b28
	ctx.lr = 0x825370FC;
	sub_82536B28(ctx, base);
	// 825370FC: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82537100: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82537104: 419A0044  beq cr6, 0x82537148
	if ctx.cr[6].eq {
	pc = 0x82537148; continue 'dispatch;
	}
	// 82537108: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8253710C: 409A0010  bne cr6, 0x8253711c
	if !ctx.cr[6].eq {
	pc = 0x8253711C; continue 'dispatch;
	}
	// 82537110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537114: 4BFFFDCD  bl 0x82536ee0
	ctx.lr = 0x82537118;
	sub_82536EE0(ctx, base);
	// 82537118: 48000034  b 0x8253714c
	pc = 0x8253714C; continue 'dispatch;
	// 8253711C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82537120: 4BFFFB79  bl 0x82536c98
	ctx.lr = 0x82537124;
	sub_82536C98(ctx, base);
	// 82537124: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82537128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253712C: 4BFFFD35  bl 0x82536e60
	ctx.lr = 0x82537130;
	sub_82536E60(ctx, base);
	// 82537130: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82537134: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82537138: 399F0070  addi r12, r31, 0x70
	ctx.r[12].s64 = ctx.r[31].s64 + 112;
	// 8253713C: 48000049  bl 0x82537184
	ctx.lr = 0x82537140;
	sub_82537164(ctx, base);
	// 82537140: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82537144: 48000008  b 0x8253714c
	pc = 0x8253714C; continue 'dispatch;
	// 82537148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253714C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82537150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537158: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8253715C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537164 size=88
    let mut pc: u32 = 0x82537164;
    'dispatch: loop {
        match pc {
            0x82537164 => {
    //   block [0x82537164..0x825371BC)
	// 82537164: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82537168: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8253716C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82537170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537174: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82537178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253717C: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82537180: 4800001C  b 0x8253719c
	pc = 0x8253719C; continue 'dispatch;
	// 82537184: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82537188: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8253718C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82537190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537194: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82537198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253719C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825371A0: 4BFFFBB9  bl 0x82536d58
	ctx.lr = 0x825371A4;
	sub_82536D58(ctx, base);
	// 825371A4: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 825371A8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825371AC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825371B0: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 825371B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825371B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825371C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825371C0 size=8
    let mut pc: u32 = 0x825371C0;
    'dispatch: loop {
        match pc {
            0x825371C0 => {
    //   block [0x825371C0..0x825371C8)
	// 825371C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825371C4: 4BFFFD1C  b 0x82536ee0
	sub_82536EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825371C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825371C8 size=48
    let mut pc: u32 = 0x825371C8;
    'dispatch: loop {
        match pc {
            0x825371C8 => {
    //   block [0x825371C8..0x825371F8)
	// 825371C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825371CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825371D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825371D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825371D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825371DC: 4BFFEAED  bl 0x82535cc8
	ctx.lr = 0x825371E0;
	sub_82535CC8(ctx, base);
	// 825371E0: 93E30090  stw r31, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 825371E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825371E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825371EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825371F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825371F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825371F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825371F8 size=36
    let mut pc: u32 = 0x825371F8;
    'dispatch: loop {
        match pc {
            0x825371F8 => {
    //   block [0x825371F8..0x8253721C)
	// 825371F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825371FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537204: 4BFFEAC5  bl 0x82535cc8
	ctx.lr = 0x82537208;
	sub_82535CC8(ctx, base);
	// 82537208: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8253720C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537220 size=12
    let mut pc: u32 = 0x82537220;
    'dispatch: loop {
        match pc {
            0x82537220 => {
    //   block [0x82537220..0x8253722C)
	// 82537220: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82537224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82537228: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253722C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253722C size=12
    let mut pc: u32 = 0x8253722C;
    'dispatch: loop {
        match pc {
            0x8253722C => {
    //   block [0x8253722C..0x82537238)
	// 8253722C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82537230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82537234: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537238 size=8
    let mut pc: u32 = 0x82537238;
    'dispatch: loop {
        match pc {
            0x82537238 => {
    //   block [0x82537238..0x82537240)
	// 82537238: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253723C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537240 size=8
    let mut pc: u32 = 0x82537240;
    'dispatch: loop {
        match pc {
            0x82537240 => {
    //   block [0x82537240..0x82537248)
	// 82537240: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 82537244: 4BFFE12C  b 0x82535370
	sub_82535370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537248 size=148
    let mut pc: u32 = 0x82537248;
    'dispatch: loop {
        match pc {
            0x82537248 => {
    //   block [0x82537248..0x825372DC)
	// 82537248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253724C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537258: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253725C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82537260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82537264: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537268: 48007019  bl 0x8253e280
	ctx.lr = 0x8253726C;
	sub_8253E280(ctx, base);
	// 8253726C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82537270: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82537274: 4182003C  beq 0x825372b0
	if ctx.cr[0].eq {
	pc = 0x825372B0; continue 'dispatch;
	}
	// 82537278: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253727C: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 82537280: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82537284: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 82537288: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253728C: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 82537290: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82537294: 40990010  ble cr6, 0x825372a4
	if !ctx.cr[6].gt {
	pc = 0x825372A4; continue 'dispatch;
	}
	// 82537298: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253729C: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 825372A0: 40990034  ble cr6, 0x825372d4
	if !ctx.cr[6].gt {
	pc = 0x825372D4; continue 'dispatch;
	}
	// 825372A4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825372A8: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 825372AC: 4082FFDC  bne 0x82537288
	if !ctx.cr[0].eq {
	pc = 0x82537288; continue 'dispatch;
	}
	// 825372B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825372B4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825372B8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825372BC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 825372C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825372C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825372C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825372CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825372D0: 4E800020  blr
	return;
	// 825372D4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 825372D8: 4BFFFFDC  b 0x825372b4
	pc = 0x825372B4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825372E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825372E0 size=196
    let mut pc: u32 = 0x825372E0;
    'dispatch: loop {
        match pc {
            0x825372E0 => {
    //   block [0x825372E0..0x825373A4)
	// 825372E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825372E4: 4BFFDDD9  bl 0x825350bc
	ctx.lr = 0x825372E8;
	sub_82535080(ctx, base);
	// 825372E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825372EC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825372F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825372F4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825372F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825372FC: 419A0018  beq cr6, 0x82537314
	if ctx.cr[6].eq {
	pc = 0x82537314; continue 'dispatch;
	}
	// 82537300: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82537304: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82537308: 4182000C  beq 0x82537314
	if ctx.cr[0].eq {
	pc = 0x82537314; continue 'dispatch;
	}
	// 8253730C: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82537310: 48000008  b 0x82537318
	pc = 0x82537318; continue 'dispatch;
	// 82537314: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82537318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253731C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537320: 48006F61  bl 0x8253e280
	ctx.lr = 0x82537324;
	sub_8253E280(ctx, base);
	// 82537324: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82537328: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253732C: 4182003C  beq 0x82537368
	if ctx.cr[0].eq {
	pc = 0x82537368; continue 'dispatch;
	}
	// 82537330: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82537334: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 82537338: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8253733C: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 82537340: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82537344: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 82537348: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8253734C: 40990010  ble cr6, 0x8253735c
	if !ctx.cr[6].gt {
	pc = 0x8253735C; continue 'dispatch;
	}
	// 82537350: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537354: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82537358: 40990024  ble cr6, 0x8253737c
	if !ctx.cr[6].gt {
	pc = 0x8253737C; continue 'dispatch;
	}
	// 8253735C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82537360: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 82537364: 4082FFDC  bne 0x82537340
	if !ctx.cr[0].eq {
	pc = 0x82537340; continue 'dispatch;
	}
	// 82537368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253736C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82537370: 409A0014  bne cr6, 0x82537384
	if !ctx.cr[6].eq {
	pc = 0x82537384; continue 'dispatch;
	}
	// 82537374: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82537378: 48000014  b 0x8253738c
	pc = 0x8253738C; continue 'dispatch;
	// 8253737C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82537380: 4BFFFFEC  b 0x8253736c
	pc = 0x8253736C; continue 'dispatch;
	// 82537384: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82537388: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 8253738C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82537390: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82537394: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82537398: 48007131  bl 0x8253e4c8
	ctx.lr = 0x8253739C;
	sub_8253E4C8(ctx, base);
	// 8253739C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825373A0: 4BFFDD6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825373A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825373A8 size=56
    let mut pc: u32 = 0x825373A8;
    'dispatch: loop {
        match pc {
            0x825373A8 => {
    //   block [0x825373A8..0x825373E0)
	// 825373A8: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 825373AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825373B0: 80EBFFFC  lwz r7, -4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 825373B4: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 825373B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825373BC: 419A0014  beq cr6, 0x825373d0
	if ctx.cr[6].eq {
	pc = 0x825373D0; continue 'dispatch;
	}
	// 825373C0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825373C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825373C8: 41820008  beq 0x825373d0
	if ctx.cr[0].eq {
	pc = 0x825373D0; continue 'dispatch;
	}
	// 825373CC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825373D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825373D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825373D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825373DC: 48007DC4  b 0x8253f1a0
	sub_8253F1A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825373E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825373E0 size=120
    let mut pc: u32 = 0x825373E0;
    'dispatch: loop {
        match pc {
            0x825373E0 => {
    //   block [0x825373E0..0x82537458)
	// 825373E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825373E4: 4BFFDCC9  bl 0x825350ac
	ctx.lr = 0x825373E8;
	sub_82535080(ctx, base);
	// 825373E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825373EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825373F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825373F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825373F8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825373FC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82537400: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82537404: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82537408: 4BFFE8C1  bl 0x82535cc8
	ctx.lr = 0x8253740C;
	sub_82535CC8(ctx, base);
	// 8253740C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82537410: 93C300B0  stw r30, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82537414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82537418: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253741C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82537420: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82537424: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82537428: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8253742C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82537430: 48007D71  bl 0x8253f1a0
	ctx.lr = 0x82537434;
	sub_8253F1A0(ctx, base);
	// 82537434: 4BFFE895  bl 0x82535cc8
	ctx.lr = 0x82537438;
	sub_82535CC8(ctx, base);
	// 82537438: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253743C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82537440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82537444: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82537448: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8253744C: 91390000  stw r9, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82537450: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82537454: 4BFFDCA8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537458 size=8
    let mut pc: u32 = 0x82537458;
    'dispatch: loop {
        match pc {
            0x82537458 => {
    //   block [0x82537458..0x82537460)
	// 82537458: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8253745C: 820DA3C0  lwz r16, -0x5c40(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537460 size=120
    let mut pc: u32 = 0x82537460;
    'dispatch: loop {
        match pc {
            0x82537460 => {
    //   block [0x82537460..0x825374D8)
	// 82537460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537464: 4BFFDC59  bl 0x825350bc
	ctx.lr = 0x82537468;
	sub_82535080(ctx, base);
	// 82537468: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8253746C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82537474: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82537478: 909F009C  stw r4, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 8253747C: 90BF00A4  stw r5, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82537480: 90DF00AC  stw r6, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[6].u32 ) };
	// 82537484: 90FF00B4  stw r7, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[7].u32 ) };
	// 82537488: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8253748C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82537490: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82537494: 90BF005C  stw r5, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82537498: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8253749C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825374A0: 4BFFE829  bl 0x82535cc8
	ctx.lr = 0x825374A4;
	sub_82535CC8(ctx, base);
	// 825374A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825374A8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825374AC: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 825374B0: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 825374B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825374B8: 4E800421  bctrl
	ctx.lr = 0x825374BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825374BC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825374C0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825374C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825374C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825374CC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825374D0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 825374D4: 4BFFDC38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825374D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825374D8 size=72
    let mut pc: u32 = 0x825374D8;
    'dispatch: loop {
        match pc {
            0x825374D8 => {
    //   block [0x825374D8..0x82537520)
	// 825374D8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825374DC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 825374E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825374E4: 9181FFF0  stw r12, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[12].u32 ) };
	// 825374E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825374EC: 393F0050  addi r9, r31, 0x50
	ctx.r[9].s64 = ctx.r[31].s64 + 80;
	// 825374F0: 811F00B4  lwz r8, 0xb4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 825374F4: 80FF00AC  lwz r7, 0xac(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 825374F8: 80DF00A4  lwz r6, 0xa4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 825374FC: 80BF009C  lwz r5, 0x9c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82537500: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82537504: 4BFFFEDD  bl 0x825373e0
	ctx.lr = 0x82537508;
	sub_825373E0(ctx, base);
	// 82537508: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8253750C: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537510: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82537514: 8181FFF0  lwz r12, -0x10(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82537518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537520 size=344
    let mut pc: u32 = 0x82537520;
    'dispatch: loop {
        match pc {
            0x82537520 => {
    //   block [0x82537520..0x82537678)
	// 82537520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537524: 4BFFDB85  bl 0x825350a8
	ctx.lr = 0x82537528;
	sub_82535080(ctx, base);
	// 82537528: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253752C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82537530: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537534: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82537538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253753C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82537540: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82537544: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82537548: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8253754C: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82537550: 48006D31  bl 0x8253e280
	ctx.lr = 0x82537554;
	sub_8253E280(ctx, base);
	// 82537554: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82537558: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8253755C: 409A0008  bne cr6, 0x82537564
	if !ctx.cr[6].eq {
	pc = 0x82537564; continue 'dispatch;
	}
	// 82537560: 48005F89  bl 0x8253d4e8
	ctx.lr = 0x82537564;
	sub_8253D4E8(ctx, base);
	// 82537564: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82537568: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8253756C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82537570: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82537574: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82537578: 419A0038  beq cr6, 0x825375b0
	if ctx.cr[6].eq {
	pc = 0x825375B0; continue 'dispatch;
	}
	// 8253757C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82537580: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 82537584: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82537588: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 8253758C: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82537590: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82537594: 40990010  ble cr6, 0x825375a4
	if !ctx.cr[6].gt {
	pc = 0x825375A4; continue 'dispatch;
	}
	// 82537598: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253759C: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825375A0: 40990010  ble cr6, 0x825375b0
	if !ctx.cr[6].gt {
	pc = 0x825375B0; continue 'dispatch;
	}
	// 825375A4: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825375A8: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 825375AC: 4082FFE0  bne 0x8253758c
	if !ctx.cr[0].eq {
	pc = 0x8253758C; continue 'dispatch;
	}
	// 825375B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825375B4: 419A0014  beq cr6, 0x825375c8
	if ctx.cr[6].eq {
	pc = 0x825375C8; continue 'dispatch;
	}
	// 825375B8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825375BC: 1D690014  mulli r11, r9, 0x14
	ctx.r[11].s64 = ctx.r[9].s64 * 20;
	// 825375C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825375C4: 3B8BFFEC  addi r28, r11, -0x14
	ctx.r[28].s64 = ctx.r[11].s64 + -20;
	// 825375C8: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 825375CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825375D0: 419A0084  beq cr6, 0x82537654
	if ctx.cr[6].eq {
	pc = 0x82537654; continue 'dispatch;
	}
	// 825375D4: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 825375D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825375DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 825375E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825375E4: 419A0024  beq cr6, 0x82537608
	if ctx.cr[6].eq {
	pc = 0x82537608; continue 'dispatch;
	}
	// 825375E8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825375EC: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825375F0: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 825375F4: 40990044  ble cr6, 0x82537638
	if !ctx.cr[6].gt {
	pc = 0x82537638; continue 'dispatch;
	}
	// 825375F8: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825375FC: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82537600: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82537604: 41990034  bgt cr6, 0x82537638
	if ctx.cr[6].gt {
	pc = 0x82537638; continue 'dispatch;
	}
	// 82537608: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253760C: 7F194000  cmpw cr6, r25, r8
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82537610: 41980028  blt cr6, 0x82537638
	if ctx.cr[6].lt {
	pc = 0x82537638; continue 'dispatch;
	}
	// 82537614: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82537618: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8253761C: 4199001C  bgt cr6, 0x82537638
	if ctx.cr[6].gt {
	pc = 0x82537638; continue 'dispatch;
	}
	// 82537620: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537624: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82537628: 409A0008  bne cr6, 0x82537630
	if !ctx.cr[6].eq {
	pc = 0x82537630; continue 'dispatch;
	}
	// 8253762C: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537630: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82537634: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82537638: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253763C: 39290014  addi r9, r9, 0x14
	ctx.r[9].s64 = ctx.r[9].s64 + 20;
	// 82537640: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82537644: 4198FF94  blt cr6, 0x825375d8
	if ctx.cr[6].lt {
	pc = 0x825375D8; continue 'dispatch;
	}
	// 82537648: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253764C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82537650: 409A0014  bne cr6, 0x82537664
	if !ctx.cr[6].eq {
	pc = 0x82537664; continue 'dispatch;
	}
	// 82537654: 931B0000  stw r24, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82537658: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253765C: 931A0000  stw r24, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82537660: 48000010  b 0x82537670
	pc = 0x82537670; continue 'dispatch;
	// 82537664: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82537668: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 * 20;
	// 8253766C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82537670: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82537674: 4BFFDA84  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537678 size=184
    let mut pc: u32 = 0x82537678;
    'dispatch: loop {
        match pc {
            0x82537678 => {
    //   block [0x82537678..0x82537730)
	// 82537678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253767C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537688: 3D40E06D  lis r10, -0x1f93
	ctx.r[10].s64 = -529727488;
	// 8253768C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82537694: 614A7363  ori r10, r10, 0x7363
	ctx.r[10].u64 = ctx.r[10].u64 | 29539;
	// 82537698: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8253769C: 409A0058  bne cr6, 0x825376f4
	if !ctx.cr[6].eq {
	pc = 0x825376F4; continue 'dispatch;
	}
	// 825376A0: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825376A4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 825376A8: 409A004C  bne cr6, 0x825376f4
	if !ctx.cr[6].eq {
	pc = 0x825376F4; continue 'dispatch;
	}
	// 825376AC: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 825376B0: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 825376B4: 614A0520  ori r10, r10, 0x520
	ctx.r[10].u64 = ctx.r[10].u64 | 1312;
	// 825376B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825376BC: 419A0024  beq cr6, 0x825376e0
	if ctx.cr[6].eq {
	pc = 0x825376E0; continue 'dispatch;
	}
	// 825376C0: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 825376C4: 614A0521  ori r10, r10, 0x521
	ctx.r[10].u64 = ctx.r[10].u64 | 1313;
	// 825376C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825376CC: 419A0014  beq cr6, 0x825376e0
	if ctx.cr[6].eq {
	pc = 0x825376E0; continue 'dispatch;
	}
	// 825376D0: 3D401993  lis r10, 0x1993
	ctx.r[10].s64 = 429064192;
	// 825376D4: 614A0522  ori r10, r10, 0x522
	ctx.r[10].u64 = ctx.r[10].u64 | 1314;
	// 825376D8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825376DC: 409A0018  bne cr6, 0x825376f4
	if !ctx.cr[6].eq {
	pc = 0x825376F4; continue 'dispatch;
	}
	// 825376E0: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 825376E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825376E8: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 825376EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825376F0: 4800000C  b 0x825376fc
	pc = 0x825376FC; continue 'dispatch;
	// 825376F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825376F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825376FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82537700: 4BFFE5C9  bl 0x82535cc8
	ctx.lr = 0x82537704;
	sub_82535CC8(ctx, base);
	// 82537704: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82537708: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8253770C: 4BFFE5BD  bl 0x82535cc8
	ctx.lr = 0x82537710;
	sub_82535CC8(ctx, base);
	// 82537710: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82537718: 93EB0094  stw r31, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 8253771C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253772C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537730 size=88
    let mut pc: u32 = 0x82537730;
    'dispatch: loop {
        match pc {
            0x82537730 => {
    //   block [0x82537730..0x82537788)
	// 82537730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253773C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82537744: 4BFFE585  bl 0x82535cc8
	ctx.lr = 0x82537748;
	sub_82535CC8(ctx, base);
	// 82537748: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8253774C: 48000014  b 0x82537760
	pc = 0x82537760; continue 'dispatch;
	// 82537750: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537754: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82537758: 419A0028  beq cr6, 0x82537780
	if ctx.cr[6].eq {
	pc = 0x82537780; continue 'dispatch;
	}
	// 8253775C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82537760: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82537764: 4082FFEC  bne 0x82537750
	if !ctx.cr[0].eq {
	pc = 0x82537750; continue 'dispatch;
	}
	// 82537768: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253776C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253777C: 4E800020  blr
	return;
	// 82537780: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537784: 4BFFFFE8  b 0x8253776c
	pc = 0x8253776C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537788 size=156
    let mut pc: u32 = 0x82537788;
    'dispatch: loop {
        match pc {
            0x82537788 => {
    //   block [0x82537788..0x82537824)
	// 82537788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253778C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253779C: 4BFFE52D  bl 0x82535cc8
	ctx.lr = 0x825377A0;
	sub_82535CC8(ctx, base);
	// 825377A0: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825377A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825377A8: 419A000C  beq cr6, 0x825377b4
	if ctx.cr[6].eq {
	pc = 0x825377B4; continue 'dispatch;
	}
	// 825377AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825377B0: 409A0008  bne cr6, 0x825377b8
	if !ctx.cr[6].eq {
	pc = 0x825377B8; continue 'dispatch;
	}
	// 825377B4: 48005D35  bl 0x8253d4e8
	ctx.lr = 0x825377B8;
	sub_8253D4E8(ctx, base);
	// 825377B8: 4BFFE511  bl 0x82535cc8
	ctx.lr = 0x825377BC;
	sub_82535CC8(ctx, base);
	// 825377BC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825377C0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825377C4: 409A0014  bne cr6, 0x825377d8
	if !ctx.cr[6].eq {
	pc = 0x825377D8; continue 'dispatch;
	}
	// 825377C8: 4BFFE501  bl 0x82535cc8
	ctx.lr = 0x825377CC;
	sub_82535CC8(ctx, base);
	// 825377CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825377D0: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 825377D4: 48000030  b 0x82537804
	pc = 0x82537804; continue 'dispatch;
	// 825377D8: 4BFFE4F1  bl 0x82535cc8
	ctx.lr = 0x825377DC;
	sub_82535CC8(ctx, base);
	// 825377DC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825377E0: 48000014  b 0x825377f4
	pc = 0x825377F4; continue 'dispatch;
	// 825377E4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825377E8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825377EC: 419A002C  beq cr6, 0x82537818
	if ctx.cr[6].eq {
	pc = 0x82537818; continue 'dispatch;
	}
	// 825377F0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825377F4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825377F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825377FC: 409AFFE8  bne cr6, 0x825377e4
	if !ctx.cr[6].eq {
	pc = 0x825377E4; continue 'dispatch;
	}
	// 82537800: 48005CE9  bl 0x8253d4e8
	ctx.lr = 0x82537804;
	sub_8253D4E8(ctx, base);
	// 82537804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253780C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537814: 4E800020  blr
	return;
	// 82537818: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253781C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82537820: 4BFFFFE4  b 0x82537804
	pc = 0x82537804; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537828 size=16
    let mut pc: u32 = 0x82537828;
    'dispatch: loop {
        match pc {
            0x82537828 => {
    //   block [0x82537828..0x82537838)
	// 82537828: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253782C: C80B2008  lfd f0, 0x2008(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 82537830: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82537834: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537838 size=164
    let mut pc: u32 = 0x82537838;
    'dispatch: loop {
        match pc {
            0x82537838 => {
    //   block [0x82537838..0x825378DC)
	// 82537838: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8253783C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82537840: 396B7018  addi r11, r11, 0x7018
	ctx.r[11].s64 = ctx.r[11].s64 + 28696;
	// 82537844: 394AEB60  addi r10, r10, -0x14a0
	ctx.r[10].s64 = ctx.r[10].s64 + -5280;
	// 82537848: C80B0000  lfd f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8253784C: FC010032  fmul f0, f1, f0
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 82537850: C98A0000  lfd f12, 0(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82537854: C96A0008  lfd f11, 8(r10)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82537858: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8253785C: C92B0050  lfd f9, 0x50(r11)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	// 82537860: C94A7088  lfd f10, 0x7088(r10)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28808 as u32) ) };
	// 82537864: FC00065C  fctid f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64 as i64 };
	// 82537868: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8253786C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82537870: FC0C0B7C  fnmsub f0, f12, f13, f1
	ctx.f[0].f64 = -(ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[1].f64);
	// 82537874: C98B0020  lfd f12, 0x20(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82537878: FDAB037C  fnmsub f13, f11, f13, f0
	ctx.f[13].f64 = -(ctx.f[11].f64 * ctx.f[13].f64 - ctx.f[0].f64);
	// 8253787C: FC0D0372  fmul f0, f13, f13
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[13].f64;
	// 82537880: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537884: FD6062BC  fnmsub f11, f0, f10, f12
	ctx.f[11].f64 = -(ctx.f[0].f64 * ctx.f[10].f64 - ctx.f[12].f64);
	// 82537888: 794A07E0  clrldi r10, r10, 0x3f
	ctx.r[10].u64 = ctx.r[10].u64 & 0x0000000000000001u64;
	// 8253788C: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 82537890: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82537894: C98A7080  lfd f12, 0x7080(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28800 as u32) ) };
	// 82537898: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8253789C: FD406278  fmsub f10, f0, f9, f12
	ctx.f[10].f64 = ctx.f[0].f64 * ctx.f[9].f64 - ctx.f[12].f64;
	// 825378A0: C98A7078  lfd f12, 0x7078(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28792 as u32) ) };
	// 825378A4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825378A8: FD6B6038  fmsub f11, f11, f0, f12
	ctx.f[11].f64 = ctx.f[11].f64 * ctx.f[0].f64 - ctx.f[12].f64;
	// 825378AC: C98B0040  lfd f12, 0x40(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 825378B0: FD4A603A  fmadd f10, f10, f0, f12
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 825378B4: C98B0030  lfd f12, 0x30(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 825378B8: FD2B603A  fmadd f9, f11, f0, f12
	ctx.f[9].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 825378BC: C96A7070  lfd f11, 0x7070(r10)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(28784 as u32) ) };
	// 825378C0: FD6A5838  fmsub f11, f10, f0, f11
	ctx.f[11].f64 = ctx.f[10].f64 * ctx.f[0].f64 - ctx.f[11].f64;
	// 825378C4: FDA90372  fmul f13, f9, f13
	ctx.f[13].f64 = ctx.f[9].f64 * ctx.f[13].f64;
	// 825378C8: FC0B603A  fmadd f0, f11, f0, f12
	ctx.f[0].f64 = ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64;
	// 825378CC: 419A0010  beq cr6, 0x825378dc
	if ctx.cr[6].eq {
		sub_825378DC(ctx, base);
		return;
	}
	// 825378D0: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 825378D4: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 825378D8: 48000008  b 0x825378e0
	sub_825378DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825378DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825378DC size=32
    let mut pc: u32 = 0x825378DC;
    'dispatch: loop {
        match pc {
            0x825378DC => {
    //   block [0x825378DC..0x825378FC)
	// 825378DC: FC0D0024  fdiv f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 825378E0: FD800A10  fabs f12, f1
	ctx.f[12].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 825378E4: C9AB0010  lfd f13, 0x10(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 825378E8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 825378EC: FD8C6828  fsub f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 - ctx.f[13].f64;
	// 825378F0: C9ABEE90  lfd f13, -0x1170(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4464 as u32) ) };
	// 825378F4: FC2C036E  fsel f1, f12, f13, f0
	ctx.f[1].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 825378F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537900 size=84
    let mut pc: u32 = 0x82537900;
    'dispatch: loop {
        match pc {
            0x82537900 => {
    //   block [0x82537900..0x82537954)
	// 82537900: 3923FFFF  addi r9, r3, -1
	ctx.r[9].s64 = ctx.r[3].s64 + -1;
	// 82537904: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82537908: 8CC40001  lbzu r6, 1(r4)
	ea = ctx.r[4].u32.wrapping_add(1 as u32);
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[4].u32 = ea;
	// 8253790C: 8CA90001  lbzu r5, 1(r9)
	ea = ctx.r[9].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[9].u32 = ea;
	// 82537910: 2F860000  cmpwi cr7, r6, 0
	ctx.cr[7].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82537914: 7C662851  subf. r3, r6, r5
	ctx.r[3].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82537918: 419E0038  beq cr7, 0x82537950
	if ctx.cr[7].eq {
	pc = 0x82537950; continue 'dispatch;
	}
	// 8253791C: 4182FFEC  beq 0x82537908
	if ctx.cr[0].eq {
	pc = 0x82537908; continue 'dispatch;
	}
	// 82537920: 2E860041  cmpwi cr5, r6, 0x41
	ctx.cr[5].compare_i32(ctx.r[6].s32, 65, &mut ctx.xer);
	// 82537924: 2F06005A  cmpwi cr6, r6, 0x5a
	ctx.cr[6].compare_i32(ctx.r[6].s32, 90, &mut ctx.xer);
	// 82537928: 4194000C  blt cr5, 0x82537934
	if ctx.cr[5].lt {
	pc = 0x82537934; continue 'dispatch;
	}
	// 8253792C: 41990008  bgt cr6, 0x82537934
	if ctx.cr[6].gt {
	pc = 0x82537934; continue 'dispatch;
	}
	// 82537930: 60C60020  ori r6, r6, 0x20
	ctx.r[6].u64 = ctx.r[6].u64 | 32;
	// 82537934: 2C050041  cmpwi r5, 0x41
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82537938: 2C85005A  cmpwi cr1, r5, 0x5a
	ctx.cr[1].compare_i32(ctx.r[5].s32, 90, &mut ctx.xer);
	// 8253793C: 4180000C  blt 0x82537948
	if ctx.cr[0].lt {
	pc = 0x82537948; continue 'dispatch;
	}
	// 82537940: 41850008  bgt cr1, 0x82537948
	if ctx.cr[1].gt {
	pc = 0x82537948; continue 'dispatch;
	}
	// 82537944: 60A50020  ori r5, r5, 0x20
	ctx.r[5].u64 = ctx.r[5].u64 | 32;
	// 82537948: 7C662851  subf. r3, r6, r5
	ctx.r[3].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253794C: 4182FFBC  beq 0x82537908
	if ctx.cr[0].eq {
	pc = 0x82537908; continue 'dispatch;
	}
	// 82537950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537958 size=284
    let mut pc: u32 = 0x82537958;
    'dispatch: loop {
        match pc {
            0x82537958 => {
    //   block [0x82537958..0x82537A74)
	// 82537958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253795C: 4BFFD761  bl 0x825350bc
	ctx.lr = 0x82537960;
	sub_82535080(ctx, base);
	// 82537960: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537964: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82537968: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8253796C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537970: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82537974: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82537978: 409A0034  bne cr6, 0x825379ac
	if !ctx.cr[6].eq {
	pc = 0x825379AC; continue 'dispatch;
	}
	// 8253797C: 48002FBD  bl 0x8253a938
	ctx.lr = 0x82537980;
	sub_8253A938(ctx, base);
	// 82537980: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537984: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537988: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253798C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537990: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537994: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537998: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253799C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825379A0: 48002E61  bl 0x8253a800
	ctx.lr = 0x825379A4;
	sub_8253A800(ctx, base);
	// 825379A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825379A8: 480000C4  b 0x82537a6c
	pc = 0x82537A6C; continue 'dispatch;
	// 825379AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825379B0: 419A000C  beq cr6, 0x825379bc
	if ctx.cr[6].eq {
	pc = 0x825379BC; continue 'dispatch;
	}
	// 825379B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825379B8: 419AFFC4  beq cr6, 0x8253797c
	if ctx.cr[6].eq {
	pc = 0x8253797C; continue 'dispatch;
	}
	// 825379BC: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 825379C0: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 825379C4: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825379C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825379CC: 41990008  bgt cr6, 0x825379d4
	if ctx.cr[6].gt {
	pc = 0x825379D4; continue 'dispatch;
	}
	// 825379D0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 825379D4: 39400042  li r10, 0x42
	ctx.r[10].s64 = 66;
	// 825379D8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825379DC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825379E0: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 825379E4: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 825379E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825379EC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 825379F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825379F4: 4E800421  bctrl
	ctx.lr = 0x825379F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825379F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825379FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82537A00: 409A000C  bne cr6, 0x82537a0c
	if !ctx.cr[6].eq {
	pc = 0x82537A0C; continue 'dispatch;
	}
	// 82537A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82537A08: 48000064  b 0x82537a6c
	pc = 0x82537A6C; continue 'dispatch;
	// 82537A0C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82537A10: 41980038  blt cr6, 0x82537a48
	if ctx.cr[6].lt {
	pc = 0x82537A48; continue 'dispatch;
	}
	// 82537A14: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82537A18: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82537A1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82537A20: 41800014  blt 0x82537a34
	if ctx.cr[0].lt {
	pc = 0x82537A34; continue 'dispatch;
	}
	// 82537A24: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82537A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82537A2C: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82537A30: 4BFFFFD4  b 0x82537a04
	pc = 0x82537A04; continue 'dispatch;
	// 82537A34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82537A38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537A3C: 4800206D  bl 0x82539aa8
	ctx.lr = 0x82537A40;
	sub_82539AA8(ctx, base);
	// 82537A40: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82537A44: 409AFFC0  bne cr6, 0x82537a04
	if !ctx.cr[6].eq {
	pc = 0x82537A04; continue 'dispatch;
	}
	// 82537A48: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82537A4C: 7D5FEA14  add r10, r31, r29
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82537A50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82537A54: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82537A58: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82537A5C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82537A60: 992AFFFF  stb r9, -1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 82537A64: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82537A68: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 82537A6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82537A70: 4BFFD69C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537A78 size=176
    let mut pc: u32 = 0x82537A78;
    'dispatch: loop {
        match pc {
            0x82537A78 => {
    //   block [0x82537A78..0x82537B28)
	// 82537A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537A88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82537A8C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82537A90: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82537A94: 409A0010  bne cr6, 0x82537aa4
	if !ctx.cr[6].eq {
	pc = 0x82537AA4; continue 'dispatch;
	}
	// 82537A98: 48002EA1  bl 0x8253a938
	ctx.lr = 0x82537A9C;
	sub_8253A938(ctx, base);
	// 82537A9C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537AA0: 48000050  b 0x82537af0
	pc = 0x82537AF0; continue 'dispatch;
	// 82537AA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82537AA8: 419AFFF0  beq cr6, 0x82537a98
	if ctx.cr[6].eq {
	pc = 0x82537A98; continue 'dispatch;
	}
	// 82537AAC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82537AB0: 419AFFE8  beq cr6, 0x82537a98
	if ctx.cr[6].eq {
	pc = 0x82537A98; continue 'dispatch;
	}
	// 82537AB4: 3D608254  lis r11, -0x7dac
	ctx.r[11].s64 = -2108424192;
	// 82537AB8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82537ABC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82537AC0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82537AC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82537AC8: 386BF4E8  addi r3, r11, -0xb18
	ctx.r[3].s64 = ctx.r[11].s64 + -2840;
	// 82537ACC: 4BFFFE8D  bl 0x82537958
	ctx.lr = 0x82537AD0;
	sub_82537958(ctx, base);
	// 82537AD0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82537AD4: 4080000C  bge 0x82537ae0
	if !ctx.cr[0].lt {
	pc = 0x82537AE0; continue 'dispatch;
	}
	// 82537AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82537ADC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82537AE0: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 82537AE4: 409A0030  bne cr6, 0x82537b14
	if !ctx.cr[6].eq {
	pc = 0x82537B14; continue 'dispatch;
	}
	// 82537AE8: 48002E51  bl 0x8253a938
	ctx.lr = 0x82537AEC;
	sub_8253A938(ctx, base);
	// 82537AEC: 39400022  li r10, 0x22
	ctx.r[10].s64 = 34;
	// 82537AF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537AF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537B00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537B04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537B08: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537B0C: 48002CF5  bl 0x8253a800
	ctx.lr = 0x82537B10;
	sub_8253A800(ctx, base);
	// 82537B10: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82537B14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537B20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537B28 size=12
    let mut pc: u32 = 0x82537B28;
    'dispatch: loop {
        match pc {
            0x82537B28 => {
    //   block [0x82537B28..0x82537B34)
	// 82537B28: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82537B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537B30: 4BFFFF48  b 0x82537a78
	sub_82537A78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537B38 size=200
    let mut pc: u32 = 0x82537B38;
    'dispatch: loop {
        match pc {
            0x82537B38 => {
    //   block [0x82537B38..0x82537C00)
	// 82537B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537B44: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82537B48: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82537B4C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82537B50: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82537B54: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82537B58: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82537B5C: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537B60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82537B64: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82537B68: 409AFFF4  bne cr6, 0x82537b5c
	if !ctx.cr[6].eq {
	pc = 0x82537B5C; continue 'dispatch;
	}
	// 82537B6C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82537B70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82537B74: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82537B78: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82537B7C: 409A0034  bne cr6, 0x82537bb0
	if !ctx.cr[6].eq {
	pc = 0x82537BB0; continue 'dispatch;
	}
	// 82537B80: 48002DB9  bl 0x8253a938
	ctx.lr = 0x82537B84;
	sub_8253A938(ctx, base);
	// 82537B84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537B88: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537B90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537B94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537B98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537B9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537BA0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537BA4: 48002C5D  bl 0x8253a800
	ctx.lr = 0x82537BA8;
	sub_8253A800(ctx, base);
	// 82537BA8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82537BAC: 48000044  b 0x82537bf0
	pc = 0x82537BF0; continue 'dispatch;
	// 82537BB0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82537BB4: 419AFFCC  beq cr6, 0x82537b80
	if ctx.cr[6].eq {
	pc = 0x82537B80; continue 'dispatch;
	}
	// 82537BB8: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82537BBC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82537BC0: 38C00049  li r6, 0x49
	ctx.r[6].s64 = 73;
	// 82537BC4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82537BC8: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82537BCC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82537BD0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82537BD4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82537BD8: 41990008  bgt cr6, 0x82537be0
	if ctx.cr[6].gt {
	pc = 0x82537BE0; continue 'dispatch;
	}
	// 82537BDC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82537BE0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82537BE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82537BE8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82537BEC: 4E800421  bctrl
	ctx.lr = 0x82537BF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82537BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82537BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537C00 size=92
    let mut pc: u32 = 0x82537C00;
    'dispatch: loop {
        match pc {
            0x82537C00 => {
    //   block [0x82537C00..0x82537C5C)
	// 82537C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537C08: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82537C0C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82537C10: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82537C14: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82537C18: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82537C1C: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82537C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537C24: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82537C28: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82537C2C: 3D608254  lis r11, -0x7dac
	ctx.r[11].s64 = -2108424192;
	// 82537C30: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82537C34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82537C38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537C3C: 386B0118  addi r3, r11, 0x118
	ctx.r[3].s64 = ctx.r[11].s64 + 280;
	// 82537C40: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82537C44: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82537C48: 4BFFFEF1  bl 0x82537b38
	ctx.lr = 0x82537C4C;
	sub_82537B38(ctx, base);
	// 82537C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537C60 size=88
    let mut pc: u32 = 0x82537C60;
    'dispatch: loop {
        match pc {
            0x82537C60 => {
    //   block [0x82537C60..0x82537CB8)
	// 82537C60: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82537C64: 419A0054  beq cr6, 0x82537cb8
	if ctx.cr[6].eq {
		sub_82537CB8(ctx, base);
		return;
	}
	// 82537C68: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537C6C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82537C70: 396AFFBF  addi r11, r10, -0x41
	ctx.r[11].s64 = ctx.r[10].s64 + -65;
	// 82537C74: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82537C78: 41990008  bgt cr6, 0x82537c80
	if ctx.cr[6].gt {
	pc = 0x82537C80; continue 'dispatch;
	}
	// 82537C7C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82537C80: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537C84: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82537C88: 392BFFBF  addi r9, r11, -0x41
	ctx.r[9].s64 = ctx.r[11].s64 + -65;
	// 82537C8C: 2B090019  cmplwi cr6, r9, 0x19
	ctx.cr[6].compare_u32(ctx.r[9].u32, 25 as u32, &mut ctx.xer);
	// 82537C90: 41990008  bgt cr6, 0x82537c98
	if ctx.cr[6].gt {
	pc = 0x82537C98; continue 'dispatch;
	}
	// 82537C94: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82537C98: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82537C9C: 41820014  beq 0x82537cb0
	if ctx.cr[0].eq {
	pc = 0x82537CB0; continue 'dispatch;
	}
	// 82537CA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82537CA4: 419A000C  beq cr6, 0x82537cb0
	if ctx.cr[6].eq {
	pc = 0x82537CB0; continue 'dispatch;
	}
	// 82537CA8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82537CAC: 419AFFBC  beq cr6, 0x82537c68
	if ctx.cr[6].eq {
	pc = 0x82537C68; continue 'dispatch;
	}
	// 82537CB0: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82537CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537CB8 size=8
    let mut pc: u32 = 0x82537CB8;
    'dispatch: loop {
        match pc {
            0x82537CB8 => {
    //   block [0x82537CB8..0x82537CC0)
	// 82537CB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537CC0 size=172
    let mut pc: u32 = 0x82537CC0;
    'dispatch: loop {
        match pc {
            0x82537CC0 => {
    //   block [0x82537CC0..0x82537D6C)
	// 82537CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537CCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537CD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82537CD4: 409A0038  bne cr6, 0x82537d0c
	if !ctx.cr[6].eq {
	pc = 0x82537D0C; continue 'dispatch;
	}
	// 82537CD8: 48002C61  bl 0x8253a938
	ctx.lr = 0x82537CDC;
	sub_8253A938(ctx, base);
	// 82537CDC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537CE0: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537CE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537CF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537CF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537CF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537CFC: 48002B05  bl 0x8253a800
	ctx.lr = 0x82537D00;
	sub_8253A800(ctx, base);
	// 82537D00: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 82537D04: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 82537D08: 48000050  b 0x82537d58
	pc = 0x82537D58; continue 'dispatch;
	// 82537D0C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82537D10: 419AFFC8  beq cr6, 0x82537cd8
	if ctx.cr[6].eq {
	pc = 0x82537CD8; continue 'dispatch;
	}
	// 82537D14: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82537D18: 617FFFFF  ori r31, r11, 0xffff
	ctx.r[31].u64 = ctx.r[11].u64 | 65535;
	// 82537D1C: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82537D20: 40990034  ble cr6, 0x82537d54
	if !ctx.cr[6].gt {
	pc = 0x82537D54; continue 'dispatch;
	}
	// 82537D24: 48002C15  bl 0x8253a938
	ctx.lr = 0x82537D28;
	sub_8253A938(ctx, base);
	// 82537D28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537D2C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537D30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537D38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537D3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537D40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537D44: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537D48: 48002AB9  bl 0x8253a800
	ctx.lr = 0x82537D4C;
	sub_8253A800(ctx, base);
	// 82537D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82537D50: 48000008  b 0x82537d58
	pc = 0x82537D58; continue 'dispatch;
	// 82537D54: 4BFFFF0D  bl 0x82537c60
	ctx.lr = 0x82537D58;
	sub_82537C60(ctx, base);
	// 82537D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537D64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537D70 size=8
    let mut pc: u32 = 0x82537D70;
    'dispatch: loop {
        match pc {
            0x82537D70 => {
    //   block [0x82537D70..0x82537D78)
	// 82537D70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82537D74: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537D78 size=12
    let mut pc: u32 = 0x82537D78;
    'dispatch: loop {
        match pc {
            0x82537D78 => {
    //   block [0x82537D78..0x82537D84)
	// 82537D78: 546B003A  rlwinm r11, r3, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82537D7C: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82537D80: 4BFFBFB8  b 0x82533d38
	sub_82533D38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537D84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537D84 size=4
    let mut pc: u32 = 0x82537D84;
    'dispatch: loop {
        match pc {
            0x82537D84 => {
    //   block [0x82537D84..0x82537D88)
	// 82537D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537D88 size=152
    let mut pc: u32 = 0x82537D88;
    'dispatch: loop {
        match pc {
            0x82537D88 => {
    //   block [0x82537D88..0x82537E20)
	// 82537D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537D98: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82537D9C: 7D6B2039  and. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82537DA0: 41820034  beq 0x82537dd4
	if ctx.cr[0].eq {
	pc = 0x82537DD4; continue 'dispatch;
	}
	// 82537DA4: 48002B95  bl 0x8253a938
	ctx.lr = 0x82537DA8;
	sub_8253A938(ctx, base);
	// 82537DA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537DAC: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537DB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537DB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537DBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537DC4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537DC8: 48002A39  bl 0x8253a800
	ctx.lr = 0x82537DCC;
	sub_8253A800(ctx, base);
	// 82537DCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82537DD0: 48000038  b 0x82537e08
	pc = 0x82537E08; continue 'dispatch;
	// 82537DD4: 2B040004  cmplwi cr6, r4, 4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4 as u32, &mut ctx.xer);
	// 82537DD8: 41990008  bgt cr6, 0x82537de0
	if ctx.cr[6].gt {
	pc = 0x82537DE0; continue 'dispatch;
	}
	// 82537DDC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82537DE0: 3BE4FFFF  addi r31, r4, -1
	ctx.r[31].s64 = ctx.r[4].s64 + -1;
	// 82537DE4: 7D7F1A14  add r11, r31, r3
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 82537DE8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82537DEC: 4BFFBE85  bl 0x82533c70
	ctx.lr = 0x82537DF0;
	sub_82533C70(ctx, base);
	// 82537DF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82537DF4: 4182FFD8  beq 0x82537dcc
	if ctx.cr[0].eq {
	pc = 0x82537DCC; continue 'dispatch;
	}
	// 82537DF8: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82537DFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82537E00: 7D6BF878  andc r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[31].u64;
	// 82537E04: 906BFFFC  stw r3, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[3].u32 ) };
	// 82537E08: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82537E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82537E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537E18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537E20 size=244
    let mut pc: u32 = 0x82537E20;
    'dispatch: loop {
        match pc {
            0x82537E20 => {
    //   block [0x82537E20..0x82537F14)
	// 82537E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82537E28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82537E2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82537E30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537E34: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82537E38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82537E3C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82537E40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82537E44: 409A0034  bne cr6, 0x82537e78
	if !ctx.cr[6].eq {
	pc = 0x82537E78; continue 'dispatch;
	}
	// 82537E48: 48002AF1  bl 0x8253a938
	ctx.lr = 0x82537E4C;
	sub_8253A938(ctx, base);
	// 82537E4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537E50: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537E58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537E60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537E64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537E68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537E6C: 48002995  bl 0x8253a800
	ctx.lr = 0x82537E70;
	sub_8253A800(ctx, base);
	// 82537E70: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82537E74: 48000088  b 0x82537efc
	pc = 0x82537EFC; continue 'dispatch;
	// 82537E78: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82537E7C: 419A000C  beq cr6, 0x82537e88
	if ctx.cr[6].eq {
	pc = 0x82537E88; continue 'dispatch;
	}
	// 82537E80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82537E84: 419AFFC4  beq cr6, 0x82537e48
	if ctx.cr[6].eq {
	pc = 0x82537E48; continue 'dispatch;
	}
	// 82537E88: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82537E8C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82537E90: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82537E94: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82537E98: 41990008  bgt cr6, 0x82537ea0
	if ctx.cr[6].gt {
	pc = 0x82537EA0; continue 'dispatch;
	}
	// 82537E9C: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82537EA0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82537EA4: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82537EA8: 39600042  li r11, 0x42
	ctx.r[11].s64 = 66;
	// 82537EAC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82537EB0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82537EB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82537EB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82537EBC: 48001DBD  bl 0x82539c78
	ctx.lr = 0x82537EC0;
	sub_82539C78(ctx, base);
	// 82537EC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82537EC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82537EC8: 419A0030  beq cr6, 0x82537ef8
	if ctx.cr[6].eq {
	pc = 0x82537EF8; continue 'dispatch;
	}
	// 82537ECC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82537ED0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82537ED4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82537ED8: 41800014  blt 0x82537eec
	if ctx.cr[0].lt {
	pc = 0x82537EEC; continue 'dispatch;
	}
	// 82537EDC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82537EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82537EE4: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82537EE8: 48000010  b 0x82537ef8
	pc = 0x82537EF8; continue 'dispatch;
	// 82537EEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82537EF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537EF4: 48001BB5  bl 0x82539aa8
	ctx.lr = 0x82537EF8;
	sub_82539AA8(ctx, base);
	// 82537EF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82537EFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82537F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82537F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82537F08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82537F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82537F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82537F18 size=12
    let mut pc: u32 = 0x82537F18;
    'dispatch: loop {
        match pc {
            0x82537F18 => {
    //   block [0x82537F18..0x82537F24)
	// 82537F18: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82537F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537F20: 4BFFFF00  b 0x82537e20
	sub_82537E20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82537F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82537F28 size=416
    let mut pc: u32 = 0x82537F28;
    'dispatch: loop {
        match pc {
            0x82537F28 => {
    //   block [0x82537F28..0x825380C8)
	// 82537F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82537F2C: 4BFFD18D  bl 0x825350b8
	ctx.lr = 0x82537F30;
	sub_82535080(ctx, base);
	// 82537F30: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82537F34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82537F38: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82537F3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82537F40: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82537F44: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82537F48: 419A0008  beq cr6, 0x82537f50
	if ctx.cr[6].eq {
	pc = 0x82537F50; continue 'dispatch;
	}
	// 82537F4C: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82537F50: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82537F54: 409A0038  bne cr6, 0x82537f8c
	if !ctx.cr[6].eq {
	pc = 0x82537F8C; continue 'dispatch;
	}
	// 82537F58: 480029E1  bl 0x8253a938
	ctx.lr = 0x82537F5C;
	sub_8253A938(ctx, base);
	// 82537F5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82537F60: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82537F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82537F68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82537F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82537F70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82537F74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82537F78: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82537F7C: 48002885  bl 0x8253a800
	ctx.lr = 0x82537F80;
	sub_8253A800(ctx, base);
	// 82537F80: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82537F84: C82B2008  lfd f1, 0x2008(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 82537F88: 48000134  b 0x825380bc
	pc = 0x825380BC; continue 'dispatch;
	// 82537F8C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82537F90: 3BCBEE80  addi r30, r11, -0x1180
	ctx.r[30].s64 = ctx.r[11].s64 + -4480;
	// 82537F94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537F98: 814B00AC  lwz r10, 0xac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82537F9C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82537FA0: 4099001C  ble cr6, 0x82537fbc
	if !ctx.cr[6].gt {
	pc = 0x82537FBC; continue 'dispatch;
	}
	// 82537FA4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82537FA8: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537FAC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82537FB0: 48002A69  bl 0x8253aa18
	ctx.lr = 0x82537FB4;
	sub_8253AA18(ctx, base);
	// 82537FB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537FB8: 48000018  b 0x82537fd0
	pc = 0x82537FD0; continue 'dispatch;
	// 82537FBC: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537FC0: 812B00C8  lwz r9, 0xc8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82537FC4: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82537FC8: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82537FCC: 55430738  rlwinm r3, r10, 0, 0x1c, 0x1c
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82537FD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82537FD4: 419A000C  beq cr6, 0x82537fe0
	if ctx.cr[6].eq {
	pc = 0x82537FE0; continue 'dispatch;
	}
	// 82537FD8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82537FDC: 4BFFFFBC  b 0x82537f98
	pc = 0x82537F98; continue 'dispatch;
	// 82537FE0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82537FE4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82537FE8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82537FEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82537FF0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82537FF4: 409AFFF4  bne cr6, 0x82537fe8
	if !ctx.cr[6].eq {
	pc = 0x82537FE8; continue 'dispatch;
	}
	// 82537FF8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82537FFC: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82538000: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82538004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538008: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8253800C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82538014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82538018: 480038F1  bl 0x8253b908
	ctx.lr = 0x8253801C;
	sub_8253B908(ctx, base);
	// 8253801C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82538020: 419A0010  beq cr6, 0x82538030
	if ctx.cr[6].eq {
	pc = 0x82538030; continue 'dispatch;
	}
	// 82538024: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538028: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8253802C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82538030: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538034: 716A0240  andi. r10, r11, 0x240
	ctx.r[10].u64 = ctx.r[11].u64 & 576;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82538038: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253803C: 4182001C  beq 0x82538058
	if ctx.cr[0].eq {
	pc = 0x82538058; continue 'dispatch;
	}
	// 82538040: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82538044: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82538048: CBEB2008  lfd f31, 0x2008(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253804C: 419A006C  beq cr6, 0x825380b8
	if ctx.cr[6].eq {
	pc = 0x825380B8; continue 'dispatch;
	}
	// 82538050: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82538054: 48000064  b 0x825380b8
	pc = 0x825380B8; continue 'dispatch;
	// 82538058: 716A0081  andi. r10, r11, 0x81
	ctx.r[10].u64 = ctx.r[11].u64 & 129;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253805C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82538060: 41820028  beq 0x82538088
	if ctx.cr[0].eq {
	pc = 0x82538088; continue 'dispatch;
	}
	// 82538064: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538068: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 8253806C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82538070: 409A0010  bne cr6, 0x82538080
	if !ctx.cr[6].eq {
	pc = 0x82538080; continue 'dispatch;
	}
	// 82538074: C80B7980  lfd f0, 0x7980(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(31104 as u32) ) };
	// 82538078: FFE00050  fneg f31, f0
	ctx.f[31].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8253807C: 48000028  b 0x825380a4
	pc = 0x825380A4; continue 'dispatch;
	// 82538080: CBEB7980  lfd f31, 0x7980(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(31104 as u32) ) };
	// 82538084: 48000020  b 0x825380a4
	pc = 0x825380A4; continue 'dispatch;
	// 82538088: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253808C: 41820028  beq 0x825380b4
	if ctx.cr[0].eq {
	pc = 0x825380B4; continue 'dispatch;
	}
	// 82538090: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82538094: C8030010  lfd f0, 0x10(r3)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82538098: CBEB2008  lfd f31, 0x2008(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253809C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 825380A0: 409A0014  bne cr6, 0x825380b4
	if !ctx.cr[6].eq {
	pc = 0x825380B4; continue 'dispatch;
	}
	// 825380A4: 48002895  bl 0x8253a938
	ctx.lr = 0x825380A8;
	sub_8253A938(ctx, base);
	// 825380A8: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 825380AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825380B0: 48000008  b 0x825380b8
	pc = 0x825380B8; continue 'dispatch;
	// 825380B4: CBE30010  lfd f31, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 825380B8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825380BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825380C0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 825380C4: 4BFFD044  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825380C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825380C8 size=8
    let mut pc: u32 = 0x825380C8;
    'dispatch: loop {
        match pc {
            0x825380C8 => {
    //   block [0x825380C8..0x825380D0)
	// 825380C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825380CC: 4BFFFE5C  b 0x82537f28
	sub_82537F28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825380D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825380D0 size=68
    let mut pc: u32 = 0x825380D0;
    'dispatch: loop {
        match pc {
            0x825380D0 => {
    //   block [0x825380D0..0x82538114)
	// 825380D0: 88A30000  lbz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825380D4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 825380D8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825380DC: 7C052000  cmpw r5, r4
	ctx.cr[0].compare_i32(ctx.r[4].s32, ctx.r[0].s32, &mut ctx.xer);
	// 825380E0: 419A0034  beq cr6, 0x82538114
	if ctx.cr[6].eq {
		sub_82538114(ctx, base);
		return;
	}
	// 825380E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825380E8: 41820018  beq 0x82538100
	if ctx.cr[0].eq {
	pc = 0x82538100; continue 'dispatch;
	}
	// 825380EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825380F0: 419A0034  beq cr6, 0x82538124
	if ctx.cr[6].eq {
		sub_82538124(ctx, base);
		return;
	}
	// 825380F4: 8CA90001  lbzu r5, 1(r9)
	ea = ctx.r[9].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[9].u32 = ea;
	// 825380F8: 7C042800  cmpw r4, r5
	ctx.cr[0].compare_i32(ctx.r[5].s32, ctx.r[0].s32, &mut ctx.xer);
	// 825380FC: 4082FFF0  bne 0x825380ec
	if !ctx.cr[0].eq {
	pc = 0x825380EC; continue 'dispatch;
	}
	// 82538100: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82538104: 8CA90001  lbzu r5, 1(r9)
	ea = ctx.r[9].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[9].u32 = ea;
	// 82538108: 7C042800  cmpw r4, r5
	ctx.cr[0].compare_i32(ctx.r[5].s32, ctx.r[0].s32, &mut ctx.xer);
	// 8253810C: 4182FFF4  beq 0x82538100
	if ctx.cr[0].eq {
	pc = 0x82538100; continue 'dispatch;
	}
	// 82538110: 4BFFFFDC  b 0x825380ec
	pc = 0x825380EC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538114 size=16
    let mut pc: u32 = 0x82538114;
    'dispatch: loop {
        match pc {
            0x82538114 => {
    //   block [0x82538114..0x82538124)
	// 82538114: 41820010  beq 0x82538124
	if ctx.cr[0].eq {
		sub_82538124(ctx, base);
		return;
	}
	// 82538118: 8CA30001  lbzu r5, 1(r3)
	ea = ctx.r[3].u32.wrapping_add(1 as u32);
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ea) } as u64;
	ctx.r[3].u32 = ea;
	// 8253811C: 2C050000  cmpwi r5, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538120: 4BFFFFF4  b 0x82538114
	pc = 0x82538114; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82538124 size=4
    let mut pc: u32 = 0x82538124;
    'dispatch: loop {
        match pc {
            0x82538124 => {
    //   block [0x82538124..0x82538128)
	// 82538124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538128 size=660
    let mut pc: u32 = 0x82538128;
    'dispatch: loop {
        match pc {
            0x82538128 => {
    //   block [0x82538128..0x825383BC)
	// 82538128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253812C: 4BFFCF6D  bl 0x82535098
	ctx.lr = 0x82538130;
	sub_82535080(ctx, base);
	// 82538130: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538134: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82538138: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8253813C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82538140: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82538144: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82538148: 7E9CA378  mr r28, r20
	ctx.r[28].u64 = ctx.r[20].u64;
	// 8253814C: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 82538150: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82538154: 419A003C  beq cr6, 0x82538190
	if ctx.cr[6].eq {
	pc = 0x82538190; continue 'dispatch;
	}
	// 82538158: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 8253815C: 419A0034  beq cr6, 0x82538190
	if ctx.cr[6].eq {
	pc = 0x82538190; continue 'dispatch;
	}
	// 82538160: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 82538164: 409A0038  bne cr6, 0x8253819c
	if !ctx.cr[6].eq {
	pc = 0x8253819C; continue 'dispatch;
	}
	// 82538168: 480027D1  bl 0x8253a938
	ctx.lr = 0x8253816C;
	sub_8253A938(ctx, base);
	// 8253816C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538170: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538178: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253817C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82538180: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538184: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538188: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253818C: 48002675  bl 0x8253a800
	ctx.lr = 0x82538190;
	sub_8253A800(ctx, base);
	// 82538190: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538194: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82538198: 4BFFCF50  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
	// 8253819C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 825381A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825381A4: 419A0014  beq cr6, 0x825381b8
	if ctx.cr[6].eq {
	pc = 0x825381B8; continue 'dispatch;
	}
	// 825381A8: 7D7FC396  divwu r11, r31, r24
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[24].u32;
	// 825381AC: 0CD80000  twi 6, r24, 0
	// 825381B0: 7F165840  cmplw cr6, r22, r11
	ctx.cr[6].compare_u32(ctx.r[22].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825381B4: 40990034  ble cr6, 0x825381e8
	if !ctx.cr[6].gt {
	pc = 0x825381E8; continue 'dispatch;
	}
	// 825381B8: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 825381BC: 419A0014  beq cr6, 0x825381d0
	if ctx.cr[6].eq {
	pc = 0x825381D0; continue 'dispatch;
	}
	// 825381C0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 825381C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825381C8: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 825381CC: 4BFFD005  bl 0x825351d0
	ctx.lr = 0x825381D0;
	sub_825351D0(ctx, base);
	// 825381D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825381D4: 419AFF94  beq cr6, 0x82538168
	if ctx.cr[6].eq {
	pc = 0x82538168; continue 'dispatch;
	}
	// 825381D8: 7D7FC396  divwu r11, r31, r24
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[24].u32;
	// 825381DC: 0CD80000  twi 6, r24, 0
	// 825381E0: 7F165840  cmplw cr6, r22, r11
	ctx.cr[6].compare_u32(ctx.r[22].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825381E4: 4199FF84  bgt cr6, 0x82538168
	if ctx.cr[6].gt {
	pc = 0x82538168; continue 'dispatch;
	}
	// 825381E8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825381EC: 7EF8B1D6  mullw r23, r24, r22
	ctx.r[23].s64 = (ctx.r[24].s32 as i64) * (ctx.r[22].s32 as i64);
	// 825381F0: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 825381F4: 716B010C  andi. r11, r11, 0x10c
	ctx.r[11].u64 = ctx.r[11].u64 & 268;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825381F8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825381FC: 4182000C  beq 0x82538208
	if ctx.cr[0].eq {
	pc = 0x82538208; continue 'dispatch;
	}
	// 82538200: 835D0018  lwz r26, 0x18(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82538204: 48000008  b 0x8253820c
	pc = 0x8253820C; continue 'dispatch;
	// 82538208: 3B401000  li r26, 0x1000
	ctx.r[26].s64 = 4096;
	// 8253820C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82538210: 419A0148  beq cr6, 0x82538358
	if ctx.cr[6].eq {
	pc = 0x82538358; continue 'dispatch;
	}
	// 82538214: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82538218: 6179FFFF  ori r25, r11, 0xffff
	ctx.r[25].u64 = ctx.r[11].u64 | 65535;
	// 8253821C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538220: 716B010C  andi. r11, r11, 0x10c
	ctx.r[11].u64 = ctx.r[11].u64 & 268;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538224: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538228: 41820068  beq 0x82538290
	if ctx.cr[0].eq {
	pc = 0x82538290; continue 'dispatch;
	}
	// 8253822C: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538230: 2C1E0000  cmpwi r30, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538234: 4182005C  beq 0x82538290
	if ctx.cr[0].eq {
	pc = 0x82538290; continue 'dispatch;
	}
	// 82538238: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8253823C: 41980148  blt cr6, 0x82538384
	if ctx.cr[6].lt {
	pc = 0x82538384; continue 'dispatch;
	}
	// 82538240: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82538244: 40980008  bge cr6, 0x8253824c
	if !ctx.cr[6].lt {
	pc = 0x8253824C; continue 'dispatch;
	}
	// 82538248: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8253824C: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82538250: 41990110  bgt cr6, 0x82538360
	if ctx.cr[6].gt {
	pc = 0x82538360; continue 'dispatch;
	}
	// 82538254: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82538258: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253825C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82538260: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82538264: 4BFFC525  bl 0x82534788
	ctx.lr = 0x82538268;
	sub_82534788(ctx, base);
	// 82538268: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253826C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538270: 7FFEF850  subf r31, r30, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82538274: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82538278: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8253827C: 7F9EE214  add r28, r30, r28
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82538280: 7F7ED850  subf r27, r30, r27
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 82538284: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82538288: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253828C: 480000C4  b 0x82538350
	pc = 0x82538350; continue 'dispatch;
	// 82538290: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82538294: 41980090  blt cr6, 0x82538324
	if ctx.cr[6].lt {
	pc = 0x82538324; continue 'dispatch;
	}
	// 82538298: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8253829C: 419A003C  beq cr6, 0x825382d8
	if ctx.cr[6].eq {
	pc = 0x825382D8; continue 'dispatch;
	}
	// 825382A0: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 825382A4: 0CDA0000  twi 6, r26, 0
	// 825382A8: 4099001C  ble cr6, 0x825382c4
	if !ctx.cr[6].gt {
	pc = 0x825382C4; continue 'dispatch;
	}
	// 825382AC: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 825382B0: 7D4BD396  divwu r10, r11, r26
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[26].u32;
	// 825382B4: 7D4AD1D6  mullw r10, r10, r26
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[26].s32 as i64);
	// 825382B8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 825382BC: 7FCBC850  subf r30, r11, r25
	ctx.r[30].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	// 825382C0: 48000028  b 0x825382e8
	pc = 0x825382E8; continue 'dispatch;
	// 825382C4: 7D7FD396  divwu r11, r31, r26
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[26].u32;
	// 825382C8: 7D6BD1D6  mullw r11, r11, r26
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[26].s32 as i64);
	// 825382CC: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 825382D0: 7FCBF850  subf r30, r11, r31
	ctx.r[30].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 825382D4: 48000014  b 0x825382e8
	pc = 0x825382E8; continue 'dispatch;
	// 825382D8: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 825382DC: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 825382E0: 41990008  bgt cr6, 0x825382e8
	if ctx.cr[6].gt {
	pc = 0x825382E8; continue 'dispatch;
	}
	// 825382E4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 825382E8: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825382EC: 41990074  bgt cr6, 0x82538360
	if ctx.cr[6].gt {
	pc = 0x82538360; continue 'dispatch;
	}
	// 825382F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825382F4: 480058CD  bl 0x8253dbc0
	ctx.lr = 0x825382F8;
	sub_8253DBC0(ctx, base);
	// 825382F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825382FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82538300: 48009839  bl 0x82541b38
	ctx.lr = 0x82538304;
	sub_82541B38(ctx, base);
	// 82538304: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82538308: 41820098  beq 0x825383a0
	if ctx.cr[0].eq {
	pc = 0x825383A0; continue 'dispatch;
	}
	// 8253830C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82538310: 419A0074  beq cr6, 0x82538384
	if ctx.cr[6].eq {
	pc = 0x82538384; continue 'dispatch;
	}
	// 82538314: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82538318: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 8253831C: 7F63D850  subf r27, r3, r27
	ctx.r[27].s64 = ctx.r[27].s64 - ctx.r[3].s64;
	// 82538320: 48000030  b 0x82538350
	pc = 0x82538350; continue 'dispatch;
	// 82538324: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82538328: 48009019  bl 0x82541340
	ctx.lr = 0x8253832C;
	sub_82541340(ctx, base);
	// 8253832C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82538330: 419A007C  beq cr6, 0x825383ac
	if ctx.cr[6].eq {
	pc = 0x825383AC; continue 'dispatch;
	}
	// 82538334: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82538338: 419A0028  beq cr6, 0x82538360
	if ctx.cr[6].eq {
	pc = 0x82538360; continue 'dispatch;
	}
	// 8253833C: 987C0000  stb r3, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 82538340: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82538344: 835D0018  lwz r26, 0x18(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82538348: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 8253834C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82538350: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82538354: 409AFEC8  bne cr6, 0x8253821c
	if !ctx.cr[6].eq {
	pc = 0x8253821C; continue 'dispatch;
	}
	// 82538358: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8253835C: 4BFFFE38  b 0x82538194
	pc = 0x82538194; continue 'dispatch;
	// 82538360: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 82538364: 419A0014  beq cr6, 0x82538378
	if ctx.cr[6].eq {
	pc = 0x82538378; continue 'dispatch;
	}
	// 82538368: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 8253836C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538370: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82538374: 4BFFCE5D  bl 0x825351d0
	ctx.lr = 0x82538378;
	sub_825351D0(ctx, base);
	// 82538378: 480025C1  bl 0x8253a938
	ctx.lr = 0x8253837C;
	sub_8253A938(ctx, base);
	// 8253837C: 39400022  li r10, 0x22
	ctx.r[10].s64 = 34;
	// 82538380: 4BFFFDF0  b 0x82538170
	pc = 0x82538170; continue 'dispatch;
	// 82538384: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538388: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8253838C: 7D5FB850  subf r10, r31, r23
	ctx.r[10].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 82538390: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82538394: 0CD80000  twi 6, r24, 0
	// 82538398: 7C6AC396  divwu r3, r10, r24
	ctx.r[3].u32 = ctx.r[10].u32 / ctx.r[24].u32;
	// 8253839C: 4BFFFDF8  b 0x82538194
	pc = 0x82538194; continue 'dispatch;
	// 825383A0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825383A4: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 825383A8: 4BFFFFE4  b 0x8253838c
	pc = 0x8253838C; continue 'dispatch;
	// 825383AC: 7D7FB850  subf r11, r31, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 825383B0: 0CD80000  twi 6, r24, 0
	// 825383B4: 7C6BC396  divwu r3, r11, r24
	ctx.r[3].u32 = ctx.r[11].u32 / ctx.r[24].u32;
	// 825383B8: 4BFFFDDC  b 0x82538194
	pc = 0x82538194; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825383C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825383C0 size=8
    let mut pc: u32 = 0x825383C0;
    'dispatch: loop {
        match pc {
            0x825383C0 => {
    //   block [0x825383C0..0x825383C8)
	// 825383C0: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 825383C4: 820DA3D8  lwz r16, -0x5c28(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825383C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825383C8 size=204
    let mut pc: u32 = 0x825383C8;
    'dispatch: loop {
        match pc {
            0x825383C8 => {
    //   block [0x825383C8..0x82538494)
	// 825383C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825383CC: 4BFFCCE5  bl 0x825350b0
	ctx.lr = 0x825383D0;
	sub_82535080(ctx, base);
	// 825383D0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 825383D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825383D8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825383DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825383E0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825383E4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825383E8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 825383EC: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 825383F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825383F4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825383F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825383FC: 419A0050  beq cr6, 0x8253844c
	if ctx.cr[6].eq {
	pc = 0x8253844C; continue 'dispatch;
	}
	// 82538400: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82538404: 419A0048  beq cr6, 0x8253844c
	if ctx.cr[6].eq {
	pc = 0x8253844C; continue 'dispatch;
	}
	// 82538408: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8253840C: 409A004C  bne cr6, 0x82538458
	if !ctx.cr[6].eq {
	pc = 0x82538458; continue 'dispatch;
	}
	// 82538410: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82538414: 419A0010  beq cr6, 0x82538424
	if ctx.cr[6].eq {
	pc = 0x82538424; continue 'dispatch;
	}
	// 82538418: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8253841C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538420: 4BFFCDB1  bl 0x825351d0
	ctx.lr = 0x82538424;
	sub_825351D0(ctx, base);
	// 82538424: 48002515  bl 0x8253a938
	ctx.lr = 0x82538428;
	sub_8253A938(ctx, base);
	// 82538428: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253842C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538430: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538438: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253843C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538440: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538444: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538448: 480023B9  bl 0x8253a800
	ctx.lr = 0x8253844C;
	sub_8253A800(ctx, base);
	// 8253844C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538450: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82538454: 4BFFCCAC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82538458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253845C: 4BFFE83D  bl 0x82536c98
	ctx.lr = 0x82538460;
	sub_82536C98(ctx, base);
	// 82538460: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538464: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82538468: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8253846C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82538470: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82538474: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82538478: 4BFFFCB1  bl 0x82538128
	ctx.lr = 0x8253847C;
	sub_82538128(ctx, base);
	// 8253847C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82538480: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538484: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 82538488: 4800002D  bl 0x825384b4
	ctx.lr = 0x8253848C;
	sub_82538494(ctx, base);
	// 8253848C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82538490: 4BFFFFC0  b 0x82538450
	pc = 0x82538450; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538494 size=88
    let mut pc: u32 = 0x82538494;
    'dispatch: loop {
        match pc {
            0x82538494 => {
    //   block [0x82538494..0x825384EC)
	// 82538494: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82538498: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8253849C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825384A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825384A4: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825384A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825384AC: 83DF00C4  lwz r30, 0xc4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 825384B0: 4800001C  b 0x825384cc
	pc = 0x825384CC; continue 'dispatch;
	// 825384B4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825384B8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 825384BC: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825384C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825384C4: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825384C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825384CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825384D0: 4BFFE889  bl 0x82536d58
	ctx.lr = 0x825384D4;
	sub_82536D58(ctx, base);
	// 825384D4: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 825384D8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825384DC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825384E0: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 825384E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825384E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825384F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825384F0 size=20
    let mut pc: u32 = 0x825384F0;
    'dispatch: loop {
        match pc {
            0x825384F0 => {
    //   block [0x825384F0..0x82538504)
	// 825384F0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 825384F4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 825384F8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825384FC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82538500: 4BFFFEC8  b 0x825383c8
	sub_825383C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538508 size=204
    let mut pc: u32 = 0x82538508;
    'dispatch: loop {
        match pc {
            0x82538508 => {
    //   block [0x82538508..0x825385D4)
	// 82538508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253850C: 4BFFCBB1  bl 0x825350bc
	ctx.lr = 0x82538510;
	sub_82535080(ctx, base);
	// 82538510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82538518: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8253851C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82538520: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538524: 716A0083  andi. r10, r11, 0x83
	ctx.r[10].u64 = ctx.r[11].u64 & 131;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82538528: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253852C: 4082001C  bne 0x82538548
	if !ctx.cr[0].eq {
	pc = 0x82538548; continue 'dispatch;
	}
	// 82538530: 48002409  bl 0x8253a938
	ctx.lr = 0x82538534;
	sub_8253A938(ctx, base);
	// 82538534: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538538: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253853C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82538540: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538544: 48000088  b 0x825385cc
	pc = 0x825385CC; continue 'dispatch;
	// 82538548: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253854C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82538550: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82538554: 409A0014  bne cr6, 0x82538568
	if !ctx.cr[6].eq {
	pc = 0x82538568; continue 'dispatch;
	}
	// 82538558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253855C: 48000195  bl 0x825386f0
	ctx.lr = 0x82538560;
	sub_825386F0(ctx, base);
	// 82538560: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82538564: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82538568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253856C: 4BFFE85D  bl 0x82536dc8
	ctx.lr = 0x82538570;
	sub_82536DC8(ctx, base);
	// 82538570: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538574: 556A0631  rlwinm. r10, r11, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82538578: 41820010  beq 0x82538588
	if ctx.cr[0].eq {
	pc = 0x82538588; continue 'dispatch;
	}
	// 8253857C: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82538580: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82538584: 48000024  b 0x825385a8
	pc = 0x825385A8; continue 'dispatch;
	// 82538588: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253858C: 4182001C  beq 0x825385a8
	if ctx.cr[0].eq {
	pc = 0x825385A8; continue 'dispatch;
	}
	// 82538590: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82538594: 41820014  beq 0x825385a8
	if ctx.cr[0].eq {
	pc = 0x825385A8; continue 'dispatch;
	}
	// 82538598: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253859C: 4082000C  bne 0x825385a8
	if !ctx.cr[0].eq {
	pc = 0x825385A8; continue 'dispatch;
	}
	// 825385A0: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 825385A4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825385A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825385AC: 48005615  bl 0x8253dbc0
	ctx.lr = 0x825385B0;
	sub_8253DBC0(ctx, base);
	// 825385B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825385B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825385B8: 480097F9  bl 0x82541db0
	ctx.lr = 0x825385BC;
	sub_82541DB0(ctx, base);
	// 825385BC: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 825385C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825385C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825385C8: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 825385CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825385D0: 4BFFCB3C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825385D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825385D8 size=8
    let mut pc: u32 = 0x825385D8;
    'dispatch: loop {
        match pc {
            0x825385D8 => {
    //   block [0x825385D8..0x825385E0)
	// 825385D8: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 825385DC: 820DA3F0  lwz r16, -0x5c10(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825385E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825385E0 size=180
    let mut pc: u32 = 0x825385E0;
    'dispatch: loop {
        match pc {
            0x825385E0 => {
    //   block [0x825385E0..0x82538694)
	// 825385E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825385E4: 4BFFCAD5  bl 0x825350b8
	ctx.lr = 0x825385E8;
	sub_82535080(ctx, base);
	// 825385E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 825385EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825385F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825385F4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 825385F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825385FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82538600: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82538604: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82538608: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8253860C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538610: 40820034  bne 0x82538644
	if !ctx.cr[0].eq {
	pc = 0x82538644; continue 'dispatch;
	}
	// 82538614: 48002325  bl 0x8253a938
	ctx.lr = 0x82538618;
	sub_8253A938(ctx, base);
	// 82538618: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253861C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538620: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538628: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253862C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538630: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538634: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538638: 480021C9  bl 0x8253a800
	ctx.lr = 0x8253863C;
	sub_8253A800(ctx, base);
	// 8253863C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82538640: 4800004C  b 0x8253868c
	pc = 0x8253868C; continue 'dispatch;
	// 82538644: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82538648: 419A0014  beq cr6, 0x8253865c
	if ctx.cr[6].eq {
	pc = 0x8253865C; continue 'dispatch;
	}
	// 8253864C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82538650: 419A000C  beq cr6, 0x8253865c
	if ctx.cr[6].eq {
	pc = 0x8253865C; continue 'dispatch;
	}
	// 82538654: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82538658: 409AFFBC  bne cr6, 0x82538614
	if !ctx.cr[6].eq {
	pc = 0x82538614; continue 'dispatch;
	}
	// 8253865C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538660: 4BFFE639  bl 0x82536c98
	ctx.lr = 0x82538664;
	sub_82536C98(ctx, base);
	// 82538664: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538668: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8253866C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82538670: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538674: 4BFFFE95  bl 0x82538508
	ctx.lr = 0x82538678;
	sub_82538508(ctx, base);
	// 82538678: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8253867C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538680: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 82538684: 48000031  bl 0x825386b4
	ctx.lr = 0x82538688;
	sub_82538694(ctx, base);
	// 82538688: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253868C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82538690: 4BFFCA78  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538694 size=88
    let mut pc: u32 = 0x82538694;
    'dispatch: loop {
        match pc {
            0x82538694 => {
    //   block [0x82538694..0x825386EC)
	// 82538694: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82538698: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8253869C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825386A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825386A4: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825386A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825386AC: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 825386B0: 4800001C  b 0x825386cc
	pc = 0x825386CC; continue 'dispatch;
	// 825386B4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825386B8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 825386BC: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825386C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825386C4: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825386C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825386CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825386D0: 4BFFE689  bl 0x82536d58
	ctx.lr = 0x825386D4;
	sub_82536D58(ctx, base);
	// 825386D4: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 825386D8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825386DC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825386E0: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 825386E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825386E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825386F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825386F0 size=552
    let mut pc: u32 = 0x825386F0;
    'dispatch: loop {
        match pc {
            0x825386F0 => {
    //   block [0x825386F0..0x82538918)
	// 825386F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825386F4: 4BFFC9B5  bl 0x825350a8
	ctx.lr = 0x825386F8;
	sub_82535080(ctx, base);
	// 825386F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825386FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82538700: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82538704: 409A0034  bne cr6, 0x82538738
	if !ctx.cr[6].eq {
	pc = 0x82538738; continue 'dispatch;
	}
	// 82538708: 48002231  bl 0x8253a938
	ctx.lr = 0x8253870C;
	sub_8253A938(ctx, base);
	// 8253870C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538710: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538718: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253871C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82538720: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538728: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253872C: 480020D5  bl 0x8253a800
	ctx.lr = 0x82538730;
	sub_8253A800(ctx, base);
	// 82538730: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82538734: 480001DC  b 0x82538910
	pc = 0x82538910; continue 'dispatch;
	// 82538738: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8253873C: 48005485  bl 0x8253dbc0
	ctx.lr = 0x82538740;
	sub_8253DBC0(ctx, base);
	// 82538740: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538744: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82538748: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253874C: 4098000C  bge cr6, 0x82538758
	if !ctx.cr[6].lt {
	pc = 0x82538758; continue 'dispatch;
	}
	// 82538750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82538754: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82538758: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8253875C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538760: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82538764: 4800964D  bl 0x82541db0
	ctx.lr = 0x82538768;
	sub_82541DB0(ctx, base);
	// 82538768: 7C781B79  or. r24, r3, r3
	ctx.r[24].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8253876C: 4180FFC4  blt 0x82538730
	if ctx.cr[0].lt {
	pc = 0x82538730; continue 'dispatch;
	}
	// 82538770: 80FB000C  lwz r7, 0xc(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538774: 70EB0108  andi. r11, r7, 0x108
	ctx.r[11].u64 = ctx.r[7].u64 & 264;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538778: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253877C: 40820010  bne 0x8253878c
	if !ctx.cr[0].eq {
	pc = 0x8253878C; continue 'dispatch;
	}
	// 82538780: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538784: 7C6BC050  subf r3, r11, r24
	ctx.r[3].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 82538788: 48000188  b 0x82538910
	pc = 0x82538910; continue 'dispatch;
	// 8253878C: 54EB07BF  clrlwi. r11, r7, 0x1e
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538790: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538794: 811B0008  lwz r8, 8(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82538798: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8253879C: 7F284850  subf r25, r8, r9
	ctx.r[25].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 825387A0: 3B4B3CE0  addi r26, r11, 0x3ce0
	ctx.r[26].s64 = ctx.r[11].s64 + 15584;
	// 825387A4: 41820060  beq 0x82538804
	if ctx.cr[0].eq {
	pc = 0x82538804; continue 'dispatch;
	}
	// 825387A8: 7F8B2E70  srawi r11, r28, 5
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 5) as i64;
	// 825387AC: 578A3572  rlwinm r10, r28, 6, 0x15, 0x19
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x03FFFFFFu64;
	// 825387B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825387B4: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825387B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825387BC: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825387C0: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825387C4: 41820030  beq 0x825387f4
	if ctx.cr[0].eq {
	pc = 0x825387F4; continue 'dispatch;
	}
	// 825387C8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 825387CC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825387D0: 40980024  bge cr6, 0x825387f4
	if !ctx.cr[6].lt {
	pc = 0x825387F4; continue 'dispatch;
	}
	// 825387D4: 552A003E  slwi r10, r9, 0
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825387D8: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825387DC: 2B06000A  cmplwi cr6, r6, 0xa
	ctx.cr[6].compare_u32(ctx.r[6].u32, 10 as u32, &mut ctx.xer);
	// 825387E0: 409A0008  bne cr6, 0x825387e8
	if !ctx.cr[6].eq {
	pc = 0x825387E8; continue 'dispatch;
	}
	// 825387E4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 825387E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825387EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825387F0: 4198FFE8  blt cr6, 0x825387d8
	if ctx.cr[6].lt {
	pc = 0x825387D8; continue 'dispatch;
	}
	// 825387F4: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 825387F8: 409A002C  bne cr6, 0x82538824
	if !ctx.cr[6].eq {
	pc = 0x82538824; continue 'dispatch;
	}
	// 825387FC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82538800: 48000110  b 0x82538910
	pc = 0x82538910; continue 'dispatch;
	// 82538804: 54EB0631  rlwinm. r11, r7, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538808: 4082FFEC  bne 0x825387f4
	if !ctx.cr[0].eq {
	pc = 0x825387F4; continue 'dispatch;
	}
	// 8253880C: 4800212D  bl 0x8253a938
	ctx.lr = 0x82538810;
	sub_8253A938(ctx, base);
	// 82538810: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538814: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538818: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8253881C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538820: 480000F0  b 0x82538910
	pc = 0x82538910; continue 'dispatch;
	// 82538824: 54EB07FF  clrlwi. r11, r7, 0x1f
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538828: 418200E4  beq 0x8253890c
	if ctx.cr[0].eq {
	pc = 0x8253890C; continue 'dispatch;
	}
	// 8253882C: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538830: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538834: 4082000C  bne 0x82538840
	if !ctx.cr[0].eq {
	pc = 0x82538840; continue 'dispatch;
	}
	// 82538838: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8253883C: 480000D0  b 0x8253890c
	pc = 0x8253890C; continue 'dispatch;
	// 82538840: 7F8B2E70  srawi r11, r28, 5
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 5) as i64;
	// 82538844: 579E3572  rlwinm r30, r28, 6, 0x15, 0x19
	ctx.r[30].u64 = ctx.r[28].u32 as u64 & 0x03FFFFFFu64;
	// 82538848: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8253884C: 7D684850  subf r11, r8, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82538850: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82538854: 7D7DD02E  lwzx r11, r29, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82538858: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8253885C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538860: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538864: 418200A4  beq 0x82538908
	if ctx.cr[0].eq {
	pc = 0x82538908; continue 'dispatch;
	}
	// 82538868: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8253886C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538870: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82538874: 4800953D  bl 0x82541db0
	ctx.lr = 0x82538878;
	sub_82541DB0(ctx, base);
	// 82538878: 7F03C000  cmpw cr6, r3, r24
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[24].s32, &mut ctx.xer);
	// 8253887C: 409A0038  bne cr6, 0x825388b4
	if !ctx.cr[6].eq {
	pc = 0x825388B4; continue 'dispatch;
	}
	// 82538880: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82538884: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82538888: 48000018  b 0x825388a0
	pc = 0x825388A0; continue 'dispatch;
	// 8253888C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538890: 2B09000A  cmplwi cr6, r9, 0xa
	ctx.cr[6].compare_u32(ctx.r[9].u32, 10 as u32, &mut ctx.xer);
	// 82538894: 409A0008  bne cr6, 0x8253889c
	if !ctx.cr[6].eq {
	pc = 0x8253889C; continue 'dispatch;
	}
	// 82538898: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253889C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825388A0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825388A4: 4198FFE8  blt cr6, 0x8253888c
	if ctx.cr[6].lt {
	pc = 0x8253888C; continue 'dispatch;
	}
	// 825388A8: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 825388AC: 556B04A5  rlwinm. r11, r11, 0, 0x12, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825388B0: 48000050  b 0x82538900
	pc = 0x82538900; continue 'dispatch;
	// 825388B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825388B8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825388BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825388C0: 480094F1  bl 0x82541db0
	ctx.lr = 0x825388C4;
	sub_82541DB0(ctx, base);
	// 825388C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 825388C8: 4180FE68  blt 0x82538730
	if ctx.cr[0].lt {
	pc = 0x82538730; continue 'dispatch;
	}
	// 825388CC: 2B1F0200  cmplwi cr6, r31, 0x200
	ctx.cr[6].compare_u32(ctx.r[31].u32, 512 as u32, &mut ctx.xer);
	// 825388D0: 4199001C  bgt cr6, 0x825388ec
	if ctx.cr[6].gt {
	pc = 0x825388EC; continue 'dispatch;
	}
	// 825388D4: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 825388D8: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825388DC: 41820010  beq 0x825388ec
	if ctx.cr[0].eq {
	pc = 0x825388EC; continue 'dispatch;
	}
	// 825388E0: 556B056B  rlwinm. r11, r11, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825388E4: 3BE00200  li r31, 0x200
	ctx.r[31].s64 = 512;
	// 825388E8: 41820008  beq 0x825388f0
	if ctx.cr[0].eq {
	pc = 0x825388F0; continue 'dispatch;
	}
	// 825388EC: 83FB0018  lwz r31, 0x18(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 825388F0: 7D7DD02E  lwzx r11, r29, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825388F4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825388F8: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825388FC: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538900: 41820008  beq 0x82538908
	if ctx.cr[0].eq {
	pc = 0x82538908; continue 'dispatch;
	}
	// 82538904: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82538908: 7F1FC050  subf r24, r31, r24
	ctx.r[24].s64 = ctx.r[24].s64 - ctx.r[31].s64;
	// 8253890C: 7C79C214  add r3, r25, r24
	ctx.r[3].u64 = ctx.r[25].u64 + ctx.r[24].u64;
	// 82538910: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82538914: 4BFFC7E4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82538918 size=8
    let mut pc: u32 = 0x82538918;
    'dispatch: loop {
        match pc {
            0x82538918 => {
    //   block [0x82538918..0x82538920)
	// 82538918: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 8253891C: 820DA408  lwz r16, -0x5bf8(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538920 size=164
    let mut pc: u32 = 0x82538920;
    'dispatch: loop {
        match pc {
            0x82538920 => {
    //   block [0x82538920..0x825389C4)
	// 82538920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82538928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8253892C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82538930: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82538934: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538938: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8253893C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82538940: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82538944: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82538948: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8253894C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538950: 40820034  bne 0x82538984
	if !ctx.cr[0].eq {
	pc = 0x82538984; continue 'dispatch;
	}
	// 82538954: 48001FE5  bl 0x8253a938
	ctx.lr = 0x82538958;
	sub_8253A938(ctx, base);
	// 82538958: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253895C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538960: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538968: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253896C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538974: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538978: 48001E89  bl 0x8253a800
	ctx.lr = 0x8253897C;
	sub_8253A800(ctx, base);
	// 8253897C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82538980: 4800002C  b 0x825389ac
	pc = 0x825389AC; continue 'dispatch;
	// 82538984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538988: 4BFFE311  bl 0x82536c98
	ctx.lr = 0x8253898C;
	sub_82536C98(ctx, base);
	// 8253898C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538990: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538994: 4BFFFD5D  bl 0x825386f0
	ctx.lr = 0x82538998;
	sub_825386F0(ctx, base);
	// 82538998: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8253899C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825389A0: 399F0070  addi r12, r31, 0x70
	ctx.r[12].s64 = ctx.r[31].s64 + 112;
	// 825389A4: 48000041  bl 0x825389e4
	ctx.lr = 0x825389A8;
	sub_825389C4(ctx, base);
	// 825389A8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825389AC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 825389B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825389B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825389B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825389BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825389C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825389C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825389C4 size=88
    let mut pc: u32 = 0x825389C4;
    'dispatch: loop {
        match pc {
            0x825389C4 => {
    //   block [0x825389C4..0x82538A1C)
	// 825389C4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825389C8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 825389CC: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825389D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825389D4: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825389D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825389DC: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 825389E0: 4800001C  b 0x825389fc
	pc = 0x825389FC; continue 'dispatch;
	// 825389E4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825389E8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 825389EC: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825389F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825389F4: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825389F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825389FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538A00: 4BFFE359  bl 0x82536d58
	ctx.lr = 0x82538A04;
	sub_82536D58(ctx, base);
	// 82538A04: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538A08: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82538A0C: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82538A10: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82538A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82538A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82538A20 size=8
    let mut pc: u32 = 0x82538A20;
    'dispatch: loop {
        match pc {
            0x82538A20 => {
    //   block [0x82538A20..0x82538A28)
	// 82538A20: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 82538A24: 820DA420  lwz r16, -0x5be0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538A28 size=312
    let mut pc: u32 = 0x82538A28;
    'dispatch: loop {
        match pc {
            0x82538A28 => {
    //   block [0x82538A28..0x82538B60)
	// 82538A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538A2C: 4BFFC685  bl 0x825350b0
	ctx.lr = 0x82538A30;
	sub_82535080(ctx, base);
	// 82538A30: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82538A34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538A38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82538A3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82538A40: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82538A44: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82538A48: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82538A4C: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82538A50: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82538A54: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82538A58: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538A5C: 40820034  bne 0x82538a90
	if !ctx.cr[0].eq {
	pc = 0x82538A90; continue 'dispatch;
	}
	// 82538A60: 48001ED9  bl 0x8253a938
	ctx.lr = 0x82538A64;
	sub_8253A938(ctx, base);
	// 82538A64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538A68: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538A70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82538A78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538A7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538A80: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538A84: 48001D7D  bl 0x8253a800
	ctx.lr = 0x82538A88;
	sub_8253A800(ctx, base);
	// 82538A88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538A8C: 480000CC  b 0x82538b58
	pc = 0x82538B58; continue 'dispatch;
	// 82538A90: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82538A94: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82538A98: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82538A9C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538AA0: 4182FFC0  beq 0x82538a60
	if ctx.cr[0].eq {
	pc = 0x82538A60; continue 'dispatch;
	}
	// 82538AA4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538AA8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82538AAC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82538AB0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82538AB4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82538AB8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538ABC: 4182FFA4  beq 0x82538a60
	if ctx.cr[0].eq {
	pc = 0x82538A60; continue 'dispatch;
	}
	// 82538AC0: 480096E1  bl 0x825421a0
	ctx.lr = 0x82538AC4;
	sub_825421A0(ctx, base);
	// 82538AC4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82538AC8: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82538ACC: 4082001C  bne 0x82538ae8
	if !ctx.cr[0].eq {
	pc = 0x82538AE8; continue 'dispatch;
	}
	// 82538AD0: 48001E69  bl 0x8253a938
	ctx.lr = 0x82538AD4;
	sub_8253A938(ctx, base);
	// 82538AD4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538AD8: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82538ADC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538AE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538AE4: 48000074  b 0x82538b58
	pc = 0x82538B58; continue 'dispatch;
	// 82538AE8: 60000000  nop
	// 82538AEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538AF0: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538AF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82538AF8: 409A0030  bne cr6, 0x82538b28
	if !ctx.cr[6].eq {
	pc = 0x82538B28; continue 'dispatch;
	}
	// 82538AFC: 48001E3D  bl 0x8253a938
	ctx.lr = 0x82538B00;
	sub_8253A938(ctx, base);
	// 82538B00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538B04: 39200016  li r9, 0x16
	ctx.r[9].s64 = 22;
	// 82538B08: 3D408254  lis r10, -0x7dac
	ctx.r[10].s64 = -2108424192;
	// 82538B0C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82538B10: 388A8B54  addi r4, r10, -0x74ac
	ctx.r[4].s64 = ctx.r[10].s64 + -29868;
	// 82538B14: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82538B18: 935F0058  stw r26, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82538B1C: 4BFFD465  bl 0x82535f80
	ctx.lr = 0x82538B20;
	sub_82535F80(ctx, base);
	// 82538B20: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538B24: 48000030  b 0x82538b54
	pc = 0x82538B54; continue 'dispatch;
	// 82538B28: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82538B2C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82538B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82538B34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82538B38: 48009409  bl 0x82541f40
	ctx.lr = 0x82538B3C;
	sub_82541F40(ctx, base);
	// 82538B3C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82538B40: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538B44: 399F00A0  addi r12, r31, 0xa0
	ctx.r[12].s64 = ctx.r[31].s64 + 160;
	// 82538B48: 48000039  bl 0x82538b80
	ctx.lr = 0x82538B4C;
	sub_82538B60(ctx, base);
	// 82538B4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82538B50: 48000008  b 0x82538b58
	pc = 0x82538B58; continue 'dispatch;
	// 82538B54: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82538B58: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82538B5C: 4BFFC5A4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538B60 size=88
    let mut pc: u32 = 0x82538B60;
    'dispatch: loop {
        match pc {
            0x82538B60 => {
    //   block [0x82538B60..0x82538BB8)
	// 82538B60: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82538B64: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82538B68: FB81FFF0  std r28, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[28].u64 ) };
	// 82538B6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538B70: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82538B74: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538B78: 839F0054  lwz r28, 0x54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82538B7C: 4800001C  b 0x82538b98
	pc = 0x82538B98; continue 'dispatch;
	// 82538B80: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82538B84: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82538B88: FB81FFF0  std r28, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[28].u64 ) };
	// 82538B8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538B90: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82538B94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538B98: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82538B9C: 4BFFE1BD  bl 0x82536d58
	ctx.lr = 0x82538BA0;
	sub_82536D58(ctx, base);
	// 82538BA0: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538BA4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82538BA8: EB81FFF0  ld r28, -0x10(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82538BAC: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82538BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82538BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82538BB8 size=8
    let mut pc: u32 = 0x82538BB8;
    'dispatch: loop {
        match pc {
            0x82538BB8 => {
    //   block [0x82538BB8..0x82538BC0)
	// 82538BB8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82538BBC: 4BFFFE6C  b 0x82538a28
	sub_82538A28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538BC0 size=180
    let mut pc: u32 = 0x82538BC0;
    'dispatch: loop {
        match pc {
            0x82538BC0 => {
    //   block [0x82538BC0..0x82538C74)
	// 82538BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538BC4: 4BFFC4F9  bl 0x825350bc
	ctx.lr = 0x82538BC8;
	sub_82535080(ctx, base);
	// 82538BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538BCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82538BD0: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82538BD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82538BD8: 409A0034  bne cr6, 0x82538c0c
	if !ctx.cr[6].eq {
	pc = 0x82538C0C; continue 'dispatch;
	}
	// 82538BDC: 48001D5D  bl 0x8253a938
	ctx.lr = 0x82538BE0;
	sub_8253A938(ctx, base);
	// 82538BE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538BE4: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538BE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538BF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82538BF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538BF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538BFC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538C00: 48001C01  bl 0x8253a800
	ctx.lr = 0x82538C04;
	sub_8253A800(ctx, base);
	// 82538C04: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82538C08: 48000064  b 0x82538c6c
	pc = 0x82538C6C; continue 'dispatch;
	// 82538C0C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538C10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82538C14: 716B0083  andi. r11, r11, 0x83
	ctx.r[11].u64 = ctx.r[11].u64 & 131;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538C18: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538C1C: 41820048  beq 0x82538c64
	if ctx.cr[0].eq {
	pc = 0x82538C64; continue 'dispatch;
	}
	// 82538C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82538C24: 4BFFE1A5  bl 0x82536dc8
	ctx.lr = 0x82538C28;
	sub_82536DC8(ctx, base);
	// 82538C28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82538C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82538C30: 48009989  bl 0x825425b8
	ctx.lr = 0x82538C34;
	sub_825425B8(ctx, base);
	// 82538C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82538C38: 48004F89  bl 0x8253dbc0
	ctx.lr = 0x82538C3C;
	sub_8253DBC0(ctx, base);
	// 82538C3C: 48009805  bl 0x82542440
	ctx.lr = 0x82538C40;
	sub_82542440(ctx, base);
	// 82538C40: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538C44: 4080000C  bge 0x82538c50
	if !ctx.cr[0].lt {
	pc = 0x82538C50; continue 'dispatch;
	}
	// 82538C48: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82538C4C: 48000018  b 0x82538c64
	pc = 0x82538C64; continue 'dispatch;
	// 82538C50: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82538C54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82538C58: 4182000C  beq 0x82538c64
	if ctx.cr[0].eq {
	pc = 0x82538C64; continue 'dispatch;
	}
	// 82538C5C: 4BFFB0DD  bl 0x82533d38
	ctx.lr = 0x82538C60;
	sub_82533D38(ctx, base);
	// 82538C60: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82538C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538C68: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82538C6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82538C70: 4BFFC49C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82538C78 size=8
    let mut pc: u32 = 0x82538C78;
    'dispatch: loop {
        match pc {
            0x82538C78 => {
    //   block [0x82538C78..0x82538C80)
	// 82538C78: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 82538C7C: 820DA438  lwz r16, -0x5bc8(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538C80 size=196
    let mut pc: u32 = 0x82538C80;
    'dispatch: loop {
        match pc {
            0x82538C80 => {
    //   block [0x82538C80..0x82538D44)
	// 82538C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82538C88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82538C8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82538C90: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82538C94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538C98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82538C9C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82538CA0: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82538CA4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82538CA8: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82538CAC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82538CB0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82538CB4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538CB8: 40820034  bne 0x82538cec
	if !ctx.cr[0].eq {
	pc = 0x82538CEC; continue 'dispatch;
	}
	// 82538CBC: 48001C7D  bl 0x8253a938
	ctx.lr = 0x82538CC0;
	sub_8253A938(ctx, base);
	// 82538CC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538CC4: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538CD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82538CD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538CD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538CDC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538CE0: 48001B21  bl 0x8253a800
	ctx.lr = 0x82538CE4;
	sub_8253A800(ctx, base);
	// 82538CE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82538CE8: 48000044  b 0x82538d2c
	pc = 0x82538D2C; continue 'dispatch;
	// 82538CEC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538CF0: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538CF4: 41820010  beq 0x82538d04
	if ctx.cr[0].eq {
	pc = 0x82538D04; continue 'dispatch;
	}
	// 82538CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82538CFC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82538D00: 48000028  b 0x82538d28
	pc = 0x82538D28; continue 'dispatch;
	// 82538D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538D08: 4BFFDF91  bl 0x82536c98
	ctx.lr = 0x82538D0C;
	sub_82536C98(ctx, base);
	// 82538D0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538D10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538D14: 4BFFFEAD  bl 0x82538bc0
	ctx.lr = 0x82538D18;
	sub_82538BC0(ctx, base);
	// 82538D18: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82538D1C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82538D20: 399F0070  addi r12, r31, 0x70
	ctx.r[12].s64 = ctx.r[31].s64 + 112;
	// 82538D24: 48000041  bl 0x82538d64
	ctx.lr = 0x82538D28;
	sub_82538D44(ctx, base);
	// 82538D28: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82538D2C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82538D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82538D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82538D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82538D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82538D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538D44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538D44 size=88
    let mut pc: u32 = 0x82538D44;
    'dispatch: loop {
        match pc {
            0x82538D44 => {
    //   block [0x82538D44..0x82538D9C)
	// 82538D44: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82538D48: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82538D4C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82538D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538D54: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82538D58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538D5C: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82538D60: 4800001C  b 0x82538d7c
	pc = 0x82538D7C; continue 'dispatch;
	// 82538D64: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82538D68: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82538D6C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82538D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538D74: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82538D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538D80: 4BFFDFD9  bl 0x82536d58
	ctx.lr = 0x82538D84;
	sub_82536D58(ctx, base);
	// 82538D84: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538D88: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82538D8C: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82538D90: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82538D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82538D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82538DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82538DA0 size=620
    let mut pc: u32 = 0x82538DA0;
    'dispatch: loop {
        match pc {
            0x82538DA0 => {
    //   block [0x82538DA0..0x8253900C)
	// 82538DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82538DA4: 4BFFC305  bl 0x825350a8
	ctx.lr = 0x82538DA8;
	sub_82535080(ctx, base);
	// 82538DA8: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82538DAC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82538DB0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82538DB4: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82538DB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82538DBC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82538DC0: 419A003C  beq cr6, 0x82538dfc
	if ctx.cr[6].eq {
	pc = 0x82538DFC; continue 'dispatch;
	}
	// 82538DC4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82538DC8: 419A0034  beq cr6, 0x82538dfc
	if ctx.cr[6].eq {
	pc = 0x82538DFC; continue 'dispatch;
	}
	// 82538DCC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82538DD0: 409A0038  bne cr6, 0x82538e08
	if !ctx.cr[6].eq {
	pc = 0x82538E08; continue 'dispatch;
	}
	// 82538DD4: 48001B65  bl 0x8253a938
	ctx.lr = 0x82538DD8;
	sub_8253A938(ctx, base);
	// 82538DD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538DDC: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82538DE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82538DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82538DE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82538DEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82538DF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538DF4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82538DF8: 48001A09  bl 0x8253a800
	ctx.lr = 0x82538DFC;
	sub_8253A800(ctx, base);
	// 82538DFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82538E00: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82538E04: 4BFFC2F4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82538E08: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82538E0C: 419AFFC8  beq cr6, 0x82538dd4
	if ctx.cr[6].eq {
	pc = 0x82538DD4; continue 'dispatch;
	}
	// 82538E10: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82538E14: 0CD90000  twi 6, r25, 0
	// 82538E18: 7D6BCB96  divwu r11, r11, r25
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[25].u32;
	// 82538E1C: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82538E20: 4199FFB4  bgt cr6, 0x82538dd4
	if ctx.cr[6].gt {
	pc = 0x82538DD4; continue 'dispatch;
	}
	// 82538E24: 7F59C1D6  mullw r26, r25, r24
	ctx.r[26].s64 = (ctx.r[25].s32 as i64) * (ctx.r[24].s32 as i64);
	// 82538E28: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82538E2C: 4BFFDCFD  bl 0x82536b28
	ctx.lr = 0x82538E30;
	sub_82536B28(ctx, base);
	// 82538E30: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82538E34: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82538E38: 419A0188  beq cr6, 0x82538fc0
	if ctx.cr[6].eq {
	pc = 0x82538FC0; continue 'dispatch;
	}
	// 82538E3C: 4BFFDCED  bl 0x82536b28
	ctx.lr = 0x82538E40;
	sub_82536B28(ctx, base);
	// 82538E40: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82538E44: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82538E48: 419A0178  beq cr6, 0x82538fc0
	if ctx.cr[6].eq {
	pc = 0x82538FC0; continue 'dispatch;
	}
	// 82538E4C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538E50: 716B010C  andi. r11, r11, 0x10c
	ctx.r[11].u64 = ctx.r[11].u64 & 268;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538E54: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538E58: 4182000C  beq 0x82538e64
	if ctx.cr[0].eq {
	pc = 0x82538E64; continue 'dispatch;
	}
	// 82538E5C: 837E0018  lwz r27, 0x18(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82538E60: 48000008  b 0x82538e68
	pc = 0x82538E68; continue 'dispatch;
	// 82538E64: 3B601000  li r27, 0x1000
	ctx.r[27].s64 = 4096;
	// 82538E68: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82538E6C: 419A0198  beq cr6, 0x82539004
	if ctx.cr[6].eq {
	pc = 0x82539004; continue 'dispatch;
	}
	// 82538E70: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538E74: 716B0108  andi. r11, r11, 0x108
	ctx.r[11].u64 = ctx.r[11].u64 & 264;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538E78: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538E7C: 41820058  beq 0x82538ed4
	if ctx.cr[0].eq {
	pc = 0x82538ED4; continue 'dispatch;
	}
	// 82538E80: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538E84: 2C1D0000  cmpwi r29, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538E88: 4182004C  beq 0x82538ed4
	if ctx.cr[0].eq {
	pc = 0x82538ED4; continue 'dispatch;
	}
	// 82538E8C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82538E90: 419800C4  blt cr6, 0x82538f54
	if ctx.cr[6].lt {
	pc = 0x82538F54; continue 'dispatch;
	}
	// 82538E94: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82538E98: 40980008  bge cr6, 0x82538ea0
	if !ctx.cr[6].lt {
	pc = 0x82538EA0; continue 'dispatch;
	}
	// 82538E9C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82538EA0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82538EA4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538EA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82538EAC: 4BFFBCA5  bl 0x82534b50
	ctx.lr = 0x82538EB0;
	sub_82534B50(ctx, base);
	// 82538EB0: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82538EB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538EB8: 7FFDF850  subf r31, r29, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 82538EBC: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 82538EC0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82538EC4: 7F9DE214  add r28, r29, r28
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82538EC8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82538ECC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82538ED0: 480000D4  b 0x82538fa4
	pc = 0x82538FA4; continue 'dispatch;
	// 82538ED4: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82538ED8: 41980098  blt cr6, 0x82538f70
	if ctx.cr[6].lt {
	pc = 0x82538F70; continue 'dispatch;
	}
	// 82538EDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82538EE0: 419A0014  beq cr6, 0x82538ef4
	if ctx.cr[6].eq {
	pc = 0x82538EF4; continue 'dispatch;
	}
	// 82538EE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538EE8: 4BFFDEE1  bl 0x82536dc8
	ctx.lr = 0x82538EEC;
	sub_82536DC8(ctx, base);
	// 82538EEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538EF0: 408200C0  bne 0x82538fb0
	if !ctx.cr[0].eq {
	pc = 0x82538FB0; continue 'dispatch;
	}
	// 82538EF4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82538EF8: 419A001C  beq cr6, 0x82538f14
	if ctx.cr[6].eq {
	pc = 0x82538F14; continue 'dispatch;
	}
	// 82538EFC: 7D7FDB96  divwu r11, r31, r27
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[27].u32;
	// 82538F00: 0CDB0000  twi 6, r27, 0
	// 82538F04: 7D6BD9D6  mullw r11, r11, r27
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82538F08: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82538F0C: 7FABF850  subf r29, r11, r31
	ctx.r[29].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82538F10: 48000008  b 0x82538f18
	pc = 0x82538F18; continue 'dispatch;
	// 82538F14: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82538F18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82538F1C: 48004CA5  bl 0x8253dbc0
	ctx.lr = 0x82538F20;
	sub_8253DBC0(ctx, base);
	// 82538F20: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82538F24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82538F28: 48005039  bl 0x8253df60
	ctx.lr = 0x82538F2C;
	sub_8253DF60(ctx, base);
	// 82538F2C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82538F30: 419A0024  beq cr6, 0x82538f54
	if ctx.cr[6].eq {
	pc = 0x82538F54; continue 'dispatch;
	}
	// 82538F34: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82538F38: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82538F3C: 41990008  bgt cr6, 0x82538f44
	if ctx.cr[6].gt {
	pc = 0x82538F44; continue 'dispatch;
	}
	// 82538F40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82538F44: 7FEBF850  subf r31, r11, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82538F48: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82538F4C: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82538F50: 40980054  bge cr6, 0x82538fa4
	if !ctx.cr[6].lt {
	pc = 0x82538FA4; continue 'dispatch;
	}
	// 82538F54: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82538F58: 7D5FD050  subf r10, r31, r26
	ctx.r[10].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 82538F5C: 0CD90000  twi 6, r25, 0
	// 82538F60: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82538F64: 7C6ACB96  divwu r3, r10, r25
	ctx.r[3].u32 = ctx.r[10].u32 / ctx.r[25].u32;
	// 82538F68: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82538F6C: 4BFFFE94  b 0x82538e00
	pc = 0x82538E00; continue 'dispatch;
	// 82538F70: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82538F74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82538F78: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82538F7C: 48000B2D  bl 0x82539aa8
	ctx.lr = 0x82538F80;
	sub_82539AA8(ctx, base);
	// 82538F80: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82538F84: 419A002C  beq cr6, 0x82538fb0
	if ctx.cr[6].eq {
	pc = 0x82538FB0; continue 'dispatch;
	}
	// 82538F88: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82538F8C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82538F90: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82538F94: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82538F98: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82538F9C: 41810008  bgt 0x82538fa4
	if ctx.cr[0].gt {
	pc = 0x82538FA4; continue 'dispatch;
	}
	// 82538FA0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82538FA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82538FA8: 409AFEC8  bne cr6, 0x82538e70
	if !ctx.cr[6].eq {
	pc = 0x82538E70; continue 'dispatch;
	}
	// 82538FAC: 48000058  b 0x82539004
	pc = 0x82539004; continue 'dispatch;
	// 82538FB0: 7D7FD050  subf r11, r31, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 82538FB4: 0CD90000  twi 6, r25, 0
	// 82538FB8: 7C6BCB96  divwu r3, r11, r25
	ctx.r[3].u32 = ctx.r[11].u32 / ctx.r[25].u32;
	// 82538FBC: 4BFFFE44  b 0x82538e00
	pc = 0x82538E00; continue 'dispatch;
	// 82538FC0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82538FC4: 419A0040  beq cr6, 0x82539004
	if ctx.cr[6].eq {
	pc = 0x82539004; continue 'dispatch;
	}
	// 82538FC8: 2B1F00FF  cmplwi cr6, r31, 0xff
	ctx.cr[6].compare_u32(ctx.r[31].u32, 255 as u32, &mut ctx.xer);
	// 82538FCC: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82538FD0: 41980008  blt cr6, 0x82538fd8
	if ctx.cr[6].lt {
	pc = 0x82538FD8; continue 'dispatch;
	}
	// 82538FD4: 3BC000FF  li r30, 0xff
	ctx.r[30].s64 = 255;
	// 82538FD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82538FDC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82538FE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82538FE4: 4BFFBB6D  bl 0x82534b50
	ctx.lr = 0x82538FE8;
	sub_82534B50(ctx, base);
	// 82538FE8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82538FEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82538FF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82538FF4: 7D5E59AE  stbx r10, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82538FF8: 4800D7D9  bl 0x825467d0
	ctx.lr = 0x82538FFC;
	sub_825467D0(ctx, base);
	// 82538FFC: 7FFEF851  subf. r31, r30, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82539000: 4082FFC8  bne 0x82538fc8
	if !ctx.cr[0].eq {
	pc = 0x82538FC8; continue 'dispatch;
	}
	// 82539004: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82539008: 4BFFFDF8  b 0x82538e00
	pc = 0x82538E00; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82539010 size=8
    let mut pc: u32 = 0x82539010;
    'dispatch: loop {
        match pc {
            0x82539010 => {
    //   block [0x82539010..0x82539018)
	// 82539010: 8270D4EC  lwz r19, -0x2b14(r16)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-11028 as u32) ) } as u64;
	// 82539014: 820DA450  lwz r16, -0x5bb0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-23472 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82539018 size=180
    let mut pc: u32 = 0x82539018;
    'dispatch: loop {
        match pc {
            0x82539018 => {
    //   block [0x82539018..0x825390CC)
	// 82539018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253901C: 4BFFC099  bl 0x825350b4
	ctx.lr = 0x82539020;
	sub_82535080(ctx, base);
	// 82539020: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82539024: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539028: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8253902C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82539030: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82539034: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82539038: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 8253903C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82539040: 419A0048  beq cr6, 0x82539088
	if ctx.cr[6].eq {
	pc = 0x82539088; continue 'dispatch;
	}
	// 82539044: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82539048: 419A0040  beq cr6, 0x82539088
	if ctx.cr[6].eq {
	pc = 0x82539088; continue 'dispatch;
	}
	// 8253904C: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82539050: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82539054: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82539058: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253905C: 40820038  bne 0x82539094
	if !ctx.cr[0].eq {
	pc = 0x82539094; continue 'dispatch;
	}
	// 82539060: 480018D9  bl 0x8253a938
	ctx.lr = 0x82539064;
	sub_8253A938(ctx, base);
	// 82539064: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539068: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253906C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82539070: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82539074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82539078: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253907C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82539080: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82539084: 4800177D  bl 0x8253a800
	ctx.lr = 0x82539088;
	sub_8253A800(ctx, base);
	// 82539088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253908C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82539090: 4BFFC074  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82539094: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82539098: 4BFFDC01  bl 0x82536c98
	ctx.lr = 0x8253909C;
	sub_82536C98(ctx, base);
	// 8253909C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825390A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825390A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825390A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825390AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825390B0: 4BFFFCF1  bl 0x82538da0
	ctx.lr = 0x825390B4;
	sub_82538DA0(ctx, base);
	// 825390B4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825390B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 825390BC: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 825390C0: 4800002D  bl 0x825390ec
	ctx.lr = 0x825390C4;
	sub_825390CC(ctx, base);
	// 825390C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825390C8: 4BFFFFC4  b 0x8253908c
	pc = 0x8253908C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825390CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825390CC size=88
    let mut pc: u32 = 0x825390CC;
    'dispatch: loop {
        match pc {
            0x825390CC => {
    //   block [0x825390CC..0x82539124)
	// 825390CC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825390D0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 825390D4: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825390D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825390DC: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 825390E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825390E4: 83DF00BC  lwz r30, 0xbc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 825390E8: 4800001C  b 0x82539104
	pc = 0x82539104; continue 'dispatch;
	// 825390EC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 825390F0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 825390F4: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 825390F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825390FC: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 82539100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82539108: 4BFFDC51  bl 0x82536d58
	ctx.lr = 0x8253910C;
	sub_82536D58(ctx, base);
	// 8253910C: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 82539110: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82539114: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82539118: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 8253911C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82539120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82539130 size=664
    let mut pc: u32 = 0x82539130;
    'dispatch: loop {
        match pc {
            0x82539130 => {
    //   block [0x82539130..0x825393C8)
	// 82539130: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
	// 82539134: 7DCB61CE  stvx v14, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[14].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[14].u8[first + i]); }
	}
	// 82539138: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
	// 8253913C: 7DEB61CE  stvx v15, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[15].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[15].u8[first + i]); }
	}
	// 82539140: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
	// 82539144: 7E0B61CE  stvx v16, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[16].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[16].u8[first + i]); }
	}
	// 82539148: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
	// 8253914C: 7E2B61CE  stvx v17, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[17].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[17].u8[first + i]); }
	}
	// 82539150: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
	// 82539154: 7E4B61CE  stvx v18, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[18].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[18].u8[first + i]); }
	}
	// 82539158: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
	// 8253915C: 7E6B61CE  stvx v19, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[19].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[19].u8[first + i]); }
	}
	// 82539160: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
	// 82539164: 7E8B61CE  stvx v20, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[20].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[20].u8[first + i]); }
	}
	// 82539168: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
	// 8253916C: 7EAB61CE  stvx v21, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[21].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[21].u8[first + i]); }
	}
	// 82539170: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
	// 82539174: 7ECB61CE  stvx v22, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[22].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[22].u8[first + i]); }
	}
	// 82539178: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
	// 8253917C: 7EEB61CE  stvx v23, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[23].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[23].u8[first + i]); }
	}
	// 82539180: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 82539184: 7F0B61CE  stvx v24, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[24].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[24].u8[first + i]); }
	}
	// 82539188: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
	// 8253918C: 7F2B61CE  stvx v25, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[25].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[25].u8[first + i]); }
	}
	// 82539190: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
	// 82539194: 7F4B61CE  stvx v26, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[26].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[26].u8[first + i]); }
	}
	// 82539198: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
	// 8253919C: 7F6B61CE  stvx v27, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[27].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[27].u8[first + i]); }
	}
	// 825391A0: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
	// 825391A4: 7F8B61CE  stvx v28, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[28].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[28].u8[first + i]); }
	}
	// 825391A8: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
	// 825391AC: 7FAB61CE  stvx v29, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[29].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[29].u8[first + i]); }
	}
	// 825391B0: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
	// 825391B4: 7FCB61CE  stvx v30, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[30].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[30].u8[first + i]); }
	}
	// 825391B8: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
	// 825391BC: 7FEB61CE  stvx v31, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[31].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[31].u8[first + i]); }
	}
	// 825391C0: 4E800020  blr
	return;
	// 825391C4: 3960FC00  li r11, -0x400
	ctx.r[11].s64 = -1024;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825393C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825393C8 size=664
    let mut pc: u32 = 0x825393C8;
    'dispatch: loop {
        match pc {
            0x825393C8 => {
    //   block [0x825393C8..0x82539660)
	// 825393C8: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
	// 825393CC: 7DCB60CE  lvx v14, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[14] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825393D0: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
	// 825393D4: 7DEB60CE  lvx v15, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[15] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825393D8: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
	// 825393DC: 7E0B60CE  lvx v16, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[16] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825393E0: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
	// 825393E4: 7E2B60CE  lvx v17, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[17] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825393E8: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
	// 825393EC: 7E4B60CE  lvx v18, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[18] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825393F0: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
	// 825393F4: 7E6B60CE  lvx v19, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[19] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825393F8: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
	// 825393FC: 7E8B60CE  lvx v20, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[20] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539400: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
	// 82539404: 7EAB60CE  lvx v21, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[21] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539408: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
	// 8253940C: 7ECB60CE  lvx v22, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[22] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539410: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
	// 82539414: 7EEB60CE  lvx v23, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[23] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539418: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 8253941C: 7F0B60CE  lvx v24, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[24] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539420: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
	// 82539424: 7F2B60CE  lvx v25, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[25] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539428: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
	// 8253942C: 7F4B60CE  lvx v26, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[26] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539430: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
	// 82539434: 7F6B60CE  lvx v27, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[27] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539438: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
	// 8253943C: 7F8B60CE  lvx v28, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[28] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539440: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
	// 82539444: 7FAB60CE  lvx v29, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[29] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539448: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
	// 8253944C: 7FCB60CE  lvx v30, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[30] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539450: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
	// 82539454: 7FEB60CE  lvx v31, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[31] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82539458: 4E800020  blr
	return;
	// 8253945C: 3960FC00  li r11, -0x400
	ctx.r[11].s64 = -1024;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82539660 size=100
    let mut pc: u32 = 0x82539660;
    'dispatch: loop {
        match pc {
            0x82539660 => {
    //   block [0x82539660..0x825396C4)
	// 82539660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82539664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82539668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253966C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82539674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82539678: 4BE91E39  bl 0x823cb4b0
	ctx.lr = 0x8253967C;
	sub_823CB4B0(ctx, base);
	// 8253967C: 3D80FE62  lis r12, -0x19e
	ctx.r[12].s64 = -27131904;
	// 82539680: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82539684: 3D600098  lis r11, 0x98
	ctx.r[11].s64 = 9961472;
	// 82539688: 618C4E21  ori r12, r12, 0x4e21
	ctx.r[12].u64 = ctx.r[12].u64 | 20001;
	// 8253968C: 616B9680  ori r11, r11, 0x9680
	ctx.r[11].u64 = ctx.r[11].u64 | 38528;
	// 82539690: 798C07C6  sldi r12, r12, 0x20
	ctx.r[12].u64 = ctx.r[12].u64.wrapping_shl(32);
	ctx.r[12].u32 = ctx.r[12].u64 as u32;
	// 82539694: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82539698: 658C2AC1  oris r12, r12, 0x2ac1
	ctx.r[12].u64 = ctx.r[12].u64 | 717291520;
	// 8253969C: 618C8000  ori r12, r12, 0x8000
	ctx.r[12].u64 = ctx.r[12].u64 | 32768;
	// 825396A0: 7D4A6214  add r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[12].u64;
	// 825396A4: 7C6A5B92  divdu r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 / ctx.r[11].u64;
	// 825396A8: 419A0008  beq cr6, 0x825396b0
	if ctx.cr[6].eq {
	pc = 0x825396B0; continue 'dispatch;
	}
	// 825396AC: F87F0000  std r3, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 825396B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825396B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825396B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825396BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825396C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825396C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825396C8 size=244
    let mut pc: u32 = 0x825396C8;
    'dispatch: loop {
        match pc {
            0x825396C8 => {
    //   block [0x825396C8..0x825397BC)
	// 825396C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825396CC: 4BFFB9E9  bl 0x825350b4
	ctx.lr = 0x825396D0;
	sub_82535080(ctx, base);
	// 825396D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825396D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825396D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825396DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 825396E0: 409A0010  bne cr6, 0x825396f0
	if !ctx.cr[6].eq {
	pc = 0x825396F0; continue 'dispatch;
	}
	// 825396E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825396E8: 4BFFA589  bl 0x82533c70
	ctx.lr = 0x825396EC;
	sub_82533C70(ctx, base);
	// 825396EC: 48000090  b 0x8253977c
	pc = 0x8253977C; continue 'dispatch;
	// 825396F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825396F4: 409A0010  bne cr6, 0x82539704
	if !ctx.cr[6].eq {
	pc = 0x82539704; continue 'dispatch;
	}
	// 825396F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825396FC: 4BFFA63D  bl 0x82533d38
	ctx.lr = 0x82539700;
	sub_82533D38(ctx, base);
	// 82539700: 48000078  b 0x82539778
	pc = 0x82539778; continue 'dispatch;
	// 82539704: 3BA0F000  li r29, -0x1000
	ctx.r[29].s64 = -4096;
	// 82539708: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8253970C: 41990054  bgt cr6, 0x82539760
	if ctx.cr[6].gt {
	pc = 0x82539760; continue 'dispatch;
	}
	// 82539710: 3F60829A  lis r27, -0x7d66
	ctx.r[27].s64 = -2103836672;
	// 82539714: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82539718: 409A0008  bne cr6, 0x82539720
	if !ctx.cr[6].eq {
	pc = 0x82539720; continue 'dispatch;
	}
	// 8253971C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82539720: 4BE9A039  bl 0x823d3758
	ctx.lr = 0x82539724;
	sub_823D3758(ctx, base);
	// 82539724: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82539728: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8253972C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82539730: 4BE99771  bl 0x823d2ea0
	ctx.lr = 0x82539734;
	sub_823D2EA0(ctx, base);
	// 82539734: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82539738: 4082007C  bne 0x825397b4
	if !ctx.cr[0].eq {
	pc = 0x825397B4; continue 'dispatch;
	}
	// 8253973C: 817B2D90  lwz r11, 0x2d90(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(11664 as u32) ) } as u64;
	// 82539740: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539744: 419A005C  beq cr6, 0x825397a0
	if ctx.cr[6].eq {
	pc = 0x825397A0; continue 'dispatch;
	}
	// 82539748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253974C: 4BFFCFAD  bl 0x825366f8
	ctx.lr = 0x82539750;
	sub_825366F8(ctx, base);
	// 82539750: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539754: 41820030  beq 0x82539784
	if ctx.cr[0].eq {
	pc = 0x82539784; continue 'dispatch;
	}
	// 82539758: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8253975C: 4099FFB8  ble cr6, 0x82539714
	if !ctx.cr[6].gt {
	pc = 0x82539714; continue 'dispatch;
	}
	// 82539760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539764: 4BFFCF95  bl 0x825366f8
	ctx.lr = 0x82539768;
	sub_825366F8(ctx, base);
	// 82539768: 480011D1  bl 0x8253a938
	ctx.lr = 0x8253976C;
	sub_8253A938(ctx, base);
	// 8253976C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539770: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82539774: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82539778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253977C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82539780: 4BFFB984  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82539784: 480011B5  bl 0x8253a938
	ctx.lr = 0x82539788;
	sub_8253A938(ctx, base);
	// 82539788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253978C: 4BE86EF5  bl 0x823c0680
	ctx.lr = 0x82539790;
	sub_823C0680(ctx, base);
	// 82539790: 48001141  bl 0x8253a8d0
	ctx.lr = 0x82539794;
	sub_8253A8D0(ctx, base);
	// 82539794: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539798: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253979C: 4BFFFFDC  b 0x82539778
	pc = 0x82539778; continue 'dispatch;
	// 825397A0: 48001199  bl 0x8253a938
	ctx.lr = 0x825397A4;
	sub_8253A938(ctx, base);
	// 825397A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825397A8: 4BE86ED9  bl 0x823c0680
	ctx.lr = 0x825397AC;
	sub_823C0680(ctx, base);
	// 825397AC: 48001125  bl 0x8253a8d0
	ctx.lr = 0x825397B0;
	sub_8253A8D0(ctx, base);
	// 825397B0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825397B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825397B8: 4BFFFFC4  b 0x8253977c
	pc = 0x8253977C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825397C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825397C0 size=176
    let mut pc: u32 = 0x825397C0;
    'dispatch: loop {
        match pc {
            0x825397C0 => {
    //   block [0x825397C0..0x82539870)
	// 825397C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825397C4: 4BFFB8F9  bl 0x825350bc
	ctx.lr = 0x825397C8;
	sub_82535080(ctx, base);
	// 825397C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825397CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825397D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825397D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825397D8: 419A0048  beq cr6, 0x82539820
	if ctx.cr[6].eq {
	pc = 0x82539820; continue 'dispatch;
	}
	// 825397DC: 3960F000  li r11, -0x1000
	ctx.r[11].s64 = -4096;
	// 825397E0: 0CC40000  twi 6, r4, 0
	// 825397E4: 7D6B2396  divwu r11, r11, r4
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[4].u32;
	// 825397E8: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 825397EC: 40980034  bge cr6, 0x82539820
	if !ctx.cr[6].lt {
	pc = 0x82539820; continue 'dispatch;
	}
	// 825397F0: 48001149  bl 0x8253a938
	ctx.lr = 0x825397F4;
	sub_8253A938(ctx, base);
	// 825397F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825397F8: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 825397FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82539800: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82539804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82539808: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253980C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82539810: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82539814: 48000FED  bl 0x8253a800
	ctx.lr = 0x82539818;
	sub_8253A800(ctx, base);
	// 82539818: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253981C: 4800004C  b 0x82539868
	pc = 0x82539868; continue 'dispatch;
	// 82539820: 7FC429D6  mullw r30, r4, r5
	ctx.r[30].s64 = (ctx.r[4].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82539824: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82539828: 419A0010  beq cr6, 0x82539838
	if ctx.cr[6].eq {
	pc = 0x82539838; continue 'dispatch;
	}
	// 8253982C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539830: 48000041  bl 0x82539870
	ctx.lr = 0x82539834;
	sub_82539870(ctx, base);
	// 82539834: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82539838: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253983C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539840: 4BFFFE89  bl 0x825396c8
	ctx.lr = 0x82539844;
	sub_825396C8(ctx, base);
	// 82539844: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82539848: 4182001C  beq 0x82539864
	if ctx.cr[0].eq {
	pc = 0x82539864; continue 'dispatch;
	}
	// 8253984C: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82539850: 40980014  bge cr6, 0x82539864
	if !ctx.cr[6].lt {
	pc = 0x82539864; continue 'dispatch;
	}
	// 82539854: 7CBDF050  subf r5, r29, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82539858: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253985C: 7C7DFA14  add r3, r29, r31
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82539860: 4BFFB971  bl 0x825351d0
	ctx.lr = 0x82539864;
	sub_825351D0(ctx, base);
	// 82539864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539868: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253986C: 4BFFB8A0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82539870 size=112
    let mut pc: u32 = 0x82539870;
    'dispatch: loop {
        match pc {
            0x82539870 => {
    //   block [0x82539870..0x825398E0)
	// 82539870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82539874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82539878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253987C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82539884: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82539888: 409A0034  bne cr6, 0x825398bc
	if !ctx.cr[6].eq {
	pc = 0x825398BC; continue 'dispatch;
	}
	// 8253988C: 480010AD  bl 0x8253a938
	ctx.lr = 0x82539890;
	sub_8253A938(ctx, base);
	// 82539890: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539894: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82539898: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253989C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825398A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825398A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825398A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825398AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825398B0: 48000F51  bl 0x8253a800
	ctx.lr = 0x825398B4;
	sub_8253A800(ctx, base);
	// 825398B4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825398B8: 48000014  b 0x825398cc
	pc = 0x825398CC; continue 'dispatch;
	// 825398BC: 4BE99E9D  bl 0x823d3758
	ctx.lr = 0x825398C0;
	sub_823D3758(ctx, base);
	// 825398C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825398C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825398C8: 4BE97CA1  bl 0x823d1568
	ctx.lr = 0x825398CC;
	sub_823D1568(ctx, base);
	// 825398CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825398D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825398D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825398D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825398DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825398E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825398E0 size=216
    let mut pc: u32 = 0x825398E0;
    'dispatch: loop {
        match pc {
            0x825398E0 => {
    //   block [0x825398E0..0x825399B8)
	// 825398E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825398E4: 4BFFB7D5  bl 0x825350b8
	ctx.lr = 0x825398E8;
	sub_82535080(ctx, base);
	// 825398E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825398EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825398F0: 3BC0F000  li r30, -0x1000
	ctx.r[30].s64 = -4096;
	// 825398F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825398F8: 419A0044  beq cr6, 0x8253993c
	if ctx.cr[6].eq {
	pc = 0x8253993C; continue 'dispatch;
	}
	// 825398FC: 7D7E1B96  divwu r11, r30, r3
	ctx.r[11].u32 = ctx.r[30].u32 / ctx.r[3].u32;
	// 82539900: 0CC30000  twi 6, r3, 0
	// 82539904: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82539908: 40980034  bge cr6, 0x8253993c
	if !ctx.cr[6].lt {
	pc = 0x8253993C; continue 'dispatch;
	}
	// 8253990C: 4800102D  bl 0x8253a938
	ctx.lr = 0x82539910;
	sub_8253A938(ctx, base);
	// 82539910: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539914: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82539918: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82539920: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82539924: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82539928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253992C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82539930: 48000ED1  bl 0x8253a800
	ctx.lr = 0x82539934;
	sub_8253A800(ctx, base);
	// 82539934: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82539938: 48000078  b 0x825399b0
	pc = 0x825399B0; continue 'dispatch;
	// 8253993C: 7FE321D7  mullw. r31, r3, r4
	ctx.r[31].s64 = (ctx.r[3].s32 as i64) * (ctx.r[4].s32 as i64);
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82539940: 40820008  bne 0x82539948
	if !ctx.cr[0].eq {
	pc = 0x82539948; continue 'dispatch;
	}
	// 82539944: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82539948: 3FA0829A  lis r29, -0x7d66
	ctx.r[29].s64 = -2103836672;
	// 8253994C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82539950: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82539954: 4199001C  bgt cr6, 0x82539970
	if ctx.cr[6].gt {
	pc = 0x82539970; continue 'dispatch;
	}
	// 82539958: 4BE99E01  bl 0x823d3758
	ctx.lr = 0x8253995C;
	sub_823D3758(ctx, base);
	// 8253995C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82539960: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82539964: 4BE9896D  bl 0x823d22d0
	ctx.lr = 0x82539968;
	sub_823D22D0(ctx, base);
	// 82539968: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253996C: 40820044  bne 0x825399b0
	if !ctx.cr[0].eq {
	pc = 0x825399B0; continue 'dispatch;
	}
	// 82539970: 817D2D90  lwz r11, 0x2d90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(11664 as u32) ) } as u64;
	// 82539974: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539978: 419A0028  beq cr6, 0x825399a0
	if ctx.cr[6].eq {
	pc = 0x825399A0; continue 'dispatch;
	}
	// 8253997C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539980: 4BFFCD79  bl 0x825366f8
	ctx.lr = 0x82539984;
	sub_825366F8(ctx, base);
	// 82539984: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539988: 4082FFC4  bne 0x8253994c
	if !ctx.cr[0].eq {
	pc = 0x8253994C; continue 'dispatch;
	}
	// 8253998C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82539990: 419AFFA4  beq cr6, 0x82539934
	if ctx.cr[6].eq {
	pc = 0x82539934; continue 'dispatch;
	}
	// 82539994: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82539998: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253999C: 4BFFFF98  b 0x82539934
	pc = 0x82539934; continue 'dispatch;
	// 825399A0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 825399A4: 419A000C  beq cr6, 0x825399b0
	if ctx.cr[6].eq {
	pc = 0x825399B0; continue 'dispatch;
	}
	// 825399A8: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 825399AC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825399B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825399B4: 4BFFB754  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825399B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825399B8 size=104
    let mut pc: u32 = 0x825399B8;
    'dispatch: loop {
        match pc {
            0x825399B8 => {
    //   block [0x825399B8..0x82539A20)
	// 825399B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825399BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825399C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825399C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825399C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825399CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825399D0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825399D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825399D8: 4BFFFF09  bl 0x825398e0
	ctx.lr = 0x825399DC;
	sub_825398E0(ctx, base);
	// 825399DC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825399E0: 40820024  bne 0x82539a04
	if !ctx.cr[0].eq {
	pc = 0x82539A04; continue 'dispatch;
	}
	// 825399E4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825399E8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825399EC: 419A0018  beq cr6, 0x82539a04
	if ctx.cr[6].eq {
	pc = 0x82539A04; continue 'dispatch;
	}
	// 825399F0: 48000F49  bl 0x8253a938
	ctx.lr = 0x825399F4;
	sub_8253A938(ctx, base);
	// 825399F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825399F8: 4182000C  beq 0x82539a04
	if ctx.cr[0].eq {
	pc = 0x82539A04; continue 'dispatch;
	}
	// 825399FC: 48000F3D  bl 0x8253a938
	ctx.lr = 0x82539A00;
	sub_8253A938(ctx, base);
	// 82539A00: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82539A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539A08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82539A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82539A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82539A14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82539A18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82539A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82539A20 size=52
    let mut pc: u32 = 0x82539A20;
    'dispatch: loop {
        match pc {
            0x82539A20 => {
    //   block [0x82539A20..0x82539A54)
	// 82539A20: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82539A24: 392BEB70  addi r9, r11, -0x1490
	ctx.r[9].s64 = ctx.r[11].s64 + -5264;
	// 82539A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82539A2C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82539A30: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82539A34: 7F034000  cmpw cr6, r3, r8
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82539A38: 419A0014  beq cr6, 0x82539a4c
	if ctx.cr[6].eq {
	pc = 0x82539A4C; continue 'dispatch;
	}
	// 82539A3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82539A40: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82539A44: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82539A48: 4198FFE8  blt cr6, 0x82539a30
	if ctx.cr[6].lt {
	pc = 0x82539A30; continue 'dispatch;
	}
	// 82539A4C: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82539A50: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82539A54 size=16
    let mut pc: u32 = 0x82539A54;
    'dispatch: loop {
        match pc {
            0x82539A54 => {
    //   block [0x82539A54..0x82539A64)
	// 82539A54: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82539A58: 39490004  addi r10, r9, 4
	ctx.r[10].s64 = ctx.r[9].s64 + 4;
	// 82539A5C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82539A60: 4800CD70  b 0x825467d0
	sub_825467D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539A64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82539A64 size=4
    let mut pc: u32 = 0x82539A64;
    'dispatch: loop {
        match pc {
            0x82539A64 => {
    //   block [0x82539A64..0x82539A68)
	// 82539A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82539A68 size=44
    let mut pc: u32 = 0x82539A68;
    'dispatch: loop {
        match pc {
            0x82539A68 => {
    //   block [0x82539A68..0x82539A94)
	// 82539A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82539A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82539A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539A74: 386000FC  li r3, 0xfc
	ctx.r[3].s64 = 252;
	// 82539A78: 4BFFFFA9  bl 0x82539a20
	ctx.lr = 0x82539A7C;
	sub_82539A20(ctx, base);
	// 82539A7C: 386000FF  li r3, 0xff
	ctx.r[3].s64 = 255;
	// 82539A80: 4BFFFFA1  bl 0x82539a20
	ctx.lr = 0x82539A84;
	sub_82539A20(ctx, base);
	// 82539A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82539A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82539A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82539A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82539A98 size=12
    let mut pc: u32 = 0x82539A98;
    'dispatch: loop {
        match pc {
            0x82539A98 => {
    //   block [0x82539A98..0x82539AA4)
	// 82539A98: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82539A9C: 906B2D6C  stw r3, 0x2d6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(11628 as u32), ctx.r[3].u32 ) };
	// 82539AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82539AA8 size=464
    let mut pc: u32 = 0x82539AA8;
    'dispatch: loop {
        match pc {
            0x82539AA8 => {
    //   block [0x82539AA8..0x82539C78)
	// 82539AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82539AAC: 4BFFB60D  bl 0x825350b8
	ctx.lr = 0x82539AB0;
	sub_82535080(ctx, base);
	// 82539AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539AB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82539AB8: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82539ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539AC0: 48004101  bl 0x8253dbc0
	ctx.lr = 0x82539AC4;
	sub_8253DBC0(ctx, base);
	// 82539AC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82539AC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82539ACC: 716A0082  andi. r10, r11, 0x82
	ctx.r[10].u64 = ctx.r[11].u64 & 130;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82539AD0: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539AD4: 40820028  bne 0x82539afc
	if !ctx.cr[0].eq {
	pc = 0x82539AFC; continue 'dispatch;
	}
	// 82539AD8: 48000E61  bl 0x8253a938
	ctx.lr = 0x82539ADC;
	sub_8253A938(ctx, base);
	// 82539ADC: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 82539AE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539AE4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82539AE8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82539AEC: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82539AF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82539AF4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82539AF8: 48000178  b 0x82539c70
	pc = 0x82539C70; continue 'dispatch;
	// 82539AFC: 556A0673  rlwinm. r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82539B00: 41820010  beq 0x82539b10
	if ctx.cr[0].eq {
	pc = 0x82539B10; continue 'dispatch;
	}
	// 82539B04: 48000E35  bl 0x8253a938
	ctx.lr = 0x82539B08;
	sub_8253A938(ctx, base);
	// 82539B08: 39400022  li r10, 0x22
	ctx.r[10].s64 = 34;
	// 82539B0C: 4BFFFFD4  b 0x82539ae0
	pc = 0x82539AE0; continue 'dispatch;
	// 82539B10: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82539B14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82539B18: 41820020  beq 0x82539b38
	if ctx.cr[0].eq {
	pc = 0x82539B38; continue 'dispatch;
	}
	// 82539B1C: 556906F7  rlwinm. r9, r11, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82539B20: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82539B24: 4182FFC8  beq 0x82539aec
	if ctx.cr[0].eq {
	pc = 0x82539AEC; continue 'dispatch;
	}
	// 82539B28: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82539B2C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82539B30: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82539B34: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82539B38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82539B3C: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82539B40: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82539B44: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82539B48: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82539B4C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82539B50: 716A010C  andi. r10, r11, 0x10c
	ctx.r[10].u64 = ctx.r[11].u64 & 268;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82539B54: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539B58: 4082003C  bne 0x82539b94
	if !ctx.cr[0].eq {
	pc = 0x82539B94; continue 'dispatch;
	}
	// 82539B5C: 4BFFCFCD  bl 0x82536b28
	ctx.lr = 0x82539B60;
	sub_82536B28(ctx, base);
	// 82539B60: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82539B64: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82539B68: 419A0014  beq cr6, 0x82539b7c
	if ctx.cr[6].eq {
	pc = 0x82539B7C; continue 'dispatch;
	}
	// 82539B6C: 4BFFCFBD  bl 0x82536b28
	ctx.lr = 0x82539B70;
	sub_82536B28(ctx, base);
	// 82539B70: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82539B74: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82539B78: 409A0014  bne cr6, 0x82539b8c
	if !ctx.cr[6].eq {
	pc = 0x82539B8C; continue 'dispatch;
	}
	// 82539B7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82539B80: 4BDB7851  bl 0x822f13d0
	ctx.lr = 0x82539B84;
	sub_822F13D0(ctx, base);
	// 82539B84: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539B88: 4082000C  bne 0x82539b94
	if !ctx.cr[0].eq {
	pc = 0x82539B94; continue 'dispatch;
	}
	// 82539B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82539B90: 48008CF1  bl 0x82542880
	ctx.lr = 0x82539B94;
	sub_82542880(ctx, base);
	// 82539B94: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82539B98: 716B0108  andi. r11, r11, 0x108
	ctx.r[11].u64 = ctx.r[11].u64 & 264;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539B9C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539BA0: 418200A8  beq 0x82539c48
	if ctx.cr[0].eq {
	pc = 0x82539C48; continue 'dispatch;
	}
	// 82539BA4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82539BA8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82539BAC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82539BB0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82539BB4: 7FC45051  subf. r30, r4, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82539BB8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82539BBC: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82539BC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82539BC4: 40810018  ble 0x82539bdc
	if !ctx.cr[0].gt {
	pc = 0x82539BDC; continue 'dispatch;
	}
	// 82539BC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82539BCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82539BD0: 48004391  bl 0x8253df60
	ctx.lr = 0x82539BD4;
	sub_8253DF60(ctx, base);
	// 82539BD4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82539BD8: 48000060  b 0x82539c38
	pc = 0x82539C38; continue 'dispatch;
	// 82539BDC: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82539BE0: 419A002C  beq cr6, 0x82539c0c
	if ctx.cr[6].eq {
	pc = 0x82539C0C; continue 'dispatch;
	}
	// 82539BE4: 2F1DFFFE  cmpwi cr6, r29, -2
	ctx.cr[6].compare_i32(ctx.r[29].s32, -2, &mut ctx.xer);
	// 82539BE8: 419A0024  beq cr6, 0x82539c0c
	if ctx.cr[6].eq {
	pc = 0x82539C0C; continue 'dispatch;
	}
	// 82539BEC: 7FAA2E70  srawi r10, r29, 5
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[29].s32 >> 5) as i64;
	// 82539BF0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82539BF4: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82539BF8: 396B3CE0  addi r11, r11, 0x3ce0
	ctx.r[11].s64 = ctx.r[11].s64 + 15584;
	// 82539BFC: 57AA3572  rlwinm r10, r29, 6, 0x15, 0x19
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x03FFFFFFu64;
	// 82539C00: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82539C04: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82539C08: 4800000C  b 0x82539c14
	pc = 0x82539C14; continue 'dispatch;
	// 82539C0C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82539C10: 396BF188  addi r11, r11, -0xe78
	ctx.r[11].s64 = ctx.r[11].s64 + -3704;
	// 82539C14: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82539C18: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539C1C: 4182001C  beq 0x82539c38
	if ctx.cr[0].eq {
	pc = 0x82539C38; continue 'dispatch;
	}
	// 82539C20: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82539C24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82539C28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82539C2C: 48008ABD  bl 0x825426e8
	ctx.lr = 0x82539C30;
	sub_825426E8(ctx, base);
	// 82539C30: 2F23FFFF  cmpdi cr6, r3, -1
	ctx.cr[6].compare_i64(ctx.r[3].s64, -1, &mut ctx.xer);
	// 82539C34: 419AFEB4  beq cr6, 0x82539ae8
	if ctx.cr[6].eq {
	pc = 0x82539AE8; continue 'dispatch;
	}
	// 82539C38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82539C3C: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82539C40: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82539C44: 4800001C  b 0x82539c60
	pc = 0x82539C60; continue 'dispatch;
	// 82539C48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82539C4C: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 82539C50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82539C54: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82539C58: 48004309  bl 0x8253df60
	ctx.lr = 0x82539C5C;
	sub_8253DF60(ctx, base);
	// 82539C5C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82539C60: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82539C64: 409AFE84  bne cr6, 0x82539ae8
	if !ctx.cr[6].eq {
	pc = 0x82539AE8; continue 'dispatch;
	}
	// 82539C68: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82539C6C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82539C70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82539C74: 4BFFB494  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82539C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82539C78 size=2932
    let mut pc: u32 = 0x82539C78;
    'dispatch: loop {
        match pc {
            0x82539C78 => {
    //   block [0x82539C78..0x8253A7EC)
	// 82539C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82539C7C: 4BFFB405  bl 0x82535080
	ctx.lr = 0x82539C80;
	sub_82535080(ctx, base);
	// 82539C80: 9421FAD0  stwu r1, -0x530(r1)
	ea = ctx.r[1].u32.wrapping_add(-1328 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82539C84: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82539C88: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82539C8C: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82539C90: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82539C94: 7F79DB78  mr r25, r27
	ctx.r[25].u64 = ctx.r[27].u64;
	// 82539C98: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82539C9C: 7F70DB78  mr r16, r27
	ctx.r[16].u64 = ctx.r[27].u64;
	// 82539CA0: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82539CA4: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82539CA8: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82539CAC: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82539CB0: 409A0034  bne cr6, 0x82539ce4
	if !ctx.cr[6].eq {
	pc = 0x82539CE4; continue 'dispatch;
	}
	// 82539CB4: 48000C85  bl 0x8253a938
	ctx.lr = 0x82539CB8;
	sub_8253A938(ctx, base);
	// 82539CB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82539CBC: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 82539CC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82539CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82539CC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82539CCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82539CD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82539CD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82539CD8: 48000B29  bl 0x8253a800
	ctx.lr = 0x82539CDC;
	sub_8253A800(ctx, base);
	// 82539CDC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82539CE0: 48000B04  b 0x8253a7e4
	pc = 0x8253A7E4; continue 'dispatch;
	// 82539CE4: 8177000C  lwz r11, 0xc(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(12 as u32) ) } as u64;
	// 82539CE8: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539CEC: 408200C4  bne 0x82539db0
	if !ctx.cr[0].eq {
	pc = 0x82539DB0; continue 'dispatch;
	}
	// 82539CF0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539CF4: 48003ECD  bl 0x8253dbc0
	ctx.lr = 0x82539CF8;
	sub_8253DBC0(ctx, base);
	// 82539CF8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82539CFC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82539D00: 3BCB3CE0  addi r30, r11, 0x3ce0
	ctx.r[30].s64 = ctx.r[11].s64 + 15584;
	// 82539D04: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82539D08: 3BABF188  addi r29, r11, -0xe78
	ctx.r[29].s64 = ctx.r[11].s64 + -3704;
	// 82539D0C: 419A003C  beq cr6, 0x82539d48
	if ctx.cr[6].eq {
	pc = 0x82539D48; continue 'dispatch;
	}
	// 82539D10: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D14: 48003EAD  bl 0x8253dbc0
	ctx.lr = 0x82539D18;
	sub_8253DBC0(ctx, base);
	// 82539D18: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 82539D1C: 419A002C  beq cr6, 0x82539d48
	if ctx.cr[6].eq {
	pc = 0x82539D48; continue 'dispatch;
	}
	// 82539D20: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D24: 48003E9D  bl 0x8253dbc0
	ctx.lr = 0x82539D28;
	sub_8253DBC0(ctx, base);
	// 82539D28: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 82539D2C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D30: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82539D34: 48003E8D  bl 0x8253dbc0
	ctx.lr = 0x82539D38;
	sub_8253DBC0(ctx, base);
	// 82539D38: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82539D3C: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 82539D40: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82539D44: 48000008  b 0x82539d4c
	pc = 0x82539D4C; continue 'dispatch;
	// 82539D48: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82539D4C: 896B0028  lbz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82539D50: 556B003D  rlwinm. r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539D54: 4082FF60  bne 0x82539cb4
	if !ctx.cr[0].eq {
	pc = 0x82539CB4; continue 'dispatch;
	}
	// 82539D58: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D5C: 48003E65  bl 0x8253dbc0
	ctx.lr = 0x82539D60;
	sub_8253DBC0(ctx, base);
	// 82539D60: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82539D64: 419A003C  beq cr6, 0x82539da0
	if ctx.cr[6].eq {
	pc = 0x82539DA0; continue 'dispatch;
	}
	// 82539D68: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D6C: 48003E55  bl 0x8253dbc0
	ctx.lr = 0x82539D70;
	sub_8253DBC0(ctx, base);
	// 82539D70: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 82539D74: 419A002C  beq cr6, 0x82539da0
	if ctx.cr[6].eq {
	pc = 0x82539DA0; continue 'dispatch;
	}
	// 82539D78: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D7C: 48003E45  bl 0x8253dbc0
	ctx.lr = 0x82539D80;
	sub_8253DBC0(ctx, base);
	// 82539D80: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 82539D84: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82539D88: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82539D8C: 48003E35  bl 0x8253dbc0
	ctx.lr = 0x82539D90;
	sub_8253DBC0(ctx, base);
	// 82539D90: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82539D94: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 82539D98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82539D9C: 48000008  b 0x82539da4
	pc = 0x82539DA4; continue 'dispatch;
	// 82539DA0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82539DA4: 896B0028  lbz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82539DA8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82539DAC: 4082FF08  bne 0x82539cb4
	if !ctx.cr[0].eq {
	pc = 0x82539CB4; continue 'dispatch;
	}
	// 82539DB0: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 82539DB4: 419AFF00  beq cr6, 0x82539cb4
	if ctx.cr[6].eq {
	pc = 0x82539CB4; continue 'dispatch;
	}
	// 82539DB8: 4BFFCD71  bl 0x82536b28
	ctx.lr = 0x82539DBC;
	sub_82536B28(ctx, base);
	// 82539DBC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82539DC0: 7F175840  cmplw cr6, r23, r11
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82539DC4: 419A09EC  beq cr6, 0x8253a7b0
	if ctx.cr[6].eq {
	pc = 0x8253A7B0; continue 'dispatch;
	}
	// 82539DC8: 4BFFCD61  bl 0x82536b28
	ctx.lr = 0x82539DCC;
	sub_82536B28(ctx, base);
	// 82539DCC: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82539DD0: 7F175840  cmplw cr6, r23, r11
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82539DD4: 419A09DC  beq cr6, 0x8253a7b0
	if ctx.cr[6].eq {
	pc = 0x8253A7B0; continue 'dispatch;
	}
	// 82539DD8: 8BB30000  lbz r29, 0(r19)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82539DDC: 7F78DB78  mr r24, r27
	ctx.r[24].u64 = ctx.r[27].u64;
	// 82539DE0: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82539DE4: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82539DE8: 7F6FDB78  mr r15, r27
	ctx.r[15].u64 = ctx.r[27].u64;
	// 82539DEC: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82539DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82539DF4: 7FAA0775  extsb. r10, r29
	ctx.r[10].s64 = ctx.r[29].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82539DF8: 418209B0  beq 0x8253a7a8
	if ctx.cr[0].eq {
	pc = 0x8253A7A8; continue 'dispatch;
	}
	// 82539DFC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82539E00: 82810070  lwz r20, 0x70(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82539E04: 83810070  lwz r28, 0x70(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82539E08: 3DC08206  lis r14, -0x7dfa
	ctx.r[14].s64 = -2113536000;
	// 82539E0C: 3ACBEE80  addi r22, r11, -0x1180
	ctx.r[22].s64 = ctx.r[11].s64 + -4480;
	// 82539E10: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82539E14: 3E208206  lis r17, -0x7dfa
	ctx.r[17].s64 = -2113536000;
	// 82539E18: 3AABEFA4  addi r21, r11, -0x105c
	ctx.r[21].s64 = ctx.r[11].s64 + -4188;
	// 82539E1C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82539E20: 3A4B75D8  addi r18, r11, 0x75d8
	ctx.r[18].s64 = ctx.r[11].s64 + 30168;
	// 82539E24: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82539E28: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82539E2C: 4198097C  blt cr6, 0x8253a7a8
	if ctx.cr[6].lt {
	pc = 0x8253A7A8; continue 'dispatch;
	}
	// 82539E30: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82539E34: 4198001C  blt cr6, 0x82539e50
	if ctx.cr[6].lt {
	pc = 0x82539E50; continue 'dispatch;
	}
	// 82539E38: 2F0A0078  cmpwi cr6, r10, 0x78
	ctx.cr[6].compare_i32(ctx.r[10].s32, 120, &mut ctx.xer);
	// 82539E3C: 41990014  bgt cr6, 0x82539e50
	if ctx.cr[6].gt {
	pc = 0x82539E50; continue 'dispatch;
	}
	// 82539E40: 7D6A9214  add r11, r10, r18
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[18].u64;
	// 82539E44: 896BFFE0  lbz r11, -0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-32 as u32) ) } as u64;
	// 82539E48: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82539E4C: 48000008  b 0x82539e54
	pc = 0x82539E54; continue 'dispatch;
	// 82539E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82539E54: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82539E58: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82539E5C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82539E60: 7D6B90AE  lbzx r11, r11, r18
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82539E64: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82539E68: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82539E6C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82539E70: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82539E74: 41990920  bgt cr6, 0x8253a794
	if ctx.cr[6].gt {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 82539E78: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82539E7C: 3D808206  lis r12, -0x7dfa
	ctx.r[12].s64 = -2113536000;
	// 82539E80: 398C76A8  addi r12, r12, 0x76a8
	ctx.r[12].s64 = ctx.r[12].s64 + 30376;
	// 82539E84: 5560083C  slwi r0, r11, 1
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82539E88: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82539E8C: 3D808254  lis r12, -0x7dac
	ctx.r[12].s64 = -2108424192;
	// 82539E90: 398C9EA4  addi r12, r12, -0x615c
	ctx.r[12].s64 = ctx.r[12].s64 + -24924;
	// 82539E94: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82539E98: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82539E9C: 60000000  nop
	// 82539EA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82539EA4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82539EA8: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82539EAC: 7F74DB78  mr r20, r27
	ctx.r[20].u64 = ctx.r[27].u64;
	// 82539EB0: 7F70DB78  mr r16, r27
	ctx.r[16].u64 = ctx.r[27].u64;
	// 82539EB4: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82539EB8: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82539EBC: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82539EC0: 480008D4  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539EC4: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82539EC8: 419A0044  beq cr6, 0x82539f0c
	if ctx.cr[6].eq {
	pc = 0x82539F0C; continue 'dispatch;
	}
	// 82539ECC: 2F0A0023  cmpwi cr6, r10, 0x23
	ctx.cr[6].compare_i32(ctx.r[10].s32, 35, &mut ctx.xer);
	// 82539ED0: 419A0034  beq cr6, 0x82539f04
	if ctx.cr[6].eq {
	pc = 0x82539F04; continue 'dispatch;
	}
	// 82539ED4: 2F0A002B  cmpwi cr6, r10, 0x2b
	ctx.cr[6].compare_i32(ctx.r[10].s32, 43, &mut ctx.xer);
	// 82539ED8: 419A0024  beq cr6, 0x82539efc
	if ctx.cr[6].eq {
	pc = 0x82539EFC; continue 'dispatch;
	}
	// 82539EDC: 2F0A002D  cmpwi cr6, r10, 0x2d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 45, &mut ctx.xer);
	// 82539EE0: 419A0014  beq cr6, 0x82539ef4
	if ctx.cr[6].eq {
	pc = 0x82539EF4; continue 'dispatch;
	}
	// 82539EE4: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82539EE8: 409A08AC  bne cr6, 0x8253a794
	if !ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 82539EEC: 637B0008  ori r27, r27, 8
	ctx.r[27].u64 = ctx.r[27].u64 | 8;
	// 82539EF0: 480008A4  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539EF4: 637B0004  ori r27, r27, 4
	ctx.r[27].u64 = ctx.r[27].u64 | 4;
	// 82539EF8: 4800089C  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539EFC: 637B0001  ori r27, r27, 1
	ctx.r[27].u64 = ctx.r[27].u64 | 1;
	// 82539F00: 48000894  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539F04: 637B0080  ori r27, r27, 0x80
	ctx.r[27].u64 = ctx.r[27].u64 | 128;
	// 82539F08: 4800088C  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539F0C: 637B0002  ori r27, r27, 2
	ctx.r[27].u64 = ctx.r[27].u64 | 2;
	// 82539F10: 48000884  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539F14: 2F0A002A  cmpwi cr6, r10, 0x2a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 42, &mut ctx.xer);
	// 82539F18: 409A0030  bne cr6, 0x82539f48
	if !ctx.cr[6].eq {
	pc = 0x82539F48; continue 'dispatch;
	}
	// 82539F1C: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 82539F20: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82539F24: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 82539F28: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82539F2C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539F30: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82539F34: 40800860  bge 0x8253a794
	if !ctx.cr[0].lt {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 82539F38: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82539F3C: 637B0004  ori r27, r27, 4
	ctx.r[27].u64 = ctx.r[27].u64 | 4;
	// 82539F40: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 82539F44: 48000014  b 0x82539f58
	pc = 0x82539F58; continue 'dispatch;
	// 82539F48: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82539F4C: 1D6B000A  mulli r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 * 10;
	// 82539F50: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82539F54: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 82539F58: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82539F5C: 48000838  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539F60: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82539F64: 48000830  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539F68: 2F0A002A  cmpwi cr6, r10, 0x2a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 42, &mut ctx.xer);
	// 82539F6C: 409A0024  bne cr6, 0x82539f90
	if !ctx.cr[6].eq {
	pc = 0x82539F90; continue 'dispatch;
	}
	// 82539F70: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 82539F74: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82539F78: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 82539F7C: 833AFFFC  lwz r25, -4(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82539F80: 2C190000  cmpwi r25, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82539F84: 40800810  bge 0x8253a794
	if !ctx.cr[0].lt {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 82539F88: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82539F8C: 48000808  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539F90: 1D79000A  mulli r11, r25, 0xa
	ctx.r[11].s64 = ctx.r[25].s64 * 10;
	// 82539F94: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82539F98: 3B2BFFD0  addi r25, r11, -0x30
	ctx.r[25].s64 = ctx.r[11].s64 + -48;
	// 82539F9C: 480007F8  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539FA0: 2F0A0049  cmpwi cr6, r10, 0x49
	ctx.cr[6].compare_i32(ctx.r[10].s32, 73, &mut ctx.xer);
	// 82539FA4: 419A004C  beq cr6, 0x82539ff0
	if ctx.cr[6].eq {
	pc = 0x82539FF0; continue 'dispatch;
	}
	// 82539FA8: 2F0A0068  cmpwi cr6, r10, 0x68
	ctx.cr[6].compare_i32(ctx.r[10].s32, 104, &mut ctx.xer);
	// 82539FAC: 419A003C  beq cr6, 0x82539fe8
	if ctx.cr[6].eq {
	pc = 0x82539FE8; continue 'dispatch;
	}
	// 82539FB0: 2F0A006C  cmpwi cr6, r10, 0x6c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 108, &mut ctx.xer);
	// 82539FB4: 419A0014  beq cr6, 0x82539fc8
	if ctx.cr[6].eq {
	pc = 0x82539FC8; continue 'dispatch;
	}
	// 82539FB8: 2F0A0077  cmpwi cr6, r10, 0x77
	ctx.cr[6].compare_i32(ctx.r[10].s32, 119, &mut ctx.xer);
	// 82539FBC: 409A07D8  bne cr6, 0x8253a794
	if !ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 82539FC0: 637B0800  ori r27, r27, 0x800
	ctx.r[27].u64 = ctx.r[27].u64 | 2048;
	// 82539FC4: 480007D0  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539FC8: 89730000  lbz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82539FCC: 2B0B006C  cmplwi cr6, r11, 0x6c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 108 as u32, &mut ctx.xer);
	// 82539FD0: 409A0010  bne cr6, 0x82539fe0
	if !ctx.cr[6].eq {
	pc = 0x82539FE0; continue 'dispatch;
	}
	// 82539FD4: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82539FD8: 637B1000  ori r27, r27, 0x1000
	ctx.r[27].u64 = ctx.r[27].u64 | 4096;
	// 82539FDC: 480007B8  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539FE0: 637B0010  ori r27, r27, 0x10
	ctx.r[27].u64 = ctx.r[27].u64 | 16;
	// 82539FE4: 480007B0  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539FE8: 637B0020  ori r27, r27, 0x20
	ctx.r[27].u64 = ctx.r[27].u64 | 32;
	// 82539FEC: 480007A8  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 82539FF0: 89730000  lbz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82539FF4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82539FF8: 2F0B0036  cmpwi cr6, r11, 0x36
	ctx.cr[6].compare_i32(ctx.r[11].s32, 54, &mut ctx.xer);
	// 82539FFC: 409A001C  bne cr6, 0x8253a018
	if !ctx.cr[6].eq {
	pc = 0x8253A018; continue 'dispatch;
	}
	// 8253A000: 89530001  lbz r10, 1(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(1 as u32) ) } as u64;
	// 8253A004: 2B0A0034  cmplwi cr6, r10, 0x34
	ctx.cr[6].compare_u32(ctx.r[10].u32, 52 as u32, &mut ctx.xer);
	// 8253A008: 409A0010  bne cr6, 0x8253a018
	if !ctx.cr[6].eq {
	pc = 0x8253A018; continue 'dispatch;
	}
	// 8253A00C: 3A730002  addi r19, r19, 2
	ctx.r[19].s64 = ctx.r[19].s64 + 2;
	// 8253A010: 637B8000  ori r27, r27, 0x8000
	ctx.r[27].u64 = ctx.r[27].u64 | 32768;
	// 8253A014: 48000780  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 8253A018: 2F0B0033  cmpwi cr6, r11, 0x33
	ctx.cr[6].compare_i32(ctx.r[11].s32, 51, &mut ctx.xer);
	// 8253A01C: 409A001C  bne cr6, 0x8253a038
	if !ctx.cr[6].eq {
	pc = 0x8253A038; continue 'dispatch;
	}
	// 8253A020: 89530001  lbz r10, 1(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(1 as u32) ) } as u64;
	// 8253A024: 2B0A0032  cmplwi cr6, r10, 0x32
	ctx.cr[6].compare_u32(ctx.r[10].u32, 50 as u32, &mut ctx.xer);
	// 8253A028: 409A0010  bne cr6, 0x8253a038
	if !ctx.cr[6].eq {
	pc = 0x8253A038; continue 'dispatch;
	}
	// 8253A02C: 3A730002  addi r19, r19, 2
	ctx.r[19].s64 = ctx.r[19].s64 + 2;
	// 8253A030: 577B045E  rlwinm r27, r27, 0, 0x11, 0xf
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A034: 48000760  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 8253A038: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 8253A03C: 419A0758  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A040: 2F0B0069  cmpwi cr6, r11, 0x69
	ctx.cr[6].compare_i32(ctx.r[11].s32, 105, &mut ctx.xer);
	// 8253A044: 419A0750  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A048: 2F0B006F  cmpwi cr6, r11, 0x6f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 111, &mut ctx.xer);
	// 8253A04C: 419A0748  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A050: 2F0B0075  cmpwi cr6, r11, 0x75
	ctx.cr[6].compare_i32(ctx.r[11].s32, 117, &mut ctx.xer);
	// 8253A054: 419A0740  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A058: 2F0B0078  cmpwi cr6, r11, 0x78
	ctx.cr[6].compare_i32(ctx.r[11].s32, 120, &mut ctx.xer);
	// 8253A05C: 419A0738  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A060: 2F0B0058  cmpwi cr6, r11, 0x58
	ctx.cr[6].compare_i32(ctx.r[11].s32, 88, &mut ctx.xer);
	// 8253A064: 419A0730  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253A06C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8253A070: 57A3063E  clrlwi r3, r29, 0x18
	ctx.r[3].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8253A074: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8253A078: 3A000000  li r16, 0
	ctx.r[16].s64 = 0;
	// 8253A07C: 480089BD  bl 0x82542a38
	ctx.lr = 0x8253A080;
	sub_82542A38(ctx, base);
	// 8253A080: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A084: 41820024  beq 0x8253a0a8
	if ctx.cr[0].eq {
	pc = 0x8253A0A8; continue 'dispatch;
	}
	// 8253A088: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8253A08C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8253A090: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8253A094: 48005325  bl 0x8253f3b8
	ctx.lr = 0x8253A098;
	sub_8253F3B8(ctx, base);
	// 8253A098: 8BB30000  lbz r29, 0(r19)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A09C: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 8253A0A0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A0A4: 4182FC10  beq 0x82539cb4
	if ctx.cr[0].eq {
	pc = 0x82539CB4; continue 'dispatch;
	}
	// 8253A0A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8253A0AC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8253A0B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8253A0B4: 48005305  bl 0x8253f3b8
	ctx.lr = 0x8253A0B8;
	sub_8253F3B8(ctx, base);
	// 8253A0B8: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253A0BC: 480006D8  b 0x8253a794
	pc = 0x8253A794; continue 'dispatch;
	// 8253A0C0: 396AFFBF  addi r11, r10, -0x41
	ctx.r[11].s64 = ctx.r[10].s64 + -65;
	// 8253A0C4: 2B0B0037  cmplwi cr6, r11, 0x37
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55 as u32, &mut ctx.xer);
	// 8253A0C8: 419904F8  bgt cr6, 0x8253a5c0
	if ctx.cr[6].gt {
	pc = 0x8253A5C0; continue 'dispatch;
	}
	// 8253A0CC: 3D808206  lis r12, -0x7dfa
	ctx.r[12].s64 = -2113536000;
	// 8253A0D0: 398C7638  addi r12, r12, 0x7638
	ctx.r[12].s64 = ctx.r[12].s64 + 30264;
	// 8253A0D4: 5560083C  slwi r0, r11, 1
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8253A0D8: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8253A0DC: 3D808254  lis r12, -0x7dac
	ctx.r[12].s64 = -2108424192;
	// 8253A0E0: 398CA0F4  addi r12, r12, -0x5f0c
	ctx.r[12].s64 = ctx.r[12].s64 + -24332;
	// 8253A0E4: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 8253A0E8: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 8253A0EC: 60000000  nop
	// 8253A0F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 8253A0F4: 736B0830  andi. r11, r27, 0x830
	ctx.r[11].u64 = ctx.r[27].u64 & 2096;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A0F8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A0FC: 40820008  bne 0x8253a104
	if !ctx.cr[0].eq {
	pc = 0x8253A104; continue 'dispatch;
	}
	// 8253A100: 637B0800  ori r27, r27, 0x800
	ctx.r[27].u64 = ctx.r[27].u64 | 2048;
	// 8253A104: 736B0810  andi. r11, r27, 0x810
	ctx.r[11].u64 = ctx.r[27].u64 & 2064;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A108: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A10C: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A110: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A114: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A118: 4182002C  beq 0x8253a144
	if ctx.cr[0].eq {
	pc = 0x8253A144; continue 'dispatch;
	}
	// 8253A11C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8253A120: A0DAFFFE  lhz r6, -2(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(-2 as u32) ) } as u64;
	// 8253A124: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8253A128: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8253A12C: 48008905  bl 0x82542a30
	ctx.lr = 0x8253A130;
	sub_82542A30(ctx, base);
	// 8253A130: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A134: 41820020  beq 0x8253a154
	if ctx.cr[0].eq {
	pc = 0x8253A154; continue 'dispatch;
	}
	// 8253A138: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8253A13C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8253A140: 48000014  b 0x8253a154
	pc = 0x8253A154; continue 'dispatch;
	// 8253A144: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253A148: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8253A14C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8253A150: 99610090  stb r11, 0x90(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u8 ) };
	// 8253A154: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253A158: 3B810090  addi r28, r1, 0x90
	ctx.r[28].s64 = ctx.r[1].s64 + 144;
	// 8253A15C: 48000464  b 0x8253a5c0
	pc = 0x8253A5C0; continue 'dispatch;
	// 8253A160: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A164: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A168: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A16C: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253A170: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A174: 4182003C  beq 0x8253a1b0
	if ctx.cr[0].eq {
	pc = 0x8253A1B0; continue 'dispatch;
	}
	// 8253A178: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253A17C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A180: 41820030  beq 0x8253a1b0
	if ctx.cr[0].eq {
	pc = 0x8253A1B0; continue 'dispatch;
	}
	// 8253A184: 57690529  rlwinm. r9, r27, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253A188: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 8253A18C: 41820018  beq 0x8253a1a4
	if ctx.cr[0].eq {
	pc = 0x8253A1A4; continue 'dispatch;
	}
	// 8253A190: A96B0000  lha r11, 0(r11)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8253A194: 3A000001  li r16, 1
	ctx.r[16].s64 = 1;
	// 8253A198: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8253A19C: 7D0B0194  addze r8, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[8].s64 = tmp.s64;
	// 8253A1A0: 4800041C  b 0x8253a5bc
	pc = 0x8253A5BC; continue 'dispatch;
	// 8253A1A4: A90B0000  lha r8, 0(r11)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8253A1A8: 3A000000  li r16, 0
	ctx.r[16].s64 = 0;
	// 8253A1AC: 48000410  b 0x8253a5bc
	pc = 0x8253A5BC; continue 'dispatch;
	// 8253A1B0: 839175CC  lwz r28, 0x75cc(r17)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(30156 as u32) ) } as u64;
	// 8253A1B4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8253A1B8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253A1BC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A1C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253A1C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253A1C8: 409AFFF4  bne cr6, 0x8253a1bc
	if !ctx.cr[6].eq {
	pc = 0x8253A1BC; continue 'dispatch;
	}
	// 8253A1CC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8253A1D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253A1D4: 5568003E  slwi r8, r11, 0
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8253A1D8: 480003E4  b 0x8253a5bc
	pc = 0x8253A5BC; continue 'dispatch;
	// 8253A1DC: 736B0830  andi. r11, r27, 0x830
	ctx.r[11].u64 = ctx.r[27].u64 & 2096;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A1E0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A1E4: 40820008  bne 0x8253a1ec
	if !ctx.cr[0].eq {
	pc = 0x8253A1EC; continue 'dispatch;
	}
	// 8253A1E8: 637B0800  ori r27, r27, 0x800
	ctx.r[27].u64 = ctx.r[27].u64 | 2048;
	// 8253A1EC: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 8253A1F0: 409A0010  bne cr6, 0x8253a200
	if !ctx.cr[6].eq {
	pc = 0x8253A200; continue 'dispatch;
	}
	// 8253A1F4: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8253A1F8: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8253A1FC: 48000008  b 0x8253a204
	pc = 0x8253A204; continue 'dispatch;
	// 8253A200: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 8253A204: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A208: 73690810  andi. r9, r27, 0x810
	ctx.r[9].u64 = ctx.r[27].u64 & 2064;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253A20C: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A210: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A214: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A218: 839AFFFC  lwz r28, -4(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253A21C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8253A220: 41820040  beq 0x8253a260
	if ctx.cr[0].eq {
	pc = 0x8253A260; continue 'dispatch;
	}
	// 8253A224: 409A0008  bne cr6, 0x8253a22c
	if !ctx.cr[6].eq {
	pc = 0x8253A22C; continue 'dispatch;
	}
	// 8253A228: 838E75D0  lwz r28, 0x75d0(r14)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(30160 as u32) ) } as u64;
	// 8253A22C: 3A000001  li r16, 1
	ctx.r[16].s64 = 1;
	// 8253A230: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8253A234: 48000018  b 0x8253a24c
	pc = 0x8253A24C; continue 'dispatch;
	// 8253A238: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A23C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8253A240: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A244: 41820010  beq 0x8253a254
	if ctx.cr[0].eq {
	pc = 0x8253A254; continue 'dispatch;
	}
	// 8253A248: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8253A24C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A250: 409AFFE8  bne cr6, 0x8253a238
	if !ctx.cr[6].eq {
	pc = 0x8253A238; continue 'dispatch;
	}
	// 8253A254: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8253A258: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8253A25C: 48000360  b 0x8253a5bc
	pc = 0x8253A5BC; continue 'dispatch;
	// 8253A260: 409A0008  bne cr6, 0x8253a268
	if !ctx.cr[6].eq {
	pc = 0x8253A268; continue 'dispatch;
	}
	// 8253A264: 839175CC  lwz r28, 0x75cc(r17)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(30156 as u32) ) } as u64;
	// 8253A268: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8253A26C: 48000018  b 0x8253a284
	pc = 0x8253A284; continue 'dispatch;
	// 8253A270: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A274: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8253A278: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253A27C: 419A0010  beq cr6, 0x8253a28c
	if ctx.cr[6].eq {
	pc = 0x8253A28C; continue 'dispatch;
	}
	// 8253A280: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253A284: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A288: 409AFFE8  bne cr6, 0x8253a270
	if !ctx.cr[6].eq {
	pc = 0x8253A270; continue 'dispatch;
	}
	// 8253A28C: 7D1C5850  subf r8, r28, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8253A290: 4800032C  b 0x8253a5bc
	pc = 0x8253A5BC; continue 'dispatch;
	// 8253A294: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A298: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A29C: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A2A0: 83FAFFFC  lwz r31, -4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253A2A4: 4BFFA125  bl 0x825343c8
	ctx.lr = 0x8253A2A8;
	sub_825343C8(ctx, base);
	// 8253A2A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A2AC: 4182FA08  beq 0x82539cb4
	if ctx.cr[0].eq {
	pc = 0x82539CB4; continue 'dispatch;
	}
	// 8253A2B0: 576B06B5  rlwinm. r11, r27, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A2B4: 4182000C  beq 0x8253a2c0
	if ctx.cr[0].eq {
	pc = 0x8253A2C0; continue 'dispatch;
	}
	// 8253A2B8: B31F0000  sth r24, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 8253A2BC: 48000008  b 0x8253a2c4
	pc = 0x8253A2C4; continue 'dispatch;
	// 8253A2C0: 931F0000  stw r24, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 8253A2C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8253A2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8253A2CC: 480004B4  b 0x8253a780
	pc = 0x8253A780; continue 'dispatch;
	// 8253A2D0: 396A0020  addi r11, r10, 0x20
	ctx.r[11].s64 = ctx.r[10].s64 + 32;
	// 8253A2D4: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 8253A2D8: 7D7D0774  extsb r29, r11
	ctx.r[29].s64 = ctx.r[11].s8 as i64;
	// 8253A2DC: 637B0040  ori r27, r27, 0x40
	ctx.r[27].u64 = ctx.r[27].u64 | 64;
	// 8253A2E0: 3B810090  addi r28, r1, 0x90
	ctx.r[28].s64 = ctx.r[1].s64 + 144;
	// 8253A2E4: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 8253A2E8: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253A2EC: 4098000C  bge cr6, 0x8253a2f8
	if !ctx.cr[6].lt {
	pc = 0x8253A2F8; continue 'dispatch;
	}
	// 8253A2F0: 3B200006  li r25, 6
	ctx.r[25].s64 = 6;
	// 8253A2F4: 48000054  b 0x8253a348
	pc = 0x8253A348; continue 'dispatch;
	// 8253A2F8: 409A0018  bne cr6, 0x8253a310
	if !ctx.cr[6].eq {
	pc = 0x8253A310; continue 'dispatch;
	}
	// 8253A2FC: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 8253A300: 2F0B0067  cmpwi cr6, r11, 0x67
	ctx.cr[6].compare_i32(ctx.r[11].s32, 103, &mut ctx.xer);
	// 8253A304: 409A0044  bne cr6, 0x8253a348
	if !ctx.cr[6].eq {
	pc = 0x8253A348; continue 'dispatch;
	}
	// 8253A308: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8253A30C: 4800003C  b 0x8253a348
	pc = 0x8253A348; continue 'dispatch;
	// 8253A310: 2F190200  cmpwi cr6, r25, 0x200
	ctx.cr[6].compare_i32(ctx.r[25].s32, 512, &mut ctx.xer);
	// 8253A314: 40990008  ble cr6, 0x8253a31c
	if !ctx.cr[6].gt {
	pc = 0x8253A31C; continue 'dispatch;
	}
	// 8253A318: 3B200200  li r25, 0x200
	ctx.r[25].s64 = 512;
	// 8253A31C: 2F1900A3  cmpwi cr6, r25, 0xa3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 163, &mut ctx.xer);
	// 8253A320: 40990028  ble cr6, 0x8253a348
	if !ctx.cr[6].gt {
	pc = 0x8253A348; continue 'dispatch;
	}
	// 8253A324: 3BF9015D  addi r31, r25, 0x15d
	ctx.r[31].s64 = ctx.r[25].s64 + 349;
	// 8253A328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253A32C: 4BFF9945  bl 0x82533c70
	ctx.lr = 0x8253A330;
	sub_82533C70(ctx, base);
	// 8253A330: 7C6F1B79  or. r15, r3, r3
	ctx.r[15].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[15].s32, 0, &mut ctx.xer);
	// 8253A334: 41820010  beq 0x8253a344
	if ctx.cr[0].eq {
	pc = 0x8253A344; continue 'dispatch;
	}
	// 8253A338: 7DFC7B78  mr r28, r15
	ctx.r[28].u64 = ctx.r[15].u64;
	// 8253A33C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8253A340: 48000008  b 0x8253a348
	pc = 0x8253A348; continue 'dispatch;
	// 8253A344: 3B2000A3  li r25, 0xa3
	ctx.r[25].s64 = 163;
	// 8253A348: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A34C: 7FBF0774  extsb r31, r29
	ctx.r[31].s64 = ctx.r[29].s8 as i64;
	// 8253A350: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A354: 7E88A378  mr r8, r20
	ctx.r[8].u64 = ctx.r[20].u64;
	// 8253A358: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A35C: 81750018  lwz r11, 0x18(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(24 as u32) ) } as u64;
	// 8253A360: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 8253A364: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8253A368: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8253A36C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8253A370: E95AFFF8  ld r10, -8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[26].u32.wrapping_add(-8 as u32) ) };
	// 8253A374: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8253A378: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8253A37C: F9410080  std r10, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u64 ) };
	// 8253A380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8253A384: 4E800421  bctrl
	ctx.lr = 0x8253A388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253A388: 577E0631  rlwinm. r30, r27, 0, 0x18, 0x18
	ctx.r[30].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8253A38C: 41820020  beq 0x8253a3ac
	if ctx.cr[0].eq {
	pc = 0x8253A3AC; continue 'dispatch;
	}
	// 8253A390: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253A394: 409A0018  bne cr6, 0x8253a3ac
	if !ctx.cr[6].eq {
	pc = 0x8253A3AC; continue 'dispatch;
	}
	// 8253A398: 81750024  lwz r11, 0x24(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(36 as u32) ) } as u64;
	// 8253A39C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8253A3A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8253A3A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8253A3A8: 4E800421  bctrl
	ctx.lr = 0x8253A3AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253A3AC: 2F1F0067  cmpwi cr6, r31, 0x67
	ctx.cr[6].compare_i32(ctx.r[31].s32, 103, &mut ctx.xer);
	// 8253A3B0: 409A0020  bne cr6, 0x8253a3d0
	if !ctx.cr[6].eq {
	pc = 0x8253A3D0; continue 'dispatch;
	}
	// 8253A3B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8253A3B8: 409A0018  bne cr6, 0x8253a3d0
	if !ctx.cr[6].eq {
	pc = 0x8253A3D0; continue 'dispatch;
	}
	// 8253A3BC: 81750020  lwz r11, 0x20(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(32 as u32) ) } as u64;
	// 8253A3C0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8253A3C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8253A3C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8253A3CC: 4E800421  bctrl
	ctx.lr = 0x8253A3D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253A3D0: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A3D4: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 8253A3D8: 409A000C  bne cr6, 0x8253a3e4
	if !ctx.cr[6].eq {
	pc = 0x8253A3E4; continue 'dispatch;
	}
	// 8253A3DC: 637B0100  ori r27, r27, 0x100
	ctx.r[27].u64 = ctx.r[27].u64 | 256;
	// 8253A3E0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8253A3E4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8253A3E8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253A3EC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A3F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253A3F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253A3F8: 409AFFF4  bne cr6, 0x8253a3ec
	if !ctx.cr[6].eq {
	pc = 0x8253A3EC; continue 'dispatch;
	}
	// 8253A3FC: 4BFFFDD0  b 0x8253a1cc
	pc = 0x8253A1CC; continue 'dispatch;
	// 8253A400: 637B0040  ori r27, r27, 0x40
	ctx.r[27].u64 = ctx.r[27].u64 | 64;
	// 8253A404: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8253A408: 48000050  b 0x8253a458
	pc = 0x8253A458; continue 'dispatch;
	// 8253A40C: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 8253A410: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8253A414: 48000008  b 0x8253a41c
	pc = 0x8253A41C; continue 'dispatch;
	// 8253A418: 39600027  li r11, 0x27
	ctx.r[11].s64 = 39;
	// 8253A41C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8253A420: 576A0631  rlwinm. r10, r27, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A424: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8253A428: 41820030  beq 0x8253a458
	if ctx.cr[0].eq {
	pc = 0x8253A458; continue 'dispatch;
	}
	// 8253A42C: 396B0051  addi r11, r11, 0x51
	ctx.r[11].s64 = ctx.r[11].s64 + 81;
	// 8253A430: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 8253A434: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 8253A438: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8253A43C: 99410058  stb r10, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u8 ) };
	// 8253A440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8253A444: 48000014  b 0x8253a458
	pc = 0x8253A458; continue 'dispatch;
	// 8253A448: 576B0631  rlwinm. r11, r27, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A44C: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8253A450: 41820008  beq 0x8253a458
	if ctx.cr[0].eq {
	pc = 0x8253A458; continue 'dispatch;
	}
	// 8253A454: 637B0200  ori r27, r27, 0x200
	ctx.r[27].u64 = ctx.r[27].u64 | 512;
	// 8253A458: 576B0421  rlwinm. r11, r27, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A45C: 4082000C  bne 0x8253a468
	if !ctx.cr[0].eq {
	pc = 0x8253A468; continue 'dispatch;
	}
	// 8253A460: 576B04E7  rlwinm. r11, r27, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A464: 41820018  beq 0x8253a47c
	if ctx.cr[0].eq {
	pc = 0x8253A47C; continue 'dispatch;
	}
	// 8253A468: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A46C: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A470: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A474: E97AFFF8  ld r11, -8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[26].u32.wrapping_add(-8 as u32) ) };
	// 8253A478: 48000054  b 0x8253a4cc
	pc = 0x8253A4CC; continue 'dispatch;
	// 8253A47C: 576B06B5  rlwinm. r11, r27, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A480: 4182002C  beq 0x8253a4ac
	if ctx.cr[0].eq {
	pc = 0x8253A4AC; continue 'dispatch;
	}
	// 8253A484: 576B0673  rlwinm. r11, r27, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A488: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A48C: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A490: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A494: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253A498: 4182000C  beq 0x8253a4a4
	if ctx.cr[0].eq {
	pc = 0x8253A4A4; continue 'dispatch;
	}
	// 8253A49C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8253A4A0: 4800002C  b 0x8253a4cc
	pc = 0x8253A4CC; continue 'dispatch;
	// 8253A4A4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253A4A8: 48000024  b 0x8253a4cc
	pc = 0x8253A4CC; continue 'dispatch;
	// 8253A4AC: 576B0673  rlwinm. r11, r27, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A4B0: 397A0007  addi r11, r26, 7
	ctx.r[11].s64 = ctx.r[26].s64 + 7;
	// 8253A4B4: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A4B8: 3B4B0008  addi r26, r11, 8
	ctx.r[26].s64 = ctx.r[11].s64 + 8;
	// 8253A4BC: 4182000C  beq 0x8253a4c8
	if ctx.cr[0].eq {
	pc = 0x8253A4C8; continue 'dispatch;
	}
	// 8253A4C0: E97AFFFE  lwa r11, -4(r26)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as i32) as i64;
	// 8253A4C4: 48000008  b 0x8253a4cc
	pc = 0x8253A4CC; continue 'dispatch;
	// 8253A4C8: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253A4CC: 576A0673  rlwinm. r10, r27, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A4D0: 41820014  beq 0x8253a4e4
	if ctx.cr[0].eq {
	pc = 0x8253A4E4; continue 'dispatch;
	}
	// 8253A4D4: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8253A4D8: 4098000C  bge cr6, 0x8253a4e4
	if !ctx.cr[6].lt {
	pc = 0x8253A4E4; continue 'dispatch;
	}
	// 8253A4DC: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8253A4E0: 637B0100  ori r27, r27, 0x100
	ctx.r[27].u64 = ctx.r[27].u64 | 256;
	// 8253A4E4: 576A0421  rlwinm. r10, r27, 0, 0x10, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A4E8: 40820010  bne 0x8253a4f8
	if !ctx.cr[0].eq {
	pc = 0x8253A4F8; continue 'dispatch;
	}
	// 8253A4EC: 576A04E7  rlwinm. r10, r27, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A4F0: 40820008  bne 0x8253a4f8
	if !ctx.cr[0].eq {
	pc = 0x8253A4F8; continue 'dispatch;
	}
	// 8253A4F4: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8253A4F8: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253A4FC: 4098000C  bge cr6, 0x8253a508
	if !ctx.cr[6].lt {
	pc = 0x8253A508; continue 'dispatch;
	}
	// 8253A500: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8253A504: 48000014  b 0x8253a518
	pc = 0x8253A518; continue 'dispatch;
	// 8253A508: 577B0776  rlwinm r27, r27, 0, 0x1d, 0x1b
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A50C: 2F190200  cmpwi cr6, r25, 0x200
	ctx.cr[6].compare_i32(ctx.r[25].s32, 512, &mut ctx.xer);
	// 8253A510: 40990008  ble cr6, 0x8253a518
	if !ctx.cr[6].gt {
	pc = 0x8253A518; continue 'dispatch;
	}
	// 8253A514: 3B200200  li r25, 0x200
	ctx.r[25].s64 = 512;
	// 8253A518: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8253A51C: 409A000C  bne cr6, 0x8253a528
	if !ctx.cr[6].eq {
	pc = 0x8253A528; continue 'dispatch;
	}
	// 8253A520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8253A524: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8253A528: 3921028F  addi r9, r1, 0x28f
	ctx.r[9].s64 = ctx.r[1].s64 + 655;
	// 8253A52C: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253A530: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 8253A534: 4199000C  bgt cr6, 0x8253a540
	if ctx.cr[6].gt {
	pc = 0x8253A540; continue 'dispatch;
	}
	// 8253A538: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8253A53C: 419A0044  beq cr6, 0x8253a580
	if ctx.cr[6].eq {
	pc = 0x8253A580; continue 'dispatch;
	}
	// 8253A540: 7D0A07B4  extsw r10, r8
	ctx.r[10].s64 = ctx.r[8].s32 as i64;
	// 8253A544: 7CEB5392  divdu r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 / ctx.r[10].u64;
	// 8253A548: 08CA0000  tdi 6, r10, 0
	// tdi: trap doubleword immediate — TODO: implement trap semantics
	// 8253A54C: 7CE751D2  mulld r7, r7, r10
	ctx.r[7].s64 = ctx.r[7].s64 * ctx.r[10].s64;
	// 8253A550: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 8253A554: 7D6B5392  divdu r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 / ctx.r[10].u64;
	// 8253A558: 08CA0000  tdi 6, r10, 0
	// tdi: trap doubleword immediate — TODO: implement trap semantics
	// 8253A55C: 54EA003E  slwi r10, r7, 0
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253A560: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 8253A564: 2F0A0039  cmpwi cr6, r10, 0x39
	ctx.cr[6].compare_i32(ctx.r[10].s32, 57, &mut ctx.xer);
	// 8253A568: 4099000C  ble cr6, 0x8253a574
	if !ctx.cr[6].gt {
	pc = 0x8253A574; continue 'dispatch;
	}
	// 8253A56C: 80E1006C  lwz r7, 0x6c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8253A570: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 8253A574: 99490000  stb r10, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253A578: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8253A57C: 4BFFFFB0  b 0x8253a52c
	pc = 0x8253A52C; continue 'dispatch;
	// 8253A580: 3961028F  addi r11, r1, 0x28f
	ctx.r[11].s64 = ctx.r[1].s64 + 655;
	// 8253A584: 576A05AD  rlwinm. r10, r27, 0, 0x16, 0x16
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A588: 7D095850  subf r8, r9, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8253A58C: 3B890001  addi r28, r9, 1
	ctx.r[28].s64 = ctx.r[9].s64 + 1;
	// 8253A590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8253A594: 4182002C  beq 0x8253a5c0
	if ctx.cr[0].eq {
	pc = 0x8253A5C0; continue 'dispatch;
	}
	// 8253A598: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8253A59C: 419A0010  beq cr6, 0x8253a5ac
	if ctx.cr[6].eq {
	pc = 0x8253A5AC; continue 'dispatch;
	}
	// 8253A5A0: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A5A4: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8253A5A8: 419A0018  beq cr6, 0x8253a5c0
	if ctx.cr[6].eq {
	pc = 0x8253A5C0; continue 'dispatch;
	}
	// 8253A5AC: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8253A5B0: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8253A5B4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253A5B8: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253A5BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8253A5C0: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8253A5C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A5C8: 409A01B8  bne cr6, 0x8253a780
	if !ctx.cr[6].eq {
	pc = 0x8253A780; continue 'dispatch;
	}
	// 8253A5CC: 576B0673  rlwinm. r11, r27, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A5D0: 41820040  beq 0x8253a610
	if ctx.cr[0].eq {
	pc = 0x8253A610; continue 'dispatch;
	}
	// 8253A5D4: 576B05EF  rlwinm. r11, r27, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A5D8: 4182000C  beq 0x8253a5e4
	if ctx.cr[0].eq {
	pc = 0x8253A5E4; continue 'dispatch;
	}
	// 8253A5DC: 3960002D  li r11, 0x2d
	ctx.r[11].s64 = 45;
	// 8253A5E0: 48000010  b 0x8253a5f0
	pc = 0x8253A5F0; continue 'dispatch;
	// 8253A5E4: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A5E8: 41820018  beq 0x8253a600
	if ctx.cr[0].eq {
	pc = 0x8253A600; continue 'dispatch;
	}
	// 8253A5EC: 3960002B  li r11, 0x2b
	ctx.r[11].s64 = 43;
	// 8253A5F0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8253A5F4: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 8253A5F8: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8253A5FC: 48000018  b 0x8253a614
	pc = 0x8253A614; continue 'dispatch;
	// 8253A600: 576B07BD  rlwinm. r11, r27, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A604: 4182000C  beq 0x8253a610
	if ctx.cr[0].eq {
	pc = 0x8253A610; continue 'dispatch;
	}
	// 8253A608: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8253A60C: 4BFFFFE4  b 0x8253a5f0
	pc = 0x8253A5F0; continue 'dispatch;
	// 8253A610: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8253A614: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253A618: 576A073B  rlwinm. r10, r27, 0, 0x1c, 0x1d
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A61C: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8253A620: 7FBE5850  subf r29, r30, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8253A624: 40820038  bne 0x8253a65c
	if !ctx.cr[0].eq {
	pc = 0x8253A65C; continue 'dispatch;
	}
	// 8253A628: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8253A62C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8253A630: 4099002C  ble cr6, 0x8253a65c
	if !ctx.cr[6].gt {
	pc = 0x8253A65C; continue 'dispatch;
	}
	// 8253A634: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8253A638: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8253A63C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8253A640: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8253A644: 48004D75  bl 0x8253f3b8
	ctx.lr = 0x8253A648;
	sub_8253F3B8(ctx, base);
	// 8253A648: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253A64C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8253A650: 419A000C  beq cr6, 0x8253a65c
	if ctx.cr[6].eq {
	pc = 0x8253A65C; continue 'dispatch;
	}
	// 8253A654: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253A658: 4199FFDC  bgt cr6, 0x8253a634
	if ctx.cr[6].gt {
	pc = 0x8253A634; continue 'dispatch;
	}
	// 8253A65C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253A660: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8253A664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253A668: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8253A66C: 48004DE5  bl 0x8253f450
	ctx.lr = 0x8253A670;
	sub_8253F450(ctx, base);
	// 8253A670: 576B0739  rlwinm. r11, r27, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A674: 41820040  beq 0x8253a6b4
	if ctx.cr[0].eq {
	pc = 0x8253A6B4; continue 'dispatch;
	}
	// 8253A678: 576B077B  rlwinm. r11, r27, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A67C: 40820038  bne 0x8253a6b4
	if !ctx.cr[0].eq {
	pc = 0x8253A6B4; continue 'dispatch;
	}
	// 8253A680: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8253A684: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8253A688: 4099002C  ble cr6, 0x8253a6b4
	if !ctx.cr[6].gt {
	pc = 0x8253A6B4; continue 'dispatch;
	}
	// 8253A68C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8253A690: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8253A694: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8253A698: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8253A69C: 48004D1D  bl 0x8253f3b8
	ctx.lr = 0x8253A6A0;
	sub_8253F3B8(ctx, base);
	// 8253A6A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253A6A4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8253A6A8: 419A000C  beq cr6, 0x8253a6b4
	if ctx.cr[6].eq {
	pc = 0x8253A6B4; continue 'dispatch;
	}
	// 8253A6AC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253A6B0: 4199FFDC  bgt cr6, 0x8253a68c
	if ctx.cr[6].gt {
	pc = 0x8253A68C; continue 'dispatch;
	}
	// 8253A6B4: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253A6B8: 2F100000  cmpwi cr6, r16, 0
	ctx.cr[6].compare_i32(ctx.r[16].s32, 0, &mut ctx.xer);
	// 8253A6BC: 419A006C  beq cr6, 0x8253a728
	if ctx.cr[6].eq {
	pc = 0x8253A728; continue 'dispatch;
	}
	// 8253A6C0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8253A6C4: 40990064  ble cr6, 0x8253a728
	if !ctx.cr[6].gt {
	pc = 0x8253A728; continue 'dispatch;
	}
	// 8253A6C8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8253A6CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253A6D0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8253A6D4: A0DE0000  lhz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A6D8: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 8253A6DC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8253A6E0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8253A6E4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8253A6E8: 48008349  bl 0x82542a30
	ctx.lr = 0x8253A6EC;
	sub_82542A30(ctx, base);
	// 8253A6EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253A6F0: 4082002C  bne 0x8253a71c
	if !ctx.cr[0].eq {
	pc = 0x8253A71C; continue 'dispatch;
	}
	// 8253A6F4: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8253A6F8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8253A6FC: 419A0020  beq cr6, 0x8253a71c
	if ctx.cr[6].eq {
	pc = 0x8253A71C; continue 'dispatch;
	}
	// 8253A700: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253A704: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8253A708: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8253A70C: 48004D45  bl 0x8253f450
	ctx.lr = 0x8253A710;
	sub_8253F450(ctx, base);
	// 8253A710: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253A714: 409AFFBC  bne cr6, 0x8253a6d0
	if !ctx.cr[6].eq {
	pc = 0x8253A6D0; continue 'dispatch;
	}
	// 8253A718: 48000020  b 0x8253a738
	pc = 0x8253A738; continue 'dispatch;
	// 8253A71C: 3B00FFFF  li r24, -1
	ctx.r[24].s64 = -1;
	// 8253A720: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 8253A724: 48000018  b 0x8253a73c
	pc = 0x8253A73C; continue 'dispatch;
	// 8253A728: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253A72C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8253A730: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8253A734: 48004D1D  bl 0x8253f450
	ctx.lr = 0x8253A738;
	sub_8253F450(ctx, base);
	// 8253A738: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253A73C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8253A740: 41980040  blt cr6, 0x8253a780
	if ctx.cr[6].lt {
	pc = 0x8253A780; continue 'dispatch;
	}
	// 8253A744: 576B077B  rlwinm. r11, r27, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253A748: 41820038  beq 0x8253a780
	if ctx.cr[0].eq {
	pc = 0x8253A780; continue 'dispatch;
	}
	// 8253A74C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8253A750: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8253A754: 4099002C  ble cr6, 0x8253a780
	if !ctx.cr[6].gt {
	pc = 0x8253A780; continue 'dispatch;
	}
	// 8253A758: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8253A75C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8253A760: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8253A764: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8253A768: 48004C51  bl 0x8253f3b8
	ctx.lr = 0x8253A76C;
	sub_8253F3B8(ctx, base);
	// 8253A76C: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253A770: 2F18FFFF  cmpwi cr6, r24, -1
	ctx.cr[6].compare_i32(ctx.r[24].s32, -1, &mut ctx.xer);
	// 8253A774: 419A000C  beq cr6, 0x8253a780
	if ctx.cr[6].eq {
	pc = 0x8253A780; continue 'dispatch;
	}
	// 8253A778: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253A77C: 4199FFDC  bgt cr6, 0x8253a758
	if ctx.cr[6].gt {
	pc = 0x8253A758; continue 'dispatch;
	}
	// 8253A780: 2B0F0000  cmplwi cr6, r15, 0
	ctx.cr[6].compare_u32(ctx.r[15].u32, 0 as u32, &mut ctx.xer);
	// 8253A784: 419A0010  beq cr6, 0x8253a794
	if ctx.cr[6].eq {
	pc = 0x8253A794; continue 'dispatch;
	}
	// 8253A788: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 8253A78C: 4BFF95AD  bl 0x82533d38
	ctx.lr = 0x8253A790;
	sub_82533D38(ctx, base);
	// 8253A790: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 8253A794: 8BB30000  lbz r29, 0(r19)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A798: 7FAA0775  extsb. r10, r29
	ctx.r[10].s64 = ctx.r[29].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253A79C: 4182000C  beq 0x8253a7a8
	if ctx.cr[0].eq {
	pc = 0x8253A7A8; continue 'dispatch;
	}
	// 8253A7A0: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253A7A4: 4BFFF680  b 0x82539e24
	pc = 0x82539E24; continue 'dispatch;
	// 8253A7A8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8253A7AC: 48000038  b 0x8253a7e4
	pc = 0x8253A7E4; continue 'dispatch;
	// 8253A7B0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8253A7B4: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 8253A7B8: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 8253A7BC: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 8253A7C0: 4BFFD759  bl 0x82537f18
	ctx.lr = 0x8253A7C4;
	sub_82537F18(ctx, base);
	// 8253A7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253A7C8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8253A7CC: 409A000C  bne cr6, 0x8253a7d8
	if !ctx.cr[6].eq {
	pc = 0x8253A7D8; continue 'dispatch;
	}
	// 8253A7D0: 3BE001FF  li r31, 0x1ff
	ctx.r[31].s64 = 511;
	// 8253A7D4: 9B61048F  stb r27, 0x48f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1167 as u32), ctx.r[27].u8 ) };
	// 8253A7D8: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 8253A7DC: 4800BFF5  bl 0x825467d0
	ctx.lr = 0x8253A7E0;
	sub_825467D0(ctx, base);
	// 8253A7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253A7E4: 38210530  addi r1, r1, 0x530
	ctx.r[1].s64 = ctx.r[1].s64 + 1328;
	// 8253A7E8: 4BFFA8E8  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253A7F0 size=12
    let mut pc: u32 = 0x8253A7F0;
    'dispatch: loop {
        match pc {
            0x8253A7F0 => {
    //   block [0x8253A7F0..0x8253A7FC)
	// 8253A7F0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8253A7F4: 906B3DE0  stw r3, 0x3de0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(15840 as u32), ctx.r[3].u32 ) };
	// 8253A7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253A800 size=68
    let mut pc: u32 = 0x8253A800;
    'dispatch: loop {
        match pc {
            0x8253A800 => {
    //   block [0x8253A800..0x8253A844)
	// 8253A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253A808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253A80C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8253A810: 816B3DE0  lwz r11, 0x3de0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15840 as u32) ) } as u64;
	// 8253A814: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A818: 41820010  beq 0x8253a828
	if ctx.cr[0].eq {
	pc = 0x8253A828; continue 'dispatch;
	}
	// 8253A81C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8253A820: 4E800421  bctrl
	ctx.lr = 0x8253A824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8253A824: 48000010  b 0x8253a834
	pc = 0x8253A834; continue 'dispatch;
	// 8253A828: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8253A82C: 4800822D  bl 0x82542a58
	ctx.lr = 0x8253A830;
	sub_82542A58(ctx, base);
	// 8253A830: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 8253A834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8253A838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253A83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253A840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253A848 size=132
    let mut pc: u32 = 0x8253A848;
    'dispatch: loop {
        match pc {
            0x8253A848 => {
    //   block [0x8253A848..0x8253A8CC)
	// 8253A848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253A84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253A850: 9421F500  stwu r1, -0xb00(r1)
	ea = ctx.r[1].u32.wrapping_add(-2816 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253A854: 38A00A40  li r5, 0xa40
	ctx.r[5].s64 = 2624;
	// 8253A858: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253A85C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8253A860: 4BFFA971  bl 0x825351d0
	ctx.lr = 0x8253A864;
	sub_825351D0(ctx, base);
	// 8253A864: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8253A868: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253A86C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8253A870: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8253A874: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8253A878: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8253A87C: 4200FFF8  bdnz 0x8253a874
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8253A874; continue 'dispatch;
	}
	// 8253A880: 3D60C000  lis r11, -0x4000
	ctx.r[11].s64 = -1073741824;
	// 8253A884: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253A888: 616B000D  ori r11, r11, 0xd
	ctx.r[11].u64 = ctx.r[11].u64 | 13;
	// 8253A88C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8253A890: 81610AF8  lwz r11, 0xaf8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2808 as u32) ) } as u64;
	// 8253A894: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8253A898: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8253A89C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8253A8A0: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 8253A8A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8253A8A8: 4BE872F1  bl 0x823c1b98
	ctx.lr = 0x8253A8AC;
	sub_823C1B98(ctx, base);
	// 8253A8AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8253A8B0: 4BE873D1  bl 0x823c1c80
	ctx.lr = 0x8253A8B4;
	sub_823C1C80(ctx, base);
	// 8253A8B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A8B8: 4082000C  bne 0x8253a8c4
	if !ctx.cr[0].eq {
	pc = 0x8253A8C4; continue 'dispatch;
	}
	// 8253A8BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8253A8C0: 48008199  bl 0x82542a58
	ctx.lr = 0x8253A8C4;
	sub_82542A58(ctx, base);
	// 8253A8C4: 3860001E  li r3, 0x1e
	ctx.r[3].s64 = 30;
	// 8253A8C8: 481D3085  bl 0x8270d94c
	ctx.lr = 0x8253A8CC;
	// extern call 0x8270D94C → crate::xboxkrnl::KeBugCheck
	crate::xboxkrnl::KeBugCheck(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253A8D0 size=64
    let mut pc: u32 = 0x8253A8D0;
    'dispatch: loop {
        match pc {
            0x8253A8D0 => {
    //   block [0x8253A8D0..0x8253A910)
	// 8253A8D0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253A8D4: 392BEC28  addi r9, r11, -0x13d8
	ctx.r[9].s64 = ctx.r[11].s64 + -5080;
	// 8253A8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253A8DC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8253A8E0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253A8E4: 7F034040  cmplw cr6, r3, r8
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8253A8E8: 419A0028  beq cr6, 0x8253a910
	if ctx.cr[6].eq {
		sub_8253A910(ctx, base);
		return;
	}
	// 8253A8EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253A8F0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8253A8F4: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 8253A8F8: 4198FFE8  blt cr6, 0x8253a8e0
	if ctx.cr[6].lt {
	pc = 0x8253A8E0; continue 'dispatch;
	}
	// 8253A8FC: 3963FFED  addi r11, r3, -0x13
	ctx.r[11].s64 = ctx.r[3].s64 + -19;
	// 8253A900: 2B0B0011  cmplwi cr6, r11, 0x11
	ctx.cr[6].compare_u32(ctx.r[11].u32, 17 as u32, &mut ctx.xer);
	// 8253A904: 4199001C  bgt cr6, 0x8253a920
	if ctx.cr[6].gt {
		sub_8253A920(ctx, base);
		return;
	}
	// 8253A908: 3860000D  li r3, 0xd
	ctx.r[3].s64 = 13;
	// 8253A90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253A910 size=16
    let mut pc: u32 = 0x8253A910;
    'dispatch: loop {
        match pc {
            0x8253A910 => {
    //   block [0x8253A910..0x8253A920)
	// 8253A910: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253A914: 39490004  addi r10, r9, 4
	ctx.r[10].s64 = ctx.r[9].s64 + 4;
	// 8253A918: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8253A91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8253A920 size=24
    let mut pc: u32 = 0x8253A920;
    'dispatch: loop {
        match pc {
            0x8253A920 => {
    //   block [0x8253A920..0x8253A938)
	// 8253A920: 3963FF44  addi r11, r3, -0xbc
	ctx.r[11].s64 = ctx.r[3].s64 + -188;
	// 8253A924: 216B000E  subfic r11, r11, 0xe
	ctx.xer.ca = ctx.r[11].u32 <= 14 as u32;
	ctx.r[11].s64 = (14 as i64) - ctx.r[11].s64;
	// 8253A928: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8253A92C: 556B073C  rlwinm r11, r11, 0, 0x1c, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253A930: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8253A934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253A938 size=56
    let mut pc: u32 = 0x8253A938;
    'dispatch: loop {
        match pc {
            0x8253A938 => {
    //   block [0x8253A938..0x8253A970)
	// 8253A938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253A93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253A940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253A944: 4BFFB295  bl 0x82535bd8
	ctx.lr = 0x8253A948;
	sub_82535BD8(ctx, base);
	// 8253A948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A94C: 40820010  bne 0x8253a95c
	if !ctx.cr[0].eq {
	pc = 0x8253A95C; continue 'dispatch;
	}
	// 8253A950: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253A954: 386BED90  addi r3, r11, -0x1270
	ctx.r[3].s64 = ctx.r[11].s64 + -4720;
	// 8253A958: 48000008  b 0x8253a960
	pc = 0x8253A960; continue 'dispatch;
	// 8253A95C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8253A960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8253A964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253A968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253A96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253A970 size=56
    let mut pc: u32 = 0x8253A970;
    'dispatch: loop {
        match pc {
            0x8253A970 => {
    //   block [0x8253A970..0x8253A9A8)
	// 8253A970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253A974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253A978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253A97C: 4BFFB25D  bl 0x82535bd8
	ctx.lr = 0x8253A980;
	sub_82535BD8(ctx, base);
	// 8253A980: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A984: 40820010  bne 0x8253a994
	if !ctx.cr[0].eq {
	pc = 0x8253A994; continue 'dispatch;
	}
	// 8253A988: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253A98C: 386BED94  addi r3, r11, -0x126c
	ctx.r[3].s64 = ctx.r[11].s64 + -4716;
	// 8253A990: 48000008  b 0x8253a998
	pc = 0x8253A998; continue 'dispatch;
	// 8253A994: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8253A998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8253A99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253A9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253A9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253A9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253A9A8 size=112
    let mut pc: u32 = 0x8253A9A8;
    'dispatch: loop {
        match pc {
            0x8253A9A8 => {
    //   block [0x8253A9A8..0x8253AA18)
	// 8253A9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253A9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253A9B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8253A9B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253A9B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253A9BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8253A9C0: 4BFFB219  bl 0x82535bd8
	ctx.lr = 0x8253A9C4;
	sub_82535BD8(ctx, base);
	// 8253A9C4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253A9C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A9CC: 3BEBED90  addi r31, r11, -0x1270
	ctx.r[31].s64 = ctx.r[11].s64 + -4720;
	// 8253A9D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8253A9D4: 41820008  beq 0x8253a9dc
	if ctx.cr[0].eq {
	pc = 0x8253A9DC; continue 'dispatch;
	}
	// 8253A9D8: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 8253A9DC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8253A9E0: 4BFFB1F9  bl 0x82535bd8
	ctx.lr = 0x8253A9E4;
	sub_82535BD8(ctx, base);
	// 8253A9E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253A9E8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8253A9EC: 41820008  beq 0x8253a9f4
	if ctx.cr[0].eq {
	pc = 0x8253A9F4; continue 'dispatch;
	}
	// 8253A9F0: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 8253A9F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253A9F8: 4BFFFED9  bl 0x8253a8d0
	ctx.lr = 0x8253A9FC;
	sub_8253A8D0(ctx, base);
	// 8253A9FC: 90670000  stw r3, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8253AA00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253AA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253AA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253AA0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8253AA10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253AA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AA18 size=40
    let mut pc: u32 = 0x8253AA18;
    'dispatch: loop {
        match pc {
            0x8253AA18 => {
    //   block [0x8253AA18..0x8253AA40)
	// 8253AA18: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8253AA1C: 2B0B0100  cmplwi cr6, r11, 0x100
	ctx.cr[6].compare_u32(ctx.r[11].u32, 256 as u32, &mut ctx.xer);
	// 8253AA20: 41990020  bgt cr6, 0x8253aa40
	if ctx.cr[6].gt {
		sub_8253AA40(ctx, base);
		return;
	}
	// 8253AA24: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253AA28: 546A083C  slwi r10, r3, 1
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253AA2C: 816BEE80  lwz r11, -0x1180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4480 as u32) ) } as u64;
	// 8253AA30: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 8253AA34: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8253AA38: 7D632038  and r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 8253AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AA40 size=8
    let mut pc: u32 = 0x8253AA40;
    'dispatch: loop {
        match pc {
            0x8253AA40 => {
    //   block [0x8253AA40..0x8253AA48)
	// 8253AA40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253AA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AA48 size=40
    let mut pc: u32 = 0x8253AA48;
    'dispatch: loop {
        match pc {
            0x8253AA48 => {
    //   block [0x8253AA48..0x8253AA70)
	// 8253AA48: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 8253AA4C: 396403FE  addi r11, r4, 0x3fe
	ctx.r[11].s64 = ctx.r[4].s64 + 1022;
	// 8253AA50: D821FFF0  stfd f1, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[1].u64 ) };
	// 8253AA54: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253AA58: A1410010  lhz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AA5C: 714A800F  andi. r10, r10, 0x800f
	ctx.r[10].u64 = ctx.r[10].u64 & 32783;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253AA60: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8253AA64: B161FFF0  sth r11, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u16 ) };
	// 8253AA68: C821FFF0  lfd f1, -0x10(r1)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253AA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AA70 size=24
    let mut pc: u32 = 0x8253AA70;
    'dispatch: loop {
        match pc {
            0x8253AA70 => {
    //   block [0x8253AA70..0x8253AA88)
	// 8253AA70: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 8253AA74: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AA78: 556BE57E  rlwinm r11, r11, 0x1c, 0x15, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8253AA7C: 396BFC02  addi r11, r11, -0x3fe
	ctx.r[11].s64 = ctx.r[11].s64 + -1022;
	// 8253AA80: 7D630734  extsh r3, r11
	ctx.r[3].s64 = ctx.r[11].s16 as i64;
	// 8253AA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AA88 size=40
    let mut pc: u32 = 0x8253AA88;
    'dispatch: loop {
        match pc {
            0x8253AA88 => {
    //   block [0x8253AA88..0x8253AAB0)
	// 8253AA88: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 8253AA8C: 3D607FF0  lis r11, 0x7ff0
	ctx.r[11].s64 = 2146435072;
	// 8253AA90: 81410010  lwz r10, 0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AA94: 81210014  lwz r9, 0x14(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 8253AA98: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253AA9C: 409A0014  bne cr6, 0x8253aab0
	if !ctx.cr[6].eq {
		sub_8253AAB0(ctx, base);
		return;
	}
	// 8253AAA0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253AAA4: 409A0028  bne cr6, 0x8253aacc
	if !ctx.cr[6].eq {
		sub_8253AACC(ctx, base);
		return;
	}
	// 8253AAA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AAB0 size=28
    let mut pc: u32 = 0x8253AAB0;
    'dispatch: loop {
        match pc {
            0x8253AAB0 => {
    //   block [0x8253AAB0..0x8253AACC)
	// 8253AAB0: 3D60FFF0  lis r11, -0x10
	ctx.r[11].s64 = -1048576;
	// 8253AAB4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253AAB8: 409A0014  bne cr6, 0x8253aacc
	if !ctx.cr[6].eq {
		sub_8253AACC(ctx, base);
		return;
	}
	// 8253AABC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253AAC0: 409A000C  bne cr6, 0x8253aacc
	if !ctx.cr[6].eq {
		sub_8253AACC(ctx, base);
		return;
	}
	// 8253AAC4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8253AAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AACC size=24
    let mut pc: u32 = 0x8253AACC;
    'dispatch: loop {
        match pc {
            0x8253AACC => {
    //   block [0x8253AACC..0x8253AAE4)
	// 8253AACC: A1610010  lhz r11, 0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AAD0: 556B0478  rlwinm r11, r11, 0, 0x11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AAD4: 2B0B7FF8  cmplwi cr6, r11, 0x7ff8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32760 as u32, &mut ctx.xer);
	// 8253AAD8: 409A000C  bne cr6, 0x8253aae4
	if !ctx.cr[6].eq {
		sub_8253AAE4(ctx, base);
		return;
	}
	// 8253AADC: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8253AAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AAE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AAE4 size=32
    let mut pc: u32 = 0x8253AAE4;
    'dispatch: loop {
        match pc {
            0x8253AAE4 => {
    //   block [0x8253AAE4..0x8253AB04)
	// 8253AAE4: 2B0B7FF0  cmplwi cr6, r11, 0x7ff0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32752 as u32, &mut ctx.xer);
	// 8253AAE8: 409A001C  bne cr6, 0x8253ab04
	if !ctx.cr[6].eq {
		sub_8253AB04(ctx, base);
		return;
	}
	// 8253AAEC: 554B037F  clrlwi. r11, r10, 0xd
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0007FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AAF0: 4082000C  bne 0x8253aafc
	if !ctx.cr[0].eq {
	pc = 0x8253AAFC; continue 'dispatch;
	}
	// 8253AAF4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253AAF8: 419A000C  beq cr6, 0x8253ab04
	if ctx.cr[6].eq {
		sub_8253AB04(ctx, base);
		return;
	}
	// 8253AAFC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8253AB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AB04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AB04 size=8
    let mut pc: u32 = 0x8253AB04;
    'dispatch: loop {
        match pc {
            0x8253AB04 => {
    //   block [0x8253AB04..0x8253AB0C)
	// 8253AB04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253AB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AB10 size=28
    let mut pc: u32 = 0x8253AB10;
    'dispatch: loop {
        match pc {
            0x8253AB10 => {
    //   block [0x8253AB10..0x8253AB2C)
	// 8253AB10: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253AB14: D8210010  stfd f1, 0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 8253AB18: C80B2008  lfd f0, 0x2008(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253AB1C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8253AB20: 409A000C  bne cr6, 0x8253ab2c
	if !ctx.cr[6].eq {
		sub_8253AB2C(ctx, base);
		return;
	}
	// 8253AB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253AB28: 480000C4  b 0x8253abec
	sub_8253ABC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AB2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AB2C size=156
    let mut pc: u32 = 0x8253AB2C;
    'dispatch: loop {
        match pc {
            0x8253AB2C => {
    //   block [0x8253AB2C..0x8253ABC8)
	// 8253AB2C: A1010010  lhz r8, 0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AB30: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8253AB34: 54EB0477  rlwinm. r11, r7, 0, 0x11, 0x1b
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AB38: 40820090  bne 0x8253abc8
	if !ctx.cr[0].eq {
		sub_8253ABC8(ctx, base);
		return;
	}
	// 8253AB3C: 80C10010  lwz r6, 0x10(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AB40: 54CA033F  clrlwi. r10, r6, 0xc
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000FFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253AB44: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 8253AB48: 4082000C  bne 0x8253ab54
	if !ctx.cr[0].eq {
	pc = 0x8253AB54; continue 'dispatch;
	}
	// 8253AB4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253AB50: 419A0078  beq cr6, 0x8253abc8
	if ctx.cr[6].eq {
		sub_8253ABC8(ctx, base);
		return;
	}
	// 8253AB54: 3920FC03  li r9, -0x3fd
	ctx.r[9].s64 = -1021;
	// 8253AB58: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8253AB5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8253AB60: 41980008  blt cr6, 0x8253ab68
	if ctx.cr[6].lt {
	pc = 0x8253AB68; continue 'dispatch;
	}
	// 8253AB64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253AB68: 54EB06F7  rlwinm. r11, r7, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AB6C: 40820034  bne 0x8253aba0
	if !ctx.cr[0].eq {
	pc = 0x8253ABA0; continue 'dispatch;
	}
	// 8253AB70: 54C6083C  slwi r6, r6, 1
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8253AB74: 554B0001  rlwinm. r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AB78: 90C10010  stw r6, 0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 8253AB7C: 4182000C  beq 0x8253ab88
	if ctx.cr[0].eq {
	pc = 0x8253AB88; continue 'dispatch;
	}
	// 8253AB80: 60C60001  ori r6, r6, 1
	ctx.r[6].u64 = ctx.r[6].u64 | 1;
	// 8253AB84: 90C10010  stw r6, 0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 8253AB88: A1010010  lhz r8, 0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) } as u64;
	// 8253AB8C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253AB90: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8253AB94: 550B06F7  rlwinm. r11, r8, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AB98: 4182FFD8  beq 0x8253ab70
	if ctx.cr[0].eq {
	pc = 0x8253AB70; continue 'dispatch;
	}
	// 8253AB9C: 91410014  stw r10, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8253ABA0: 710BFFEF  andi. r11, r8, 0xffef
	ctx.r[11].u64 = ctx.r[8].u64 & 65519;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ABA4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8253ABA8: B1610010  sth r11, 0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 8253ABAC: 419A000C  beq cr6, 0x8253abb8
	if ctx.cr[6].eq {
	pc = 0x8253ABB8; continue 'dispatch;
	}
	// 8253ABB0: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 8253ABB4: B1610010  sth r11, 0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 8253ABB8: C8010010  lfd f0, 0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(16 as u32) ) };
	// 8253ABBC: D801FFF8  stfd f0, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 8253ABC0: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8253ABC4: 48000014  b 0x8253abd8
	sub_8253ABC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253ABC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253ABC8 size=44
    let mut pc: u32 = 0x8253ABC8;
    'dispatch: loop {
        match pc {
            0x8253ABC8 => {
    //   block [0x8253ABC8..0x8253ABF4)
	// 8253ABC8: 556BE53E  rlwinm r11, r11, 0x1c, 0x14, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8253ABCC: D821FFF8  stfd f1, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[1].u64 ) };
	// 8253ABD0: D821FFF0  stfd f1, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[1].u64 ) };
	// 8253ABD4: 392BFC02  addi r9, r11, -0x3fe
	ctx.r[9].s64 = ctx.r[11].s64 + -1022;
	// 8253ABD8: A161FFF8  lhz r11, -8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253ABDC: 716B800F  andi. r11, r11, 0x800f
	ctx.r[11].u64 = ctx.r[11].u64 & 32783;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ABE0: 616B3FE0  ori r11, r11, 0x3fe0
	ctx.r[11].u64 = ctx.r[11].u64 | 16352;
	// 8253ABE4: B161FFF0  sth r11, -0x10(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u16 ) };
	// 8253ABE8: C821FFF0  lfd f1, -0x10(r1)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253ABEC: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8253ABF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AC00 size=56
    let mut pc: u32 = 0x8253AC00;
    'dispatch: loop {
        match pc {
            0x8253AC00 => {
    //   block [0x8253AC00..0x8253AC38)
	// 8253AC00: 3D808206  lis r12, -0x7dfa
	ctx.r[12].s64 = -2113536000;
	// 8253AC04: C88C7740  lfd f4, 0x7740(r12)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(30528 as u32) ) };
	// 8253AC08: 3D808206  lis r12, -0x7dfa
	ctx.r[12].s64 = -2113536000;
	// 8253AC0C: C8AC7748  lfd f5, 0x7748(r12)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[12].u32.wrapping_add(30536 as u32) ) };
	// 8253AC10: FC012000  fcmpu cr0, f1, f4
	ctx.cr[0].compare_f64(ctx.f[1].f64, ctx.f[4].f64);
	// 8253AC14: 41C2002C  beq- 0x8253ac40
	if ctx.cr[0].eq {
		sub_8253AC38(ctx, base);
		return;
	}
	// 8253AC18: FCC00A10  fabs f6, f1
	ctx.f[6].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 8253AC1C: FC062800  fcmpu cr0, f6, f5
	ctx.cr[0].compare_f64(ctx.f[6].f64, ctx.f[5].f64);
	// 8253AC20: 40C00020  bge- 0x8253ac40
	if !ctx.cr[0].lt {
		sub_8253AC38(ctx, base);
		return;
	}
	// 8253AC24: FC012000  fcmpu cr0, f1, f4
	ctx.cr[0].compare_f64(ctx.f[1].f64, ctx.f[4].f64);
	// 8253AC28: 41800010  blt 0x8253ac38
	if ctx.cr[0].lt {
		sub_8253AC38(ctx, base);
		return;
	}
	// 8253AC2C: FC81282A  fadd f4, f1, f5
	ctx.f[4].f64 = ctx.f[1].f64 + ctx.f[5].f64;
	// 8253AC30: FC242828  fsub f1, f4, f5
	ctx.f[1].f64 = ctx.f[4].f64 - ctx.f[5].f64;
	// 8253AC34: 4800000C  b 0x8253ac40
	sub_8253AC38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253AC38 size=12
    let mut pc: u32 = 0x8253AC38;
    'dispatch: loop {
        match pc {
            0x8253AC38 => {
    //   block [0x8253AC38..0x8253AC44)
	// 8253AC38: FC812828  fsub f4, f1, f5
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[1].f64 - ctx.f[5].f64;
	// 8253AC3C: FC24282A  fadd f1, f4, f5
	ctx.f[1].f64 = ctx.f[4].f64 + ctx.f[5].f64;
	// 8253AC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253AC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8253AC48 size=1032
    let mut pc: u32 = 0x8253AC48;
    'dispatch: loop {
        match pc {
            0x8253AC48 => {
    //   block [0x8253AC48..0x8253B050)
	// 8253AC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253AC4C: 4BFFA461  bl 0x825350ac
	ctx.lr = 0x8253AC50;
	sub_82535080(ctx, base);
	// 8253AC50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253AC54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253AC58: 906100B4  stw r3, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 8253AC5C: 54AA06F7  rlwinm. r10, r5, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253AC60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253AC64: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8253AC68: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8253AC6C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8253AC70: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8253AC74: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AC78: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8253AC7C: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8253AC80: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AC84: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8253AC88: 41820020  beq 0x8253aca8
	if ctx.cr[0].eq {
	pc = 0x8253ACA8; continue 'dispatch;
	}
	// 8253AC8C: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AC90: 3F60C000  lis r27, -0x4000
	ctx.r[27].s64 = -1073741824;
	// 8253AC94: 637B008F  ori r27, r27, 0x8f
	ctx.r[27].u64 = ctx.r[27].u64 | 143;
	// 8253AC98: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253AC9C: 654A8000  oris r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 2147483648;
	// 8253ACA0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8253ACA4: 48000008  b 0x8253acac
	pc = 0x8253ACAC; continue 'dispatch;
	// 8253ACA8: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253ACAC: 54AB07BD  rlwinm. r11, r5, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ACB0: 4182001C  beq 0x8253accc
	if ctx.cr[0].eq {
	pc = 0x8253ACCC; continue 'dispatch;
	}
	// 8253ACB4: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ACB8: 3F60C000  lis r27, -0x4000
	ctx.r[27].s64 = -1073741824;
	// 8253ACBC: 637B0093  ori r27, r27, 0x93
	ctx.r[27].u64 = ctx.r[27].u64 | 147;
	// 8253ACC0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253ACC4: 654A4000  oris r10, r10, 0x4000
	ctx.r[10].u64 = ctx.r[10].u64 | 1073741824;
	// 8253ACC8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8253ACCC: 54AB07FF  clrlwi. r11, r5, 0x1f
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ACD0: 4182001C  beq 0x8253acec
	if ctx.cr[0].eq {
	pc = 0x8253ACEC; continue 'dispatch;
	}
	// 8253ACD4: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ACD8: 3F60C000  lis r27, -0x4000
	ctx.r[27].s64 = -1073741824;
	// 8253ACDC: 637B0091  ori r27, r27, 0x91
	ctx.r[27].u64 = ctx.r[27].u64 | 145;
	// 8253ACE0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253ACE4: 654A2000  oris r10, r10, 0x2000
	ctx.r[10].u64 = ctx.r[10].u64 | 536870912;
	// 8253ACE8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8253ACEC: 54AB077B  rlwinm. r11, r5, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ACF0: 4182001C  beq 0x8253ad0c
	if ctx.cr[0].eq {
	pc = 0x8253AD0C; continue 'dispatch;
	}
	// 8253ACF4: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ACF8: 3F60C000  lis r27, -0x4000
	ctx.r[27].s64 = -1073741824;
	// 8253ACFC: 637B008E  ori r27, r27, 0x8e
	ctx.r[27].u64 = ctx.r[27].u64 | 142;
	// 8253AD00: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253AD04: 654A1000  oris r10, r10, 0x1000
	ctx.r[10].u64 = ctx.r[10].u64 | 268435456;
	// 8253AD08: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8253AD0C: 54AB0739  rlwinm. r11, r5, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AD10: 4182001C  beq 0x8253ad2c
	if ctx.cr[0].eq {
	pc = 0x8253AD2C; continue 'dispatch;
	}
	// 8253AD14: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AD18: 3F60C000  lis r27, -0x4000
	ctx.r[27].s64 = -1073741824;
	// 8253AD1C: 637B0090  ori r27, r27, 0x90
	ctx.r[27].u64 = ctx.r[27].u64 | 144;
	// 8253AD20: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253AD24: 654A0800  oris r10, r10, 0x800
	ctx.r[10].u64 = ctx.r[10].u64 | 134217728;
	// 8253AD28: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8253AD2C: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AD30: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AD34: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8253AD38: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AD3C: 5149A108  rlwimi r9, r10, 0x14, 4, 4
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(20) as u64) & 0x0000000008000000) | (ctx.r[9].u64 & 0xFFFFFFFFF7FFFFFF);
	// 8253AD40: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8253AD44: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AD48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AD4C: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8253AD50: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AD54: 5149C0C6  rlwimi r9, r10, 0x18, 3, 3
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(24) as u64) & 0x0000000010000000) | (ctx.r[9].u64 & 0xFFFFFFFFEFFFFFFF);
	// 8253AD58: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8253AD5C: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AD60: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AD64: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8253AD68: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AD6C: 5149B884  rlwimi r9, r10, 0x17, 2, 2
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(23) as u64) & 0x0000000020000000) | (ctx.r[9].u64 & 0xFFFFFFFFDFFFFFFF);
	// 8253AD70: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8253AD74: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AD78: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AD7C: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8253AD80: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AD84: 5149C842  rlwimi r9, r10, 0x19, 1, 1
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(25) as u64) & 0x0000000040000000) | (ctx.r[9].u64 & 0xFFFFFFFFBFFFFFFF);
	// 8253AD88: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8253AD8C: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AD90: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AD94: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8253AD98: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AD9C: 5149E000  rlwimi r9, r10, 0x1c, 0, 0
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(28) as u64) & 0x0000000080000000) | (ctx.r[9].u64 & 0xFFFFFFFF7FFFFFFF);
	// 8253ADA0: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8253ADA4: 4800099D  bl 0x8253b740
	ctx.lr = 0x8253ADA8;
	sub_8253B740(ctx, base);
	// 8253ADA8: 546B0085  rlwinm. r11, r3, 0, 2, 2
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ADAC: 41820014  beq 0x8253adc0
	if ctx.cr[0].eq {
	pc = 0x8253ADC0; continue 'dispatch;
	}
	// 8253ADB0: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ADB4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253ADB8: 654A0800  oris r10, r10, 0x800
	ctx.r[10].u64 = ctx.r[10].u64 | 134217728;
	// 8253ADBC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8253ADC0: 546B014B  rlwinm. r11, r3, 0, 5, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ADC4: 41820014  beq 0x8253add8
	if ctx.cr[0].eq {
	pc = 0x8253ADD8; continue 'dispatch;
	}
	// 8253ADC8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ADCC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253ADD0: 654A1000  oris r10, r10, 0x1000
	ctx.r[10].u64 = ctx.r[10].u64 | 268435456;
	// 8253ADD4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8253ADD8: 546B00C7  rlwinm. r11, r3, 0, 3, 3
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ADDC: 41820014  beq 0x8253adf0
	if ctx.cr[0].eq {
	pc = 0x8253ADF0; continue 'dispatch;
	}
	// 8253ADE0: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ADE4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253ADE8: 654A2000  oris r10, r10, 0x2000
	ctx.r[10].u64 = ctx.r[10].u64 | 536870912;
	// 8253ADEC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8253ADF0: 546B0109  rlwinm. r11, r3, 0, 4, 4
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253ADF4: 41820014  beq 0x8253ae08
	if ctx.cr[0].eq {
	pc = 0x8253AE08; continue 'dispatch;
	}
	// 8253ADF8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253ADFC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253AE00: 654A4000  oris r10, r10, 0x4000
	ctx.r[10].u64 = ctx.r[10].u64 | 1073741824;
	// 8253AE04: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8253AE08: 546B018D  rlwinm. r11, r3, 0, 6, 6
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AE0C: 41820014  beq 0x8253ae20
	if ctx.cr[0].eq {
	pc = 0x8253AE20; continue 'dispatch;
	}
	// 8253AE10: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE14: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253AE18: 654A8000  oris r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 2147483648;
	// 8253AE1C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8253AE20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AE24: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8253AE28: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8253AE2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8253AE30: 41980044  blt cr6, 0x8253ae74
	if ctx.cr[6].lt {
	pc = 0x8253AE74; continue 'dispatch;
	}
	// 8253AE34: 419A0030  beq cr6, 0x8253ae64
	if ctx.cr[6].eq {
	pc = 0x8253AE64; continue 'dispatch;
	}
	// 8253AE38: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8253AE3C: 41980018  blt cr6, 0x8253ae54
	if ctx.cr[6].lt {
	pc = 0x8253AE54; continue 'dispatch;
	}
	// 8253AE40: 409A0044  bne cr6, 0x8253ae84
	if !ctx.cr[6].eq {
	pc = 0x8253AE84; continue 'dispatch;
	}
	// 8253AE44: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AE4C: 53CAF002  rlwimi r10, r30, 0x1e, 0, 1
	ctx.r[10].u64 = (((ctx.r[30].u32).rotate_left(30) as u64) & 0x00000000C0000000) | (ctx.r[10].u64 & 0xFFFFFFFF3FFFFFFF);
	// 8253AE50: 48000030  b 0x8253ae80
	pc = 0x8253AE80; continue 'dispatch;
	// 8253AE54: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE58: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AE5C: 53CAF802  rlwimi r10, r30, 0x1f, 0, 1
	ctx.r[10].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x00000000C0000000) | (ctx.r[10].u64 & 0xFFFFFFFF3FFFFFFF);
	// 8253AE60: 48000020  b 0x8253ae80
	pc = 0x8253AE80; continue 'dispatch;
	// 8253AE64: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AE6C: 654AC000  oris r10, r10, 0xc000
	ctx.r[10].u64 = ctx.r[10].u64 | 3221225472;
	// 8253AE70: 48000010  b 0x8253ae80
	pc = 0x8253AE80; continue 'dispatch;
	// 8253AE74: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE78: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AE7C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8253AE80: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253AE84: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE88: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253AE8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AE90: 53CAD888  rlwimi r10, r30, 0x1b, 2, 4
	ctx.r[10].u64 = (((ctx.r[30].u32).rotate_left(27) as u64) & 0x0000000038000000) | (ctx.r[10].u64 & 0xFFFFFFFFC7FFFFFF);
	// 8253AE94: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253AE98: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AE9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AEA0: 538A7960  rlwimi r10, r28, 0xf, 5, 0x10
	ctx.r[10].u64 = (((ctx.r[28].u32).rotate_left(15) as u64) & 0x0000000007FF8000) | (ctx.r[10].u64 & 0xFFFFFFFFF8007FFF);
	// 8253AEA4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253AEA8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AEAC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8253AEB0: 654A8000  oris r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 2147483648;
	// 8253AEB4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8253AEB8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AEBC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8253AEC0: 419A0048  beq cr6, 0x8253af08
	if ctx.cr[6].eq {
	pc = 0x8253AF08; continue 'dispatch;
	}
	// 8253AEC4: 554A0140  rlwinm r10, r10, 0, 5, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AEC8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8253AECC: C01D0000  lfs f0, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8253AED0: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AED4: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8253AED8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AEDC: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253AEE0: 654A8000  oris r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 2147483648;
	// 8253AEE4: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8253AEE8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AEEC: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253AEF0: 554A0140  rlwinm r10, r10, 0, 5, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AEF4: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8253AEF8: C01A0000  lfs f0, 0(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8253AEFC: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AF00: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8253AF04: 48000044  b 0x8253af48
	pc = 0x8253AF48; continue 'dispatch;
	// 8253AF08: 53CAD848  rlwimi r10, r30, 0x1b, 1, 4
	ctx.r[10].u64 = (((ctx.r[30].u32).rotate_left(27) as u64) & 0x0000000078000000) | (ctx.r[10].u64 & 0xFFFFFFFF87FFFFFF);
	// 8253AF0C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8253AF10: C81D0000  lfd f0, 0(r29)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 8253AF14: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AF18: D80B0010  stfd f0, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.f[0].u64 ) };
	// 8253AF1C: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AF20: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253AF24: 654A8000  oris r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 2147483648;
	// 8253AF28: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8253AF2C: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AF30: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253AF34: 53CAD848  rlwimi r10, r30, 0x1b, 1, 4
	ctx.r[10].u64 = (((ctx.r[30].u32).rotate_left(27) as u64) & 0x0000000078000000) | (ctx.r[10].u64 & 0xFFFFFFFF87FFFFFF);
	// 8253AF38: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8253AF3C: C81A0000  lfd f0, 0(r26)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) };
	// 8253AF40: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AF44: D80B0050  stfd f0, 0x50(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8253AF48: 48007BD1  bl 0x82542b18
	ctx.lr = 0x8253AF4C;
	sub_82542B18(ctx, base);
	// 8253AF4C: 38C100B4  addi r6, r1, 0xb4
	ctx.r[6].s64 = ctx.r[1].s64 + 180;
	// 8253AF50: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8253AF54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253AF58: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8253AF5C: 4BE86DB5  bl 0x823c1d10
	ctx.lr = 0x8253AF60;
	sub_823C1D10(ctx, base);
	// 8253AF60: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253AF64: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AF68: 556B0109  rlwinm. r11, r11, 0, 4, 4
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AF6C: 41820010  beq 0x8253af7c
	if ctx.cr[0].eq {
	pc = 0x8253AF7C; continue 'dispatch;
	}
	// 8253AF70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AF74: 556B066E  rlwinm r11, r11, 0, 0x19, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AF78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253AF7C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AF80: 556B00C7  rlwinm. r11, r11, 0, 3, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AF84: 41820010  beq 0x8253af94
	if ctx.cr[0].eq {
	pc = 0x8253AF94; continue 'dispatch;
	}
	// 8253AF88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AF8C: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AF90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253AF94: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AF98: 556B0085  rlwinm. r11, r11, 0, 2, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AF9C: 41820010  beq 0x8253afac
	if ctx.cr[0].eq {
	pc = 0x8253AFAC; continue 'dispatch;
	}
	// 8253AFA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AFA4: 556B06B0  rlwinm r11, r11, 0, 0x1a, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AFA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253AFAC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AFB0: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AFB4: 41820010  beq 0x8253afc4
	if ctx.cr[0].eq {
	pc = 0x8253AFC4; continue 'dispatch;
	}
	// 8253AFB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AFBC: 556B06F2  rlwinm r11, r11, 0, 0x1b, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AFC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253AFC4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253AFC8: 556B0001  rlwinm. r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253AFCC: 41820010  beq 0x8253afdc
	if ctx.cr[0].eq {
	pc = 0x8253AFDC; continue 'dispatch;
	}
	// 8253AFD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AFD4: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253AFD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253AFDC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253AFE0: 556B17BE  srwi r11, r11, 0x1e
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(30);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253AFE4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8253AFE8: 41980038  blt cr6, 0x8253b020
	if ctx.cr[6].lt {
	pc = 0x8253B020; continue 'dispatch;
	}
	// 8253AFEC: 419A0028  beq cr6, 0x8253b014
	if ctx.cr[6].eq {
	pc = 0x8253B014; continue 'dispatch;
	}
	// 8253AFF0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8253AFF4: 41980014  blt cr6, 0x8253b008
	if ctx.cr[6].lt {
	pc = 0x8253B008; continue 'dispatch;
	}
	// 8253AFF8: 409A0034  bne cr6, 0x8253b02c
	if !ctx.cr[6].eq {
	pc = 0x8253B02C; continue 'dispatch;
	}
	// 8253AFFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253B000: 53CB07BE  rlwimi r11, r30, 0, 0x1e, 0x1f
	ctx.r[11].u64 = (((ctx.r[30].u32).rotate_left(0) as u64) & 0x0000000000000003) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFC);
	// 8253B004: 48000024  b 0x8253b028
	pc = 0x8253B028; continue 'dispatch;
	// 8253B008: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253B00C: 53CB0FBE  rlwimi r11, r30, 1, 0x1e, 0x1f
	ctx.r[11].u64 = (((ctx.r[30].u32).rotate_left(1) as u64) & 0x0000000000000003) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFC);
	// 8253B010: 48000018  b 0x8253b028
	pc = 0x8253B028; continue 'dispatch;
	// 8253B014: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253B018: 616B0003  ori r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64 | 3;
	// 8253B01C: 4800000C  b 0x8253b028
	pc = 0x8253B028; continue 'dispatch;
	// 8253B020: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253B024: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B028: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253B02C: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253B030: 419A0010  beq cr6, 0x8253b040
	if ctx.cr[6].eq {
	pc = 0x8253B040; continue 'dispatch;
	}
	// 8253B034: C00A0050  lfs f0, 0x50(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8253B038: D01A0000  stfs f0, 0(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8253B03C: 4800000C  b 0x8253b048
	pc = 0x8253B048; continue 'dispatch;
	// 8253B040: C80A0050  lfd f0, 0x50(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) };
	// 8253B044: D81A0000  stfd f0, 0(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 8253B048: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8253B04C: 4BFFA0B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B050 size=596
    let mut pc: u32 = 0x8253B050;
    'dispatch: loop {
        match pc {
            0x8253B050 => {
    //   block [0x8253B050..0x8253B2A4)
	// 8253B050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B054: 4BFFA061  bl 0x825350b4
	ctx.lr = 0x8253B058;
	sub_82535080(ctx, base);
	// 8253B058: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8253B05C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B060: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8253B064: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8253B068: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8253B06C: 57BC06FE  clrlwi r28, r29, 0x1b
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x0000001Fu64;
	// 8253B070: 57AB0739  rlwinm. r11, r29, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B074: 4182001C  beq 0x8253b090
	if ctx.cr[0].eq {
	pc = 0x8253B090; continue 'dispatch;
	}
	// 8253B078: 576B0631  rlwinm. r11, r27, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B07C: 41820014  beq 0x8253b090
	if ctx.cr[0].eq {
	pc = 0x8253B090; continue 'dispatch;
	}
	// 8253B080: 3C602000  lis r3, 0x2000
	ctx.r[3].s64 = 536870912;
	// 8253B084: 480006F1  bl 0x8253b774
	ctx.lr = 0x8253B088;
	sub_8253B774(ctx, base);
	// 8253B088: 579C0776  rlwinm r28, r28, 0, 0x1d, 0x1b
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B08C: 480001E8  b 0x8253b274
	pc = 0x8253B274; continue 'dispatch;
	// 8253B090: 57AB077B  rlwinm. r11, r29, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B094: 4182001C  beq 0x8253b0b0
	if ctx.cr[0].eq {
	pc = 0x8253B0B0; continue 'dispatch;
	}
	// 8253B098: 576B06F7  rlwinm. r11, r27, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B09C: 41820014  beq 0x8253b0b0
	if ctx.cr[0].eq {
	pc = 0x8253B0B0; continue 'dispatch;
	}
	// 8253B0A0: 3C600400  lis r3, 0x400
	ctx.r[3].s64 = 67108864;
	// 8253B0A4: 480006D1  bl 0x8253b774
	ctx.lr = 0x8253B0A8;
	sub_8253B774(ctx, base);
	// 8253B0A8: 579C07B8  rlwinm r28, r28, 0, 0x1e, 0x1c
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B0AC: 480001C8  b 0x8253b274
	pc = 0x8253B274; continue 'dispatch;
	// 8253B0B0: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B0B4: 418200C8  beq 0x8253b17c
	if ctx.cr[0].eq {
	pc = 0x8253B17C; continue 'dispatch;
	}
	// 8253B0B8: 576B0673  rlwinm. r11, r27, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B0BC: 418200C0  beq 0x8253b17c
	if ctx.cr[0].eq {
	pc = 0x8253B17C; continue 'dispatch;
	}
	// 8253B0C0: 3C601000  lis r3, 0x1000
	ctx.r[3].s64 = 268435456;
	// 8253B0C4: 480006B1  bl 0x8253b774
	ctx.lr = 0x8253B0C8;
	sub_8253B774(ctx, base);
	// 8253B0C8: 576B07BE  clrlwi r11, r27, 0x1e
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000003u64;
	// 8253B0CC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8253B0D0: 41980074  blt cr6, 0x8253b144
	if ctx.cr[6].lt {
	pc = 0x8253B144; continue 'dispatch;
	}
	// 8253B0D4: 419A0050  beq cr6, 0x8253b124
	if ctx.cr[6].eq {
	pc = 0x8253B124; continue 'dispatch;
	}
	// 8253B0D8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8253B0DC: 41980028  blt cr6, 0x8253b104
	if ctx.cr[6].lt {
	pc = 0x8253B104; continue 'dispatch;
	}
	// 8253B0E0: 409A0094  bne cr6, 0x8253b174
	if !ctx.cr[6].eq {
	pc = 0x8253B174; continue 'dispatch;
	}
	// 8253B0E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B0E8: C9BE0000  lfd f13, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8253B0EC: C80B2008  lfd f0, 0x2008(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B0F0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8253B0F4: 40990070  ble cr6, 0x8253b164
	if !ctx.cr[6].gt {
	pc = 0x8253B164; continue 'dispatch;
	}
	// 8253B0F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B0FC: C80BEE98  lfd f0, -0x1168(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4456 as u32) ) };
	// 8253B100: 48000070  b 0x8253b170
	pc = 0x8253B170; continue 'dispatch;
	// 8253B104: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B108: C9BE0000  lfd f13, 0(r30)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8253B10C: C80B2008  lfd f0, 0x2008(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B110: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8253B114: 41990044  bgt cr6, 0x8253b158
	if ctx.cr[6].gt {
	pc = 0x8253B158; continue 'dispatch;
	}
	// 8253B118: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B11C: C80BEE98  lfd f0, -0x1168(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4456 as u32) ) };
	// 8253B120: 4800004C  b 0x8253b16c
	pc = 0x8253B16C; continue 'dispatch;
	// 8253B124: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B128: C9BE0000  lfd f13, 0(r30)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8253B12C: C80B2008  lfd f0, 0x2008(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B130: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B134: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8253B138: C80BEE98  lfd f0, -0x1168(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4456 as u32) ) };
	// 8253B13C: 41990034  bgt cr6, 0x8253b170
	if ctx.cr[6].gt {
	pc = 0x8253B170; continue 'dispatch;
	}
	// 8253B140: 4800002C  b 0x8253b16c
	pc = 0x8253B16C; continue 'dispatch;
	// 8253B144: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B148: C9BE0000  lfd f13, 0(r30)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8253B14C: C80B2008  lfd f0, 0x2008(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B150: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8253B154: 40990010  ble cr6, 0x8253b164
	if !ctx.cr[6].gt {
	pc = 0x8253B164; continue 'dispatch;
	}
	// 8253B158: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B15C: C80BEE88  lfd f0, -0x1178(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4472 as u32) ) };
	// 8253B160: 48000010  b 0x8253b170
	pc = 0x8253B170; continue 'dispatch;
	// 8253B164: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B168: C80BEE88  lfd f0, -0x1178(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-4472 as u32) ) };
	// 8253B16C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8253B170: D81E0000  stfd f0, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 8253B174: 579C003C  rlwinm r28, r28, 0, 0, 0x1e
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B178: 480000FC  b 0x8253b274
	pc = 0x8253B274; continue 'dispatch;
	// 8253B17C: 57AB07BD  rlwinm. r11, r29, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B180: 418200F4  beq 0x8253b274
	if ctx.cr[0].eq {
	pc = 0x8253B274; continue 'dispatch;
	}
	// 8253B184: 576B06B5  rlwinm. r11, r27, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B188: 418200EC  beq 0x8253b274
	if ctx.cr[0].eq {
	pc = 0x8253B274; continue 'dispatch;
	}
	// 8253B18C: 57AB06F7  rlwinm. r11, r29, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B190: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8253B194: 41820008  beq 0x8253b19c
	if ctx.cr[0].eq {
	pc = 0x8253B19C; continue 'dispatch;
	}
	// 8253B198: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8253B19C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B1A0: C83E0000  lfd f1, 0(r30)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8253B1A4: CBEB2008  lfd f31, 0x2008(r11)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B1A8: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8253B1AC: 419A00B0  beq cr6, 0x8253b25c
	if ctx.cr[6].eq {
	pc = 0x8253B25C; continue 'dispatch;
	}
	// 8253B1B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253B1B4: 4BFFF95D  bl 0x8253ab10
	ctx.lr = 0x8253B1B8;
	sub_8253AB10(ctx, base);
	// 8253B1B8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253B1BC: D8210050  stfd f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[1].u64 ) };
	// 8253B1C0: 396BFA00  addi r11, r11, -0x600
	ctx.r[11].s64 = ctx.r[11].s64 + -1536;
	// 8253B1C4: 2F0BFBCE  cmpwi cr6, r11, -0x432
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1074, &mut ctx.xer);
	// 8253B1C8: 40980010  bge cr6, 0x8253b1d8
	if !ctx.cr[6].lt {
	pc = 0x8253B1D8; continue 'dispatch;
	}
	// 8253B1CC: FC0107F2  fmul f0, f1, f31
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[31].f64;
	// 8253B1D0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8253B1D4: 48000080  b 0x8253b254
	pc = 0x8253B254; continue 'dispatch;
	// 8253B1D8: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8253B1DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8253B1E0: 41980008  blt cr6, 0x8253b1e8
	if ctx.cr[6].lt {
	pc = 0x8253B1E8; continue 'dispatch;
	}
	// 8253B1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8253B1E8: A1410050  lhz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253B1EC: 2F0BFC03  cmpwi cr6, r11, -0x3fd
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1021, &mut ctx.xer);
	// 8253B1F0: 554A073E  clrlwi r10, r10, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 8253B1F4: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 8253B1F8: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8253B1FC: 40980048  bge cr6, 0x8253b244
	if !ctx.cr[6].lt {
	pc = 0x8253B244; continue 'dispatch;
	}
	// 8253B200: 212BFC03  subfic r9, r11, -0x3fd
	ctx.xer.ca = ctx.r[11].u32 <= -1021 as u32;
	ctx.r[9].s64 = (-1021 as i64) - ctx.r[11].s64;
	// 8253B204: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253B208: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253B20C: 556707FF  clrlwi. r7, r11, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8253B210: 41820010  beq 0x8253b220
	if ctx.cr[0].eq {
	pc = 0x8253B220; continue 'dispatch;
	}
	// 8253B214: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253B218: 409A0008  bne cr6, 0x8253b220
	if !ctx.cr[6].eq {
	pc = 0x8253B220; continue 'dispatch;
	}
	// 8253B21C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8253B220: 554707FF  clrlwi. r7, r10, 0x1f
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8253B224: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253B228: 41820008  beq 0x8253b230
	if ctx.cr[0].eq {
	pc = 0x8253B230; continue 'dispatch;
	}
	// 8253B22C: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 8253B230: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253B234: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253B238: 4082FFD4  bne 0x8253b20c
	if !ctx.cr[0].eq {
	pc = 0x8253B20C; continue 'dispatch;
	}
	// 8253B23C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8253B240: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8253B244: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8253B248: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8253B24C: 419A0008  beq cr6, 0x8253b254
	if ctx.cr[6].eq {
	pc = 0x8253B254; continue 'dispatch;
	}
	// 8253B250: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8253B254: D81E0000  stfd f0, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 8253B258: 48000008  b 0x8253b260
	pc = 0x8253B260; continue 'dispatch;
	// 8253B25C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8253B260: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253B264: 419A000C  beq cr6, 0x8253b270
	if ctx.cr[6].eq {
	pc = 0x8253B270; continue 'dispatch;
	}
	// 8253B268: 3C600800  lis r3, 0x800
	ctx.r[3].s64 = 134217728;
	// 8253B26C: 48000509  bl 0x8253b774
	ctx.lr = 0x8253B270;
	sub_8253B774(ctx, base);
	// 8253B270: 579C07FA  rlwinm r28, r28, 0, 0x1f, 0x1d
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B274: 57AB06F7  rlwinm. r11, r29, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B278: 41820018  beq 0x8253b290
	if ctx.cr[0].eq {
	pc = 0x8253B290; continue 'dispatch;
	}
	// 8253B27C: 576B0739  rlwinm. r11, r27, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B280: 41820010  beq 0x8253b290
	if ctx.cr[0].eq {
	pc = 0x8253B290; continue 'dispatch;
	}
	// 8253B284: 3C600200  lis r3, 0x200
	ctx.r[3].s64 = 33554432;
	// 8253B288: 480004ED  bl 0x8253b774
	ctx.lr = 0x8253B28C;
	sub_8253B774(ctx, base);
	// 8253B28C: 579C0734  rlwinm r28, r28, 0, 0x1c, 0x1a
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B290: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 8253B294: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8253B298: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8253B29C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8253B2A0: 4BFF9E64  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B2A8 size=72
    let mut pc: u32 = 0x8253B2A8;
    'dispatch: loop {
        match pc {
            0x8253B2A8 => {
    //   block [0x8253B2A8..0x8253B2F0)
	// 8253B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253B2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B2B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8253B2B8: 419A001C  beq cr6, 0x8253b2d4
	if ctx.cr[6].eq {
	pc = 0x8253B2D4; continue 'dispatch;
	}
	// 8253B2BC: 40990024  ble cr6, 0x8253b2e0
	if !ctx.cr[6].gt {
	pc = 0x8253B2E0; continue 'dispatch;
	}
	// 8253B2C0: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8253B2C4: 4199001C  bgt cr6, 0x8253b2e0
	if ctx.cr[6].gt {
	pc = 0x8253B2E0; continue 'dispatch;
	}
	// 8253B2C8: 4BFFF671  bl 0x8253a938
	ctx.lr = 0x8253B2CC;
	sub_8253A938(ctx, base);
	// 8253B2CC: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 8253B2D0: 4800000C  b 0x8253b2dc
	pc = 0x8253B2DC; continue 'dispatch;
	// 8253B2D4: 4BFFF665  bl 0x8253a938
	ctx.lr = 0x8253B2D8;
	sub_8253A938(ctx, base);
	// 8253B2D8: 39600021  li r11, 0x21
	ctx.r[11].s64 = 33;
	// 8253B2DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253B2E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8253B2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253B2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253B2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B2F0 size=16
    let mut pc: u32 = 0x8253B2F0;
    'dispatch: loop {
        match pc {
            0x8253B2F0 => {
    //   block [0x8253B2F0..0x8253B300)
	// 8253B2F0: 546B06B5  rlwinm. r11, r3, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B2F4: 4182000C  beq 0x8253b300
	if ctx.cr[0].eq {
		sub_8253B300(ctx, base);
		return;
	}
	// 8253B2F8: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8253B2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B300 size=16
    let mut pc: u32 = 0x8253B300;
    'dispatch: loop {
        match pc {
            0x8253B300 => {
    //   block [0x8253B300..0x8253B310)
	// 8253B300: 546B0739  rlwinm. r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B304: 4182000C  beq 0x8253b310
	if ctx.cr[0].eq {
		sub_8253B310(ctx, base);
		return;
	}
	// 8253B308: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253B30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B310 size=16
    let mut pc: u32 = 0x8253B310;
    'dispatch: loop {
        match pc {
            0x8253B310 => {
    //   block [0x8253B310..0x8253B320)
	// 8253B310: 546B077B  rlwinm. r11, r3, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B314: 4182000C  beq 0x8253b320
	if ctx.cr[0].eq {
		sub_8253B320(ctx, base);
		return;
	}
	// 8253B318: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8253B31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B320 size=16
    let mut pc: u32 = 0x8253B320;
    'dispatch: loop {
        match pc {
            0x8253B320 => {
    //   block [0x8253B320..0x8253B330)
	// 8253B320: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B324: 4182000C  beq 0x8253b330
	if ctx.cr[0].eq {
		sub_8253B330(ctx, base);
		return;
	}
	// 8253B328: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8253B32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B330 size=8
    let mut pc: u32 = 0x8253B330;
    'dispatch: loop {
        match pc {
            0x8253B330 => {
    //   block [0x8253B330..0x8253B338)
	// 8253B330: 54630F7A  rlwinm r3, r3, 1, 0x1d, 0x1d
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8253B334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B338 size=260
    let mut pc: u32 = 0x8253B338;
    'dispatch: loop {
        match pc {
            0x8253B338 => {
    //   block [0x8253B338..0x8253B43C)
	// 8253B338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253B340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253B344: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8253B348: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B34C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B350: FFE01890  fmr f31, f3
	ctx.f[31].f64 = ctx.f[3].f64;
	// 8253B354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253B358: D82100B0  stfd f1, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.f[1].u64 ) };
	// 8253B35C: 396BEEB0  addi r11, r11, -0x1150
	ctx.r[11].s64 = ctx.r[11].s64 + -4432;
	// 8253B360: D84100B8  stfd f2, 0xb8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.f[2].u64 ) };
	// 8253B364: DBE100C0  stfd f31, 0xc0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.f[31].u64 ) };
	// 8253B368: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8253B36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253B370: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253B374: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253B378: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8253B37C: 419A0088  beq cr6, 0x8253b404
	if ctx.cr[6].eq {
	pc = 0x8253B404; continue 'dispatch;
	}
	// 8253B380: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8253B384: 390B00E8  addi r8, r11, 0xe8
	ctx.r[8].s64 = ctx.r[11].s64 + 232;
	// 8253B388: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8253B38C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8253B390: 4198FFE4  blt cr6, 0x8253b374
	if ctx.cr[6].lt {
	pc = 0x8253B374; continue 'dispatch;
	}
	// 8253B394: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253B398: 3C80C007  lis r4, -0x3ff9
	ctx.r[4].s64 = -1073283072;
	// 8253B39C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8253B3A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253B3A4: 6084FEFF  ori r4, r4, 0xfeff
	ctx.r[4].u64 = ctx.r[4].u64 | 65279;
	// 8253B3A8: 419A006C  beq cr6, 0x8253b414
	if ctx.cr[6].eq {
	pc = 0x8253B414; continue 'dispatch;
	}
	// 8253B3AC: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 8253B3B0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8253B3B4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8253B3B8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8253B3BC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8253B3C0: 816100B8  lwz r11, 0xb8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 8253B3C4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8253B3C8: 816100BC  lwz r11, 0xbc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 8253B3CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8253B3D0: 816100C0  lwz r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 8253B3D4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8253B3D8: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 8253B3DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8253B3E0: 48000331  bl 0x8253b710
	ctx.lr = 0x8253B3E4;
	sub_8253B710(ctx, base);
	// 8253B3E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8253B3E8: 4BDB5FE9  bl 0x822f13d0
	ctx.lr = 0x8253B3EC;
	sub_822F13D0(ctx, base);
	// 8253B3EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253B3F0: 4082000C  bne 0x8253b3fc
	if !ctx.cr[0].eq {
	pc = 0x8253B3FC; continue 'dispatch;
	}
	// 8253B3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B3F8: 4BFFFEB1  bl 0x8253b2a8
	ctx.lr = 0x8253B3FC;
	sub_8253B2A8(ctx, base);
	// 8253B3FC: C8210068  lfd f1, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8253B400: 48000024  b 0x8253b424
	pc = 0x8253B424; continue 'dispatch;
	// 8253B404: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253B408: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8253B40C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8253B410: 4BFFFF88  b 0x8253b398
	pc = 0x8253B398; continue 'dispatch;
	// 8253B414: 480002FD  bl 0x8253b710
	ctx.lr = 0x8253B418;
	sub_8253B710(ctx, base);
	// 8253B418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B41C: 4BFFFE8D  bl 0x8253b2a8
	ctx.lr = 0x8253B420;
	sub_8253B2A8(ctx, base);
	// 8253B420: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8253B424: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8253B428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253B42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253B430: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8253B434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253B438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B440 size=136
    let mut pc: u32 = 0x8253B440;
    'dispatch: loop {
        match pc {
            0x8253B440 => {
    //   block [0x8253B440..0x8253B4C8)
	// 8253B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253B448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253B44C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8253B450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B454: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B458: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8253B45C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8253B460: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8253B464: 816BF5DC  lwz r11, -0xa24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2596 as u32) ) } as u64;
	// 8253B468: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B46C: 409A0020  bne cr6, 0x8253b48c
	if !ctx.cr[6].eq {
	pc = 0x8253B48C; continue 'dispatch;
	}
	// 8253B470: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B474: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8253B478: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8253B47C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253B480: C84B2008  lfd f2, 0x2008(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B484: 4BFFFEB5  bl 0x8253b338
	ctx.lr = 0x8253B488;
	sub_8253B338(ctx, base);
	// 8253B488: 48000028  b 0x8253b4b0
	pc = 0x8253B4B0; continue 'dispatch;
	// 8253B48C: 4BFFF4AD  bl 0x8253a938
	ctx.lr = 0x8253B490;
	sub_8253A938(ctx, base);
	// 8253B490: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253B494: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 8253B498: 3C80C007  lis r4, -0x3ff9
	ctx.r[4].s64 = -1073283072;
	// 8253B49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B4A0: 6084FEFF  ori r4, r4, 0xfeff
	ctx.r[4].u64 = ctx.r[4].u64 | 65279;
	// 8253B4A4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253B4A8: 48000269  bl 0x8253b710
	ctx.lr = 0x8253B4AC;
	sub_8253B710(ctx, base);
	// 8253B4AC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8253B4B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253B4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253B4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253B4BC: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8253B4C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253B4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B4C8 size=128
    let mut pc: u32 = 0x8253B4C8;
    'dispatch: loop {
        match pc {
            0x8253B4C8 => {
    //   block [0x8253B4C8..0x8253B548)
	// 8253B4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253B4D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253B4D4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8253B4D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B4DC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B4E0: FFE1102A  fadd f31, f1, f2
	ctx.f[31].f64 = ctx.f[1].f64 + ctx.f[2].f64;
	// 8253B4E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8253B4E8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8253B4EC: 816BF5DC  lwz r11, -0xa24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2596 as u32) ) } as u64;
	// 8253B4F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B4F4: 409A0018  bne cr6, 0x8253b50c
	if !ctx.cr[6].eq {
	pc = 0x8253B50C; continue 'dispatch;
	}
	// 8253B4F8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8253B4FC: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8253B500: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253B504: 4BFFFE35  bl 0x8253b338
	ctx.lr = 0x8253B508;
	sub_8253B338(ctx, base);
	// 8253B508: 48000028  b 0x8253b530
	pc = 0x8253B530; continue 'dispatch;
	// 8253B50C: 4BFFF42D  bl 0x8253a938
	ctx.lr = 0x8253B510;
	sub_8253A938(ctx, base);
	// 8253B510: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253B514: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 8253B518: 3C80C007  lis r4, -0x3ff9
	ctx.r[4].s64 = -1073283072;
	// 8253B51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B520: 6084FEFF  ori r4, r4, 0xfeff
	ctx.r[4].u64 = ctx.r[4].u64 | 65279;
	// 8253B524: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253B528: 480001E9  bl 0x8253b710
	ctx.lr = 0x8253B52C;
	sub_8253B710(ctx, base);
	// 8253B52C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8253B530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253B534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253B538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253B53C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8253B540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253B544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B548 size=212
    let mut pc: u32 = 0x8253B548;
    'dispatch: loop {
        match pc {
            0x8253B548 => {
    //   block [0x8253B548..0x8253B61C)
	// 8253B548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B54C: 4BFF9B71  bl 0x825350bc
	ctx.lr = 0x8253B550;
	sub_82535080(ctx, base);
	// 8253B550: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8253B554: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B558: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8253B55C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8253B560: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8253B564: DBE10110  stfd f31, 0x110(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.f[31].u64 ) };
	// 8253B568: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8253B56C: D8410118  stfd f2, 0x118(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), ctx.f[2].u64 ) };
	// 8253B570: 38810118  addi r4, r1, 0x118
	ctx.r[4].s64 = ctx.r[1].s64 + 280;
	// 8253B574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253B578: 93C10124  stw r30, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[30].u32 ) };
	// 8253B57C: 4BFFFAD5  bl 0x8253b050
	ctx.lr = 0x8253B580;
	sub_8253B050(ctx, base);
	// 8253B580: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253B584: 40820034  bne 0x8253b5b8
	if !ctx.cr[0].eq {
	pc = 0x8253B5B8; continue 'dispatch;
	}
	// 8253B588: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8253B58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253B590: 39010118  addi r8, r1, 0x118
	ctx.r[8].s64 = ctx.r[1].s64 + 280;
	// 8253B594: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 8253B598: 38E10110  addi r7, r1, 0x110
	ctx.r[7].s64 = ctx.r[1].s64 + 272;
	// 8253B59C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8253B5A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8253B5A4: 38810124  addi r4, r1, 0x124
	ctx.r[4].s64 = ctx.r[1].s64 + 292;
	// 8253B5A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8253B5AC: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8253B5B0: 4BFFF699  bl 0x8253ac48
	ctx.lr = 0x8253B5B4;
	sub_8253AC48(ctx, base);
	// 8253B5B4: 83C10124  lwz r30, 0x124(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8253B5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B5BC: 4BFFFD35  bl 0x8253b2f0
	ctx.lr = 0x8253B5C0;
	sub_8253B2F0(ctx, base);
	// 8253B5C0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B5C4: 816BF5DC  lwz r11, -0xa24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2596 as u32) ) } as u64;
	// 8253B5C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B5CC: 409A002C  bne cr6, 0x8253b5f8
	if !ctx.cr[6].eq {
	pc = 0x8253B5F8; continue 'dispatch;
	}
	// 8253B5D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253B5D4: 419A0024  beq cr6, 0x8253b5f8
	if ctx.cr[6].eq {
	pc = 0x8253B5F8; continue 'dispatch;
	}
	// 8253B5D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253B5DC: C8610118  lfd f3, 0x118(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(280 as u32) ) };
	// 8253B5E0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8253B5E4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8253B5E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8253B5EC: C84B2008  lfd f2, 0x2008(r11)
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253B5F0: 4BFFFD49  bl 0x8253b338
	ctx.lr = 0x8253B5F4;
	sub_8253B338(ctx, base);
	// 8253B5F4: 4800001C  b 0x8253b610
	pc = 0x8253B610; continue 'dispatch;
	// 8253B5F8: 4BFFFCB1  bl 0x8253b2a8
	ctx.lr = 0x8253B5FC;
	sub_8253B2A8(ctx, base);
	// 8253B5FC: 3C80C007  lis r4, -0x3ff9
	ctx.r[4].s64 = -1073283072;
	// 8253B600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253B604: 6084FEFF  ori r4, r4, 0xfeff
	ctx.r[4].u64 = ctx.r[4].u64 | 65279;
	// 8253B608: 48000109  bl 0x8253b710
	ctx.lr = 0x8253B60C;
	sub_8253B710(ctx, base);
	// 8253B60C: C8210118  lfd f1, 0x118(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(280 as u32) ) };
	// 8253B610: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8253B614: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8253B618: 4BFF9AF4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B620 size=228
    let mut pc: u32 = 0x8253B620;
    'dispatch: loop {
        match pc {
            0x8253B620 => {
    //   block [0x8253B620..0x8253B704)
	// 8253B620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B624: 4BFF9A99  bl 0x825350bc
	ctx.lr = 0x8253B628;
	sub_82535080(ctx, base);
	// 8253B628: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8253B62C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8253B630: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B634: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 8253B638: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8253B63C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8253B640: DBC10110  stfd f30, 0x110(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.f[30].u64 ) };
	// 8253B644: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8253B648: D8610120  stfd f3, 0x120(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.f[3].u64 ) };
	// 8253B64C: 38810120  addi r4, r1, 0x120
	ctx.r[4].s64 = ctx.r[1].s64 + 288;
	// 8253B650: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8253B654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253B658: 93C1012C  stw r30, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[30].u32 ) };
	// 8253B65C: 4BFFF9F5  bl 0x8253b050
	ctx.lr = 0x8253B660;
	sub_8253B050(ctx, base);
	// 8253B660: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253B664: 4082003C  bne 0x8253b6a0
	if !ctx.cr[0].eq {
	pc = 0x8253B6A0; continue 'dispatch;
	}
	// 8253B668: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 8253B66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253B670: 39010120  addi r8, r1, 0x120
	ctx.r[8].s64 = ctx.r[1].s64 + 288;
	// 8253B674: DBE10080  stfd f31, 0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.f[31].u64 ) };
	// 8253B678: 556B017E  clrlwi r11, r11, 5
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x07FFFFFFu64;
	// 8253B67C: 38E10110  addi r7, r1, 0x110
	ctx.r[7].s64 = ctx.r[1].s64 + 272;
	// 8253B680: 656B8800  oris r11, r11, 0x8800
	ctx.r[11].u64 = ctx.r[11].u64 | 2281701376;
	// 8253B684: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8253B688: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8253B68C: 3881012C  addi r4, r1, 0x12c
	ctx.r[4].s64 = ctx.r[1].s64 + 300;
	// 8253B690: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8253B694: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8253B698: 4BFFF5B1  bl 0x8253ac48
	ctx.lr = 0x8253B69C;
	sub_8253AC48(ctx, base);
	// 8253B69C: 83C1012C  lwz r30, 0x12c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 8253B6A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B6A4: 4BFFFC4D  bl 0x8253b2f0
	ctx.lr = 0x8253B6A8;
	sub_8253B2F0(ctx, base);
	// 8253B6A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253B6AC: 816BF5DC  lwz r11, -0xa24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2596 as u32) ) } as u64;
	// 8253B6B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B6B4: 409A0028  bne cr6, 0x8253b6dc
	if !ctx.cr[6].eq {
	pc = 0x8253B6DC; continue 'dispatch;
	}
	// 8253B6B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253B6BC: 419A0020  beq cr6, 0x8253b6dc
	if ctx.cr[6].eq {
	pc = 0x8253B6DC; continue 'dispatch;
	}
	// 8253B6C0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8253B6C4: C8610120  lfd f3, 0x120(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(288 as u32) ) };
	// 8253B6C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8253B6CC: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8253B6D0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8253B6D4: 4BFFFC65  bl 0x8253b338
	ctx.lr = 0x8253B6D8;
	sub_8253B338(ctx, base);
	// 8253B6D8: 4800001C  b 0x8253b6f4
	pc = 0x8253B6F4; continue 'dispatch;
	// 8253B6DC: 4BFFFBCD  bl 0x8253b2a8
	ctx.lr = 0x8253B6E0;
	sub_8253B2A8(ctx, base);
	// 8253B6E0: 3C80C007  lis r4, -0x3ff9
	ctx.r[4].s64 = -1073283072;
	// 8253B6E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253B6E8: 6084FEFF  ori r4, r4, 0xfeff
	ctx.r[4].u64 = ctx.r[4].u64 | 65279;
	// 8253B6EC: 48000025  bl 0x8253b710
	ctx.lr = 0x8253B6F0;
	sub_8253B710(ctx, base);
	// 8253B6F0: C8210120  lfd f1, 0x120(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(288 as u32) ) };
	// 8253B6F4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8253B6F8: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8253B6FC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8253B700: 4BFF9A0C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B710 size=48
    let mut pc: u32 = 0x8253B710;
    'dispatch: loop {
        match pc {
            0x8253B710 => {
    //   block [0x8253B710..0x8253B740)
	// 8253B710: FC00048E  mffs f0
	ctx.f[0].u64 = ctx.fpscr.load_from_host();
	// 8253B714: D801FFF8  stfd f0, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 8253B718: 686500F8  xori r5, r3, 0xf8
	ctx.r[5].u64 = ctx.r[3].u64 ^ 248;
	// 8253B71C: 8061FFFC  lwz r3, -4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253B720: 7CA52038  and r5, r5, r4
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[4].u64;
	// 8253B724: 7C662078  andc r6, r3, r4
	ctx.r[6].u64 = ctx.r[3].u64 & !ctx.r[4].u64;
	// 8253B728: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 8253B72C: 90C1FFFC  stw r6, -4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), ctx.r[6].u32 ) };
	// 8253B730: C801FFF8  lfd f0, -8(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8253B734: 686300F8  xori r3, r3, 0xf8
	ctx.r[3].u64 = ctx.r[3].u64 ^ 248;
	// 8253B738: FDFE058E  mtfsf 0xff, f0
	ctx.fpscr.store_from_guest(ctx.f[0].u32);
	// 8253B73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B740 size=16
    let mut pc: u32 = 0x8253B740;
    'dispatch: loop {
        match pc {
            0x8253B740 => {
    //   block [0x8253B740..0x8253B750)
	// 8253B740: FC00048E  mffs f0
	ctx.f[0].u64 = ctx.fpscr.load_from_host();
	// 8253B744: D801FFF8  stfd f0, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 8253B748: 8061FFFC  lwz r3, -4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B750 size=36
    let mut pc: u32 = 0x8253B750;
    'dispatch: loop {
        match pc {
            0x8253B750 => {
    //   block [0x8253B750..0x8253B774)
	// 8253B750: FC00048E  mffs f0
	ctx.f[0].u64 = ctx.fpscr.load_from_host();
	// 8253B754: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8253B758: D801FFF8  stfd f0, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 8253B75C: 80A1FFFC  lwz r5, -4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253B760: 7C652838  and r5, r3, r5
	ctx.r[5].u64 = ctx.r[3].u64 & ctx.r[5].u64;
	// 8253B764: 90A1FFFC  stw r5, -4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), ctx.r[5].u32 ) };
	// 8253B768: C821FFF8  lfd f1, -8(r1)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8253B76C: FDFE0D8E  mtfsf 0xff, f1
	ctx.fpscr.store_from_guest(ctx.f[1].u32);
	// 8253B770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B774 size=32
    let mut pc: u32 = 0x8253B774;
    'dispatch: loop {
        match pc {
            0x8253B774 => {
    //   block [0x8253B774..0x8253B794)
	// 8253B774: FC00048E  mffs f0
	ctx.f[0].u64 = ctx.fpscr.load_from_host();
	// 8253B778: D801FFF8  stfd f0, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 8253B77C: 80A1FFFC  lwz r5, -4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253B780: 7C652B78  or r5, r3, r5
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[5].u64;
	// 8253B784: 90A1FFFC  stw r5, -4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), ctx.r[5].u32 ) };
	// 8253B788: C821FFF8  lfd f1, -8(r1)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8253B78C: FDFE0D8E  mtfsf 0xff, f1
	ctx.fpscr.store_from_guest(ctx.f[1].u32);
	// 8253B790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B794(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B794 size=16
    let mut pc: u32 = 0x8253B794;
    'dispatch: loop {
        match pc {
            0x8253B794 => {
    //   block [0x8253B794..0x8253B7A4)
	// 8253B794: 9061FFFC  stw r3, -4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), ctx.r[3].u32 ) };
	// 8253B798: C821FFF8  lfd f1, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8253B79C: FDFE0D8E  mtfsf 0xff, f1
	ctx.fpscr.store_from_guest(ctx.f[1].u32);
	// 8253B7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B7A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253B7A4 size=16
    let mut pc: u32 = 0x8253B7A4;
    'dispatch: loop {
        match pc {
            0x8253B7A4 => {
    //   block [0x8253B7A4..0x8253B7B4)
	// 8253B7A4: FC00048E  mffs f0
	ctx.f[0].u64 = ctx.fpscr.load_from_host();
	// 8253B7A8: D801FFF8  stfd f0, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 8253B7AC: 8061FFFC  lwz r3, -4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8253B7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B7B8 size=232
    let mut pc: u32 = 0x8253B7B8;
    'dispatch: loop {
        match pc {
            0x8253B7B8 => {
    //   block [0x8253B7B8..0x8253B8A0)
	// 8253B7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B7BC: 4BFF9901  bl 0x825350bc
	ctx.lr = 0x8253B7C0;
	sub_82535080(ctx, base);
	// 8253B7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253B7C8: 480023F9  bl 0x8253dbc0
	ctx.lr = 0x8253B7CC;
	sub_8253DBC0(ctx, base);
	// 8253B7CC: 4BDB5C05  bl 0x822f13d0
	ctx.lr = 0x8253B7D0;
	sub_822F13D0(ctx, base);
	// 8253B7D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253B7D4: 418200C0  beq 0x8253b894
	if ctx.cr[0].eq {
	pc = 0x8253B894; continue 'dispatch;
	}
	// 8253B7D8: 4BFFB351  bl 0x82536b28
	ctx.lr = 0x8253B7DC;
	sub_82536B28(ctx, base);
	// 8253B7DC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8253B7E0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253B7E4: 409A000C  bne cr6, 0x8253b7f0
	if !ctx.cr[6].eq {
	pc = 0x8253B7F0; continue 'dispatch;
	}
	// 8253B7E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253B7EC: 48000018  b 0x8253b804
	pc = 0x8253B804; continue 'dispatch;
	// 8253B7F0: 4BFFB339  bl 0x82536b28
	ctx.lr = 0x8253B7F4;
	sub_82536B28(ctx, base);
	// 8253B7F4: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 8253B7F8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253B7FC: 409A0098  bne cr6, 0x8253b894
	if !ctx.cr[6].eq {
	pc = 0x8253B894; continue 'dispatch;
	}
	// 8253B800: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8253B804: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8253B808: 814B2D64  lwz r10, 0x2d64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11620 as u32) ) } as u64;
	// 8253B80C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253B810: 914B2D64  stw r10, 0x2d64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(11620 as u32), ctx.r[10].u32 ) };
	// 8253B814: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253B818: 716B010C  andi. r11, r11, 0x10c
	ctx.r[11].u64 = ctx.r[11].u64 & 268;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B81C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253B820: 40820074  bne 0x8253b894
	if !ctx.cr[0].eq {
	pc = 0x8253B894; continue 'dispatch;
	}
	// 8253B824: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8253B828: 553D103A  slwi r29, r9, 2
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8253B82C: 3BCB2D94  addi r30, r11, 0x2d94
	ctx.r[30].s64 = ctx.r[11].s64 + 11668;
	// 8253B830: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8253B834: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253B838: 40820034  bne 0x8253b86c
	if !ctx.cr[0].eq {
	pc = 0x8253B86C; continue 'dispatch;
	}
	// 8253B83C: 38601000  li r3, 0x1000
	ctx.r[3].s64 = 4096;
	// 8253B840: 4BFF8431  bl 0x82533c70
	ctx.lr = 0x8253B844;
	sub_82533C70(ctx, base);
	// 8253B844: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253B848: 7C7DF12E  stwx r3, r29, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 8253B84C: 40820020  bne 0x8253b86c
	if !ctx.cr[0].eq {
	pc = 0x8253B86C; continue 'dispatch;
	}
	// 8253B850: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 8253B854: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8253B858: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8253B85C: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8253B860: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8253B864: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253B868: 48000018  b 0x8253b880
	pc = 0x8253B880; continue 'dispatch;
	// 8253B86C: 39601000  li r11, 0x1000
	ctx.r[11].s64 = 4096;
	// 8253B870: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8253B874: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8253B878: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8253B87C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8253B880: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253B884: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253B888: 616B1102  ori r11, r11, 0x1102
	ctx.r[11].u64 = ctx.r[11].u64 | 4354;
	// 8253B88C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8253B890: 48000008  b 0x8253b898
	pc = 0x8253B898; continue 'dispatch;
	// 8253B894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253B898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253B89C: 4BFF9870  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B8A0 size=100
    let mut pc: u32 = 0x8253B8A0;
    'dispatch: loop {
        match pc {
            0x8253B8A0 => {
    //   block [0x8253B8A0..0x8253B904)
	// 8253B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253B8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253B8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B8B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253B8B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253B8B8: 419A0038  beq cr6, 0x8253b8f0
	if ctx.cr[6].eq {
	pc = 0x8253B8F0; continue 'dispatch;
	}
	// 8253B8BC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253B8C0: 556B04E7  rlwinm. r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B8C4: 4182002C  beq 0x8253b8f0
	if ctx.cr[0].eq {
	pc = 0x8253B8F0; continue 'dispatch;
	}
	// 8253B8C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253B8CC: 4BFFB4FD  bl 0x82536dc8
	ctx.lr = 0x8253B8D0;
	sub_82536DC8(ctx, base);
	// 8253B8D0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253B8D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253B8D8: 554A062C  rlwinm r10, r10, 0, 0x18, 0x16
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B8DC: 554A0524  rlwinm r10, r10, 0, 0x14, 0x12
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253B8E0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8253B8E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253B8E8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8253B8EC: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8253B8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8253B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253B8FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253B900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B908 size=180
    let mut pc: u32 = 0x8253B908;
    'dispatch: loop {
        match pc {
            0x8253B908 => {
    //   block [0x8253B908..0x8253B9BC)
	// 8253B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B90C: 4BFF97A9  bl 0x825350b4
	ctx.lr = 0x8253B910;
	sub_82535080(ctx, base);
	// 8253B910: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B914: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8253B918: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8253B91C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8253B920: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8253B924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253B928: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8253B92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253B930: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253B934: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8253B938: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253B93C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8253B940: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8253B944: 4800007D  bl 0x8253b9c0
	ctx.lr = 0x8253B948;
	sub_8253B9C0(ctx, base);
	// 8253B948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253B94C: 57EB077B  rlwinm. r11, r31, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B950: 41820014  beq 0x8253b964
	if ctx.cr[0].eq {
	pc = 0x8253B964; continue 'dispatch;
	}
	// 8253B954: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 8253B958: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8253B95C: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 8253B960: 48000038  b 0x8253b998
	pc = 0x8253B998; continue 'dispatch;
	// 8253B964: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8253B968: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8253B96C: 480071DD  bl 0x82542b48
	ctx.lr = 0x8253B970;
	sub_82542B48(ctx, base);
	// 8253B970: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B974: 4082000C  bne 0x8253b980
	if !ctx.cr[0].eq {
	pc = 0x8253B980; continue 'dispatch;
	}
	// 8253B978: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8253B97C: 409A0008  bne cr6, 0x8253b984
	if !ctx.cr[6].eq {
	pc = 0x8253B984; continue 'dispatch;
	}
	// 8253B980: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	// 8253B984: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253B988: 4082000C  bne 0x8253b994
	if !ctx.cr[0].eq {
	pc = 0x8253B994; continue 'dispatch;
	}
	// 8253B98C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8253B990: 409A0008  bne cr6, 0x8253b998
	if !ctx.cr[6].eq {
	pc = 0x8253B998; continue 'dispatch;
	}
	// 8253B994: 63DE0100  ori r30, r30, 0x100
	ctx.r[30].u64 = ctx.r[30].u64 | 256;
	// 8253B998: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253B99C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8253B9A0: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8253B9A4: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8253B9A8: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8253B9AC: F95D0010  std r10, 0x10(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8253B9B0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8253B9B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8253B9B8: 4BFF974C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253B9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253B9C0 size=2432
    let mut pc: u32 = 0x8253B9C0;
    'dispatch: loop {
        match pc {
            0x8253B9C0 => {
    //   block [0x8253B9C0..0x8253C340)
	// 8253B9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253B9C4: 4BFF96C9  bl 0x8253508c
	ctx.lr = 0x8253B9C8;
	sub_82535080(ctx, base);
	// 8253B9C8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253B9CC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8253B9D0: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 8253B9D4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8253B9D8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8253B9DC: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 8253B9E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8253B9E4: 7EB2AB78  mr r18, r21
	ctx.r[18].u64 = ctx.r[21].u64;
	// 8253B9E8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8253B9EC: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 8253B9F0: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 8253B9F4: 7EBAAB78  mr r26, r21
	ctx.r[26].u64 = ctx.r[21].u64;
	// 8253B9F8: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 8253B9FC: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 8253BA00: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 8253BA04: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8253BA08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253BA0C: 409A0034  bne cr6, 0x8253ba40
	if !ctx.cr[6].eq {
	pc = 0x8253BA40; continue 'dispatch;
	}
	// 8253BA10: 4BFFEF29  bl 0x8253a938
	ctx.lr = 0x8253BA14;
	sub_8253A938(ctx, base);
	// 8253BA14: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253BA18: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253BA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253BA20: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253BA24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253BA28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253BA2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253BA30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253BA34: 4BFFEDCD  bl 0x8253a800
	ctx.lr = 0x8253BA38;
	sub_8253A800(ctx, base);
	// 8253BA38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253BA3C: 480008FC  b 0x8253c338
	pc = 0x8253C338; continue 'dispatch;
	// 8253BA40: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 8253BA44: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BA48: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253BA4C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8253BA50: 419A001C  beq cr6, 0x8253ba6c
	if ctx.cr[6].eq {
	pc = 0x8253BA6C; continue 'dispatch;
	}
	// 8253BA54: 2F0A0009  cmpwi cr6, r10, 9
	ctx.cr[6].compare_i32(ctx.r[10].s32, 9, &mut ctx.xer);
	// 8253BA58: 419A0014  beq cr6, 0x8253ba6c
	if ctx.cr[6].eq {
	pc = 0x8253BA6C; continue 'dispatch;
	}
	// 8253BA5C: 2F0A000A  cmpwi cr6, r10, 0xa
	ctx.cr[6].compare_i32(ctx.r[10].s32, 10, &mut ctx.xer);
	// 8253BA60: 419A000C  beq cr6, 0x8253ba6c
	if ctx.cr[6].eq {
	pc = 0x8253BA6C; continue 'dispatch;
	}
	// 8253BA64: 2F0A000D  cmpwi cr6, r10, 0xd
	ctx.cr[6].compare_i32(ctx.r[10].s32, 13, &mut ctx.xer);
	// 8253BA68: 409A000C  bne cr6, 0x8253ba74
	if !ctx.cr[6].eq {
	pc = 0x8253BA74; continue 'dispatch;
	}
	// 8253BA6C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BA70: 4BFFFFD4  b 0x8253ba44
	pc = 0x8253BA44; continue 'dispatch;
	// 8253BA74: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8253BA78: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 8253BA7C: 60F48000  ori r20, r7, 0x8000
	ctx.r[20].u64 = ctx.r[7].u64 | 32768;
	// 8253BA80: 80EAF5D8  lwz r7, -0xa28(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2600 as u32) ) } as u64;
	// 8253BA84: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BA88: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 8253BA8C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BA90: 4199036C  bgt cr6, 0x8253bdfc
	if ctx.cr[6].gt {
	pc = 0x8253BDFC; continue 'dispatch;
	}
	// 8253BA94: 3D808206  lis r12, -0x7dfa
	ctx.r[12].s64 = -2113536000;
	// 8253BA98: 398C7818  addi r12, r12, 0x7818
	ctx.r[12].s64 = ctx.r[12].s64 + 30744;
	// 8253BA9C: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8253BAA0: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8253BAA4: 3D808254  lis r12, -0x7dac
	ctx.r[12].s64 = -2108424192;
	// 8253BAA8: 398CBABC  addi r12, r12, -0x4544
	ctx.r[12].s64 = ctx.r[12].s64 + -17732;
	// 8253BAAC: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 8253BAB0: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 8253BAB4: 60000000  nop
	// 8253BAB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 8253BABC: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BAC0: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8253BAC4: 41980018  blt cr6, 0x8253badc
	if ctx.cr[6].lt {
	pc = 0x8253BADC; continue 'dispatch;
	}
	// 8253BAC8: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BACC: 41990010  bgt cr6, 0x8253badc
	if ctx.cr[6].gt {
	pc = 0x8253BADC; continue 'dispatch;
	}
	// 8253BAD0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8253BAD4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8253BAD8: 4BFFFFAC  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BADC: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BAE0: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BAE4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253BAE8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8253BAEC: 409A000C  bne cr6, 0x8253baf8
	if !ctx.cr[6].eq {
	pc = 0x8253BAF8; continue 'dispatch;
	}
	// 8253BAF0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8253BAF4: 4BFFFF90  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BAF8: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8253BAFC: 419A0028  beq cr6, 0x8253bb24
	if ctx.cr[6].eq {
	pc = 0x8253BB24; continue 'dispatch;
	}
	// 8253BB00: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8253BB04: 419A0014  beq cr6, 0x8253bb18
	if ctx.cr[6].eq {
	pc = 0x8253BB18; continue 'dispatch;
	}
	// 8253BB08: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BB0C: 409A029C  bne cr6, 0x8253bda8
	if !ctx.cr[6].eq {
	pc = 0x8253BDA8; continue 'dispatch;
	}
	// 8253BB10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8253BB14: 4BFFFF70  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BB18: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8253BB1C: 7E92A378  mr r18, r20
	ctx.r[18].u64 = ctx.r[20].u64;
	// 8253BB20: 4BFFFF64  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BB24: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8253BB28: 7EB2AB78  mr r18, r21
	ctx.r[18].u64 = ctx.r[21].u64;
	// 8253BB2C: 4BFFFF58  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BB30: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BB34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8253BB38: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8253BB3C: 4198000C  blt cr6, 0x8253bb48
	if ctx.cr[6].lt {
	pc = 0x8253BB48; continue 'dispatch;
	}
	// 8253BB40: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BB44: 4099FF8C  ble cr6, 0x8253bad0
	if !ctx.cr[6].gt {
	pc = 0x8253BAD0; continue 'dispatch;
	}
	// 8253BB48: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BB4C: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BB50: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253BB54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8253BB58: 409A000C  bne cr6, 0x8253bb64
	if !ctx.cr[6].eq {
	pc = 0x8253BB64; continue 'dispatch;
	}
	// 8253BB5C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8253BB60: 4BFFFF24  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BB64: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8253BB68: 419A0038  beq cr6, 0x8253bba0
	if ctx.cr[6].eq {
	pc = 0x8253BBA0; continue 'dispatch;
	}
	// 8253BB6C: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8253BB70: 419A0030  beq cr6, 0x8253bba0
	if ctx.cr[6].eq {
	pc = 0x8253BBA0; continue 'dispatch;
	}
	// 8253BB74: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BB78: 419AFF98  beq cr6, 0x8253bb10
	if ctx.cr[6].eq {
	pc = 0x8253BB10; continue 'dispatch;
	}
	// 8253BB7C: 2F0B0043  cmpwi cr6, r11, 0x43
	ctx.cr[6].compare_i32(ctx.r[11].s32, 67, &mut ctx.xer);
	// 8253BB80: 40990228  ble cr6, 0x8253bda8
	if !ctx.cr[6].gt {
	pc = 0x8253BDA8; continue 'dispatch;
	}
	// 8253BB84: 2F0B0045  cmpwi cr6, r11, 0x45
	ctx.cr[6].compare_i32(ctx.r[11].s32, 69, &mut ctx.xer);
	// 8253BB88: 40990010  ble cr6, 0x8253bb98
	if !ctx.cr[6].gt {
	pc = 0x8253BB98; continue 'dispatch;
	}
	// 8253BB8C: 396BFF9C  addi r11, r11, -0x64
	ctx.r[11].s64 = ctx.r[11].s64 + -100;
	// 8253BB90: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8253BB94: 41990214  bgt cr6, 0x8253bda8
	if ctx.cr[6].gt {
	pc = 0x8253BDA8; continue 'dispatch;
	}
	// 8253BB98: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8253BB9C: 4BFFFEE8  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BBA0: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8253BBA4: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 8253BBA8: 4BFFFEDC  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BBAC: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BBB0: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8253BBB4: 4198000C  blt cr6, 0x8253bbc0
	if ctx.cr[6].lt {
	pc = 0x8253BBC0; continue 'dispatch;
	}
	// 8253BBB8: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BBBC: 4099FF14  ble cr6, 0x8253bad0
	if !ctx.cr[6].gt {
	pc = 0x8253BAD0; continue 'dispatch;
	}
	// 8253BBC0: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BBC4: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BBC8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253BBCC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8253BBD0: 419AFF20  beq cr6, 0x8253baf0
	if ctx.cr[6].eq {
	pc = 0x8253BAF0; continue 'dispatch;
	}
	// 8253BBD4: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BBD8: 419AFF38  beq cr6, 0x8253bb10
	if ctx.cr[6].eq {
	pc = 0x8253BB10; continue 'dispatch;
	}
	// 8253BBDC: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 8253BBE0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8253BBE4: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8253BBE8: 419A06EC  beq cr6, 0x8253c2d4
	if ctx.cr[6].eq {
	pc = 0x8253C2D4; continue 'dispatch;
	}
	// 8253BBEC: 2B060018  cmplwi cr6, r6, 0x18
	ctx.cr[6].compare_u32(ctx.r[6].u32, 24 as u32, &mut ctx.xer);
	// 8253BBF0: 40990028  ble cr6, 0x8253bc18
	if !ctx.cr[6].gt {
	pc = 0x8253BC18; continue 'dispatch;
	}
	// 8253BBF4: 89610097  lbz r11, 0x97(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(151 as u32) ) } as u64;
	// 8253BBF8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8253BBFC: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8253BC00: 4198000C  blt cr6, 0x8253bc0c
	if ctx.cr[6].lt {
	pc = 0x8253BC0C; continue 'dispatch;
	}
	// 8253BC04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253BC08: 99610097  stb r11, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[11].u8 ) };
	// 8253BC0C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8253BC10: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8253BC14: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253BC18: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8253BC1C: 419A06A4  beq cr6, 0x8253c2c0
	if ctx.cr[6].eq {
	pc = 0x8253C2C0; continue 'dispatch;
	}
	// 8253BC20: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8253BC24: 48000260  b 0x8253be84
	pc = 0x8253BE84; continue 'dispatch;
	// 8253BC28: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8253BC2C: 48000034  b 0x8253bc60
	pc = 0x8253BC60; continue 'dispatch;
	// 8253BC30: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BC34: 41990038  bgt cr6, 0x8253bc6c
	if ctx.cr[6].gt {
	pc = 0x8253BC6C; continue 'dispatch;
	}
	// 8253BC38: 2B060019  cmplwi cr6, r6, 0x19
	ctx.cr[6].compare_u32(ctx.r[6].u32, 25 as u32, &mut ctx.xer);
	// 8253BC3C: 40980018  bge cr6, 0x8253bc54
	if !ctx.cr[6].lt {
	pc = 0x8253BC54; continue 'dispatch;
	}
	// 8253BC40: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 8253BC44: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8253BC48: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253BC4C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8253BC50: 48000008  b 0x8253bc58
	pc = 0x8253BC58; continue 'dispatch;
	// 8253BC54: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253BC58: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BC5C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BC60: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BC64: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BC68: 4098FFC8  bge cr6, 0x8253bc30
	if !ctx.cr[6].lt {
	pc = 0x8253BC30; continue 'dispatch;
	}
	// 8253BC6C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BC70: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BC74: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BC78: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253BC7C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8253BC80: 419AFEDC  beq cr6, 0x8253bb5c
	if ctx.cr[6].eq {
	pc = 0x8253BB5C; continue 'dispatch;
	}
	// 8253BC84: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8253BC88: 419AFF18  beq cr6, 0x8253bba0
	if ctx.cr[6].eq {
	pc = 0x8253BBA0; continue 'dispatch;
	}
	// 8253BC8C: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8253BC90: 419AFF10  beq cr6, 0x8253bba0
	if ctx.cr[6].eq {
	pc = 0x8253BBA0; continue 'dispatch;
	}
	// 8253BC94: 4BFFFEE8  b 0x8253bb7c
	pc = 0x8253BB7C; continue 'dispatch;
	// 8253BC98: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8253BC9C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8253BCA0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8253BCA4: 409A0054  bne cr6, 0x8253bcf8
	if !ctx.cr[6].eq {
	pc = 0x8253BCF8; continue 'dispatch;
	}
	// 8253BCA8: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BCAC: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BCB0: 409A0048  bne cr6, 0x8253bcf8
	if !ctx.cr[6].eq {
	pc = 0x8253BCF8; continue 'dispatch;
	}
	// 8253BCB4: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BCB8: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8253BCBC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BCC0: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 8253BCC4: 419AFFF0  beq cr6, 0x8253bcb4
	if ctx.cr[6].eq {
	pc = 0x8253BCB4; continue 'dispatch;
	}
	// 8253BCC8: 48000030  b 0x8253bcf8
	pc = 0x8253BCF8; continue 'dispatch;
	// 8253BCCC: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BCD0: 41990034  bgt cr6, 0x8253bd04
	if ctx.cr[6].gt {
	pc = 0x8253BD04; continue 'dispatch;
	}
	// 8253BCD4: 2B060019  cmplwi cr6, r6, 0x19
	ctx.cr[6].compare_u32(ctx.r[6].u32, 25 as u32, &mut ctx.xer);
	// 8253BCD8: 40980018  bge cr6, 0x8253bcf0
	if !ctx.cr[6].lt {
	pc = 0x8253BCF0; continue 'dispatch;
	}
	// 8253BCDC: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 8253BCE0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8253BCE4: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8253BCE8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253BCEC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8253BCF0: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BCF4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BCF8: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BCFC: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BD00: 4098FFCC  bge cr6, 0x8253bccc
	if !ctx.cr[6].lt {
	pc = 0x8253BCCC; continue 'dispatch;
	}
	// 8253BD04: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BD08: 4BFFFF7C  b 0x8253bc84
	pc = 0x8253BC84; continue 'dispatch;
	// 8253BD0C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BD10: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8253BD14: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BD18: 4198FEC4  blt cr6, 0x8253bbdc
	if ctx.cr[6].lt {
	pc = 0x8253BBDC; continue 'dispatch;
	}
	// 8253BD1C: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BD20: 4199FEBC  bgt cr6, 0x8253bbdc
	if ctx.cr[6].gt {
	pc = 0x8253BBDC; continue 'dispatch;
	}
	// 8253BD24: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8253BD28: 4BFFFDAC  b 0x8253bad4
	pc = 0x8253BAD4; continue 'dispatch;
	// 8253BD2C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BD30: 38A8FFFE  addi r5, r8, -2
	ctx.r[5].s64 = ctx.r[8].s64 + -2;
	// 8253BD34: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8253BD38: 41980014  blt cr6, 0x8253bd4c
	if ctx.cr[6].lt {
	pc = 0x8253BD4C; continue 'dispatch;
	}
	// 8253BD3C: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BD40: 4199000C  bgt cr6, 0x8253bd4c
	if ctx.cr[6].gt {
	pc = 0x8253BD4C; continue 'dispatch;
	}
	// 8253BD44: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 8253BD48: 4BFFFD8C  b 0x8253bad4
	pc = 0x8253BAD4; continue 'dispatch;
	// 8253BD4C: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8253BD50: 419A001C  beq cr6, 0x8253bd6c
	if ctx.cr[6].eq {
	pc = 0x8253BD6C; continue 'dispatch;
	}
	// 8253BD54: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8253BD58: 419A0090  beq cr6, 0x8253bde8
	if ctx.cr[6].eq {
	pc = 0x8253BDE8; continue 'dispatch;
	}
	// 8253BD5C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BD60: 409AFE7C  bne cr6, 0x8253bbdc
	if !ctx.cr[6].eq {
	pc = 0x8253BBDC; continue 'dispatch;
	}
	// 8253BD64: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8253BD68: 4BFFFD1C  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BD6C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8253BD70: 4BFFFD14  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BD74: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BD78: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8253BD7C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BD80: 409A0014  bne cr6, 0x8253bd94
	if !ctx.cr[6].eq {
	pc = 0x8253BD94; continue 'dispatch;
	}
	// 8253BD84: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BD88: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BD8C: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 8253BD90: 419AFFF4  beq cr6, 0x8253bd84
	if ctx.cr[6].eq {
	pc = 0x8253BD84; continue 'dispatch;
	}
	// 8253BD94: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BD98: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8253BD9C: 4198000C  blt cr6, 0x8253bda8
	if ctx.cr[6].lt {
	pc = 0x8253BDA8; continue 'dispatch;
	}
	// 8253BDA0: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BDA4: 4099FFA0  ble cr6, 0x8253bd44
	if !ctx.cr[6].gt {
	pc = 0x8253BD44; continue 'dispatch;
	}
	// 8253BDA8: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8253BDAC: 4BFFFE34  b 0x8253bbe0
	pc = 0x8253BBE0; continue 'dispatch;
	// 8253BDB0: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BDB4: 2F0B0031  cmpwi cr6, r11, 0x31
	ctx.cr[6].compare_i32(ctx.r[11].s32, 49, &mut ctx.xer);
	// 8253BDB8: 4198FFA4  blt cr6, 0x8253bd5c
	if ctx.cr[6].lt {
	pc = 0x8253BD5C; continue 'dispatch;
	}
	// 8253BDBC: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BDC0: 4099FF84  ble cr6, 0x8253bd44
	if !ctx.cr[6].gt {
	pc = 0x8253BD44; continue 'dispatch;
	}
	// 8253BDC4: 4BFFFF98  b 0x8253bd5c
	pc = 0x8253BD5C; continue 'dispatch;
	// 8253BDC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253BDCC: 419A0028  beq cr6, 0x8253bdf4
	if ctx.cr[6].eq {
	pc = 0x8253BDF4; continue 'dispatch;
	}
	// 8253BDD0: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BDD4: 38A8FFFF  addi r5, r8, -1
	ctx.r[5].s64 = ctx.r[8].s64 + -1;
	// 8253BDD8: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8253BDDC: 419AFF90  beq cr6, 0x8253bd6c
	if ctx.cr[6].eq {
	pc = 0x8253BD6C; continue 'dispatch;
	}
	// 8253BDE0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8253BDE4: 409AFDF8  bne cr6, 0x8253bbdc
	if !ctx.cr[6].eq {
	pc = 0x8253BBDC; continue 'dispatch;
	}
	// 8253BDE8: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8253BDEC: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 8253BDF0: 4BFFFC94  b 0x8253ba84
	pc = 0x8253BA84; continue 'dispatch;
	// 8253BDF4: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 8253BDF8: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8253BDFC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8253BE00: 409AFC84  bne cr6, 0x8253ba84
	if !ctx.cr[6].eq {
	pc = 0x8253BA84; continue 'dispatch;
	}
	// 8253BE04: 4BFFFDDC  b 0x8253bbe0
	pc = 0x8253BBE0; continue 'dispatch;
	// 8253BE08: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8253BE0C: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8253BE10: 48000028  b 0x8253be38
	pc = 0x8253BE38; continue 'dispatch;
	// 8253BE14: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BE18: 41990034  bgt cr6, 0x8253be4c
	if ctx.cr[6].gt {
	pc = 0x8253BE4C; continue 'dispatch;
	}
	// 8253BE1C: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 8253BE20: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8253BE24: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 8253BE28: 2F091450  cmpwi cr6, r9, 0x1450
	ctx.cr[6].compare_i32(ctx.r[9].s32, 5200, &mut ctx.xer);
	// 8253BE2C: 4199001C  bgt cr6, 0x8253be48
	if ctx.cr[6].gt {
	pc = 0x8253BE48; continue 'dispatch;
	}
	// 8253BE30: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BE34: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BE38: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BE3C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BE40: 4098FFD4  bge cr6, 0x8253be14
	if !ctx.cr[6].lt {
	pc = 0x8253BE14; continue 'dispatch;
	}
	// 8253BE44: 48000008  b 0x8253be4c
	pc = 0x8253BE4C; continue 'dispatch;
	// 8253BE48: 39201451  li r9, 0x1451
	ctx.r[9].s64 = 5201;
	// 8253BE4C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8253BE50: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 8253BE54: 48000018  b 0x8253be6c
	pc = 0x8253BE6C; continue 'dispatch;
	// 8253BE58: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8253BE5C: 4199FF4C  bgt cr6, 0x8253bda8
	if ctx.cr[6].gt {
	pc = 0x8253BDA8; continue 'dispatch;
	}
	// 8253BE60: 89680000  lbz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BE64: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253BE68: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8253BE6C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8253BE70: 4098FFE8  bge cr6, 0x8253be58
	if !ctx.cr[6].lt {
	pc = 0x8253BE58; continue 'dispatch;
	}
	// 8253BE74: 4BFFFF34  b 0x8253bda8
	pc = 0x8253BDA8; continue 'dispatch;
	// 8253BE78: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253BE7C: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8253BE80: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253BE84: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BE88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253BE8C: 419AFFEC  beq cr6, 0x8253be78
	if ctx.cr[6].eq {
	pc = 0x8253BE78; continue 'dispatch;
	}
	// 8253BE90: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8253BE94: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8253BE98: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8253BE9C: 48007ADD  bl 0x82543978
	ctx.lr = 0x8253BEA0;
	sub_82543978(ctx, base);
	// 8253BEA0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8253BEA4: 40980008  bge cr6, 0x8253beac
	if !ctx.cr[6].lt {
	pc = 0x8253BEAC; continue 'dispatch;
	}
	// 8253BEA8: 7FBD00D0  neg r29, r29
	ctx.r[29].s64 = -ctx.r[29].s64;
	// 8253BEAC: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 8253BEB0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8253BEB4: 409A0008  bne cr6, 0x8253bebc
	if !ctx.cr[6].eq {
	pc = 0x8253BEBC; continue 'dispatch;
	}
	// 8253BEB8: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8253BEBC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8253BEC0: 409A0008  bne cr6, 0x8253bec8
	if !ctx.cr[6].eq {
	pc = 0x8253BEC8; continue 'dispatch;
	}
	// 8253BEC4: 7D785850  subf r11, r24, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[24].s64;
	// 8253BEC8: 2F0B1450  cmpwi cr6, r11, 0x1450
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5200, &mut ctx.xer);
	// 8253BECC: 41990420  bgt cr6, 0x8253c2ec
	if ctx.cr[6].gt {
	pc = 0x8253C2EC; continue 'dispatch;
	}
	// 8253BED0: 2F0BEBB0  cmpwi cr6, r11, -0x1450
	ctx.cr[6].compare_i32(ctx.r[11].s32, -5200, &mut ctx.xer);
	// 8253BED4: 41980430  blt cr6, 0x8253c304
	if ctx.cr[6].lt {
	pc = 0x8253C304; continue 'dispatch;
	}
	// 8253BED8: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8253BEDC: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 8253BEE0: 394AF610  addi r10, r10, -0x9f0
	ctx.r[10].s64 = ctx.r[10].s64 + -2544;
	// 8253BEE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253BEE8: 3B2AFFA0  addi r25, r10, -0x60
	ctx.r[25].s64 = ctx.r[10].s64 + -96;
	// 8253BEEC: 419A03C0  beq cr6, 0x8253c2ac
	if ctx.cr[6].eq {
	pc = 0x8253C2AC; continue 'dispatch;
	}
	// 8253BEF0: 40980014  bge cr6, 0x8253bf04
	if !ctx.cr[6].lt {
	pc = 0x8253BF04; continue 'dispatch;
	}
	// 8253BEF4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8253BEF8: 7F0B00D0  neg r24, r11
	ctx.r[24].s64 = -ctx.r[11].s64;
	// 8253BEFC: 394AF770  addi r10, r10, -0x890
	ctx.r[10].s64 = ctx.r[10].s64 + -2192;
	// 8253BF00: 3B2AFFA0  addi r25, r10, -0x60
	ctx.r[25].s64 = ctx.r[10].s64 + -96;
	// 8253BF04: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 8253BF08: 409A0008  bne cr6, 0x8253bf10
	if !ctx.cr[6].eq {
	pc = 0x8253BF10; continue 'dispatch;
	}
	// 8253BF0C: B2A1006A  sth r21, 0x6a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[21].u16 ) };
	// 8253BF10: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8253BF14: 419A0398  beq cr6, 0x8253c2ac
	if ctx.cr[6].eq {
	pc = 0x8253C2AC; continue 'dispatch;
	}
	// 8253BF18: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8253BF1C: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 8253BF20: 617AFFFF  ori r26, r11, 0xffff
	ctx.r[26].u64 = ctx.r[11].u64 | 65535;
	// 8253BF24: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8253BF28: 3AE08000  li r23, -0x8000
	ctx.r[23].s64 = -32768;
	// 8253BF2C: 61718000  ori r17, r11, 0x8000
	ctx.r[17].u64 = ctx.r[11].u64 | 32768;
	// 8253BF30: 570B077F  clrlwi. r11, r24, 0x1d
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253BF34: 3B390054  addi r25, r25, 0x54
	ctx.r[25].s64 = ctx.r[25].s64 + 84;
	// 8253BF38: 7F181E70  srawi r24, r24, 3
	ctx.xer.ca = (ctx.r[24].s32 < 0) && ((ctx.r[24].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[24].s32 >> 3) as i64;
	// 8253BF3C: 41820368  beq 0x8253c2a4
	if ctx.cr[0].eq {
	pc = 0x8253C2A4; continue 'dispatch;
	}
	// 8253BF40: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 8253BF44: 7C8BCA14  add r4, r11, r25
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8253BF48: A164000A  lhz r11, 0xa(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(10 as u32) ) } as u64;
	// 8253BF4C: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 8253BF50: 41980020  blt cr6, 0x8253bf70
	if ctx.cr[6].lt {
	pc = 0x8253BF70; continue 'dispatch;
	}
	// 8253BF54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8253BF58: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8253BF5C: 4BFF8BF5  bl 0x82534b50
	ctx.lr = 0x8253BF60;
	sub_82534B50(ctx, base);
	// 8253BF60: 81610076  lwz r11, 0x76(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(118 as u32) ) } as u64;
	// 8253BF64: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8253BF68: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253BF6C: 91610076  stw r11, 0x76(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[11].u32 ) };
	// 8253BF70: 92A10058  stw r21, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[21].u32 ) };
	// 8253BF74: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 8253BF78: 92A10054  stw r21, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[21].u32 ) };
	// 8253BF7C: 92A10050  stw r21, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[21].u32 ) };
	// 8253BF80: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253BF84: A1610060  lhz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253BF88: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8253BF8C: 7D6A5278  xor r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 ^ ctx.r[10].u64;
	// 8253BF90: 556B047E  clrlwi r11, r11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 8253BF94: 555B0420  rlwinm r27, r10, 0, 0x10, 0x10
	ctx.r[27].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253BF98: 550A047E  clrlwi r10, r8, 0x11
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x00007FFFu64;
	// 8253BF9C: 2B0B7FFF  cmplwi cr6, r11, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32767 as u32, &mut ctx.xer);
	// 8253BFA0: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8253BFA4: 553D043E  clrlwi r29, r9, 0x10
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 8253BFA8: 409802E4  bge cr6, 0x8253c28c
	if !ctx.cr[6].lt {
	pc = 0x8253C28C; continue 'dispatch;
	}
	// 8253BFAC: 2B0A7FFF  cmplwi cr6, r10, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32767 as u32, &mut ctx.xer);
	// 8253BFB0: 409802DC  bge cr6, 0x8253c28c
	if !ctx.cr[6].lt {
	pc = 0x8253C28C; continue 'dispatch;
	}
	// 8253BFB4: 57A9043E  clrlwi r9, r29, 0x10
	ctx.r[9].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8253BFB8: 2B09BFFD  cmplwi cr6, r9, 0xbffd
	ctx.cr[6].compare_u32(ctx.r[9].u32, 49149 as u32, &mut ctx.xer);
	// 8253BFBC: 419902D0  bgt cr6, 0x8253c28c
	if ctx.cr[6].gt {
	pc = 0x8253C28C; continue 'dispatch;
	}
	// 8253BFC0: 2B093FBF  cmplwi cr6, r9, 0x3fbf
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16319 as u32, &mut ctx.xer);
	// 8253BFC4: 4199000C  bgt cr6, 0x8253bfd0
	if ctx.cr[6].gt {
	pc = 0x8253BFD0; continue 'dispatch;
	}
	// 8253BFC8: 92A10060  stw r21, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[21].u32 ) };
	// 8253BFCC: 480002D0  b 0x8253c29c
	pc = 0x8253C29C; continue 'dispatch;
	// 8253BFD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253BFD4: 409A0038  bne cr6, 0x8253c00c
	if !ctx.cr[6].eq {
	pc = 0x8253C00C; continue 'dispatch;
	}
	// 8253BFD8: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 8253BFDC: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253BFE0: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253BFE4: 5529007F  clrlwi. r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253BFE8: 40820024  bne 0x8253c00c
	if !ctx.cr[0].eq {
	pc = 0x8253C00C; continue 'dispatch;
	}
	// 8253BFEC: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8253BFF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253BFF4: 409A0018  bne cr6, 0x8253c00c
	if !ctx.cr[6].eq {
	pc = 0x8253C00C; continue 'dispatch;
	}
	// 8253BFF8: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8253BFFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253C000: 409A000C  bne cr6, 0x8253c00c
	if !ctx.cr[6].eq {
	pc = 0x8253C00C; continue 'dispatch;
	}
	// 8253C004: B2A10060  sth r21, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[21].u16 ) };
	// 8253C008: 4800029C  b 0x8253c2a4
	pc = 0x8253C2A4; continue 'dispatch;
	// 8253C00C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253C010: 409A0034  bne cr6, 0x8253c044
	if !ctx.cr[6].eq {
	pc = 0x8253C044; continue 'dispatch;
	}
	// 8253C014: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C018: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8253C01C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C020: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C024: 554A007F  clrlwi. r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253C028: 4082001C  bne 0x8253c044
	if !ctx.cr[0].eq {
	pc = 0x8253C044; continue 'dispatch;
	}
	// 8253C02C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253C030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253C034: 409A0010  bne cr6, 0x8253c044
	if !ctx.cr[6].eq {
	pc = 0x8253C044; continue 'dispatch;
	}
	// 8253C038: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8253C03C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253C040: 419AFF88  beq cr6, 0x8253bfc8
	if ctx.cr[6].eq {
	pc = 0x8253BFC8; continue 'dispatch;
	}
	// 8253C044: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 8253C048: 39010056  addi r8, r1, 0x56
	ctx.r[8].s64 = ctx.r[1].s64 + 86;
	// 8253C04C: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8253C050: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253C054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253C058: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253C05C: 40990064  ble cr6, 0x8253c0c0
	if !ctx.cr[6].gt {
	pc = 0x8253C0C0; continue 'dispatch;
	}
	// 8253C060: 3941006A  addi r10, r1, 0x6a
	ctx.r[10].s64 = ctx.r[1].s64 + 106;
	// 8253C064: 38A40002  addi r5, r4, 2
	ctx.r[5].s64 = ctx.r[4].s64 + 2;
	// 8253C068: 7CCB5050  subf r6, r11, r10
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8253C06C: A1450000  lhz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C070: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 8253C074: A1260000  lhz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C078: 81680002  lwz r11, 2(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 8253C07C: 7D2A49D6  mullw r9, r10, r9
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8253C080: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8253C084: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253C088: 4198000C  blt cr6, 0x8253c094
	if ctx.cr[6].lt {
	pc = 0x8253C094; continue 'dispatch;
	}
	// 8253C08C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8253C090: 40980008  bge cr6, 0x8253c098
	if !ctx.cr[6].lt {
	pc = 0x8253C098; continue 'dispatch;
	}
	// 8253C094: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8253C098: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8253C09C: 91480002  stw r10, 2(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u32 ) };
	// 8253C0A0: 419A0010  beq cr6, 0x8253c0b0
	if ctx.cr[6].eq {
	pc = 0x8253C0B0; continue 'dispatch;
	}
	// 8253C0A4: A1680000  lhz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C0A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C0AC: B1680000  sth r11, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8253C0B0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253C0B4: 38C6FFFE  addi r6, r6, -2
	ctx.r[6].s64 = ctx.r[6].s64 + -2;
	// 8253C0B8: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 8253C0BC: 4181FFB0  bgt 0x8253c06c
	if ctx.cr[0].gt {
	pc = 0x8253C06C; continue 'dispatch;
	}
	// 8253C0C0: 3463FFFF  addic. r3, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253C0C4: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8253C0C8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8253C0CC: 4181FF84  bgt 0x8253c050
	if ctx.cr[0].gt {
	pc = 0x8253C050; continue 'dispatch;
	}
	// 8253C0D0: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8253C0D4: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 8253C0D8: 396BC002  addi r11, r11, -0x3ffe
	ctx.r[11].s64 = ctx.r[11].s64 + -16382;
	// 8253C0DC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C0E0: 7D6A0735  extsh. r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253C0E4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8253C0E8: 40810050  ble 0x8253c138
	if !ctx.cr[0].gt {
	pc = 0x8253C138; continue 'dispatch;
	}
	// 8253C0EC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253C0F0: 54C90001  rlwinm. r9, r6, 0, 0, 0
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C0F4: 40820044  bne 0x8253c138
	if !ctx.cr[0].eq {
	pc = 0x8253C138; continue 'dispatch;
	}
	// 8253C0F8: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253C0FC: 55490FFE  srwi r9, r10, 0x1f
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8253C100: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C104: 54E80FFE  srwi r8, r7, 0x1f
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shr(31);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8253C108: 54E7083C  slwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8253C10C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8253C110: 7CE94B78  or r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 8253C114: 54C6083C  slwi r6, r6, 1
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8253C118: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C11C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253C120: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8253C124: 7CC94378  or r9, r6, r8
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 8253C128: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8253C12C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8253C130: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C134: 4181FFB8  bgt 0x8253c0ec
	if ctx.cr[0].gt {
	pc = 0x8253C0EC; continue 'dispatch;
	}
	// 8253C138: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C13C: 4181009C  bgt 0x8253c1d8
	if ctx.cr[0].gt {
	pc = 0x8253C1D8; continue 'dispatch;
	}
	// 8253C140: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C144: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8253C148: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C14C: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C150: 40800088  bge 0x8253c1d8
	if !ctx.cr[0].lt {
	pc = 0x8253C1D8; continue 'dispatch;
	}
	// 8253C154: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253C158: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253C15C: A121005A  lhz r9, 0x5a(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8253C160: 552907FF  clrlwi. r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C164: 41820008  beq 0x8253c16c
	if ctx.cr[0].eq {
	pc = 0x8253C16C; continue 'dispatch;
	}
	// 8253C168: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8253C16C: 54C907FF  clrlwi. r9, r6, 0x1f
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C170: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 8253C174: 40820008  bne 0x8253c17c
	if !ctx.cr[0].eq {
	pc = 0x8253C17C; continue 'dispatch;
	}
	// 8253C178: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8253C17C: 54E907FF  clrlwi. r9, r7, 0x1f
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C180: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 8253C184: 40820008  bne 0x8253c18c
	if !ctx.cr[0].eq {
	pc = 0x8253C18C; continue 'dispatch;
	}
	// 8253C188: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8253C18C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C190: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8253C194: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C198: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8253C19C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C1A0: 54E7F87E  srwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8253C1A4: 54C6F87E  srwi r6, r6, 1
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8253C1A8: 7CE74378  or r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8253C1AC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8253C1B0: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8253C1B4: 4180FFA8  blt 0x8253c15c
	if ctx.cr[0].lt {
	pc = 0x8253C15C; continue 'dispatch;
	}
	// 8253C1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8253C1BC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8253C1C0: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 8253C1C4: 419A0014  beq cr6, 0x8253c1d8
	if ctx.cr[6].eq {
	pc = 0x8253C1D8; continue 'dispatch;
	}
	// 8253C1C8: A141005A  lhz r10, 0x5a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8253C1CC: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8253C1D0: B141005A  sth r10, 0x5a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[10].u16 ) };
	// 8253C1D4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8253C1D8: A121005A  lhz r9, 0x5a(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8253C1DC: 2B098000  cmplwi cr6, r9, 0x8000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32768 as u32, &mut ctx.xer);
	// 8253C1E0: 41990018  bgt cr6, 0x8253c1f8
	if ctx.cr[6].gt {
	pc = 0x8253C1F8; continue 'dispatch;
	}
	// 8253C1E4: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 8253C1E8: 554A03FE  clrlwi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0001FFFFu64;
	// 8253C1EC: 61298000  ori r9, r9, 0x8000
	ctx.r[9].u64 = ctx.r[9].u64 | 32768;
	// 8253C1F0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8253C1F4: 409A0064  bne cr6, 0x8253c258
	if !ctx.cr[6].eq {
	pc = 0x8253C258; continue 'dispatch;
	}
	// 8253C1F8: 81410056  lwz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8253C1FC: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8253C200: 409A0050  bne cr6, 0x8253c250
	if !ctx.cr[6].eq {
	pc = 0x8253C250; continue 'dispatch;
	}
	// 8253C204: 81410052  lwz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8253C208: 92A10056  stw r21, 0x56(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[21].u32 ) };
	// 8253C20C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8253C210: 409A0034  bne cr6, 0x8253c244
	if !ctx.cr[6].eq {
	pc = 0x8253C244; continue 'dispatch;
	}
	// 8253C214: A1410050  lhz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253C218: 92A10052  stw r21, 0x52(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[21].u32 ) };
	// 8253C21C: 2B0AFFFF  cmplwi cr6, r10, 0xffff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65535 as u32, &mut ctx.xer);
	// 8253C220: 409A0018  bne cr6, 0x8253c238
	if !ctx.cr[6].eq {
	pc = 0x8253C238; continue 'dispatch;
	}
	// 8253C224: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C228: B2810050  sth r20, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[20].u16 ) };
	// 8253C22C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C230: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C234: 48000024  b 0x8253c258
	pc = 0x8253C258; continue 'dispatch;
	// 8253C238: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253C23C: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8253C240: 48000018  b 0x8253c258
	pc = 0x8253C258; continue 'dispatch;
	// 8253C244: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253C248: 91410052  stw r10, 0x52(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u32 ) };
	// 8253C24C: 4800000C  b 0x8253c258
	pc = 0x8253C258; continue 'dispatch;
	// 8253C250: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253C254: 91410056  stw r10, 0x56(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u32 ) };
	// 8253C258: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253C25C: 2B0B7FFF  cmplwi cr6, r11, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32767 as u32, &mut ctx.xer);
	// 8253C260: 4098002C  bge cr6, 0x8253c28c
	if !ctx.cr[6].lt {
	pc = 0x8253C28C; continue 'dispatch;
	}
	// 8253C264: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8253C268: A1210058  lhz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8253C26C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8253C270: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253C274: B121006A  sth r9, 0x6a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[9].u16 ) };
	// 8253C278: 91410066  stw r10, 0x66(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[10].u32 ) };
	// 8253C27C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253C280: B1610060  sth r11, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 8253C284: 91410062  stw r10, 0x62(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[10].u32 ) };
	// 8253C288: 4800001C  b 0x8253c2a4
	pc = 0x8253C2A4; continue 'dispatch;
	// 8253C28C: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 8253C290: 576B043F  clrlwi. r11, r27, 0x10
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253C294: 40820008  bne 0x8253c29c
	if !ctx.cr[0].eq {
	pc = 0x8253C29C; continue 'dispatch;
	}
	// 8253C298: 92210060  stw r17, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[17].u32 ) };
	// 8253C29C: 92A10068  stw r21, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[21].u32 ) };
	// 8253C2A0: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 8253C2A4: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8253C2A8: 409AFC88  bne cr6, 0x8253bf30
	if !ctx.cr[6].eq {
	pc = 0x8253BF30; continue 'dispatch;
	}
	// 8253C2AC: A161006A  lhz r11, 0x6a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(106 as u32) ) } as u64;
	// 8253C2B0: 81010066  lwz r8, 0x66(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 8253C2B4: 81210062  lwz r9, 0x62(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 8253C2B8: A1410060  lhz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8253C2BC: 4800005C  b 0x8253c318
	pc = 0x8253C318; continue 'dispatch;
	// 8253C2C0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8253C2C4: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8253C2C8: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8253C2CC: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8253C2D0: 48000048  b 0x8253c318
	pc = 0x8253C318; continue 'dispatch;
	// 8253C2D4: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8253C2D8: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8253C2DC: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8253C2E0: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8253C2E4: 3AA00004  li r21, 4
	ctx.r[21].s64 = 4;
	// 8253C2E8: 48000030  b 0x8253c318
	pc = 0x8253C318; continue 'dispatch;
	// 8253C2EC: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8253C2F0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8253C2F4: 39407FFF  li r10, 0x7fff
	ctx.r[10].s64 = 32767;
	// 8253C2F8: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8253C2FC: 3AA00002  li r21, 2
	ctx.r[21].s64 = 2;
	// 8253C300: 48000018  b 0x8253c318
	pc = 0x8253C318; continue 'dispatch;
	// 8253C304: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8253C308: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8253C30C: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8253C310: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8253C314: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 8253C318: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8253C31C: B173000A  sth r11, 0xa(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 8253C320: 5647043E  clrlwi r7, r18, 0x10
	ctx.r[7].u64 = ctx.r[18].u32 as u64 & 0x0000FFFFu64;
	// 8253C324: 91130006  stw r8, 6(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[8].u32 ) };
	// 8253C328: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8253C32C: 91330002  stw r9, 2(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(2 as u32), ctx.r[9].u32 ) };
	// 8253C330: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8253C334: B1530000  sth r10, 0(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8253C338: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8253C33C: 4BFF8DA0  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253C340 size=160
    let mut pc: u32 = 0x8253C340;
    'dispatch: loop {
        match pc {
            0x8253C340 => {
    //   block [0x8253C340..0x8253C3E0)
	// 8253C340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253C344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253C348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253C34C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253C350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8253C354: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C358: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8253C35C: 4BFF8FFD  bl 0x82535358
	ctx.lr = 0x8253C360;
	sub_82535358(ctx, base);
	// 8253C360: 2F030065  cmpwi cr6, r3, 0x65
	ctx.cr[6].compare_i32(ctx.r[3].s32, 101, &mut ctx.xer);
	// 8253C364: 419A0018  beq cr6, 0x8253c37c
	if ctx.cr[6].eq {
	pc = 0x8253C37C; continue 'dispatch;
	}
	// 8253C368: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253C36C: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C370: 4BFF6AB9  bl 0x82532e28
	ctx.lr = 0x8253C374;
	sub_82532E28(ctx, base);
	// 8253C374: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253C378: 4082FFF0  bne 0x8253c368
	if !ctx.cr[0].eq {
	pc = 0x8253C368; continue 'dispatch;
	}
	// 8253C37C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C380: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8253C384: 4BFF8FD5  bl 0x82535358
	ctx.lr = 0x8253C388;
	sub_82535358(ctx, base);
	// 8253C388: 2F030078  cmpwi cr6, r3, 0x78
	ctx.cr[6].compare_i32(ctx.r[3].s32, 120, &mut ctx.xer);
	// 8253C38C: 409A0008  bne cr6, 0x8253c394
	if !ctx.cr[6].eq {
	pc = 0x8253C394; continue 'dispatch;
	}
	// 8253C390: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 8253C394: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253C398: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C39C: 816BF5D8  lwz r11, -0xa28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2600 as u32) ) } as u64;
	// 8253C3A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C3A4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C3A8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253C3AC: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8253C3B0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C3B4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253C3B8: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8253C3BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C3C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253C3C4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8253C3C8: 409AFFE8  bne cr6, 0x8253c3b0
	if !ctx.cr[6].eq {
	pc = 0x8253C3B0; continue 'dispatch;
	}
	// 8253C3CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8253C3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253C3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253C3D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253C3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C3E0 size=72
    let mut pc: u32 = 0x8253C3E0;
    'dispatch: loop {
        match pc {
            0x8253C3E0 => {
    //   block [0x8253C3E0..0x8253C428)
	// 8253C3E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C3E4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8253C3E8: 810AF5D8  lwz r8, -0xa28(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2600 as u32) ) } as u64;
	// 8253C3EC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253C3F0: 41820028  beq 0x8253c418
	if ctx.cr[0].eq {
	pc = 0x8253C418; continue 'dispatch;
	}
	// 8253C3F4: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C3F8: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C3FC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253C400: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8253C404: 419A0014  beq cr6, 0x8253c418
	if ctx.cr[6].eq {
	pc = 0x8253C418; continue 'dispatch;
	}
	// 8253C408: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8253C40C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C410: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253C414: 4082FFEC  bne 0x8253c400
	if !ctx.cr[0].eq {
	pc = 0x8253C400; continue 'dispatch;
	}
	// 8253C418: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C41C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8253C420: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8253C424: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C428 size=4
    let mut pc: u32 = 0x8253C428;
    'dispatch: loop {
        match pc {
            0x8253C428 => {
    //   block [0x8253C428..0x8253C42C)
	// 8253C428: 48000018  b 0x8253c440
	sub_8253C42C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C42C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C42C size=104
    let mut pc: u32 = 0x8253C42C;
    'dispatch: loop {
        match pc {
            0x8253C42C => {
    //   block [0x8253C42C..0x8253C494)
	// 8253C42C: 2F0A0065  cmpwi cr6, r10, 0x65
	ctx.cr[6].compare_i32(ctx.r[10].s32, 101, &mut ctx.xer);
	// 8253C430: 419A001C  beq cr6, 0x8253c44c
	if ctx.cr[6].eq {
	pc = 0x8253C44C; continue 'dispatch;
	}
	// 8253C434: 2F0A0045  cmpwi cr6, r10, 0x45
	ctx.cr[6].compare_i32(ctx.r[10].s32, 69, &mut ctx.xer);
	// 8253C438: 419A0014  beq cr6, 0x8253c44c
	if ctx.cr[6].eq {
	pc = 0x8253C44C; continue 'dispatch;
	}
	// 8253C43C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C440: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C444: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8253C448: 4082FFE4  bne 0x8253c42c
	if !ctx.cr[0].eq {
	pc = 0x8253C42C; continue 'dispatch;
	}
	// 8253C44C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8253C450: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253C454: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C458: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 8253C45C: 419AFFF4  beq cr6, 0x8253c450
	if ctx.cr[6].eq {
	pc = 0x8253C450; continue 'dispatch;
	}
	// 8253C460: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C464: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C468: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C46C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8253C470: 409A0008  bne cr6, 0x8253c478
	if !ctx.cr[6].eq {
	pc = 0x8253C478; continue 'dispatch;
	}
	// 8253C474: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253C478: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C47C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C480: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8253C484: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253C488: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253C48C: 4082FFEC  bne 0x8253c478
	if !ctx.cr[0].eq {
	pc = 0x8253C478; continue 'dispatch;
	}
	// 8253C490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C498 size=24
    let mut pc: u32 = 0x8253C498;
    'dispatch: loop {
        match pc {
            0x8253C498 => {
    //   block [0x8253C498..0x8253C4B0)
	// 8253C498: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8253C49C: C9A30000  lfd f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253C4A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8253C4A4: C80B2008  lfd f0, 0x2008(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8200 as u32) ) };
	// 8253C4A8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8253C4AC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C4B0 size=8
    let mut pc: u32 = 0x8253C4B0;
    'dispatch: loop {
        match pc {
            0x8253C4B0 => {
    //   block [0x8253C4B0..0x8253C4B8)
	// 8253C4B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253C4B8 size=92
    let mut pc: u32 = 0x8253C4B8;
    'dispatch: loop {
        match pc {
            0x8253C4B8 => {
    //   block [0x8253C4B8..0x8253C514)
	// 8253C4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253C4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8253C4C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8253C4C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253C4C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253C4CC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8253C4D0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8253C4D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8253C4D8: 419A0018  beq cr6, 0x8253c4f0
	if ctx.cr[6].eq {
	pc = 0x8253C4F0; continue 'dispatch;
	}
	// 8253C4DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8253C4E0: 48007729  bl 0x82543c08
	ctx.lr = 0x8253C4E4;
	sub_82543C08(ctx, base);
	// 8253C4E4: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8253C4E8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8253C4EC: 48000014  b 0x8253c500
	pc = 0x8253C500; continue 'dispatch;
	// 8253C4F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8253C4F4: 480077BD  bl 0x82543cb0
	ctx.lr = 0x8253C4F8;
	sub_82543CB0(ctx, base);
	// 8253C4F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253C4FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8253C500: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8253C504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8253C508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8253C50C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8253C510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C518 size=8
    let mut pc: u32 = 0x8253C518;
    'dispatch: loop {
        match pc {
            0x8253C518 => {
    //   block [0x8253C518..0x8253C520)
	// 8253C518: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C51C: 4BFFFF9C  b 0x8253c4b8
	sub_8253C4B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C520 size=8
    let mut pc: u32 = 0x8253C520;
    'dispatch: loop {
        match pc {
            0x8253C520 => {
    //   block [0x8253C520..0x8253C528)
	// 8253C520: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C524: 4BFFFE1C  b 0x8253c340
	sub_8253C340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8253C528 size=8
    let mut pc: u32 = 0x8253C528;
    'dispatch: loop {
        match pc {
            0x8253C528 => {
    //   block [0x8253C528..0x8253C530)
	// 8253C528: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C52C: 4BFFFEB4  b 0x8253c3e0
	sub_8253C3E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253C530 size=636
    let mut pc: u32 = 0x8253C530;
    'dispatch: loop {
        match pc {
            0x8253C530 => {
    //   block [0x8253C530..0x8253C7AC)
	// 8253C530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253C534: 4BFF8B79  bl 0x825350ac
	ctx.lr = 0x8253C538;
	sub_82535080(ctx, base);
	// 8253C538: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253C53C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8253C540: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8253C544: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8253C548: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8253C54C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8253C550: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8253C554: 409A0034  bne cr6, 0x8253c588
	if !ctx.cr[6].eq {
	pc = 0x8253C588; continue 'dispatch;
	}
	// 8253C558: 4BFFE3E1  bl 0x8253a938
	ctx.lr = 0x8253C55C;
	sub_8253A938(ctx, base);
	// 8253C55C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253C560: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253C564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C568: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253C570: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C578: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253C57C: 4BFFE285  bl 0x8253a800
	ctx.lr = 0x8253C580;
	sub_8253A800(ctx, base);
	// 8253C580: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8253C584: 48000220  b 0x8253c7a4
	pc = 0x8253C7A4; continue 'dispatch;
	// 8253C588: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8253C58C: 409A0030  bne cr6, 0x8253c5bc
	if !ctx.cr[6].eq {
	pc = 0x8253C5BC; continue 'dispatch;
	}
	// 8253C590: 4BFFE3A9  bl 0x8253a938
	ctx.lr = 0x8253C594;
	sub_8253A938(ctx, base);
	// 8253C594: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 8253C598: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8253C59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C5A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253C5A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C5AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C5B0: 4BFFE251  bl 0x8253a800
	ctx.lr = 0x8253C5B4;
	sub_8253A800(ctx, base);
	// 8253C5B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253C5B8: 480001EC  b 0x8253c7a4
	pc = 0x8253C7A4; continue 'dispatch;
	// 8253C5BC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253C5C0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8253C5C4: 41990008  bgt cr6, 0x8253c5cc
	if ctx.cr[6].gt {
	pc = 0x8253C5CC; continue 'dispatch;
	}
	// 8253C5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253C5CC: 396B0009  addi r11, r11, 9
	ctx.r[11].s64 = ctx.r[11].s64 + 9;
	// 8253C5D0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253C5D4: 41990010  bgt cr6, 0x8253c5e4
	if ctx.cr[6].gt {
	pc = 0x8253C5E4; continue 'dispatch;
	}
	// 8253C5D8: 4BFFE361  bl 0x8253a938
	ctx.lr = 0x8253C5DC;
	sub_8253A938(ctx, base);
	// 8253C5DC: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 8253C5E0: 4BFFFFB8  b 0x8253c598
	pc = 0x8253C598; continue 'dispatch;
	// 8253C5E4: 7D1C0775  extsb. r28, r8
	ctx.r[28].s64 = ctx.r[8].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8253C5E8: 41820050  beq 0x8253c638
	if ctx.cr[0].eq {
	pc = 0x8253C638; continue 'dispatch;
	}
	// 8253C5EC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C5F0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253C5F4: 396BFFD3  addi r11, r11, -0x2d
	ctx.r[11].s64 = ctx.r[11].s64 + -45;
	// 8253C5F8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8253C5FC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8253C600: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8253C604: 40990034  ble cr6, 0x8253c638
	if !ctx.cr[6].gt {
	pc = 0x8253C638; continue 'dispatch;
	}
	// 8253C608: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8253C60C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253C610: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C614: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253C618: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253C61C: 409AFFF4  bne cr6, 0x8253c610
	if !ctx.cr[6].eq {
	pc = 0x8253C610; continue 'dispatch;
	}
	// 8253C620: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8253C624: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 8253C628: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253C62C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253C630: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 8253C634: 4BFF8D3D  bl 0x82535370
	ctx.lr = 0x8253C638;
	sub_82535370(ctx, base);
	// 8253C638: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C63C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8253C640: 3B40002D  li r26, 0x2d
	ctx.r[26].s64 = 45;
	// 8253C644: 2F0A002D  cmpwi cr6, r10, 0x2d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 45, &mut ctx.xer);
	// 8253C648: 409A000C  bne cr6, 0x8253c654
	if !ctx.cr[6].eq {
	pc = 0x8253C654; continue 'dispatch;
	}
	// 8253C64C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 8253C650: 9B5E0000  stb r26, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8253C654: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8253C658: 40990028  ble cr6, 0x8253c680
	if !ctx.cr[6].gt {
	pc = 0x8253C680; continue 'dispatch;
	}
	// 8253C65C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8253C660: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8253C664: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253C668: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253C66C: 816BF5D8  lwz r11, -0xa28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2600 as u32) ) } as u64;
	// 8253C670: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C674: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C678: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8253C67C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253C680: 7F8A0034  cntlzw r10, r28
	ctx.r[10].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 8253C684: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 8253C688: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8253C68C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8253C690: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8253C694: 409A000C  bne cr6, 0x8253c6a0
	if !ctx.cr[6].eq {
	pc = 0x8253C6A0; continue 'dispatch;
	}
	// 8253C698: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8253C69C: 4800000C  b 0x8253c6a8
	pc = 0x8253C6A8; continue 'dispatch;
	// 8253C6A0: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8253C6A4: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8253C6A8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8253C6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253C6B0: 38AB7824  addi r5, r11, 0x7824
	ctx.r[5].s64 = ctx.r[11].s64 + 30756;
	// 8253C6B4: 4BFF8BBD  bl 0x82535270
	ctx.lr = 0x8253C6B8;
	sub_82535270(ctx, base);
	// 8253C6B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253C6BC: 4182001C  beq 0x8253c6d8
	if ctx.cr[0].eq {
	pc = 0x8253C6D8; continue 'dispatch;
	}
	// 8253C6C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C6C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253C6CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C6D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C6D4: 4BFFE175  bl 0x8253a848
	ctx.lr = 0x8253C6D8;
	sub_8253A848(ctx, base);
	// 8253C6D8: 387F0002  addi r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 2;
	// 8253C6DC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8253C6E0: 419A000C  beq cr6, 0x8253c6ec
	if ctx.cr[6].eq {
	pc = 0x8253C6EC; continue 'dispatch;
	}
	// 8253C6E4: 39600045  li r11, 0x45
	ctx.r[11].s64 = 69;
	// 8253C6E8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253C6EC: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8253C6F0: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 8253C6F4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C6F8: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8253C6FC: 419A007C  beq cr6, 0x8253c778
	if ctx.cr[6].eq {
	pc = 0x8253C778; continue 'dispatch;
	}
	// 8253C700: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253C704: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253C708: 4080000C  bge 0x8253c714
	if !ctx.cr[0].lt {
	pc = 0x8253C714; continue 'dispatch;
	}
	// 8253C70C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8253C710: 9B4A0000  stb r26, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8253C714: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253C718: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 8253C71C: 41980024  blt cr6, 0x8253c740
	if ctx.cr[6].lt {
	pc = 0x8253C740; continue 'dispatch;
	}
	// 8253C720: 38E00064  li r7, 0x64
	ctx.r[7].s64 = 100;
	// 8253C724: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C728: 7D2B3BD6  divw r9, r11, r7
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[7].s32;
	// 8253C72C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8253C730: 7D0B3BD6  divw r8, r11, r7
	ctx.r[8].s32 = ctx.r[11].s32 / ctx.r[7].s32;
	// 8253C734: 1D080064  mulli r8, r8, 0x64
	ctx.r[8].s64 = ctx.r[8].s64 * 100;
	// 8253C738: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253C73C: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8253C740: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253C744: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8253C748: 41980024  blt cr6, 0x8253c76c
	if ctx.cr[6].lt {
	pc = 0x8253C76C; continue 'dispatch;
	}
	// 8253C74C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8253C750: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C754: 7D2B3BD6  divw r9, r11, r7
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[7].s32;
	// 8253C758: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8253C75C: 7D0B3BD6  divw r8, r11, r7
	ctx.r[8].s32 = ctx.r[11].s32 / ctx.r[7].s32;
	// 8253C760: 1D08000A  mulli r8, r8, 0xa
	ctx.r[8].s64 = ctx.r[8].s64 * 10;
	// 8253C764: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253C768: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8253C76C: 892A0001  lbz r9, 1(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8253C770: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8253C774: 996A0001  stb r11, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8253C778: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8253C77C: 816B2F68  lwz r11, 0x2f68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12136 as u32) ) } as u64;
	// 8253C780: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253C784: 4182001C  beq 0x8253c7a0
	if ctx.cr[0].eq {
	pc = 0x8253C7A0; continue 'dispatch;
	}
	// 8253C788: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C78C: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8253C790: 409A0010  bne cr6, 0x8253c7a0
	if !ctx.cr[6].eq {
	pc = 0x8253C7A0; continue 'dispatch;
	}
	// 8253C794: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8253C798: 38830001  addi r4, r3, 1
	ctx.r[4].s64 = ctx.r[3].s64 + 1;
	// 8253C79C: 4BFF8BD5  bl 0x82535370
	ctx.lr = 0x8253C7A0;
	sub_82535370(ctx, base);
	// 8253C7A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C7A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8253C7A8: 4BFF8954  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253C7B0 size=280
    let mut pc: u32 = 0x8253C7B0;
    'dispatch: loop {
        match pc {
            0x8253C7B0 => {
    //   block [0x8253C7B0..0x8253C8C8)
	// 8253C7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253C7B4: 4BFF8901  bl 0x825350b4
	ctx.lr = 0x8253C7B8;
	sub_82535080(ctx, base);
	// 8253C7B8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253C7BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253C7C0: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253C7C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8253C7C8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8253C7CC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8253C7D0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8253C7D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253C7D8: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8253C7DC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8253C7E0: 480077E9  bl 0x82543fc8
	ctx.lr = 0x8253C7E4;
	sub_82543FC8(ctx, base);
	// 8253C7E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8253C7E8: 409A0034  bne cr6, 0x8253c81c
	if !ctx.cr[6].eq {
	pc = 0x8253C81C; continue 'dispatch;
	}
	// 8253C7EC: 4BFFE14D  bl 0x8253a938
	ctx.lr = 0x8253C7F0;
	sub_8253A938(ctx, base);
	// 8253C7F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253C7F4: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253C7F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C800: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253C804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C808: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C80C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253C810: 4BFFDFF1  bl 0x8253a800
	ctx.lr = 0x8253C814;
	sub_8253A800(ctx, base);
	// 8253C814: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8253C818: 480000A8  b 0x8253c8c0
	pc = 0x8253C8C0; continue 'dispatch;
	// 8253C81C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8253C820: 419AFFCC  beq cr6, 0x8253c7ec
	if ctx.cr[6].eq {
	pc = 0x8253C7EC; continue 'dispatch;
	}
	// 8253C824: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253C828: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 8253C82C: 409A000C  bne cr6, 0x8253c838
	if !ctx.cr[6].eq {
	pc = 0x8253C838; continue 'dispatch;
	}
	// 8253C830: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8253C834: 48000028  b 0x8253c85c
	pc = 0x8253C85C; continue 'dispatch;
	// 8253C838: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8253C83C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8253C840: 41990008  bgt cr6, 0x8253c848
	if ctx.cr[6].gt {
	pc = 0x8253C848; continue 'dispatch;
	}
	// 8253C844: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253C848: 3949FFD3  addi r10, r9, -0x2d
	ctx.r[10].s64 = ctx.r[9].s64 + -45;
	// 8253C84C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8253C850: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8253C854: 7D4AE850  subf r10, r10, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[10].s64;
	// 8253C858: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8253C85C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8253C860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8253C864: 41990008  bgt cr6, 0x8253c86c
	if ctx.cr[6].gt {
	pc = 0x8253C86C; continue 'dispatch;
	}
	// 8253C868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8253C86C: 3969FFD3  addi r11, r9, -0x2d
	ctx.r[11].s64 = ctx.r[9].s64 + -45;
	// 8253C870: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253C874: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8253C878: 38BE0001  addi r5, r30, 1
	ctx.r[5].s64 = ctx.r[30].s64 + 1;
	// 8253C87C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8253C880: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8253C884: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8253C888: 480074D1  bl 0x82543d58
	ctx.lr = 0x8253C88C;
	sub_82543D58(ctx, base);
	// 8253C88C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253C890: 41820010  beq 0x8253c8a0
	if ctx.cr[0].eq {
	pc = 0x8253C8A0; continue 'dispatch;
	}
	// 8253C894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253C898: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253C89C: 48000024  b 0x8253c8c0
	pc = 0x8253C8C0; continue 'dispatch;
	// 8253C8A0: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8253C8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8253C8A8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8253C8AC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8253C8B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8253C8B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8253C8B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253C8BC: 4BFFFC75  bl 0x8253c530
	ctx.lr = 0x8253C8C0;
	sub_8253C530(ctx, base);
	// 8253C8C0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8253C8C4: 4BFF8840  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8253C8C8 size=1096
    let mut pc: u32 = 0x8253C8C8;
    'dispatch: loop {
        match pc {
            0x8253C8C8 => {
    //   block [0x8253C8C8..0x8253CD10)
	// 8253C8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253C8CC: 4BFF87E5  bl 0x825350b0
	ctx.lr = 0x8253C8D0;
	sub_82535080(ctx, base);
	// 8253C8D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253C8D4: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
	// 8253C8D8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8253C8DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253C8E0: 3BC003FF  li r30, 0x3ff
	ctx.r[30].s64 = 1023;
	// 8253C8E4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8253C8E8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8253C8EC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8253C8F0: 40980008  bge cr6, 0x8253c8f8
	if !ctx.cr[6].lt {
	pc = 0x8253C8F8; continue 'dispatch;
	}
	// 8253C8F4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8253C8F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8253C8FC: 409A0034  bne cr6, 0x8253c930
	if !ctx.cr[6].eq {
	pc = 0x8253C930; continue 'dispatch;
	}
	// 8253C900: 4BFFE039  bl 0x8253a938
	ctx.lr = 0x8253C904;
	sub_8253A938(ctx, base);
	// 8253C904: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253C908: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253C90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C910: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253C918: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C91C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C920: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253C924: 4BFFDEDD  bl 0x8253a800
	ctx.lr = 0x8253C928;
	sub_8253A800(ctx, base);
	// 8253C928: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8253C92C: 480003DC  b 0x8253cd08
	pc = 0x8253CD08; continue 'dispatch;
	// 8253C930: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8253C934: 409A0030  bne cr6, 0x8253c964
	if !ctx.cr[6].eq {
	pc = 0x8253C964; continue 'dispatch;
	}
	// 8253C938: 4BFFE001  bl 0x8253a938
	ctx.lr = 0x8253C93C;
	sub_8253A938(ctx, base);
	// 8253C93C: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 8253C940: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8253C944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C948: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253C94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253C950: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253C954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253C958: 4BFFDEA9  bl 0x8253a800
	ctx.lr = 0x8253C95C;
	sub_8253A800(ctx, base);
	// 8253C95C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253C960: 480003A8  b 0x8253cd08
	pc = 0x8253CD08; continue 'dispatch;
	// 8253C964: 3966000B  addi r11, r6, 0xb
	ctx.r[11].s64 = ctx.r[6].s64 + 11;
	// 8253C968: 9B5F0000  stb r26, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8253C96C: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8253C970: 41990010  bgt cr6, 0x8253c980
	if ctx.cr[6].gt {
	pc = 0x8253C980; continue 'dispatch;
	}
	// 8253C974: 4BFFDFC5  bl 0x8253a938
	ctx.lr = 0x8253C978;
	sub_8253A938(ctx, base);
	// 8253C978: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 8253C97C: 4BFFFFC4  b 0x8253c940
	pc = 0x8253C940; continue 'dispatch;
	// 8253C980: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253C984: 556A053C  rlwinm r10, r11, 0, 0x14, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253C988: 2B2A0FFE  cmpldi cr6, r10, 0xffe
	ctx.cr[6].compare_u64(ctx.r[10].u64, 4094, &mut ctx.xer);
	// 8253C98C: 409A009C  bne cr6, 0x8253ca28
	if !ctx.cr[6].eq {
	pc = 0x8253CA28; continue 'dispatch;
	}
	// 8253C990: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 8253C994: 409A000C  bne cr6, 0x8253c9a0
	if !ctx.cr[6].eq {
	pc = 0x8253C9A0; continue 'dispatch;
	}
	// 8253C998: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8253C99C: 48000008  b 0x8253c9a4
	pc = 0x8253C9A4; continue 'dispatch;
	// 8253C9A0: 38A5FFFE  addi r5, r5, -2
	ctx.r[5].s64 = ctx.r[5].s64 + -2;
	// 8253C9A4: 3BDF0002  addi r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 2;
	// 8253C9A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8253C9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253C9B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253C9B4: 4BFFFDFD  bl 0x8253c7b0
	ctx.lr = 0x8253C9B8;
	sub_8253C7B0(ctx, base);
	// 8253C9B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253C9BC: 4182000C  beq 0x8253c9c8
	if ctx.cr[0].eq {
	pc = 0x8253C9C8; continue 'dispatch;
	}
	// 8253C9C0: 9B5F0000  stb r26, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8253C9C4: 48000344  b 0x8253cd08
	pc = 0x8253CD08; continue 'dispatch;
	// 8253C9C8: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253C9CC: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 8253C9D0: 409A000C  bne cr6, 0x8253c9dc
	if !ctx.cr[6].eq {
	pc = 0x8253C9DC; continue 'dispatch;
	}
	// 8253C9D4: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253C9D8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253C9DC: 217D0000  subfic r11, r29, 0
	ctx.xer.ca = ctx.r[29].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[29].s64;
	// 8253C9E0: 9B9F0000  stb r28, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253C9E4: 38800065  li r4, 0x65
	ctx.r[4].s64 = 101;
	// 8253C9E8: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8253C9EC: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8253C9F0: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253C9F4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8253C9F8: 394A0078  addi r10, r10, 0x78
	ctx.r[10].s64 = ctx.r[10].s64 + 120;
	// 8253C9FC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253CA00: 4BFFB6D1  bl 0x825380d0
	ctx.lr = 0x8253CA04;
	sub_825380D0(ctx, base);
	// 8253CA04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253CA08: 418202FC  beq 0x8253cd04
	if ctx.cr[0].eq {
	pc = 0x8253CD04; continue 'dispatch;
	}
	// 8253CA0C: 217D0000  subfic r11, r29, 0
	ctx.xer.ca = ctx.r[29].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[29].s64;
	// 8253CA10: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8253CA14: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253CA18: 396B0070  addi r11, r11, 0x70
	ctx.r[11].s64 = ctx.r[11].s64 + 112;
	// 8253CA1C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CA20: 9B430003  stb r26, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[26].u8 ) };
	// 8253CA24: 480002E0  b 0x8253cd04
	pc = 0x8253CD04; continue 'dispatch;
	// 8253CA28: 796B07E0  clrldi r11, r11, 0x3f
	ctx.r[11].u64 = ctx.r[11].u64 & 0x0000000000000001u64;
	// 8253CA2C: 3B60002D  li r27, 0x2d
	ctx.r[27].s64 = 45;
	// 8253CA30: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8253CA34: 419A000C  beq cr6, 0x8253ca40
	if ctx.cr[6].eq {
	pc = 0x8253CA40; continue 'dispatch;
	}
	// 8253CA38: 9B7F0000  stb r27, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8253CA3C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8253CA40: 217D0000  subfic r11, r29, 0
	ctx.xer.ca = ctx.r[29].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[29].s64;
	// 8253CA44: 9B9F0000  stb r28, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253CA48: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8253CA4C: 213D0000  subfic r9, r29, 0
	ctx.xer.ca = ctx.r[29].u32 <= 0 as u32;
	ctx.r[9].s64 = (0 as i64) - ctx.r[29].s64;
	// 8253CA50: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253CA54: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8253CA58: 390A0078  addi r8, r10, 0x78
	ctx.r[8].s64 = ctx.r[10].s64 + 120;
	// 8253CA5C: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8253CA60: 552A0034  rlwinm r10, r9, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 8253CA64: 394A0061  addi r10, r10, 0x61
	ctx.r[10].s64 = ctx.r[10].s64 + 97;
	// 8253CA68: 38AAFFC6  addi r5, r10, -0x3a
	ctx.r[5].s64 = ctx.r[10].s64 + -58;
	// 8253CA6C: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8253CA70: E9430000  ld r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CA74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CA78: 554A053C  rlwinm r10, r10, 0, 0x14, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8253CA7C: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8253CA80: 409A002C  bne cr6, 0x8253caac
	if !ctx.cr[6].eq {
	pc = 0x8253CAAC; continue 'dispatch;
	}
	// 8253CA84: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253CA88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CA8C: E9430000  ld r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CA90: 794A04E4  rldicr r10, r10, 0, 0x33
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(0) & 0xFFFFFFFFFFFFF000;
	// 8253CA94: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 8253CA98: 409A000C  bne cr6, 0x8253caa4
	if !ctx.cr[6].eq {
	pc = 0x8253CAA4; continue 'dispatch;
	}
	// 8253CA9C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8253CAA0: 48000018  b 0x8253cab8
	pc = 0x8253CAB8; continue 'dispatch;
	// 8253CAA4: 3BC003FE  li r30, 0x3fe
	ctx.r[30].s64 = 1022;
	// 8253CAA8: 48000010  b 0x8253cab8
	pc = 0x8253CAB8; continue 'dispatch;
	// 8253CAAC: 39400031  li r10, 0x31
	ctx.r[10].s64 = 49;
	// 8253CAB0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253CAB4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CAB8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8253CABC: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 8253CAC0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8253CAC4: 409A000C  bne cr6, 0x8253cad0
	if !ctx.cr[6].eq {
	pc = 0x8253CAD0; continue 'dispatch;
	}
	// 8253CAC8: 9B440000  stb r26, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8253CACC: 48000018  b 0x8253cae4
	pc = 0x8253CAE4; continue 'dispatch;
	// 8253CAD0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253CAD4: 816BF5D8  lwz r11, -0xa28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2600 as u32) ) } as u64;
	// 8253CAD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CADC: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CAE0: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CAE4: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CAE8: 796B04E4  rldicr r11, r11, 0, 0x33
	ctx.r[11].u64 = (ctx.r[11].u64).rotate_left(0) & 0xFFFFFFFFFFFFF000;
	// 8253CAEC: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 8253CAF0: 409900EC  ble cr6, 0x8253cbdc
	if !ctx.cr[6].gt {
	pc = 0x8253CBDC; continue 'dispatch;
	}
	// 8253CAF4: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 8253CAF8: 794A83C6  sldi r10, r10, 0x30
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(48);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8253CAFC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8253CB00: 40990054  ble cr6, 0x8253cb54
	if !ctx.cr[6].gt {
	pc = 0x8253CB54; continue 'dispatch;
	}
	// 8253CB04: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CB08: 7CE90734  extsh r9, r7
	ctx.r[9].s64 = ctx.r[7].s16 as i64;
	// 8253CB0C: 796BA302  rldicl r11, r11, 0x34, 0xc
	ctx.r[11].u64 = ctx.r[11].u64 & 0x0000000000000FFFu64;
	// 8253CB10: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8253CB14: 7D6B4C36  srd r11, r11, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 8253CB18: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253CB1C: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 8253CB20: 5569043E  clrlwi r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253CB24: 2B090039  cmplwi cr6, r9, 0x39
	ctx.cr[6].compare_u32(ctx.r[9].u32, 57 as u32, &mut ctx.xer);
	// 8253CB28: 40990010  ble cr6, 0x8253cb38
	if !ctx.cr[6].gt {
	pc = 0x8253CB38; continue 'dispatch;
	}
	// 8253CB2C: 54AB043E  clrlwi r11, r5, 0x10
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 8253CB30: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8253CB34: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253CB38: 3927FFFC  addi r9, r7, -4
	ctx.r[9].s64 = ctx.r[7].s64 + -4;
	// 8253CB3C: 99680000  stb r11, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CB40: 794AE102  rldicl r10, r10, 0x3c, 4
	ctx.r[10].u64 = ctx.r[10].u64 & 0x000000000000000Fu64;
	// 8253CB44: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8253CB48: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8253CB4C: 7D270735  extsh. r7, r9
	ctx.r[7].s64 = ctx.r[9].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8253CB50: 4080FFAC  bge 0x8253cafc
	if !ctx.cr[0].lt {
	pc = 0x8253CAFC; continue 'dispatch;
	}
	// 8253CB54: 7CEB0735  extsh. r11, r7
	ctx.r[11].s64 = ctx.r[7].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253CB58: 41800084  blt 0x8253cbdc
	if ctx.cr[0].lt {
	pc = 0x8253CBDC; continue 'dispatch;
	}
	// 8253CB5C: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CB60: 7CE90734  extsh r9, r7
	ctx.r[9].s64 = ctx.r[7].s16 as i64;
	// 8253CB64: 796BA302  rldicl r11, r11, 0x34, 0xc
	ctx.r[11].u64 = ctx.r[11].u64 & 0x0000000000000FFFu64;
	// 8253CB68: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 8253CB6C: 7D6B4C36  srd r11, r11, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 8253CB70: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8253CB74: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8253CB78: 40990064  ble cr6, 0x8253cbdc
	if !ctx.cr[6].gt {
	pc = 0x8253CBDC; continue 'dispatch;
	}
	// 8253CB7C: 3968FFFF  addi r11, r8, -1
	ctx.r[11].s64 = ctx.r[8].s64 + -1;
	// 8253CB80: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CB84: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253CB88: 2F0A0066  cmpwi cr6, r10, 0x66
	ctx.cr[6].compare_i32(ctx.r[10].s32, 102, &mut ctx.xer);
	// 8253CB8C: 419A000C  beq cr6, 0x8253cb98
	if ctx.cr[6].eq {
	pc = 0x8253CB98; continue 'dispatch;
	}
	// 8253CB90: 2F0A0046  cmpwi cr6, r10, 0x46
	ctx.cr[6].compare_i32(ctx.r[10].s32, 70, &mut ctx.xer);
	// 8253CB94: 409A0010  bne cr6, 0x8253cba4
	if !ctx.cr[6].eq {
	pc = 0x8253CBA4; continue 'dispatch;
	}
	// 8253CB98: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253CB9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253CBA0: 4BFFFFE0  b 0x8253cb80
	pc = 0x8253CB80; continue 'dispatch;
	// 8253CBA4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8253CBA8: 419A0028  beq cr6, 0x8253cbd0
	if ctx.cr[6].eq {
	pc = 0x8253CBD0; continue 'dispatch;
	}
	// 8253CBAC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CBB0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8253CBB4: 2F0A0039  cmpwi cr6, r10, 0x39
	ctx.cr[6].compare_i32(ctx.r[10].s32, 57, &mut ctx.xer);
	// 8253CBB8: 409A000C  bne cr6, 0x8253cbc4
	if !ctx.cr[6].eq {
	pc = 0x8253CBC4; continue 'dispatch;
	}
	// 8253CBBC: 3945003A  addi r10, r5, 0x3a
	ctx.r[10].s64 = ctx.r[5].s64 + 58;
	// 8253CBC0: 48000008  b 0x8253cbc8
	pc = 0x8253CBC8; continue 'dispatch;
	// 8253CBC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253CBC8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253CBCC: 48000010  b 0x8253cbdc
	pc = 0x8253CBDC; continue 'dispatch;
	// 8253CBD0: 894BFFFF  lbz r10, -1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8253CBD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253CBD8: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 8253CBDC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8253CBE0: 40990028  ble cr6, 0x8253cc08
	if !ctx.cr[6].gt {
	pc = 0x8253CC08; continue 'dispatch;
	}
	// 8253CBE4: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8253CBE8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8253CBEC: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8253CBF0: 41820014  beq 0x8253cc04
	if ctx.cr[0].eq {
	pc = 0x8253CC04; continue 'dispatch;
	}
	// 8253CBF4: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8253CBF8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8253CBFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CC00: 4200FFF8  bdnz 0x8253cbf8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8253CBF8; continue 'dispatch;
	}
	// 8253CC04: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 8253CC08: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CC0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253CC10: 409A0008  bne cr6, 0x8253cc18
	if !ctx.cr[6].eq {
	pc = 0x8253CC18; continue 'dispatch;
	}
	// 8253CC14: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8253CC18: 217D0000  subfic r11, r29, 0
	ctx.xer.ca = ctx.r[29].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[29].s64;
	// 8253CC1C: 39480001  addi r10, r8, 1
	ctx.r[10].s64 = ctx.r[8].s64 + 1;
	// 8253CC20: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8253CC24: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8253CC28: 396B0070  addi r11, r11, 0x70
	ctx.r[11].s64 = ctx.r[11].s64 + 112;
	// 8253CC2C: 99680000  stb r11, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CC30: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CC34: 796BFD62  rldicl r11, r11, 0x3f, 0x35
	ctx.r[11].u64 = ctx.r[11].u64 & 0x0000000000000001u64;
	// 8253CC38: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8253CC3C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8253CC40: 41980010  blt cr6, 0x8253cc50
	if ctx.cr[6].lt {
	pc = 0x8253CC50; continue 'dispatch;
	}
	// 8253CC44: 3920002B  li r9, 0x2b
	ctx.r[9].s64 = 43;
	// 8253CC48: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253CC4C: 4800000C  b 0x8253cc58
	pc = 0x8253CC58; continue 'dispatch;
	// 8253CC50: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8253CC54: 9B6A0000  stb r27, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8253CC58: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253CC5C: 2F2B03E8  cmpdi cr6, r11, 0x3e8
	ctx.cr[6].compare_i64(ctx.r[11].s64, 1000, &mut ctx.xer);
	// 8253CC60: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8253CC64: 9B8A0000  stb r28, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253CC68: 41980030  blt cr6, 0x8253cc98
	if ctx.cr[6].lt {
	pc = 0x8253CC98; continue 'dispatch;
	}
	// 8253CC6C: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8253CC70: 7CEB4BD2  divd r7, r11, r9
	ctx.r[7].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8253CC74: 7CCB4BD2  divd r6, r11, r9
	ctx.r[6].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8253CC78: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8253CC7C: 1CE603E8  mulli r7, r6, 0x3e8
	ctx.r[7].s64 = ctx.r[6].s64 * 1000;
	// 8253CC80: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8253CC84: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 8253CC88: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253CC8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253CC90: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8253CC94: 409A000C  bne cr6, 0x8253cca0
	if !ctx.cr[6].eq {
	pc = 0x8253CCA0; continue 'dispatch;
	}
	// 8253CC98: 2F2B0064  cmpdi cr6, r11, 0x64
	ctx.cr[6].compare_i64(ctx.r[11].s64, 100, &mut ctx.xer);
	// 8253CC9C: 41980028  blt cr6, 0x8253ccc4
	if ctx.cr[6].lt {
	pc = 0x8253CCC4; continue 'dispatch;
	}
	// 8253CCA0: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 8253CCA4: 7CEB4BD2  divd r7, r11, r9
	ctx.r[7].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8253CCA8: 7CCB4BD2  divd r6, r11, r9
	ctx.r[6].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8253CCAC: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8253CCB0: 1CE60064  mulli r7, r6, 0x64
	ctx.r[7].s64 = ctx.r[6].s64 * 100;
	// 8253CCB4: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8253CCB8: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 8253CCBC: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253CCC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253CCC4: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8253CCC8: 409A000C  bne cr6, 0x8253ccd4
	if !ctx.cr[6].eq {
	pc = 0x8253CCD4; continue 'dispatch;
	}
	// 8253CCCC: 2F2B000A  cmpdi cr6, r11, 0xa
	ctx.cr[6].compare_i64(ctx.r[11].s64, 10, &mut ctx.xer);
	// 8253CCD0: 41980028  blt cr6, 0x8253ccf8
	if ctx.cr[6].lt {
	pc = 0x8253CCF8; continue 'dispatch;
	}
	// 8253CCD4: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8253CCD8: 7D0B4BD2  divd r8, r11, r9
	ctx.r[8].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8253CCDC: 7CEB4BD2  divd r7, r11, r9
	ctx.r[7].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 8253CCE0: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8253CCE4: 1D07000A  mulli r8, r7, 0xa
	ctx.r[8].s64 = ctx.r[7].s64 * 10;
	// 8253CCE8: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 8253CCEC: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8253CCF0: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8253CCF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8253CCF8: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 8253CCFC: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CD00: 9B4A0001  stb r26, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 8253CD04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253CD08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8253CD0C: 4BFF83F4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253CD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253CD10 size=468
    let mut pc: u32 = 0x8253CD10;
    'dispatch: loop {
        match pc {
            0x8253CD10 => {
    //   block [0x8253CD10..0x8253CEE4)
	// 8253CD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253CD14: 4BFF839D  bl 0x825350b0
	ctx.lr = 0x8253CD18;
	sub_82535080(ctx, base);
	// 8253CD18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253CD1C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8253CD20: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8253CD24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8253CD28: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253CD2C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 8253CD30: 409A0034  bne cr6, 0x8253cd64
	if !ctx.cr[6].eq {
	pc = 0x8253CD64; continue 'dispatch;
	}
	// 8253CD34: 4BFFDC05  bl 0x8253a938
	ctx.lr = 0x8253CD38;
	sub_8253A938(ctx, base);
	// 8253CD38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253CD3C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253CD40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253CD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253CD48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253CD4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253CD50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253CD54: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253CD58: 4BFFDAA9  bl 0x8253a800
	ctx.lr = 0x8253CD5C;
	sub_8253A800(ctx, base);
	// 8253CD5C: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8253CD60: 4800017C  b 0x8253cedc
	pc = 0x8253CEDC; continue 'dispatch;
	// 8253CD64: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8253CD68: 419AFFCC  beq cr6, 0x8253cd34
	if ctx.cr[6].eq {
	pc = 0x8253CD34; continue 'dispatch;
	}
	// 8253CD6C: 7CFA0775  extsb. r26, r7
	ctx.r[26].s64 = ctx.r[7].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8253CD70: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
	// 8253CD74: 41820030  beq 0x8253cda4
	if ctx.cr[0].eq {
	pc = 0x8253CDA4; continue 'dispatch;
	}
	// 8253CD78: 7F0AD800  cmpw cr6, r10, r27
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8253CD7C: 409A0028  bne cr6, 0x8253cda4
	if !ctx.cr[6].eq {
	pc = 0x8253CDA4; continue 'dispatch;
	}
	// 8253CD80: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8253CD88: 396BFFD3  addi r11, r11, -0x2d
	ctx.r[11].s64 = ctx.r[11].s64 + -45;
	// 8253CD8C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8253CD90: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8253CD94: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8253CD98: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8253CD9C: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253CDA0: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 8253CDA4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CDA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8253CDAC: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8253CDB0: 409A000C  bne cr6, 0x8253cdbc
	if !ctx.cr[6].eq {
	pc = 0x8253CDBC; continue 'dispatch;
	}
	// 8253CDB4: 3BC30001  addi r30, r3, 1
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	// 8253CDB8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CDBC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253CDC0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253CDC4: 41810044  bgt 0x8253ce08
	if ctx.cr[0].gt {
	pc = 0x8253CE08; continue 'dispatch;
	}
	// 8253CDC8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8253CDCC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253CDD0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CDD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CDD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253CDDC: 409AFFF4  bne cr6, 0x8253cdd0
	if !ctx.cr[6].eq {
	pc = 0x8253CDD0; continue 'dispatch;
	}
	// 8253CDE0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8253CDE4: 3BFE0001  addi r31, r30, 1
	ctx.r[31].s64 = ctx.r[30].s64 + 1;
	// 8253CDE8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253CDEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253CDF0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253CDF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253CDF8: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 8253CDFC: 4BFF8575  bl 0x82535370
	ctx.lr = 0x8253CE00;
	sub_82535370(ctx, base);
	// 8253CE00: 9B9E0000  stb r28, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8253CE04: 48000008  b 0x8253ce0c
	pc = 0x8253CE0C; continue 'dispatch;
	// 8253CE08: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8253CE0C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8253CE10: 409900C8  ble cr6, 0x8253ced8
	if !ctx.cr[6].gt {
	pc = 0x8253CED8; continue 'dispatch;
	}
	// 8253CE14: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8253CE18: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253CE1C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CE20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CE24: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253CE28: 409AFFF4  bne cr6, 0x8253ce1c
	if !ctx.cr[6].eq {
	pc = 0x8253CE1C; continue 'dispatch;
	}
	// 8253CE2C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8253CE30: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 8253CE34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253CE38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8253CE3C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253CE40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253CE44: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 8253CE48: 4BFF8529  bl 0x82535370
	ctx.lr = 0x8253CE4C;
	sub_82535370(ctx, base);
	// 8253CE4C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8253CE50: 816BF5D8  lwz r11, -0xa28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2600 as u32) ) } as u64;
	// 8253CE54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CE58: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CE5C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CE60: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8253CE64: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253CE68: 40800070  bge 0x8253ced8
	if !ctx.cr[0].lt {
	pc = 0x8253CED8; continue 'dispatch;
	}
	// 8253CE6C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8253CE70: 419A000C  beq cr6, 0x8253ce7c
	if ctx.cr[6].eq {
	pc = 0x8253CE7C; continue 'dispatch;
	}
	// 8253CE74: 7F6B00D0  neg r27, r11
	ctx.r[27].s64 = -ctx.r[11].s64;
	// 8253CE78: 48000014  b 0x8253ce8c
	pc = 0x8253CE8C; continue 'dispatch;
	// 8253CE7C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8253CE80: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8253CE84: 41980008  blt cr6, 0x8253ce8c
	if ctx.cr[6].lt {
	pc = 0x8253CE8C; continue 'dispatch;
	}
	// 8253CE88: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 8253CE8C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8253CE90: 419A0038  beq cr6, 0x8253cec8
	if ctx.cr[6].eq {
	pc = 0x8253CEC8; continue 'dispatch;
	}
	// 8253CE94: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8253CE98: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8253CE9C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253CEA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8253CEA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8253CEA8: 409AFFF4  bne cr6, 0x8253ce9c
	if !ctx.cr[6].eq {
	pc = 0x8253CE9C; continue 'dispatch;
	}
	// 8253CEAC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8253CEB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253CEB4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253CEB8: 7C7EDA14  add r3, r30, r27
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 8253CEBC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8253CEC0: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 8253CEC4: 4BFF84AD  bl 0x82535370
	ctx.lr = 0x8253CEC8;
	sub_82535370(ctx, base);
	// 8253CEC8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8253CECC: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8253CED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8253CED4: 4BFF82FD  bl 0x825351d0
	ctx.lr = 0x8253CED8;
	sub_825351D0(ctx, base);
	// 8253CED8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253CEDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8253CEE0: 4BFF8220  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253CEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253CEE8 size=236
    let mut pc: u32 = 0x8253CEE8;
    'dispatch: loop {
        match pc {
            0x8253CEE8 => {
    //   block [0x8253CEE8..0x8253CFD4)
	// 8253CEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253CEEC: 4BFF81CD  bl 0x825350b8
	ctx.lr = 0x8253CEF0;
	sub_82535080(ctx, base);
	// 8253CEF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253CEF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253CEF8: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CEFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8253CF00: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8253CF04: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8253CF08: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8253CF0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253CF10: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8253CF14: 480070B5  bl 0x82543fc8
	ctx.lr = 0x8253CF18;
	sub_82543FC8(ctx, base);
	// 8253CF18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8253CF1C: 409A0034  bne cr6, 0x8253cf50
	if !ctx.cr[6].eq {
	pc = 0x8253CF50; continue 'dispatch;
	}
	// 8253CF20: 4BFFDA19  bl 0x8253a938
	ctx.lr = 0x8253CF24;
	sub_8253A938(ctx, base);
	// 8253CF24: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253CF28: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253CF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253CF30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253CF34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253CF38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253CF3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253CF40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253CF44: 4BFFD8BD  bl 0x8253a800
	ctx.lr = 0x8253CF48;
	sub_8253A800(ctx, base);
	// 8253CF48: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8253CF4C: 48000080  b 0x8253cfcc
	pc = 0x8253CFCC; continue 'dispatch;
	// 8253CF50: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8253CF54: 419AFFCC  beq cr6, 0x8253cf20
	if ctx.cr[6].eq {
	pc = 0x8253CF20; continue 'dispatch;
	}
	// 8253CF58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253CF5C: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 8253CF60: 409A000C  bne cr6, 0x8253cf6c
	if !ctx.cr[6].eq {
	pc = 0x8253CF6C; continue 'dispatch;
	}
	// 8253CF64: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8253CF68: 48000014  b 0x8253cf7c
	pc = 0x8253CF7C; continue 'dispatch;
	// 8253CF6C: 394BFFD3  addi r10, r11, -0x2d
	ctx.r[10].s64 = ctx.r[11].s64 + -45;
	// 8253CF70: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8253CF74: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8253CF78: 7C8AE850  subf r4, r10, r29
	ctx.r[4].s64 = ctx.r[29].s64 - ctx.r[10].s64;
	// 8253CF7C: 396BFFD3  addi r11, r11, -0x2d
	ctx.r[11].s64 = ctx.r[11].s64 + -45;
	// 8253CF80: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253CF84: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8253CF88: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253CF8C: 7CABF214  add r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8253CF90: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8253CF94: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8253CF98: 48006DC1  bl 0x82543d58
	ctx.lr = 0x8253CF9C;
	sub_82543D58(ctx, base);
	// 8253CF9C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253CFA0: 41820010  beq 0x8253cfb0
	if ctx.cr[0].eq {
	pc = 0x8253CFB0; continue 'dispatch;
	}
	// 8253CFA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253CFA8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253CFAC: 48000020  b 0x8253cfcc
	pc = 0x8253CFCC; continue 'dispatch;
	// 8253CFB0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8253CFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253CFB8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253CFBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8253CFC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8253CFC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253CFC8: 4BFFFD49  bl 0x8253cd10
	ctx.lr = 0x8253CFCC;
	sub_8253CD10(ctx, base);
	// 8253CFCC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8253CFD0: 4BFF8138  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8253CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8253CFD8 size=336
    let mut pc: u32 = 0x8253CFD8;
    'dispatch: loop {
        match pc {
            0x8253CFD8 => {
    //   block [0x8253CFD8..0x8253D128)
	// 8253CFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8253CFDC: 4BFF80D1  bl 0x825350ac
	ctx.lr = 0x8253CFE0;
	sub_82535080(ctx, base);
	// 8253CFE0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8253CFE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8253CFE8: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8253CFEC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8253CFF0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8253CFF4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8253CFF8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8253CFFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8253D000: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8253D004: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8253D008: 48006FC1  bl 0x82543fc8
	ctx.lr = 0x8253D00C;
	sub_82543FC8(ctx, base);
	// 8253D00C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8253D010: 409A0034  bne cr6, 0x8253d044
	if !ctx.cr[6].eq {
	pc = 0x8253D044; continue 'dispatch;
	}
	// 8253D014: 4BFFD925  bl 0x8253a938
	ctx.lr = 0x8253D018;
	sub_8253A938(ctx, base);
	// 8253D018: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8253D01C: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 8253D020: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8253D024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8253D028: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8253D02C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8253D030: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8253D034: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8253D038: 4BFFD7C9  bl 0x8253a800
	ctx.lr = 0x8253D03C;
	sub_8253A800(ctx, base);
	// 8253D03C: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8253D040: 480000E0  b 0x8253d120
	pc = 0x8253D120; continue 'dispatch;
	// 8253D044: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8253D048: 419AFFCC  beq cr6, 0x8253d014
	if ctx.cr[6].eq {
	pc = 0x8253D014; continue 'dispatch;
	}
	// 8253D04C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8253D050: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8253D054: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253D058: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8253D05C: 396BFFD3  addi r11, r11, -0x2d
	ctx.r[11].s64 = ctx.r[11].s64 + -45;
	// 8253D060: 3BAAFFFF  addi r29, r10, -1
	ctx.r[29].s64 = ctx.r[10].s64 + -1;
	// 8253D064: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8253D068: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8253D06C: 7F8BFA14  add r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8253D070: 419A0008  beq cr6, 0x8253d078
	if ctx.cr[6].eq {
	pc = 0x8253D078; continue 'dispatch;
	}
	// 8253D074: 7C8BF050  subf r4, r11, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8253D078: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253D07C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8253D080: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8253D084: 48006CD5  bl 0x82543d58
	ctx.lr = 0x8253D088;
	sub_82543D58(ctx, base);
	// 8253D088: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8253D08C: 41820010  beq 0x8253d09c
	if ctx.cr[0].eq {
	pc = 0x8253D09C; continue 'dispatch;
	}
	// 8253D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8253D094: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8253D098: 48000088  b 0x8253d120
	pc = 0x8253D120; continue 'dispatch;
	// 8253D09C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8253D0A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8253D0A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8253D0A8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8253D0AC: 41980008  blt cr6, 0x8253d0b4
	if ctx.cr[6].lt {
	pc = 0x8253D0B4; continue 'dispatch;
	}
	// 8253D0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8253D0B4: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8253D0B8: 41980048  blt cr6, 0x8253d100
	if ctx.cr[6].lt {
	pc = 0x8253D100; continue 'dispatch;
	}
	// 8253D0BC: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8253D0C0: 40980040  bge cr6, 0x8253d100
	if !ctx.cr[6].lt {
	pc = 0x8253D100; continue 'dispatch;
	}
	// 8253D0C4: 7D4B0775  extsb. r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8253D0C8: 41820018  beq 0x8253d0e0
	if ctx.cr[0].eq {
	pc = 0x8253D0E0; continue 'dispatch;
	}
	// 8253D0CC: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8253D0D0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8253D0D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8253D0D8: 409AFFF4  bne cr6, 0x8253d0cc
	if !ctx.cr[6].eq {
	pc = 0x8253D0CC; continue 'dispatch;
	}
	// 8253D0DC: 997CFFFE  stb r11, -2(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(-2 as u32), ctx.r[11].u8 ) };
	// 8253D0E0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8253D0E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8253D0E8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8253D0EC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8253D0F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253D0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253D0F8: 4BFFFC19  bl 0x8253cd10
	ctx.lr = 0x8253D0FC;
	sub_8253CD10(ctx, base);
	// 8253D0FC: 48000024  b 0x8253d120
	pc = 0x8253D120; continue 'dispatch;
	// 8253D100: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 8253D104: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8253D108: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8253D10C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8253D110: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8253D114: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8253D118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8253D11C: 4BFFF415  bl 0x8253c530
	ctx.lr = 0x8253D120;
	sub_8253C530(ctx, base);
	// 8253D120: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8253D124: 4BFF7FD8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


