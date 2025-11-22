pub fn sub_82EEFB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFB90 size=16
    let mut pc: u32 = 0x82EEFB90;
    'dispatch: loop {
        match pc {
            0x82EEFB90 => {
    //   block [0x82EEFB90..0x82EEFBA0)
	// 82EEFB90: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EEFB94: 388000AC  li r4, 0xac
	ctx.r[4].s64 = 172;
	// 82EEFB98: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EEFB9C: 4BFFDCA4  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFBA0 size=20
    let mut pc: u32 = 0x82EEFBA0;
    'dispatch: loop {
        match pc {
            0x82EEFBA0 => {
    //   block [0x82EEFBA0..0x82EEFBB4)
	// 82EEFBA0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82EEFBA4: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 82EEFBA8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EEFBAC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EEFBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EEFBB8 size=384
    let mut pc: u32 = 0x82EEFBB8;
    'dispatch: loop {
        match pc {
            0x82EEFBB8 => {
    //   block [0x82EEFBB8..0x82EEFD38)
	// 82EEFBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EEFBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EEFBC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EEFBC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EEFBC8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EEFBCC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EEFBD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EEFBD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFBD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EEFBDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EEFBE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EEFBE4: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFBE8: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82EEFBEC: C3EA9F7C  lfs f31, -0x6084(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EEFBF0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EEFBF4: 4BFB0FCD  bl 0x82ea0bc0
	ctx.lr = 0x82EEFBF8;
	sub_82EA0BC0(ctx, base);
	// 82EEFBF8: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EEFBFC: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 82EEFC00: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82EEFC04: 54E6DFFE  rlwinm r6, r7, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82EEFC08: 68C50001  xori r5, r6, 1
	ctx.r[5].u64 = ctx.r[6].u64 ^ 1;
	// 82EEFC0C: 7CA40774  extsb r4, r5
	ctx.r[4].s64 = ctx.r[5].s8 as i64;
	// 82EEFC10: 98BF0000  stb r5, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 82EEFC14: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EEFC18: 419A0024  beq cr6, 0x82eefc3c
	if ctx.cr[6].eq {
	pc = 0x82EEFC3C; continue 'dispatch;
	}
	// 82EEFC1C: 389E0050  addi r4, r30, 0x50
	ctx.r[4].s64 = ctx.r[30].s64 + 80;
	// 82EEFC20: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EEFC24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EEFC28: 4BFB0F99  bl 0x82ea0bc0
	ctx.lr = 0x82EEFC2C;
	sub_82EA0BC0(ctx, base);
	// 82EEFC2C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EEFC30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EEFC34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFC38: 409A0008  bne cr6, 0x82eefc40
	if !ctx.cr[6].eq {
	pc = 0x82EEFC40; continue 'dispatch;
	}
	// 82EEFC3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFC40: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EEFC44: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFC48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EEFC4C: 419A001C  beq cr6, 0x82eefc68
	if ctx.cr[6].eq {
	pc = 0x82EEFC68; continue 'dispatch;
	}
	// 82EEFC50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EEFC54: C1BE009C  lfs f13, 0x9c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EEFC58: C00B0790  lfs f0, 0x790(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1936 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EEFC5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFC60: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EEFC64: 419A0008  beq cr6, 0x82eefc6c
	if ctx.cr[6].eq {
	pc = 0x82EEFC6C; continue 'dispatch;
	}
	// 82EEFC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFC6C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EEFC70: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFC74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EEFC78: 419A001C  beq cr6, 0x82eefc94
	if ctx.cr[6].eq {
	pc = 0x82EEFC94; continue 'dispatch;
	}
	// 82EEFC7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EEFC80: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EEFC84: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EEFC88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFC8C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EEFC90: 40980008  bge cr6, 0x82eefc98
	if !ctx.cr[6].lt {
	pc = 0x82EEFC98; continue 'dispatch;
	}
	// 82EEFC94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFC98: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EEFC9C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFCA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EEFCA4: 419A001C  beq cr6, 0x82eefcc0
	if ctx.cr[6].eq {
	pc = 0x82EEFCC0; continue 'dispatch;
	}
	// 82EEFCA8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82EEFCAC: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EEFCB0: C00B7590  lfs f0, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EEFCB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFCB8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EEFCBC: 41980008  blt cr6, 0x82eefcc4
	if ctx.cr[6].lt {
	pc = 0x82EEFCC4; continue 'dispatch;
	}
	// 82EEFCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFCC4: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EEFCC8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFCCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EEFCD0: 419A0018  beq cr6, 0x82eefce8
	if ctx.cr[6].eq {
	pc = 0x82EEFCE8; continue 'dispatch;
	}
	// 82EEFCD4: C01E00B0  lfs f0, 0xb0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EEFCD8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFCDC: C1BE00B4  lfs f13, 0xb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EEFCE0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82EEFCE4: 40990008  ble cr6, 0x82eefcec
	if !ctx.cr[6].gt {
	pc = 0x82EEFCEC; continue 'dispatch;
	}
	// 82EEFCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFCEC: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82EEFCF0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFCF4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EEFCF8: 419A0018  beq cr6, 0x82eefd10
	if ctx.cr[6].eq {
	pc = 0x82EEFD10; continue 'dispatch;
	}
	// 82EEFCFC: C01E0088  lfs f0, 0x88(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EEFD00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EEFD04: C1BE008C  lfs f13, 0x8c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EEFD08: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82EEFD0C: 40990008  ble cr6, 0x82eefd14
	if !ctx.cr[6].gt {
	pc = 0x82EEFD14; continue 'dispatch;
	}
	// 82EEFD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFD14: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EEFD18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EEFD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EEFD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EEFD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EEFD28: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EEFD2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EEFD30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EEFD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFD38 size=24
    let mut pc: u32 = 0x82EEFD38;
    'dispatch: loop {
        match pc {
            0x82EEFD38 => {
    //   block [0x82EEFD38..0x82EEFD50)
	// 82EEFD38: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82EEFD3C: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EEFD40: 5549FEF6  rlwinm r9, r10, 0x1f, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82EEFD44: 69280010  xori r8, r9, 0x10
	ctx.r[8].u64 = ctx.r[9].u64 ^ 16;
	// 82EEFD48: 9903009A  stb r8, 0x9a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(154 as u32), ctx.r[8].u8 ) };
	// 82EEFD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFD50 size=40
    let mut pc: u32 = 0x82EEFD50;
    'dispatch: loop {
        match pc {
            0x82EEFD50 => {
    //   block [0x82EEFD50..0x82EEFD78)
	// 82EEFD50: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82EEFD54: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EEFD58: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFD78 size=40
    let mut pc: u32 = 0x82EEFD78;
    'dispatch: loop {
        match pc {
            0x82EEFD78 => {
    //   block [0x82EEFD78..0x82EEFDA0)
	// 82EEFD78: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82EEFD7C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EEFD80: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFDA0 size=8
    let mut pc: u32 = 0x82EEFDA0;
    'dispatch: loop {
        match pc {
            0x82EEFDA0 => {
    //   block [0x82EEFDA0..0x82EEFDA8)
	// 82EEFDA0: 38600013  li r3, 0x13
	ctx.r[3].s64 = 19;
	// 82EEFDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFDA8 size=304
    let mut pc: u32 = 0x82EEFDA8;
    'dispatch: loop {
        match pc {
            0x82EEFDA8 => {
    //   block [0x82EEFDA8..0x82EEFED8)
	// 82EEFDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EEFDAC: 482B83B9  bl 0x831a8164
	ctx.lr = 0x82EEFDB0;
	sub_831A8130(ctx, base);
	// 82EEFDB0: 3921FFC0  addi r9, r1, -0x40
	ctx.r[9].s64 = ctx.r[1].s64 + -64;
	// 82EEFDB4: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82EEFDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EEFDBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EEFDC0: 38E1FFC0  addi r7, r1, -0x40
	ctx.r[7].s64 = ctx.r[1].s64 + -64;
	// 82EEFDC4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EEFDC8: 38A6713C  addi r5, r6, 0x713c
	ctx.r[5].s64 = ctx.r[6].s64 + 28988;
	// 82EEFDCC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EEFDD0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EEFED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EEFED8 size=340
    let mut pc: u32 = 0x82EEFED8;
    'dispatch: loop {
        match pc {
            0x82EEFED8 => {
    //   block [0x82EEFED8..0x82EF002C)
	// 82EEFED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EEFEDC: 482B8285  bl 0x831a8160
	ctx.lr = 0x82EEFEE0;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0030 size=16
    let mut pc: u32 = 0x82EF0030;
    'dispatch: loop {
        match pc {
            0x82EF0030 => {
    //   block [0x82EF0030..0x82EF0040)
	// 82EF0030: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF0034: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82EF0038: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EF003C: 4BFFD804  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0040 size=32
    let mut pc: u32 = 0x82EF0040;
    'dispatch: loop {
        match pc {
            0x82EF0040 => {
    //   block [0x82EF0040..0x82EF0060)
	// 82EF0040: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82EF0044: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF0048: 419A0018  beq cr6, 0x82ef0060
	if ctx.cr[6].eq {
		sub_82EF0060(ctx, base);
		return;
	}
	// 82EF004C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF0050: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82EF0054: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF0058: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF005C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0060 size=16
    let mut pc: u32 = 0x82EF0060;
    'dispatch: loop {
        match pc {
            0x82EF0060 => {
    //   block [0x82EF0060..0x82EF0070)
	// 82EF0060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF0064: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF0068: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF006C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF0070 size=48
    let mut pc: u32 = 0x82EF0070;
    'dispatch: loop {
        match pc {
            0x82EF0070 => {
    //   block [0x82EF0070..0x82EF00A0)
	// 82EF0070: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF0074: C1A40070  lfs f13, 0x70(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF0078: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF007C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EF0080: 40990014  ble cr6, 0x82ef0094
	if !ctx.cr[6].gt {
	pc = 0x82EF0094; continue 'dispatch;
	}
	// 82EF0084: C1A40074  lfs f13, 0x74(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF0088: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF008C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EF0090: 41990008  bgt cr6, 0x82ef0098
	if ctx.cr[6].gt {
	pc = 0x82EF0098; continue 'dispatch;
	}
	// 82EF0094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF0098: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF009C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF00A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF00A0 size=8
    let mut pc: u32 = 0x82EF00A0;
    'dispatch: loop {
        match pc {
            0x82EF00A0 => {
    //   block [0x82EF00A0..0x82EF00A8)
	// 82EF00A0: 3860000F  li r3, 0xf
	ctx.r[3].s64 = 15;
	// 82EF00A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF00A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF00A8 size=136
    let mut pc: u32 = 0x82EF00A8;
    'dispatch: loop {
        match pc {
            0x82EF00A8 => {
    //   block [0x82EF00A8..0x82EF0130)
	// 82EF00A8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EF00AC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82EF00B0: 392A73F4  addi r9, r10, 0x73f4
	ctx.r[9].s64 = ctx.r[10].s64 + 29684;
	// 82EF00B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EF00B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EF00BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF00C0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82EF00C4: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82EF00C8: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82EF00CC: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82EF00D0: B0C30010  sth r6, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[6].u16 ) };
	// 82EF00D4: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82EF00D8: B0A30040  sth r5, 0x40(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[5].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0130 size=368
    let mut pc: u32 = 0x82EF0130;
    'dispatch: loop {
        match pc {
            0x82EF0130 => {
    //   block [0x82EF0130..0x82EF02A0)
	// 82EF0130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0134: 482B8031  bl 0x831a8164
	ctx.lr = 0x82EF0138;
	sub_831A8130(ctx, base);
	// 82EF0138: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF02A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF02A0 size=156
    let mut pc: u32 = 0x82EF02A0;
    'dispatch: loop {
        match pc {
            0x82EF02A0 => {
    //   block [0x82EF02A0..0x82EF033C)
	// 82EF02A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF02A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF02A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF02AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF02B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF02B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF02B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF02BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF02C0: 419A001C  beq cr6, 0x82ef02dc
	if ctx.cr[6].eq {
	pc = 0x82EF02DC; continue 'dispatch;
	}
	// 82EF02C4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF02C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF02CC: 419A0010  beq cr6, 0x82ef02dc
	if ctx.cr[6].eq {
	pc = 0x82EF02DC; continue 'dispatch;
	}
	// 82EF02D0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF02D4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF02D8: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF02DC: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF02E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF02E4: 419A003C  beq cr6, 0x82ef0320
	if ctx.cr[6].eq {
	pc = 0x82EF0320; continue 'dispatch;
	}
	// 82EF02E8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF02EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF02F0: 419A0030  beq cr6, 0x82ef0320
	if ctx.cr[6].eq {
	pc = 0x82EF0320; continue 'dispatch;
	}
	// 82EF02F4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF02F8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF02FC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF0300: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF0304: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF0308: 409A0018  bne cr6, 0x82ef0320
	if !ctx.cr[6].eq {
	pc = 0x82EF0320; continue 'dispatch;
	}
	// 82EF030C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0310: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF0314: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0318: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF031C: 4E800421  bctrl
	ctx.lr = 0x82EF0320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF0320: 93FE00AC  stw r31, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82EF0324: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF0328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF032C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF0330: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF0334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF0338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0340 size=16
    let mut pc: u32 = 0x82EF0340;
    'dispatch: loop {
        match pc {
            0x82EF0340 => {
    //   block [0x82EF0340..0x82EF0350)
	// 82EF0340: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF0344: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82EF0348: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EF034C: 4BFFD4F4  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0350 size=20
    let mut pc: u32 = 0x82EF0350;
    'dispatch: loop {
        match pc {
            0x82EF0350 => {
    //   block [0x82EF0350..0x82EF0364)
	// 82EF0350: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82EF0354: 39400048  li r10, 0x48
	ctx.r[10].s64 = 72;
	// 82EF0358: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF035C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF0360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF0368 size=156
    let mut pc: u32 = 0x82EF0368;
    'dispatch: loop {
        match pc {
            0x82EF0368 => {
    //   block [0x82EF0368..0x82EF0404)
	// 82EF0368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF036C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF0370: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF0374: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF0378: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EF037C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0380: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF0384: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF0388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF038C: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82EF0390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF0394: C3EB9F7C  lfs f31, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF0398: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF039C: 4BFB0825  bl 0x82ea0bc0
	ctx.lr = 0x82EF03A0;
	sub_82EA0BC0(ctx, base);
	// 82EF03A0: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF03A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF03A8: 419A0034  beq cr6, 0x82ef03dc
	if ctx.cr[6].eq {
	pc = 0x82EF03DC; continue 'dispatch;
	}
	// 82EF03AC: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82EF03B0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF03B4: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EF03B8: 4BFB0809  bl 0x82ea0bc0
	ctx.lr = 0x82EF03BC;
	sub_82EA0BC0(ctx, base);
	// 82EF03BC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF03C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF03C4: 419A0018  beq cr6, 0x82ef03dc
	if ctx.cr[6].eq {
	pc = 0x82EF03DC; continue 'dispatch;
	}
	// 82EF03C8: C01E00C8  lfs f0, 0xc8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF03CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF03D0: C1BE00CC  lfs f13, 0xcc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF03D4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82EF03D8: 40990008  ble cr6, 0x82ef03e0
	if !ctx.cr[6].gt {
	pc = 0x82EF03E0; continue 'dispatch;
	}
	// 82EF03DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF03E0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF03E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF03E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF03EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF03F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF03F4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EF03F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF03FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF0400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0408 size=8
    let mut pc: u32 = 0x82EF0408;
    'dispatch: loop {
        match pc {
            0x82EF0408 => {
    //   block [0x82EF0408..0x82EF0410)
	// 82EF0408: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82EF040C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF0410 size=132
    let mut pc: u32 = 0x82EF0410;
    'dispatch: loop {
        match pc {
            0x82EF0410 => {
    //   block [0x82EF0410..0x82EF0494)
	// 82EF0410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF0418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF041C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0420: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF0424: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EF0428: 394B7614  addi r10, r11, 0x7614
	ctx.r[10].s64 = ctx.r[11].s64 + 30228;
	// 82EF042C: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF0430: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF0434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF0438: 419A003C  beq cr6, 0x82ef0474
	if ctx.cr[6].eq {
	pc = 0x82EF0474; continue 'dispatch;
	}
	// 82EF043C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF0440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF0444: 419A0030  beq cr6, 0x82ef0474
	if ctx.cr[6].eq {
	pc = 0x82EF0474; continue 'dispatch;
	}
	// 82EF0448: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF044C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF0450: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF0454: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF0458: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF045C: 409A0018  bne cr6, 0x82ef0474
	if !ctx.cr[6].eq {
	pc = 0x82EF0474; continue 'dispatch;
	}
	// 82EF0460: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0464: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF0468: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF046C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF0470: 4E800421  bctrl
	ctx.lr = 0x82EF0474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF0474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF0478: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF047C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF0480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF0484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF0488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF048C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF0490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0498 size=28
    let mut pc: u32 = 0x82EF0498;
    'dispatch: loop {
        match pc {
            0x82EF0498 => {
    //   block [0x82EF0498..0x82EF04B4)
	// 82EF0498: 7CAB0774  extsb r11, r5
	ctx.r[11].s64 = ctx.r[5].s8 as i64;
	// 82EF049C: 98A300A2  stb r5, 0xa2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[5].u8 ) };
	// 82EF04A0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF04A4: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF04A8: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EF04AC: 992300B2  stb r9, 0xb2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(178 as u32), ctx.r[9].u8 ) };
	// 82EF04B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF04B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF04B4 size=12
    let mut pc: u32 = 0x82EF04B4;
    'dispatch: loop {
        match pc {
            0x82EF04B4 => {
    //   block [0x82EF04B4..0x82EF04C0)
	// 82EF04B4: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF04B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF04BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF04C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF04C0 size=12
    let mut pc: u32 = 0x82EF04C0;
    'dispatch: loop {
        match pc {
            0x82EF04C0 => {
    //   block [0x82EF04C0..0x82EF04CC)
	// 82EF04C0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF04C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF04C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF04CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF04CC size=28
    let mut pc: u32 = 0x82EF04CC;
    'dispatch: loop {
        match pc {
            0x82EF04CC => {
    //   block [0x82EF04CC..0x82EF04E8)
	// 82EF04CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF04D0: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF04D4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EF04D8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EF04DC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EF04E0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF04E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF04E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF04E8 size=336
    let mut pc: u32 = 0x82EF04E8;
    'dispatch: loop {
        match pc {
            0x82EF04E8 => {
    //   block [0x82EF04E8..0x82EF0638)
	// 82EF04E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF04EC: 482B7C71  bl 0x831a815c
	ctx.lr = 0x82EF04F0;
	sub_831A8130(ctx, base);
	// 82EF04F0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EF04F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF04F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EF04FC: 392A7614  addi r9, r10, 0x7614
	ctx.r[9].s64 = ctx.r[10].s64 + 30228;
	// 82EF0500: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF0504: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82EF0508: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82EF050C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF0510: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82EF0514: B3E30010  sth r31, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 82EF0518: 38C1FFB0  addi r6, r1, -0x50
	ctx.r[6].s64 = ctx.r[1].s64 + -80;
	// 82EF051C: B0E300A0  sth r7, 0xa0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[7].u16 ) };
	// 82EF0520: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82EF0524: 990300A2  stb r8, 0xa2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[8].u8 ) };
	// 82EF0528: 3BC0000D  li r30, 0xd
	ctx.r[30].s64 = 13;
	// 82EF052C: B16300A4  sth r11, 0xa4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u16 ) };
	// 82EF0530: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82EF0534: B16300A6  sth r11, 0xa6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(166 as u32), ctx.r[11].u16 ) };
	// 82EF0538: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF0638 size=536
    let mut pc: u32 = 0x82EF0638;
    'dispatch: loop {
        match pc {
            0x82EF0638 => {
    //   block [0x82EF0638..0x82EF0850)
	// 82EF0638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF063C: 482B7B29  bl 0x831a8164
	ctx.lr = 0x82EF0640;
	sub_831A8130(ctx, base);
	// 82EF0640: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0850 size=8
    let mut pc: u32 = 0x82EF0850;
    'dispatch: loop {
        match pc {
            0x82EF0850 => {
    //   block [0x82EF0850..0x82EF0858)
	// 82EF0850: 38600066  li r3, 0x66
	ctx.r[3].s64 = 102;
	// 82EF0854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0858 size=132
    let mut pc: u32 = 0x82EF0858;
    'dispatch: loop {
        match pc {
            0x82EF0858 => {
    //   block [0x82EF0858..0x82EF08DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF08DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF08DC size=28
    let mut pc: u32 = 0x82EF08DC;
    'dispatch: loop {
        match pc {
            0x82EF08DC => {
    //   block [0x82EF08DC..0x82EF08F8)
	// 82EF08DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF08F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF08F8 size=88
    let mut pc: u32 = 0x82EF08F8;
    'dispatch: loop {
        match pc {
            0x82EF08F8 => {
    //   block [0x82EF08F8..0x82EF0950)
	// 82EF08F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF08FC: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82EF0900: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF0904: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF0908: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EF090C: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF0910: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF0914: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF0918: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EF091C: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF0920: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF0924: 1D4B0534  mulli r10, r11, 0x534
	ctx.r[10].s64 = ctx.r[11].s64 * 1332;
	// 82EF0928: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF092C: 38EA0053  addi r7, r10, 0x53
	ctx.r[7].s64 = ctx.r[10].s64 + 83;
	// 82EF0930: 7CCB4A14  add r6, r11, r9
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF0934: 54EB0036  rlwinm r11, r7, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF0938: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF093C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EF0940: 90A40008  stw r5, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EF0944: 90640004  stw r3, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF0948: 90A4000C  stw r5, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82EF094C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0950 size=52
    let mut pc: u32 = 0x82EF0950;
    'dispatch: loop {
        match pc {
            0x82EF0950 => {
    //   block [0x82EF0950..0x82EF0984)
	// 82EF0950: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF0954: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF0958: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF095C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF0960: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82EF0964: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF0968: 550A003A  rlwinm r10, r8, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF096C: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF0970: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF0974: 54EB1838  slwi r11, r7, 3
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF0978: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF097C: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF0980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF0988 size=72
    let mut pc: u32 = 0x82EF0988;
    'dispatch: loop {
        match pc {
            0x82EF0988 => {
    //   block [0x82EF0988..0x82EF09D0)
	// 82EF0988: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF098C: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF0990: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82EF0994: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF0998: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EF099C: 7D065030  slw r6, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82EF09A0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF09A4: 7CE55030  slw r5, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82EF09A8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF09AC: 8129001C  lwz r9, 0x1c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF09B0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF09B4: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF09B8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF09BC: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82EF09C0: 7D493078  andc r9, r10, r6
	ctx.r[9].u64 = ctx.r[10].u64 & !ctx.r[6].u64;
	// 82EF09C4: 7D282B78  or r8, r9, r5
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82EF09C8: 7D0B21AE  stbx r8, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[8].u8) };
	// 82EF09CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF09D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF09D0 size=72
    let mut pc: u32 = 0x82EF09D0;
    'dispatch: loop {
        match pc {
            0x82EF09D0 => {
    //   block [0x82EF09D0..0x82EF0A18)
	// 82EF09D0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF09D4: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF09D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82EF09DC: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF09E0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82EF09E4: 7D065030  slw r6, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82EF09E8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF09EC: 7CE55030  slw r5, r7, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[7].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82EF09F0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF09F4: 8129001C  lwz r9, 0x1c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF09F8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF09FC: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF0A00: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF0A04: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82EF0A08: 7D493078  andc r9, r10, r6
	ctx.r[9].u64 = ctx.r[10].u64 & !ctx.r[6].u64;
	// 82EF0A0C: 7D282B78  or r8, r9, r5
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82EF0A10: 7D0B21AE  stbx r8, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[8].u8) };
	// 82EF0A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF0A18 size=164
    let mut pc: u32 = 0x82EF0A18;
    'dispatch: loop {
        match pc {
            0x82EF0A18 => {
    //   block [0x82EF0A18..0x82EF0ABC)
	// 82EF0A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF0A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF0A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF0A2C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF0A30: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82EF0A34: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82EF0A38: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82EF0A3C: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82EF0A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF0A44: 3889B748  addi r4, r9, -0x48b8
	ctx.r[4].s64 = ctx.r[9].s64 + -18616;
	// 82EF0A48: C188C664  lfs f12, -0x399c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF0A4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF0A50: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF0A54: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EF0A58: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF0A5C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF0A60: C00708A8  lfs f0, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF0A64: B15F000C  sth r10, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 82EF0A68: C1A6B6EC  lfs f13, -0x4914(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-18708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF0A6C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EF0A70: C165964C  lfs f11, -0x69b4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF0A74: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF0A78: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EF0A7C: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82EF0A80: D19F0024  stfs f12, 0x24(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82EF0A84: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82EF0A88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF0A8C: D1BF002C  stfs f13, 0x2c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82EF0A90: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82EF0A94: D1BF0034  stfs f13, 0x34(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82EF0A98: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82EF0A9C: D17F003C  stfs f11, 0x3c(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82EF0AA0: 4BFFC939  bl 0x82eed3d8
	ctx.lr = 0x82EF0AA4;
	sub_82EED3D8(ctx, base);
	// 82EF0AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF0AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF0AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF0AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF0AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF0AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF0AC0 size=100
    let mut pc: u32 = 0x82EF0AC0;
    'dispatch: loop {
        match pc {
            0x82EF0AC0 => {
    //   block [0x82EF0AC0..0x82EF0B24)
	// 82EF0AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF0AC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF0ACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF0AD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0AD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF0AD8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF0ADC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF0AE0: 392BB748  addi r9, r11, -0x48b8
	ctx.r[9].s64 = ctx.r[11].s64 + -18616;
	// 82EF0AE4: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82EF0AE8: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF0AEC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF0AF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF0AF4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF0AF8: 4BFFC8E1  bl 0x82eed3d8
	ctx.lr = 0x82EF0AFC;
	sub_82EED3D8(ctx, base);
	// 82EF0AFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF0B00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF0B04: 4BFFC8D5  bl 0x82eed3d8
	ctx.lr = 0x82EF0B08;
	sub_82EED3D8(ctx, base);
	// 82EF0B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF0B0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF0B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF0B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF0B18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF0B1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF0B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF0B28 size=240
    let mut pc: u32 = 0x82EF0B28;
    'dispatch: loop {
        match pc {
            0x82EF0B28 => {
    //   block [0x82EF0B28..0x82EF0C18)
	// 82EF0B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0B2C: 482B7639  bl 0x831a8164
	ctx.lr = 0x82EF0B30;
	sub_831A8130(ctx, base);
	// 82EF0B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0B34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF0B38: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF0B3C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EF0B40: 392BB748  addi r9, r11, -0x48b8
	ctx.r[9].s64 = ctx.r[11].s64 + -18616;
	// 82EF0B44: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF0B48: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF0B4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF0B50: 4099007C  ble cr6, 0x82ef0bcc
	if !ctx.cr[6].gt {
	pc = 0x82EF0BCC; continue 'dispatch;
	}
	// 82EF0B54: 3B800040  li r28, 0x40
	ctx.r[28].s64 = 64;
	// 82EF0B58: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82EF0B5C: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 82EF0B60: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF0B64: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF0B68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF0B6C: 419A0040  beq cr6, 0x82ef0bac
	if ctx.cr[6].eq {
	pc = 0x82EF0BAC; continue 'dispatch;
	}
	// 82EF0B70: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF0B74: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF0B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF0B7C: 419A0030  beq cr6, 0x82ef0bac
	if ctx.cr[6].eq {
	pc = 0x82EF0BAC; continue 'dispatch;
	}
	// 82EF0B80: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF0B84: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF0B88: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF0B8C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF0B90: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF0B94: 409A0018  bne cr6, 0x82ef0bac
	if !ctx.cr[6].eq {
	pc = 0x82EF0BAC; continue 'dispatch;
	}
	// 82EF0B98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0B9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF0BA0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0BA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF0BA8: 4E800421  bctrl
	ctx.lr = 0x82EF0BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF0BAC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF0BB0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EF0BB4: 4082FFAC  bne 0x82ef0b60
	if !ctx.cr[0].eq {
	pc = 0x82EF0B60; continue 'dispatch;
	}
	// 82EF0BB8: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF0BBC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82EF0BC0: 3B9C0050  addi r28, r28, 0x50
	ctx.r[28].s64 = ctx.r[28].s64 + 80;
	// 82EF0BC4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF0BC8: 4198FF90  blt cr6, 0x82ef0b58
	if ctx.cr[6].lt {
	pc = 0x82EF0B58; continue 'dispatch;
	}
	// 82EF0BCC: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF0BD0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF0BD4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF0BD8: 409A002C  bne cr6, 0x82ef0c04
	if !ctx.cr[6].eq {
	pc = 0x82EF0C04; continue 'dispatch;
	}
	// 82EF0BDC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF0BE0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0BE4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF0BE8: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF0BEC: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82EF0BF0: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF0BF4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EF0BF8: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF0BFC: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EF0C00: 4BFAFBB1  bl 0x82ea07b0
	ctx.lr = 0x82EF0C04;
	sub_82EA07B0(ctx, base);
	// 82EF0C04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF0C08: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF0C0C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF0C10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF0C14: 482B75A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF0C18 size=256
    let mut pc: u32 = 0x82EF0C18;
    'dispatch: loop {
        match pc {
            0x82EF0C18 => {
    //   block [0x82EF0C18..0x82EF0D18)
	// 82EF0C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0C1C: 482B7541  bl 0x831a815c
	ctx.lr = 0x82EF0C20;
	sub_831A8130(ctx, base);
	// 82EF0C20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0C24: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF0C28: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82EF0C2C: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF0C30: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF0C34: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF0C38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF0C3C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EF0C40: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82EF0C44: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF0C48: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 82EF0C4C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF0C50: 409A0010  bne cr6, 0x82ef0c60
	if !ctx.cr[6].eq {
	pc = 0x82EF0C60; continue 'dispatch;
	}
	// 82EF0C54: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82EF0C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF0C5C: 4BFB5C25  bl 0x82ea6880
	ctx.lr = 0x82EF0C60;
	sub_82EA6880(ctx, base);
	// 82EF0C60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF0C64: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EF0C68: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0C6C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82EF0C70: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF0C74: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82EF0C78: 7CAB4A14  add r5, r11, r9
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF0C7C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF0D18 size=76
    let mut pc: u32 = 0x82EF0D18;
    'dispatch: loop {
        match pc {
            0x82EF0D18 => {
    //   block [0x82EF0D18..0x82EF0D64)
	// 82EF0D18: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF0D1C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF0D20: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82EF0D24: D1860000  stfs f12, 0(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EF0D28: C163000C  lfs f11, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF0D2C: C1440004  lfs f10, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF0D30: ED2B02B2  fmuls f9, f11, f10
	ctx.f[9].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 82EF0D34: D1260004  stfs f9, 4(r6)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EF0D38: C1030004  lfs f8, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82EF0D3C: D1060008  stfs f8, 8(r6)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EF0D40: C0E30010  lfs f7, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82EF0D44: D0E6000C  stfs f7, 0xc(r6)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF0D48: C0C30014  lfs f6, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82EF0D4C: D0C60010  stfs f6, 0x10(r6)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF0D50: C0A40008  lfs f5, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82EF0D54: C0830000  lfs f4, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82EF0D58: EC650132  fmuls f3, f5, f4
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[4].f64) as f32) as f64);
	// 82EF0D5C: D065001C  stfs f3, 0x1c(r5)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82EF0D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF0D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF0D68 size=1932
    let mut pc: u32 = 0x82EF0D68;
    'dispatch: loop {
        match pc {
            0x82EF0D68 => {
    //   block [0x82EF0D68..0x82EF14F4)
	// 82EF0D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF0D6C: 482B73C5  bl 0x831a8130
	ctx.lr = 0x82EF0D70;
	sub_831A8130(ctx, base);
	// 82EF0D70: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82EF0D74: 9421F490  stwu r1, -0xb70(r1)
	ea = ctx.r[1].u32.wrapping_add(-2928 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF0D78: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EF0D7C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF0D80: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82EF0D84: 93610B8C  stw r27, 0xb8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2956 as u32), ctx.r[27].u32 ) };
	// 82EF0D88: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF0D8C: 93810B94  stw r28, 0xb94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2964 as u32), ctx.r[28].u32 ) };
	// 82EF0D90: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82EF0D94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF0D98: 80BB004C  lwz r5, 0x4c(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF0D9C: 397B004C  addi r11, r27, 0x4c
	ctx.r[11].s64 = ctx.r[27].s64 + 76;
	// 82EF0DA0: 48016AF1  bl 0x82f07890
	ctx.lr = 0x82EF0DA4;
	sub_82F07890(ctx, base);
	// 82EF0DA4: 39410140  addi r10, r1, 0x140
	ctx.r[10].s64 = ctx.r[1].s64 + 320;
	// 82EF0DA8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EF0DAC: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF0DB0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF0DB4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EF0DB8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF0DBC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EF0DC0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF0DC4: 4200FFF0  bdnz 0x82ef0db4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF0DB4; continue 'dispatch;
	}
	// 82EF0DC8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82EF0DCC: 83FB0048  lwz r31, 0x48(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF0DD0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82EF0DD4: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF0DD8: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82EF0DDC: 392101AC  addi r9, r1, 0x1ac
	ctx.r[9].s64 = ctx.r[1].s64 + 428;
	// 82EF0DE0: 932101A4  stw r25, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[25].u32 ) };
	// 82EF0DE4: 93210188  stw r25, 0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[25].u32 ) };
	// 82EF0DE8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82EF0DEC: 912101A0  stw r9, 0x1a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[9].u32 ) };
	// 82EF0DF0: 38E102CC  addi r7, r1, 0x2cc
	ctx.r[7].s64 = ctx.r[1].s64 + 716;
	// 82EF0DF4: 9321018C  stw r25, 0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), ctx.r[25].u32 ) };
	// 82EF0DF8: 3BBF002C  addi r29, r31, 0x2c
	ctx.r[29].s64 = ctx.r[31].s64 + 44;
	// 82EF0DFC: 916101A8  stw r11, 0x1a8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(424 as u32), ctx.r[11].u32 ) };
	// 82EF0E00: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF0E04: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF0E08: 3A0BFFFF  addi r16, r11, -1
	ctx.r[16].s64 = ctx.r[11].s64 + -1;
	// 82EF0E0C: 80A60090  lwz r5, 0x90(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EF0E10: 7FC55050  subf r30, r5, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82EF0E14: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF0E18: 2F100020  cmpwi cr6, r16, 0x20
	ctx.cr[6].compare_i32(ctx.r[16].s32, 32, &mut ctx.xer);
	// 82EF0E1C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82EF0E20: 90E102C0  stw r7, 0x2c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(704 as u32), ctx.r[7].u32 ) };
	// 82EF0E24: 932102C4  stw r25, 0x2c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(708 as u32), ctx.r[25].u32 ) };
	// 82EF0E28: 910102C8  stw r8, 0x2c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(712 as u32), ctx.r[8].u32 ) };
	// 82EF0E2C: 40990020  ble cr6, 0x82ef0e4c
	if !ctx.cr[6].gt {
	pc = 0x82EF0E4C; continue 'dispatch;
	}
	// 82EF0E30: 2F100040  cmpwi cr6, r16, 0x40
	ctx.cr[6].compare_i32(ctx.r[16].s32, 64, &mut ctx.xer);
	// 82EF0E34: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82EF0E38: 41980008  blt cr6, 0x82ef0e40
	if ctx.cr[6].lt {
	pc = 0x82EF0E40; continue 'dispatch;
	}
	// 82EF0E3C: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82EF0E40: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82EF0E44: 386102C0  addi r3, r1, 0x2c0
	ctx.r[3].s64 = ctx.r[1].s64 + 704;
	// 82EF0E48: 4BFB59B1  bl 0x82ea67f8
	ctx.lr = 0x82EF0E4C;
	sub_82EA67F8(ctx, base);
	// 82EF0E4C: 920102C4  stw r16, 0x2c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(708 as u32), ctx.r[16].u32 ) };
	// 82EF0E50: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF0E54: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0E58: 812101A4  lwz r9, 0x1a4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 82EF0E5C: 810101A0  lwz r8, 0x1a0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(416 as u32) ) } as u64;
	// 82EF0E60: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF0E64: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF0E68: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0E6C: 38CB00E0  addi r6, r11, 0xe0
	ctx.r[6].s64 = ctx.r[11].s64 + 224;
	// 82EF0E70: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EF0E74: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF0E78: 7CBE5850  subf r5, r30, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EF0E7C: 91610174  stw r11, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 82EF0E80: 7CA7412E  stwx r5, r7, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u32) };
	// 82EF0E84: D0010160  stfs f0, 0x160(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82EF0E88: 81610148  lwz r11, 0x148(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(328 as u32) ) } as u64;
	// 82EF0E8C: 808101A4  lwz r4, 0x1a4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 82EF0E90: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 82EF0E94: 906101A4  stw r3, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[3].u32 ) };
	// 82EF0E98: 90C1017C  stw r6, 0x17c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), ctx.r[6].u32 ) };
	// 82EF0E9C: 9161015C  stw r11, 0x15c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), ctx.r[11].u32 ) };
	// 82EF0EA0: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0EA4: 4802285D  bl 0x82f13700
	ctx.lr = 0x82EF0EA8;
	sub_82F13700(ctx, base);
	// 82EF0EA8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0EAC: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82EF0EB0: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82EF0EB4: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82EF0EB8: 48022851  bl 0x82f13708
	ctx.lr = 0x82EF0EBC;
	sub_82F13708(ctx, base);
	// 82EF0EBC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EF0EC0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82EF0EC4: 7F36CB78  mr r22, r25
	ctx.r[22].u64 = ctx.r[25].u64;
	// 82EF0EC8: 2F100000  cmpwi cr6, r16, 0
	ctx.cr[6].compare_i32(ctx.r[16].s32, 0, &mut ctx.xer);
	// 82EF0ECC: C3E908A4  lfs f31, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF0ED0: 40990514  ble cr6, 0x82ef13e4
	if !ctx.cr[6].gt {
	pc = 0x82EF13E4; continue 'dispatch;
	}
	// 82EF0ED4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF0ED8: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82EF0EDC: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82EF0EE0: 396BC5A0  addi r11, r11, -0x3a60
	ctx.r[11].s64 = ctx.r[11].s64 + -14944;
	// 82EF0EE4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82EF0EE8: 39F7001C  addi r15, r23, 0x1c
	ctx.r[15].s64 = ctx.r[23].s64 + 28;
	// 82EF0EEC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82EF0EF0: 3A3F0028  addi r17, r31, 0x28
	ctx.r[17].s64 = ctx.r[31].s64 + 40;
	// 82EF0EF4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF0EF8: 3B570018  addi r26, r23, 0x18
	ctx.r[26].s64 = ctx.r[23].s64 + 24;
	// 82EF0EFC: 3A400010  li r18, 0x10
	ctx.r[18].s64 = 16;
	// 82EF0F00: 3A600030  li r19, 0x30
	ctx.r[19].s64 = 48;
	// 82EF0F04: 3A800020  li r20, 0x20
	ctx.r[20].s64 = 32;
	// 82EF0F08: 8101017C  lwz r8, 0x17c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 82EF0F0C: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 82EF0F10: 81210174  lwz r9, 0x174(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF0F14: 80E101A8  lwz r7, 0x1a8(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(424 as u32) ) } as u64;
	// 82EF0F18: 80C101A4  lwz r6, 0x1a4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 82EF0F1C: 54E500BE  clrlwi r5, r7, 2
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF0F20: 91010178  stw r8, 0x178(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[8].u32 ) };
	// 82EF0F24: 91210170  stw r9, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[9].u32 ) };
	// 82EF0F28: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EF0F2C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF0F30: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF0F34: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF0F38: 3BEB00D0  addi r31, r11, 0xd0
	ctx.r[31].s64 = ctx.r[11].s64 + 208;
	// 82EF0F3C: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EF0F40: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF0F44: 91610174  stw r11, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 82EF0F48: 7FDE5850  subf r30, r30, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EF0F4C: 409A0010  bne cr6, 0x82ef0f5c
	if !ctx.cr[6].eq {
	pc = 0x82EF0F5C; continue 'dispatch;
	}
	// 82EF0F50: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EF0F54: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 82EF0F58: 4BFB5929  bl 0x82ea6880
	ctx.lr = 0x82EF0F5C;
	sub_82EA6880(ctx, base);
	// 82EF0F5C: 814101A4  lwz r10, 0x1a4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 82EF0F60: 38E10250  addi r7, r1, 0x250
	ctx.r[7].s64 = ctx.r[1].s64 + 592;
	// 82EF0F64: 80C101A0  lwz r6, 0x1a0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(416 as u32) ) } as u64;
	// 82EF0F68: 38810230  addi r4, r1, 0x230
	ctx.r[4].s64 = ctx.r[1].s64 + 560;
	// 82EF0F6C: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF0F70: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82EF0F74: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82EF0F78: 7FC5312E  stwx r30, r5, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[6].u32), ctx.r[30].u32) };
	// 82EF0F7C: 9161017C  stw r11, 0x17c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), ctx.r[11].u32 ) };
	// 82EF0F80: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF0F84: 810101A4  lwz r8, 0x1a4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 82EF0F88: 7D395214  add r9, r25, r10
	ctx.r[9].u64 = ctx.r[25].u64 + ctx.r[10].u64;
	// 82EF0F8C: 81410178  lwz r10, 0x178(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(376 as u32) ) } as u64;
	// 82EF0F90: 38C80001  addi r6, r8, 1
	ctx.r[6].s64 = ctx.r[8].s64 + 1;
	// 82EF0F94: 39090010  addi r8, r9, 0x10
	ctx.r[8].s64 = ctx.r[9].s64 + 16;
	// 82EF0F98: 90C101A4  stw r6, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF14F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF14F8 size=100
    let mut pc: u32 = 0x82EF14F8;
    'dispatch: loop {
        match pc {
            0x82EF14F8 => {
    //   block [0x82EF14F8..0x82EF155C)
	// 82EF14F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF14FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF1500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF1504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF1508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF150C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF1510: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF1514: 4BFFF615  bl 0x82ef0b28
	ctx.lr = 0x82EF1518;
	sub_82EF0B28(ctx, base);
	// 82EF1518: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EF151C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1520: 419A0020  beq cr6, 0x82ef1540
	if ctx.cr[6].eq {
	pc = 0x82EF1540; continue 'dispatch;
	}
	// 82EF1524: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1528: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF152C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EF1530: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF1534: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF1538: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF153C: 4BFAF275  bl 0x82ea07b0
	ctx.lr = 0x82EF1540;
	sub_82EA07B0(ctx, base);
	// 82EF1540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF1544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF1548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF154C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF1550: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF1554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF1558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1560 size=400
    let mut pc: u32 = 0x82EF1560;
    'dispatch: loop {
        match pc {
            0x82EF1560 => {
    //   block [0x82EF1560..0x82EF16F0)
	// 82EF1560: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82EF1564: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EF1568: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF16F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF16F0 size=16
    let mut pc: u32 = 0x82EF16F0;
    'dispatch: loop {
        match pc {
            0x82EF16F0 => {
    //   block [0x82EF16F0..0x82EF1700)
	// 82EF16F0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF16F4: 38800094  li r4, 0x94
	ctx.r[4].s64 = 148;
	// 82EF16F8: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EF16FC: 4BFFC144  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1700 size=32
    let mut pc: u32 = 0x82EF1700;
    'dispatch: loop {
        match pc {
            0x82EF1700 => {
    //   block [0x82EF1700..0x82EF1720)
	// 82EF1700: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82EF1704: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF1708: 419A0018  beq cr6, 0x82ef1720
	if ctx.cr[6].eq {
		sub_82EF1720(ctx, base);
		return;
	}
	// 82EF170C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF1710: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82EF1714: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF1718: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF171C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1720 size=16
    let mut pc: u32 = 0x82EF1720;
    'dispatch: loop {
        match pc {
            0x82EF1720 => {
    //   block [0x82EF1720..0x82EF1730)
	// 82EF1720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF1724: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF1728: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF172C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF1730 size=140
    let mut pc: u32 = 0x82EF1730;
    'dispatch: loop {
        match pc {
            0x82EF1730 => {
    //   block [0x82EF1730..0x82EF17BC)
	// 82EF1730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF1734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF1738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF173C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF1740: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EF1744: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF1748: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF174C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF1750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF1754: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82EF1758: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF175C: C3EB9F7C  lfs f31, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF1760: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF1764: 4BFAF45D  bl 0x82ea0bc0
	ctx.lr = 0x82EF1768;
	sub_82EA0BC0(ctx, base);
	// 82EF1768: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF176C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF1770: 419A0024  beq cr6, 0x82ef1794
	if ctx.cr[6].eq {
	pc = 0x82EF1794; continue 'dispatch;
	}
	// 82EF1774: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82EF1778: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF177C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EF1780: 4BFAF441  bl 0x82ea0bc0
	ctx.lr = 0x82EF1784;
	sub_82EA0BC0(ctx, base);
	// 82EF1784: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1788: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF178C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF1790: 409A0008  bne cr6, 0x82ef1798
	if !ctx.cr[6].eq {
	pc = 0x82EF1798; continue 'dispatch;
	}
	// 82EF1794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF1798: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF179C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF17A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF17A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF17A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF17AC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EF17B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF17B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF17B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF17C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF17C0 size=8
    let mut pc: u32 = 0x82EF17C0;
    'dispatch: loop {
        match pc {
            0x82EF17C0 => {
    //   block [0x82EF17C0..0x82EF17C8)
	// 82EF17C0: 3860000E  li r3, 0xe
	ctx.r[3].s64 = 14;
	// 82EF17C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF17C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF17C8 size=468
    let mut pc: u32 = 0x82EF17C8;
    'dispatch: loop {
        match pc {
            0x82EF17C8 => {
    //   block [0x82EF17C8..0x82EF199C)
	// 82EF17C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF17CC: 482B699D  bl 0x831a8168
	ctx.lr = 0x82EF17D0;
	sub_831A8130(ctx, base);
	// 82EF17D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF17D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF17D8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF17DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF17E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF17E4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82EF17E8: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EF17EC: 4BFB56C5  bl 0x82ea6eb0
	ctx.lr = 0x82EF17F0;
	sub_82EA6EB0(ctx, base);
	// 82EF17F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF17F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF17F8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82EF17FC: 4BFB56B5  bl 0x82ea6eb0
	ctx.lr = 0x82EF1800;
	sub_82EA6EB0(ctx, base);
	// 82EF1800: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82EF1804: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82EF1808: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82EF180C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82EF1810: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF19A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF19A0 size=180
    let mut pc: u32 = 0x82EF19A0;
    'dispatch: loop {
        match pc {
            0x82EF19A0 => {
    //   block [0x82EF19A0..0x82EF1A54)
	// 82EF19A0: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82EF19A4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EF19A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF19AC: 38897904  addi r4, r9, 0x7904
	ctx.r[4].s64 = ctx.r[9].s64 + 30980;
	// 82EF19B0: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82EF19B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF19B8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1A58 size=380
    let mut pc: u32 = 0x82EF1A58;
    'dispatch: loop {
        match pc {
            0x82EF1A58 => {
    //   block [0x82EF1A58..0x82EF1BD4)
	// 82EF1A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF1A5C: 482B66FD  bl 0x831a8158
	ctx.lr = 0x82EF1A60;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF1BD8 size=156
    let mut pc: u32 = 0x82EF1BD8;
    'dispatch: loop {
        match pc {
            0x82EF1BD8 => {
    //   block [0x82EF1BD8..0x82EF1C74)
	// 82EF1BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF1BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF1BE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF1BE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF1BE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF1BEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF1BF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF1BF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF1BF8: 419A001C  beq cr6, 0x82ef1c14
	if ctx.cr[6].eq {
	pc = 0x82EF1C14; continue 'dispatch;
	}
	// 82EF1BFC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF1C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1C04: 419A0010  beq cr6, 0x82ef1c14
	if ctx.cr[6].eq {
	pc = 0x82EF1C14; continue 'dispatch;
	}
	// 82EF1C08: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF1C0C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF1C10: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF1C14: 807E00B0  lwz r3, 0xb0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF1C18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF1C1C: 419A003C  beq cr6, 0x82ef1c58
	if ctx.cr[6].eq {
	pc = 0x82EF1C58; continue 'dispatch;
	}
	// 82EF1C20: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF1C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1C28: 419A0030  beq cr6, 0x82ef1c58
	if ctx.cr[6].eq {
	pc = 0x82EF1C58; continue 'dispatch;
	}
	// 82EF1C2C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF1C30: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF1C34: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF1C38: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF1C3C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF1C40: 409A0018  bne cr6, 0x82ef1c58
	if !ctx.cr[6].eq {
	pc = 0x82EF1C58; continue 'dispatch;
	}
	// 82EF1C44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1C48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF1C4C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1C50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF1C54: 4E800421  bctrl
	ctx.lr = 0x82EF1C58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF1C58: 93FE00B0  stw r31, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82EF1C5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF1C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF1C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF1C68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF1C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF1C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1C78 size=16
    let mut pc: u32 = 0x82EF1C78;
    'dispatch: loop {
        match pc {
            0x82EF1C78 => {
    //   block [0x82EF1C78..0x82EF1C88)
	// 82EF1C78: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF1C7C: 388000C6  li r4, 0xc6
	ctx.r[4].s64 = 198;
	// 82EF1C80: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EF1C84: 4BFFBBBC  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1C88 size=20
    let mut pc: u32 = 0x82EF1C88;
    'dispatch: loop {
        match pc {
            0x82EF1C88 => {
    //   block [0x82EF1C88..0x82EF1C9C)
	// 82EF1C88: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82EF1C8C: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82EF1C90: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF1C94: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF1C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF1CA0 size=168
    let mut pc: u32 = 0x82EF1CA0;
    'dispatch: loop {
        match pc {
            0x82EF1CA0 => {
    //   block [0x82EF1CA0..0x82EF1D48)
	// 82EF1CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF1CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF1CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF1CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF1CB0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EF1CB4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF1CB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF1CBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF1CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF1CC4: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82EF1CC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF1CCC: C3EB9F7C  lfs f31, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF1CD0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF1CD4: 4BFAEEED  bl 0x82ea0bc0
	ctx.lr = 0x82EF1CD8;
	sub_82EA0BC0(ctx, base);
	// 82EF1CD8: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1CDC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF1CE0: 419A0040  beq cr6, 0x82ef1d20
	if ctx.cr[6].eq {
	pc = 0x82EF1D20; continue 'dispatch;
	}
	// 82EF1CE4: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82EF1CE8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF1CEC: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EF1CF0: 4BFAEED1  bl 0x82ea0bc0
	ctx.lr = 0x82EF1CF4;
	sub_82EA0BC0(ctx, base);
	// 82EF1CF4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1CFC: 419A0024  beq cr6, 0x82ef1d20
	if ctx.cr[6].eq {
	pc = 0x82EF1D20; continue 'dispatch;
	}
	// 82EF1D00: 897E00A2  lbz r11, 0xa2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(162 as u32) ) } as u64;
	// 82EF1D04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1D08: 419A0010  beq cr6, 0x82ef1d18
	if ctx.cr[6].eq {
	pc = 0x82EF1D18; continue 'dispatch;
	}
	// 82EF1D0C: 897E00C2  lbz r11, 0xc2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(194 as u32) ) } as u64;
	// 82EF1D10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1D14: 419A000C  beq cr6, 0x82ef1d20
	if ctx.cr[6].eq {
	pc = 0x82EF1D20; continue 'dispatch;
	}
	// 82EF1D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF1D1C: 48000008  b 0x82ef1d24
	pc = 0x82EF1D24; continue 'dispatch;
	// 82EF1D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF1D24: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF1D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF1D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF1D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF1D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF1D38: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EF1D3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF1D40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF1D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1D48 size=8
    let mut pc: u32 = 0x82EF1D48;
    'dispatch: loop {
        match pc {
            0x82EF1D48 => {
    //   block [0x82EF1D48..0x82EF1D50)
	// 82EF1D48: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82EF1D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF1D50 size=132
    let mut pc: u32 = 0x82EF1D50;
    'dispatch: loop {
        match pc {
            0x82EF1D50 => {
    //   block [0x82EF1D50..0x82EF1DD4)
	// 82EF1D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF1D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF1D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF1D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF1D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF1D64: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EF1D68: 394B7B5C  addi r10, r11, 0x7b5c
	ctx.r[10].s64 = ctx.r[11].s64 + 31580;
	// 82EF1D6C: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF1D70: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF1D74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF1D78: 419A003C  beq cr6, 0x82ef1db4
	if ctx.cr[6].eq {
	pc = 0x82EF1DB4; continue 'dispatch;
	}
	// 82EF1D7C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF1D80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF1D84: 419A0030  beq cr6, 0x82ef1db4
	if ctx.cr[6].eq {
	pc = 0x82EF1DB4; continue 'dispatch;
	}
	// 82EF1D88: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF1D8C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF1D90: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF1D94: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF1D98: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF1D9C: 409A0018  bne cr6, 0x82ef1db4
	if !ctx.cr[6].eq {
	pc = 0x82EF1DB4; continue 'dispatch;
	}
	// 82EF1DA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1DA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF1DA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF1DAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF1DB0: 4E800421  bctrl
	ctx.lr = 0x82EF1DB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF1DB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF1DB8: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF1DBC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF1DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF1DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF1DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF1DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF1DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF1DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF1DD8 size=720
    let mut pc: u32 = 0x82EF1DD8;
    'dispatch: loop {
        match pc {
            0x82EF1DD8 => {
    //   block [0x82EF1DD8..0x82EF20A8)
	// 82EF1DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF1DDC: 482B6389  bl 0x831a8164
	ctx.lr = 0x82EF1DE0;
	sub_831A8130(ctx, base);
	// 82EF1DE0: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF20A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF20A8 size=28
    let mut pc: u32 = 0x82EF20A8;
    'dispatch: loop {
        match pc {
            0x82EF20A8 => {
    //   block [0x82EF20A8..0x82EF20C4)
	// 82EF20A8: 7CAB0774  extsb r11, r5
	ctx.r[11].s64 = ctx.r[5].s8 as i64;
	// 82EF20AC: 98A300A2  stb r5, 0xa2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[5].u8 ) };
	// 82EF20B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF20B4: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF20B8: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82EF20BC: 992300B6  stb r9, 0xb6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(182 as u32), ctx.r[9].u8 ) };
	// 82EF20C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF20C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF20C4 size=12
    let mut pc: u32 = 0x82EF20C4;
    'dispatch: loop {
        match pc {
            0x82EF20C4 => {
    //   block [0x82EF20C4..0x82EF20D0)
	// 82EF20C4: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF20C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF20CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF20D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF20D0 size=12
    let mut pc: u32 = 0x82EF20D0;
    'dispatch: loop {
        match pc {
            0x82EF20D0 => {
    //   block [0x82EF20D0..0x82EF20DC)
	// 82EF20D0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF20D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF20D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF20DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF20DC size=28
    let mut pc: u32 = 0x82EF20DC;
    'dispatch: loop {
        match pc {
            0x82EF20DC => {
    //   block [0x82EF20DC..0x82EF20F8)
	// 82EF20DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF20E0: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF20E4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EF20E8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EF20EC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EF20F0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF20F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF20F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF20F8 size=336
    let mut pc: u32 = 0x82EF20F8;
    'dispatch: loop {
        match pc {
            0x82EF20F8 => {
    //   block [0x82EF20F8..0x82EF2248)
	// 82EF20F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF20FC: 482B606D  bl 0x831a8168
	ctx.lr = 0x82EF2100;
	sub_831A8130(ctx, base);
	// 82EF2100: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EF2104: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF210C: 39097B5C  addi r8, r9, 0x7b5c
	ctx.r[8].s64 = ctx.r[9].s64 + 31580;
	// 82EF2110: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF2114: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82EF2118: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF211C: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 82EF2120: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF2124: B0E30010  sth r7, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u16 ) };
	// 82EF2128: 3881FFC0  addi r4, r1, -0x40
	ctx.r[4].s64 = ctx.r[1].s64 + -64;
	// 82EF212C: B0C300A0  sth r6, 0xa0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[6].u16 ) };
	// 82EF2130: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82EF2134: 994300A2  stb r10, 0xa2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[10].u8 ) };
	// 82EF2138: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82EF213C: B16300A4  sth r11, 0xa4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u16 ) };
	// 82EF2140: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82EF2144: B16300A6  sth r11, 0xa6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(166 as u32), ctx.r[11].u16 ) };
	// 82EF2148: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2248 size=364
    let mut pc: u32 = 0x82EF2248;
    'dispatch: loop {
        match pc {
            0x82EF2248 => {
    //   block [0x82EF2248..0x82EF23B4)
	// 82EF2248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF224C: 482B5F0D  bl 0x831a8158
	ctx.lr = 0x82EF2250;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF23B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF23B8 size=16
    let mut pc: u32 = 0x82EF23B8;
    'dispatch: loop {
        match pc {
            0x82EF23B8 => {
    //   block [0x82EF23B8..0x82EF23C8)
	// 82EF23B8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF23BC: 38800084  li r4, 0x84
	ctx.r[4].s64 = 132;
	// 82EF23C0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EF23C4: 4BFFB47C  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF23C8 size=20
    let mut pc: u32 = 0x82EF23C8;
    'dispatch: loop {
        match pc {
            0x82EF23C8 => {
    //   block [0x82EF23C8..0x82EF23DC)
	// 82EF23C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82EF23CC: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 82EF23D0: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF23D4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF23D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF23E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF23E0 size=140
    let mut pc: u32 = 0x82EF23E0;
    'dispatch: loop {
        match pc {
            0x82EF23E0 => {
    //   block [0x82EF23E0..0x82EF246C)
	// 82EF23E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF23E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF23E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF23EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF23F0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EF23F4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF23F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF23FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF2400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF2404: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82EF2408: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF240C: C3EB9F7C  lfs f31, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF2410: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF2414: 4BFAE7AD  bl 0x82ea0bc0
	ctx.lr = 0x82EF2418;
	sub_82EA0BC0(ctx, base);
	// 82EF2418: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF241C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF2420: 419A0024  beq cr6, 0x82ef2444
	if ctx.cr[6].eq {
	pc = 0x82EF2444; continue 'dispatch;
	}
	// 82EF2424: 389E0050  addi r4, r30, 0x50
	ctx.r[4].s64 = ctx.r[30].s64 + 80;
	// 82EF2428: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF242C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EF2430: 4BFAE791  bl 0x82ea0bc0
	ctx.lr = 0x82EF2434;
	sub_82EA0BC0(ctx, base);
	// 82EF2434: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF243C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF2440: 409A0008  bne cr6, 0x82ef2448
	if !ctx.cr[6].eq {
	pc = 0x82EF2448; continue 'dispatch;
	}
	// 82EF2444: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2448: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF244C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF2450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF2454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF245C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EF2460: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF2464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF2468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2470 size=8
    let mut pc: u32 = 0x82EF2470;
    'dispatch: loop {
        match pc {
            0x82EF2470 => {
    //   block [0x82EF2470..0x82EF2478)
	// 82EF2470: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 82EF2474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2478 size=204
    let mut pc: u32 = 0x82EF2478;
    'dispatch: loop {
        match pc {
            0x82EF2478 => {
    //   block [0x82EF2478..0x82EF2544)
	// 82EF2478: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EF247C: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 82EF2480: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82EF2484: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82EF2488: 38A87C64  addi r5, r8, 0x7c64
	ctx.r[5].s64 = ctx.r[8].s64 + 31844;
	// 82EF248C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF2490: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF2548 size=544
    let mut pc: u32 = 0x82EF2548;
    'dispatch: loop {
        match pc {
            0x82EF2548 => {
    //   block [0x82EF2548..0x82EF2768)
	// 82EF2548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF254C: 482B5C11  bl 0x831a815c
	ctx.lr = 0x82EF2550;
	sub_831A8130(ctx, base);
	// 82EF2550: C0060000  lfs f0, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF2554: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2558: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF255C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82EF2560: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82EF2564: C1860008  lfs f12, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF2568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF256C: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82EF2570: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82EF2574: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82EF2578: 40980010  bge cr6, 0x82ef2588
	if !ctx.cr[6].lt {
	pc = 0x82EF2588; continue 'dispatch;
	}
	// 82EF257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF2580: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82EF2584: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF2588: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82EF258C: 40980008  bge cr6, 0x82ef2594
	if !ctx.cr[6].lt {
	pc = 0x82EF2594; continue 'dispatch;
	}
	// 82EF2590: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82EF2594: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2768 size=32
    let mut pc: u32 = 0x82EF2768;
    'dispatch: loop {
        match pc {
            0x82EF2768 => {
    //   block [0x82EF2768..0x82EF2788)
	// 82EF2768: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82EF276C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2770: 419A0018  beq cr6, 0x82ef2788
	if ctx.cr[6].eq {
		sub_82EF2788(ctx, base);
		return;
	}
	// 82EF2774: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82EF2778: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 82EF277C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF2780: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF2784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2788 size=16
    let mut pc: u32 = 0x82EF2788;
    'dispatch: loop {
        match pc {
            0x82EF2788 => {
    //   block [0x82EF2788..0x82EF2798)
	// 82EF2788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF278C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF2790: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF2794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2798 size=16
    let mut pc: u32 = 0x82EF2798;
    'dispatch: loop {
        match pc {
            0x82EF2798 => {
    //   block [0x82EF2798..0x82EF27A8)
	// 82EF2798: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF279C: 38800096  li r4, 0x96
	ctx.r[4].s64 = 150;
	// 82EF27A0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82EF27A4: 4BFFB09C  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF27A8 size=140
    let mut pc: u32 = 0x82EF27A8;
    'dispatch: loop {
        match pc {
            0x82EF27A8 => {
    //   block [0x82EF27A8..0x82EF2834)
	// 82EF27A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF27AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF27B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF27B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF27B8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EF27BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF27C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF27C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF27C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF27CC: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82EF27D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF27D4: C3EB9F7C  lfs f31, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF27D8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF27DC: 4BFAE3E5  bl 0x82ea0bc0
	ctx.lr = 0x82EF27E0;
	sub_82EA0BC0(ctx, base);
	// 82EF27E0: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF27E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF27E8: 419A0024  beq cr6, 0x82ef280c
	if ctx.cr[6].eq {
	pc = 0x82EF280C; continue 'dispatch;
	}
	// 82EF27EC: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82EF27F0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF27F4: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EF27F8: 4BFAE3C9  bl 0x82ea0bc0
	ctx.lr = 0x82EF27FC;
	sub_82EA0BC0(ctx, base);
	// 82EF27FC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF2804: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF2808: 409A0008  bne cr6, 0x82ef2810
	if !ctx.cr[6].eq {
	pc = 0x82EF2810; continue 'dispatch;
	}
	// 82EF280C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2810: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF2814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF2818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF281C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF2824: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EF2828: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF282C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF2830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2838 size=8
    let mut pc: u32 = 0x82EF2838;
    'dispatch: loop {
        match pc {
            0x82EF2838 => {
    //   block [0x82EF2838..0x82EF2840)
	// 82EF2838: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF283C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2840 size=628
    let mut pc: u32 = 0x82EF2840;
    'dispatch: loop {
        match pc {
            0x82EF2840 => {
    //   block [0x82EF2840..0x82EF2AB4)
	// 82EF2840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2844: 482B5915  bl 0x831a8158
	ctx.lr = 0x82EF2848;
	sub_831A8130(ctx, base);
	// 82EF2848: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2AB8 size=568
    let mut pc: u32 = 0x82EF2AB8;
    'dispatch: loop {
        match pc {
            0x82EF2AB8 => {
    //   block [0x82EF2AB8..0x82EF2CF0)
	// 82EF2AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2ABC: 482B56AD  bl 0x831a8168
	ctx.lr = 0x82EF2AC0;
	sub_831A8130(ctx, base);
	// 82EF2AC0: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2CF0 size=196
    let mut pc: u32 = 0x82EF2CF0;
    'dispatch: loop {
        match pc {
            0x82EF2CF0 => {
    //   block [0x82EF2CF0..0x82EF2DB4)
	// 82EF2CF0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EF2CF4: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82EF2CF8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EF2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF2D00: 38897D44  addi r4, r9, 0x7d44
	ctx.r[4].s64 = ctx.r[9].s64 + 32068;
	// 82EF2D04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF2D08: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF2DB8 size=96
    let mut pc: u32 = 0x82EF2DB8;
    'dispatch: loop {
        match pc {
            0x82EF2DB8 => {
    //   block [0x82EF2DB8..0x82EF2E18)
	// 82EF2DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF2DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF2DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF2DCC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EF2DD0: 419A0030  beq cr6, 0x82ef2e00
	if ctx.cr[6].eq {
	pc = 0x82EF2E00; continue 'dispatch;
	}
	// 82EF2DD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2DD8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82EF2DDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF2DE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF2DE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF2DE8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82EF2DEC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF2DF0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF2DF4: 4800E5F5  bl 0x82f013e8
	ctx.lr = 0x82EF2DF8;
	sub_82F013E8(ctx, base);
	// 82EF2DF8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF2DFC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF2E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF2E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF2E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF2E10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF2E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2E18 size=8
    let mut pc: u32 = 0x82EF2E18;
    'dispatch: loop {
        match pc {
            0x82EF2E18 => {
    //   block [0x82EF2E18..0x82EF2E20)
	// 82EF2E18: 38600065  li r3, 0x65
	ctx.r[3].s64 = 101;
	// 82EF2E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2E20 size=84
    let mut pc: u32 = 0x82EF2E20;
    'dispatch: loop {
        match pc {
            0x82EF2E20 => {
    //   block [0x82EF2E20..0x82EF2E74)
	// 82EF2E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2E24: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82EF2E28: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF2E2C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF2E30: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EF2E34: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF2E38: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EF2E3C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF2E40: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF2E44: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF2E48: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF2E4C: 1D4B0134  mulli r10, r11, 0x134
	ctx.r[10].s64 = ctx.r[11].s64 * 308;
	// 82EF2E50: 38EA0033  addi r7, r10, 0x33
	ctx.r[7].s64 = ctx.r[10].s64 + 51;
	// 82EF2E54: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF2E58: 54EA0036  rlwinm r10, r7, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF2E5C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF2E60: 38CA0010  addi r6, r10, 0x10
	ctx.r[6].s64 = ctx.r[10].s64 + 16;
	// 82EF2E64: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF2E68: 90C40004  stw r6, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82EF2E6C: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF2E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2E78 size=40
    let mut pc: u32 = 0x82EF2E78;
    'dispatch: loop {
        match pc {
            0x82EF2E78 => {
    //   block [0x82EF2E78..0x82EF2EA0)
	// 82EF2E78: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82EF2E7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2E80: 419A0020  beq cr6, 0x82ef2ea0
	if ctx.cr[6].eq {
		sub_82EF2EA0(ctx, base);
		return;
	}
	// 82EF2E84: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF2E88: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF2E8C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF2E90: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF2E94: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF2E98: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF2E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF2EA0 size=16
    let mut pc: u32 = 0x82EF2EA0;
    'dispatch: loop {
        match pc {
            0x82EF2EA0 => {
    //   block [0x82EF2EA0..0x82EF2EB0)
	// 82EF2EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2EA4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF2EA8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF2EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF2EB0 size=152
    let mut pc: u32 = 0x82EF2EB0;
    'dispatch: loop {
        match pc {
            0x82EF2EB0 => {
    //   block [0x82EF2EB0..0x82EF2F48)
	// 82EF2EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF2EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF2EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF2EC4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF2EC8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82EF2ECC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82EF2ED0: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82EF2ED4: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82EF2ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2EDC: 3889B780  addi r4, r9, -0x4880
	ctx.r[4].s64 = ctx.r[9].s64 + -18560;
	// 82EF2EE0: C008C664  lfs f0, -0x399c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF2EE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF2EE8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF2EEC: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EF2EF0: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF2EF4: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF2EF8: C1A708A8  lfs f13, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF2EFC: B15F000C  sth r10, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 82EF2F00: C186B6EC  lfs f12, -0x4914(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-18708 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF2F04: C165964C  lfs f11, -0x69b4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF2F08: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EF2F0C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF2F10: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EF2F14: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82EF2F18: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82EF2F1C: D1BF0028  stfs f13, 0x28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82EF2F20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF2F24: D19F002C  stfs f12, 0x2c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82EF2F28: D17F0030  stfs f11, 0x30(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82EF2F2C: 4BFFA4AD  bl 0x82eed3d8
	ctx.lr = 0x82EF2F30;
	sub_82EED3D8(ctx, base);
	// 82EF2F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF2F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF2F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF2F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF2F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF2F48 size=100
    let mut pc: u32 = 0x82EF2F48;
    'dispatch: loop {
        match pc {
            0x82EF2F48 => {
    //   block [0x82EF2F48..0x82EF2FAC)
	// 82EF2F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF2F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF2F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF2F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF2F60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF2F64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF2F68: 392BB780  addi r9, r11, -0x4880
	ctx.r[9].s64 = ctx.r[11].s64 + -18560;
	// 82EF2F6C: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82EF2F70: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF2F74: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF2F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF2F7C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF2F80: 4BFFA459  bl 0x82eed3d8
	ctx.lr = 0x82EF2F84;
	sub_82EED3D8(ctx, base);
	// 82EF2F84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF2F88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF2F8C: 4BFFA44D  bl 0x82eed3d8
	ctx.lr = 0x82EF2F90;
	sub_82EED3D8(ctx, base);
	// 82EF2F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF2F94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF2F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF2FA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF2FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF2FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF2FB0 size=108
    let mut pc: u32 = 0x82EF2FB0;
    'dispatch: loop {
        match pc {
            0x82EF2FB0 => {
    //   block [0x82EF2FB0..0x82EF301C)
	// 82EF2FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2FB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF2FBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF2FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF2FC4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF2FC8: 394BB780  addi r10, r11, -0x4880
	ctx.r[10].s64 = ctx.r[11].s64 + -18560;
	// 82EF2FCC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF2FD0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF2FD4: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF2FD8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF2FDC: 409A0020  bne cr6, 0x82ef2ffc
	if !ctx.cr[6].eq {
	pc = 0x82EF2FFC; continue 'dispatch;
	}
	// 82EF2FE0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2FE4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EF2FE8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EF2FEC: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF2FF0: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF2FF4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EF2FF8: 4BFAD7B9  bl 0x82ea07b0
	ctx.lr = 0x82EF2FFC;
	sub_82EA07B0(ctx, base);
	// 82EF2FFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF3000: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF3004: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF300C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3020 size=108
    let mut pc: u32 = 0x82EF3020;
    'dispatch: loop {
        match pc {
            0x82EF3020 => {
    //   block [0x82EF3020..0x82EF308C)
	// 82EF3020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3024: 482B5149  bl 0x831a816c
	ctx.lr = 0x82EF3028;
	sub_831A8130(ctx, base);
	// 82EF3028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF302C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF3030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF3034: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF3038: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF303C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF3040: 3BE30018  addi r31, r3, 0x18
	ctx.r[31].s64 = ctx.r[3].s64 + 24;
	// 82EF3044: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF3048: 409A0010  bne cr6, 0x82ef3058
	if !ctx.cr[6].eq {
	pc = 0x82EF3058; continue 'dispatch;
	}
	// 82EF304C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82EF3050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF3054: 4BFB382D  bl 0x82ea6880
	ctx.lr = 0x82EF3058;
	sub_82EA6880(ctx, base);
	// 82EF3058: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF305C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EF3060: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3064: 556A2834  slwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF3068: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82EF306C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF3070: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF3090 size=636
    let mut pc: u32 = 0x82EF3090;
    'dispatch: loop {
        match pc {
            0x82EF3090 => {
    //   block [0x82EF3090..0x82EF330C)
	// 82EF3090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3094: 482B50A9  bl 0x831a813c
	ctx.lr = 0x82EF3098;
	sub_831A8130(ctx, base);
	// 82EF3098: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF309C: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82EF30A0: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 82EF30A4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EF30A8: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82EF30AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82EF30B0: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82EF30B4: 80B3004C  lwz r5, 0x4c(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF30B8: 480147D9  bl 0x82f07890
	ctx.lr = 0x82EF30BC;
	sub_82F07890(ctx, base);
	// 82EF30BC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82EF30C0: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82EF30C4: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF30C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF30CC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EF30D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF30D4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EF30D8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF30DC: 4200FFF0  bdnz 0x82ef30cc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF30CC; continue 'dispatch;
	}
	// 82EF30E0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82EF30E4: 82930048  lwz r20, 0x48(r19)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF30E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF30EC: 81330030  lwz r9, 0x30(r19)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF30F0: 61660020  ori r6, r11, 0x20
	ctx.r[6].u64 = ctx.r[11].u64 | 32;
	// 82EF30F4: 80E10068  lwz r7, 0x68(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF30F8: 390100EC  addi r8, r1, 0xec
	ctx.r[8].s64 = ctx.r[1].s64 + 236;
	// 82EF30FC: 93C100E4  stw r30, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 82EF3100: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82EF3104: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82EF3108: 93C100AC  stw r30, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82EF310C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF3110: 910100E0  stw r8, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[8].u32 ) };
	// 82EF3114: 90C100E8  stw r6, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[6].u32 ) };
	// 82EF3118: 8074002C  lwz r3, 0x2c(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF311C: 81140014  lwz r8, 0x14(r20)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF3120: C00508A8  lfs f0, 0x8a8(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3124: 81540030  lwz r10, 0x30(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF3128: 3A4AFFFF  addi r18, r10, -1
	ctx.r[18].s64 = ctx.r[10].s64 + -1;
	// 82EF312C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3130: 80C80090  lwz r6, 0x90(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EF3134: 7EA64850  subf r21, r6, r9
	ctx.r[21].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 82EF3138: 812B0090  lwz r9, 0x90(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EF313C: 38AB00E0  addi r5, r11, 0xe0
	ctx.r[5].s64 = ctx.r[11].s64 + 224;
	// 82EF3140: 7D69AA14  add r11, r9, r21
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[21].u64;
	// 82EF3144: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82EF3148: 7C755850  subf r3, r21, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[21].s64;
	// 82EF314C: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82EF3150: 908100E4  stw r4, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[4].u32 ) };
	// 82EF3154: 906100EC  stw r3, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[3].u32 ) };
	// 82EF3158: 90A1009C  stw r5, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[5].u32 ) };
	// 82EF315C: 90E1007C  stw r7, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[7].u32 ) };
	// 82EF3160: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3164: 48020595  bl 0x82f136f8
	ctx.lr = 0x82EF3168;
	sub_82F136F8(ctx, base);
	// 82EF3168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF316C: 2F120000  cmpwi cr6, r18, 0
	ctx.cr[6].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82EF3170: 40990140  ble cr6, 0x82ef32b0
	if !ctx.cr[6].gt {
	pc = 0x82EF32B0; continue 'dispatch;
	}
	// 82EF3174: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EF3178: 7E5D9378  mr r29, r18
	ctx.r[29].u64 = ctx.r[18].u64;
	// 82EF317C: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82EF3180: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82EF3184: 3B000020  li r24, 0x20
	ctx.r[24].s64 = 32;
	// 82EF3188: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF318C: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF3190: 812100E8  lwz r9, 0xe8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 82EF3194: 810100E4  lwz r8, 0xe4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82EF3198: 552700BE  clrlwi r7, r9, 2
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF319C: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82EF31A0: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82EF31A4: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EF31A8: 8174002C  lwz r11, 0x2c(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF31AC: 7CDE5A14  add r6, r30, r11
	ctx.r[6].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EF31B0: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF31B4: 3B6B00D0  addi r27, r11, 0xd0
	ctx.r[27].s64 = ctx.r[11].s64 + 208;
	// 82EF31B8: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EF31BC: 7D6BAA14  add r11, r11, r21
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 82EF31C0: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82EF31C4: 7F555850  subf r26, r21, r11
	ctx.r[26].s64 = ctx.r[11].s64 - ctx.r[21].s64;
	// 82EF31C8: 409A0010  bne cr6, 0x82ef31d8
	if !ctx.cr[6].eq {
	pc = 0x82EF31D8; continue 'dispatch;
	}
	// 82EF31CC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EF31D0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82EF31D4: 4BFB36AD  bl 0x82ea6880
	ctx.lr = 0x82EF31D8;
	sub_82EA6880(ctx, base);
	// 82EF31D8: 814100E4  lwz r10, 0xe4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82EF31DC: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF31E0: 80C100E0  lwz r6, 0xe0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 82EF31E4: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82EF31E8: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF31EC: 397B0010  addi r11, r27, 0x10
	ctx.r[11].s64 = ctx.r[27].s64 + 16;
	// 82EF31F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EF31F4: 7F45312E  stwx r26, r5, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[6].u32), ctx.r[26].u32) };
	// 82EF31F8: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EF31FC: 80C100E4  lwz r6, 0xe4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82EF3200: 38A60001  addi r5, r6, 1
	ctx.r[5].s64 = ctx.r[6].s64 + 1;
	// 82EF3204: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82EF3208: 90A100E4  stw r5, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[5].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3310 size=164
    let mut pc: u32 = 0x82EF3310;
    'dispatch: loop {
        match pc {
            0x82EF3310 => {
    //   block [0x82EF3310..0x82EF33B4)
	// 82EF3310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF331C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF3320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF3328: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF332C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF3330: 394BB780  addi r10, r11, -0x4880
	ctx.r[10].s64 = ctx.r[11].s64 + -18560;
	// 82EF3334: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3338: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF333C: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF3340: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF3344: 409A0020  bne cr6, 0x82ef3364
	if !ctx.cr[6].eq {
	pc = 0x82EF3364; continue 'dispatch;
	}
	// 82EF3348: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF334C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EF3350: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EF3354: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF3358: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF335C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EF3360: 4BFAD451  bl 0x82ea07b0
	ctx.lr = 0x82EF3364;
	sub_82EA07B0(ctx, base);
	// 82EF3364: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF3368: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EF336C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EF3370: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF3374: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF3378: 419A0020  beq cr6, 0x82ef3398
	if ctx.cr[6].eq {
	pc = 0x82EF3398; continue 'dispatch;
	}
	// 82EF337C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3380: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF3384: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EF3388: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF338C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF3390: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF3394: 4BFAD41D  bl 0x82ea07b0
	ctx.lr = 0x82EF3398;
	sub_82EA07B0(ctx, base);
	// 82EF3398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF339C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF33A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF33A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF33A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF33AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF33B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF33B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF33B8 size=164
    let mut pc: u32 = 0x82EF33B8;
    'dispatch: loop {
        match pc {
            0x82EF33B8 => {
    //   block [0x82EF33B8..0x82EF345C)
	// 82EF33B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF33BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF33C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF33C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF33C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF33CC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF33D0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF33D4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82EF33D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF33DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF33E0: 4BFAD351  bl 0x82ea0730
	ctx.lr = 0x82EF33E4;
	sub_82EA0730(ctx, base);
	// 82EF33E4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF33E8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF33EC: 38E99F74  addi r7, r9, -0x608c
	ctx.r[7].s64 = ctx.r[9].s64 + -24716;
	// 82EF33F0: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82EF33F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF33F8: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF33FC: 38889F88  addi r4, r8, -0x6078
	ctx.r[4].s64 = ctx.r[8].s64 + -24696;
	// 82EF3400: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82EF3404: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82EF3408: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF340C: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3410: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF3414: 392B9F9C  addi r9, r11, -0x6064
	ctx.r[9].s64 = ctx.r[11].s64 + -24676;
	// 82EF3418: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82EF341C: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3420: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF3424: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF3428: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF342C: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF3430: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF3434: D1830014  stfs f12, 0x14(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF3438: C17F0018  lfs f11, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF343C: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF3440: 891F001C  lbz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF3444: 9903001C  stb r8, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[8].u8 ) };
	// 82EF3448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF344C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF3460 size=80
    let mut pc: u32 = 0x82EF3460;
    'dispatch: loop {
        match pc {
            0x82EF3460 => {
    //   block [0x82EF3460..0x82EF34B0)
	// 82EF3460: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF3464: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EF3468: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EF346C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF3470: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EF3474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF3478: C1ABB7B4  lfs f13, -0x484c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18508 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF347C: 38A8A19C  addi r5, r8, -0x5e64
	ctx.r[5].s64 = ctx.r[8].s64 + -24164;
	// 82EF3480: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82EF3484: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82EF3488: 98C30008  stb r6, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u8 ) };
	// 82EF348C: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3490: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EF3494: C18A9F64  lfs f12, -0x609c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24732 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF3498: 98830008  stb r4, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u8 ) };
	// 82EF349C: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF34A0: D1830010  stfs f12, 0x10(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF34A4: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF34A8: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF34AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF34B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF34B0 size=60
    let mut pc: u32 = 0x82EF34B0;
    'dispatch: loop {
        match pc {
            0x82EF34B0 => {
    //   block [0x82EF34B0..0x82EF34EC)
	// 82EF34B0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF34B4: FC001850  fneg f0, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[3].u64 ^ 0x8000_0000_0000_0000u64;
	// 82EF34B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF34BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF34C0: 390BA19C  addi r8, r11, -0x5e64
	ctx.r[8].s64 = ctx.r[11].s64 + -24164;
	// 82EF34C4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF34C8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82EF34CC: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82EF34D0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF34D4: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF34D8: D0630010  stfs f3, 0x10(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF34DC: 98E30008  stb r7, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 82EF34E0: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF34E4: D0430018  stfs f2, 0x18(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF34E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF34F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF34F0 size=156
    let mut pc: u32 = 0x82EF34F0;
    'dispatch: loop {
        match pc {
            0x82EF34F0 => {
    //   block [0x82EF34F0..0x82EF358C)
	// 82EF34F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF34F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF34F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF34FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3500: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3504: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF3508: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF350C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82EF3510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF3514: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF3518: 4BFAD219  bl 0x82ea0730
	ctx.lr = 0x82EF351C;
	sub_82EA0730(ctx, base);
	// 82EF351C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF3520: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF3524: 38E99F74  addi r7, r9, -0x608c
	ctx.r[7].s64 = ctx.r[9].s64 + -24716;
	// 82EF3528: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82EF352C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF3530: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF3534: 38889F88  addi r4, r8, -0x6078
	ctx.r[4].s64 = ctx.r[8].s64 + -24696;
	// 82EF3538: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82EF353C: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82EF3540: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF3544: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3548: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF354C: 392BA19C  addi r9, r11, -0x5e64
	ctx.r[9].s64 = ctx.r[11].s64 + -24164;
	// 82EF3550: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82EF3554: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3558: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF355C: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF3560: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF3564: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF3568: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF356C: D1830014  stfs f12, 0x14(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF3570: C17F0018  lfs f11, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF3574: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF3578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF357C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF3590 size=100
    let mut pc: u32 = 0x82EF3590;
    'dispatch: loop {
        match pc {
            0x82EF3590 => {
    //   block [0x82EF3590..0x82EF35F4)
	// 82EF3590: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF3594: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EF3598: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82EF359C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82EF35A0: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82EF35A4: C1ABB7B4  lfs f13, -0x484c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18508 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF35A8: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82EF35AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF35B0: C18A9F64  lfs f12, -0x609c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24732 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF35B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF35B8: C169ACFC  lfs f11, -0x5304(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-21252 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF35BC: 3886A41C  addi r4, r6, -0x5be4
	ctx.r[4].s64 = ctx.r[6].s64 + -23524;
	// 82EF35C0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82EF35C4: 98A30008  stb r5, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u8 ) };
	// 82EF35C8: C00808A8  lfs f0, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF35CC: C1479524  lfs f10, -0x6adc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF35D0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF35D4: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF35D8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82EF35DC: D1830010  stfs f12, 0x10(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF35E0: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF35E4: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF35E8: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82EF35EC: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82EF35F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF35F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF35F8 size=172
    let mut pc: u32 = 0x82EF35F8;
    'dispatch: loop {
        match pc {
            0x82EF35F8 => {
    //   block [0x82EF35F8..0x82EF36A4)
	// 82EF35F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF35FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF3604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3608: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF360C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF3610: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF3614: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82EF3618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF361C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF3620: 4BFAD111  bl 0x82ea0730
	ctx.lr = 0x82EF3624;
	sub_82EA0730(ctx, base);
	// 82EF3624: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF3628: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF362C: 38E99F74  addi r7, r9, -0x608c
	ctx.r[7].s64 = ctx.r[9].s64 + -24716;
	// 82EF3630: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82EF3634: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF3638: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF363C: 38889F88  addi r4, r8, -0x6078
	ctx.r[4].s64 = ctx.r[8].s64 + -24696;
	// 82EF3640: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82EF3644: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82EF3648: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF364C: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3650: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF3654: 392BA41C  addi r9, r11, -0x5be4
	ctx.r[9].s64 = ctx.r[11].s64 + -23524;
	// 82EF3658: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82EF365C: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3660: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF3664: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF3668: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF366C: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF3670: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF3674: D1830014  stfs f12, 0x14(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF3678: C17F0018  lfs f11, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF367C: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF3680: C15F001C  lfs f10, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF3684: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82EF3688: C13F0020  lfs f9, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EF368C: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82EF3690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF369C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF36A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF36A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF36A8 size=156
    let mut pc: u32 = 0x82EF36A8;
    'dispatch: loop {
        match pc {
            0x82EF36A8 => {
    //   block [0x82EF36A8..0x82EF3744)
	// 82EF36A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF36AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF36B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF36B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF36B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF36BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF36C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF36C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF36C8: 419A001C  beq cr6, 0x82ef36e4
	if ctx.cr[6].eq {
	pc = 0x82EF36E4; continue 'dispatch;
	}
	// 82EF36CC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF36D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF36D4: 419A0010  beq cr6, 0x82ef36e4
	if ctx.cr[6].eq {
	pc = 0x82EF36E4; continue 'dispatch;
	}
	// 82EF36D8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF36DC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF36E0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF36E4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF36E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF36EC: 419A003C  beq cr6, 0x82ef3728
	if ctx.cr[6].eq {
	pc = 0x82EF3728; continue 'dispatch;
	}
	// 82EF36F0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF36F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF36F8: 419A0030  beq cr6, 0x82ef3728
	if ctx.cr[6].eq {
	pc = 0x82EF3728; continue 'dispatch;
	}
	// 82EF36FC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF3700: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF3704: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF3708: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF370C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF3710: 409A0018  bne cr6, 0x82ef3728
	if !ctx.cr[6].eq {
	pc = 0x82EF3728; continue 'dispatch;
	}
	// 82EF3714: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3718: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF371C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3720: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF3724: 4E800421  bctrl
	ctx.lr = 0x82EF3728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3728: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82EF372C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF3730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3738: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF373C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3748 size=20
    let mut pc: u32 = 0x82EF3748;
    'dispatch: loop {
        match pc {
            0x82EF3748 => {
    //   block [0x82EF3748..0x82EF375C)
	// 82EF3748: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82EF374C: 3940003C  li r10, 0x3c
	ctx.r[10].s64 = 60;
	// 82EF3750: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF3754: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3760 size=16
    let mut pc: u32 = 0x82EF3760;
    'dispatch: loop {
        match pc {
            0x82EF3760 => {
    //   block [0x82EF3760..0x82EF3770)
	// 82EF3760: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF3764: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EF3768: 38AB0060  addi r5, r11, 0x60
	ctx.r[5].s64 = ctx.r[11].s64 + 96;
	// 82EF376C: 4BFB36FC  b 0x82ea6e68
	sub_82EA6E68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3770 size=8
    let mut pc: u32 = 0x82EF3770;
    'dispatch: loop {
        match pc {
            0x82EF3770 => {
    //   block [0x82EF3770..0x82EF3778)
	// 82EF3770: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82EF3774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3778 size=292
    let mut pc: u32 = 0x82EF3778;
    'dispatch: loop {
        match pc {
            0x82EF3778 => {
    //   block [0x82EF3778..0x82EF389C)
	// 82EF3778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF377C: 482B49ED  bl 0x831a8168
	ctx.lr = 0x82EF3780;
	sub_831A8130(ctx, base);
	// 82EF3780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3784: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EF3788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF378C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF3790: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF3794: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF3798: 419A001C  beq cr6, 0x82ef37b4
	if ctx.cr[6].eq {
	pc = 0x82EF37B4; continue 'dispatch;
	}
	// 82EF379C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF37A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF37A4: 419A0010  beq cr6, 0x82ef37b4
	if ctx.cr[6].eq {
	pc = 0x82EF37B4; continue 'dispatch;
	}
	// 82EF37A8: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF37AC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF37B0: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF37B4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF37B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF37BC: 419A003C  beq cr6, 0x82ef37f8
	if ctx.cr[6].eq {
	pc = 0x82EF37F8; continue 'dispatch;
	}
	// 82EF37C0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF37C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF37C8: 419A0030  beq cr6, 0x82ef37f8
	if ctx.cr[6].eq {
	pc = 0x82EF37F8; continue 'dispatch;
	}
	// 82EF37CC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF37D0: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF37D4: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF37D8: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF37DC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF37E0: 409A0018  bne cr6, 0x82ef37f8
	if !ctx.cr[6].eq {
	pc = 0x82EF37F8; continue 'dispatch;
	}
	// 82EF37E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF37E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF37EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF37F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF37F4: 4E800421  bctrl
	ctx.lr = 0x82EF37F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF37F8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EF37FC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EF3800: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82EF3804: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82EF3808: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82EF380C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF38A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF38A0 size=220
    let mut pc: u32 = 0x82EF38A0;
    'dispatch: loop {
        match pc {
            0x82EF38A0 => {
    //   block [0x82EF38A0..0x82EF397C)
	// 82EF38A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF38A4: 482B48C9  bl 0x831a816c
	ctx.lr = 0x82EF38A8;
	sub_831A8130(ctx, base);
	// 82EF38A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF38AC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EF38B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF38B4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF38B8: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82EF38BC: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3980 size=132
    let mut pc: u32 = 0x82EF3980;
    'dispatch: loop {
        match pc {
            0x82EF3980 => {
    //   block [0x82EF3980..0x82EF3A04)
	// 82EF3980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF398C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF3994: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF3998: 394BA55C  addi r10, r11, -0x5aa4
	ctx.r[10].s64 = ctx.r[11].s64 + -23204;
	// 82EF399C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF39A0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF39A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF39A8: 419A003C  beq cr6, 0x82ef39e4
	if ctx.cr[6].eq {
	pc = 0x82EF39E4; continue 'dispatch;
	}
	// 82EF39AC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF39B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF39B4: 419A0030  beq cr6, 0x82ef39e4
	if ctx.cr[6].eq {
	pc = 0x82EF39E4; continue 'dispatch;
	}
	// 82EF39B8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF39BC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF39C0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF39C4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF39C8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF39CC: 409A0018  bne cr6, 0x82ef39e4
	if !ctx.cr[6].eq {
	pc = 0x82EF39E4; continue 'dispatch;
	}
	// 82EF39D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF39D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF39D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF39DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF39E0: 4E800421  bctrl
	ctx.lr = 0x82EF39E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF39E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF39E8: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF39EC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF39F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF39F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF39F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF39FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3A08 size=52
    let mut pc: u32 = 0x82EF3A08;
    'dispatch: loop {
        match pc {
            0x82EF3A08 => {
    //   block [0x82EF3A08..0x82EF3A3C)
	// 82EF3A08: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82EF3A0C: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82EF3A10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF3A14: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EF3A18: 39000180  li r8, 0x180
	ctx.r[8].s64 = 384;
	// 82EF3A1C: 91440014  stw r10, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF3A20: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82EF3A24: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF3A28: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82EF3A2C: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF3A30: 90E40008  stw r7, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82EF3A34: 90C4000C  stw r6, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EF3A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3A40 size=40
    let mut pc: u32 = 0x82EF3A40;
    'dispatch: loop {
        match pc {
            0x82EF3A40 => {
    //   block [0x82EF3A40..0x82EF3A68)
	// 82EF3A40: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF3A44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3A48: 419A0014  beq cr6, 0x82ef3a5c
	if ctx.cr[6].eq {
	pc = 0x82EF3A5C; continue 'dispatch;
	}
	// 82EF3A4C: 89640020  lbz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF3A50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3A54: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF3A58: 409A0008  bne cr6, 0x82ef3a60
	if !ctx.cr[6].eq {
	pc = 0x82EF3A60; continue 'dispatch;
	}
	// 82EF3A5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF3A60: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF3A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3A68 size=512
    let mut pc: u32 = 0x82EF3A68;
    'dispatch: loop {
        match pc {
            0x82EF3A68 => {
    //   block [0x82EF3A68..0x82EF3C68)
	// 82EF3A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3A6C: 482B46ED  bl 0x831a8158
	ctx.lr = 0x82EF3A70;
	sub_831A8130(ctx, base);
	// 82EF3A70: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82EF3A74: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3A78: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82EF3A7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF3A80: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EF3A84: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EF3A88: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82EF3A8C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EF3A90: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EF3A94: 419A001C  beq cr6, 0x82ef3ab0
	if ctx.cr[6].eq {
	pc = 0x82EF3AB0; continue 'dispatch;
	}
	// 82EF3A98: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3A9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3AA0: 419A0010  beq cr6, 0x82ef3ab0
	if ctx.cr[6].eq {
	pc = 0x82EF3AB0; continue 'dispatch;
	}
	// 82EF3AA4: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF3AA8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF3AAC: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF3AB0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF3AB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF3AB8: 419A003C  beq cr6, 0x82ef3af4
	if ctx.cr[6].eq {
	pc = 0x82EF3AF4; continue 'dispatch;
	}
	// 82EF3ABC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3AC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3AC4: 419A0030  beq cr6, 0x82ef3af4
	if ctx.cr[6].eq {
	pc = 0x82EF3AF4; continue 'dispatch;
	}
	// 82EF3AC8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF3ACC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF3AD0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF3AD4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF3AD8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF3ADC: 409A0018  bne cr6, 0x82ef3af4
	if !ctx.cr[6].eq {
	pc = 0x82EF3AF4; continue 'dispatch;
	}
	// 82EF3AE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3AE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF3AE8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3AEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF3AF0: 4E800421  bctrl
	ctx.lr = 0x82EF3AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3AF4: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3C68 size=1140
    let mut pc: u32 = 0x82EF3C68;
    'dispatch: loop {
        match pc {
            0x82EF3C68 => {
    //   block [0x82EF3C68..0x82EF40DC)
	// 82EF3C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3C6C: 482B44F9  bl 0x831a8164
	ctx.lr = 0x82EF3C70;
	sub_831A8130(ctx, base);
	// 82EF3C70: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF40E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF40E0 size=8
    let mut pc: u32 = 0x82EF40E0;
    'dispatch: loop {
        match pc {
            0x82EF40E0 => {
    //   block [0x82EF40E0..0x82EF40E8)
	// 82EF40E0: 9883000C  stb r4, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u8 ) };
	// 82EF40E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF40E8 size=12
    let mut pc: u32 = 0x82EF40E8;
    'dispatch: loop {
        match pc {
            0x82EF40E8 => {
    //   block [0x82EF40E8..0x82EF40F4)
	// 82EF40E8: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF40EC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF40F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF40F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF40F8 size=12
    let mut pc: u32 = 0x82EF40F8;
    'dispatch: loop {
        match pc {
            0x82EF40F8 => {
    //   block [0x82EF40F8..0x82EF4104)
	// 82EF40F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF40FC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF4100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF4108 size=8
    let mut pc: u32 = 0x82EF4108;
    'dispatch: loop {
        match pc {
            0x82EF4108 => {
    //   block [0x82EF4108..0x82EF4110)
	// 82EF4108: C0230008  lfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF4110 size=8
    let mut pc: u32 = 0x82EF4110;
    'dispatch: loop {
        match pc {
            0x82EF4110 => {
    //   block [0x82EF4110..0x82EF4118)
	// 82EF4110: D0230008  stfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EF4114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF4118 size=224
    let mut pc: u32 = 0x82EF4118;
    'dispatch: loop {
        match pc {
            0x82EF4118 => {
    //   block [0x82EF4118..0x82EF41F8)
	// 82EF4118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF4124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF4128: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EF412C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4130: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EF4134: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF4138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF413C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF4140: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF4144: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82EF4148: 4098000C  bge cr6, 0x82ef4154
	if !ctx.cr[6].lt {
	pc = 0x82EF4154; continue 'dispatch;
	}
	// 82EF414C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF4150: 4800000C  b 0x82ef415c
	pc = 0x82EF415C; continue 'dispatch;
	// 82EF4154: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF4158: 4BFB2A01  bl 0x82ea6b58
	ctx.lr = 0x82EF415C;
	sub_82EA6B58(ctx, base);
	// 82EF415C: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 82EF4160: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4164: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF4168: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82EF416C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EF4170: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82EF4174: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF4178: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82EF417C: ED7F6028  fsubs f11, f31, f12
	ctx.f[11].f64 = (((ctx.f[31].f64 - ctx.f[12].f64) as f32) as f64);
	// 82EF4180: D1610050  stfs f11, 0x50(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82EF4184: 40980048  bge cr6, 0x82ef41cc
	if !ctx.cr[6].lt {
	pc = 0x82EF41CC; continue 'dispatch;
	}
	// 82EF4188: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EF418C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF4190: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82EF4194: 54672036  slwi r7, r3, 4
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF4198: 38C8FFA0  addi r6, r8, -0x60
	ctx.r[6].s64 = ctx.r[8].s64 + -96;
	// 82EF419C: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF41F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF41F8 size=724
    let mut pc: u32 = 0x82EF41F8;
    'dispatch: loop {
        match pc {
            0x82EF41F8 => {
    //   block [0x82EF41F8..0x82EF44CC)
	// 82EF41F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF41FC: 482B3F59  bl 0x831a8154
	ctx.lr = 0x82EF4200;
	sub_831A8130(ctx, base);
	// 82EF4200: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82EF4204: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4208: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF420C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF4210: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF4214: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82EF4218: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF421C: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82EF4220: 40980008  bge cr6, 0x82ef4228
	if !ctx.cr[6].lt {
	pc = 0x82EF4228; continue 'dispatch;
	}
	// 82EF4224: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF4228: 4BFB2931  bl 0x82ea6b58
	ctx.lr = 0x82EF422C;
	sub_82EA6B58(ctx, base);
	// 82EF422C: 839E0024  lwz r28, 0x24(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4230: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82EF4234: 3B5E0024  addi r26, r30, 0x24
	ctx.r[26].s64 = ctx.r[30].s64 + 36;
	// 82EF4238: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EF423C: 4198000C  blt cr6, 0x82ef4248
	if ctx.cr[6].lt {
	pc = 0x82EF4248; continue 'dispatch;
	}
	// 82EF4240: 395CFFFF  addi r10, r28, -1
	ctx.r[10].s64 = ctx.r[28].s64 + -1;
	// 82EF4244: 386AFFFF  addi r3, r10, -1
	ctx.r[3].s64 = ctx.r[10].s64 + -1;
	// 82EF4248: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF424C: 54682036  slwi r8, r3, 4
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF44D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF44D0 size=660
    let mut pc: u32 = 0x82EF44D0;
    'dispatch: loop {
        match pc {
            0x82EF44D0 => {
    //   block [0x82EF44D0..0x82EF4764)
	// 82EF44D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF44D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF44D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF44DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF44E0: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 82EF44E4: 482B4595  bl 0x831a8a78
	ctx.lr = 0x82EF44E8;
	sub_831A8A40(ctx, base);
	// 82EF44E8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF44EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF44F0: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82EF44F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF44F8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF44FC: C3DF0008  lfs f30, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EF4500: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF4504: EF9FF024  fdivs f28, f31, f30
	ctx.f[28].f64 = ((ctx.f[31].f64 / ctx.f[30].f64) as f32) as f64;
	// 82EF4508: 4BFB2651  bl 0x82ea6b58
	ctx.lr = 0x82EF450C;
	sub_82EA6B58(ctx, base);
	// 82EF450C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4510: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF4514: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF4518: 41980010  blt cr6, 0x82ef4528
	if ctx.cr[6].lt {
	pc = 0x82EF4528; continue 'dispatch;
	}
	// 82EF451C: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82EF4520: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82EF4524: 48000014  b 0x82ef4538
	pc = 0x82EF4538; continue 'dispatch;
	// 82EF4528: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EF452C: 4098000C  bge cr6, 0x82ef4538
	if !ctx.cr[6].lt {
	pc = 0x82EF4538; continue 'dispatch;
	}
	// 82EF4530: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF4534: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF4538: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF453C: 546A2036  slwi r10, r3, 4
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4540: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF4544: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82EF4548: 7C6607B4  extsw r6, r3
	ctx.r[6].s64 = ctx.r[3].s32 as i64;
	// 82EF454C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4768 size=32
    let mut pc: u32 = 0x82EF4768;
    'dispatch: loop {
        match pc {
            0x82EF4768 => {
    //   block [0x82EF4768..0x82EF4788)
	// 82EF4768: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF476C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF4770: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 82EF4774: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82EF4778: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF477C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82EF4780: FC206818  frsp f1, f13
	ctx.f[1].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82EF4784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF4788 size=172
    let mut pc: u32 = 0x82EF4788;
    'dispatch: loop {
        match pc {
            0x82EF4788 => {
    //   block [0x82EF4788..0x82EF4834)
	// 82EF4788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF478C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF4794: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82EF4798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF479C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF47A0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EF47A4: 4BFB23B5  bl 0x82ea6b58
	ctx.lr = 0x82EF47A8;
	sub_82EA6B58(ctx, base);
	// 82EF47A8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF47AC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF47B0: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF47B4: 41980020  blt cr6, 0x82ef47d4
	if ctx.cr[6].lt {
	pc = 0x82EF47D4; continue 'dispatch;
	}
	// 82EF47B8: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF47BC: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82EF47C0: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF47C4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF47C8: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF47CC: C1AAFFFC  lfs f13, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF47D0: 48000024  b 0x82ef47f4
	pc = 0x82EF47F4; continue 'dispatch;
	// 82EF47D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EF47D8: 40980008  bge cr6, 0x82ef47e0
	if !ctx.cr[6].lt {
	pc = 0x82EF47E0; continue 'dispatch;
	}
	// 82EF47DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF47E0: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF47E4: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF47E8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF47EC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF47F0: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF47F4: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 82EF47F8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82EF47FC: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF4800: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82EF4804: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EF4808: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82EF480C: FD606018  frsp f11, f12
	ctx.f[11].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82EF4810: 7D495C2E  lfsx f10, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF4814: ED3F5828  fsubs f9, f31, f11
	ctx.f[9].f64 = (((ctx.f[31].f64 - ctx.f[11].f64) as f32) as f64);
	// 82EF4818: EC29503A  fmadds f1, f9, f0, f10
	ctx.f[1].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 82EF481C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF4820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4828: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF482C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF4830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4838 size=348
    let mut pc: u32 = 0x82EF4838;
    'dispatch: loop {
        match pc {
            0x82EF4838 => {
    //   block [0x82EF4838..0x82EF4994)
	// 82EF4838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF4844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF4848: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF484C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF4850: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF4854: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82EF4858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF485C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4860: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF4864: 4E800421  bctrl
	ctx.lr = 0x82EF4868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF4868: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82EF486C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF4870: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82EF4874: 38C969F0  addi r6, r9, 0x69f0
	ctx.r[6].s64 = ctx.r[9].s64 + 27120;
	// 82EF4878: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF487C: 3C808209  lis r4, -0x7df7
	ctx.r[4].s64 = -2113339392;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF4998 size=92
    let mut pc: u32 = 0x82EF4998;
    'dispatch: loop {
        match pc {
            0x82EF4998 => {
    //   block [0x82EF4998..0x82EF49F4)
	// 82EF4998: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF499C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EF49A0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF49A4: 390AA2F0  addi r8, r10, -0x5d10
	ctx.r[8].s64 = ctx.r[10].s64 + -23824;
	// 82EF49A8: 38E9A824  addi r7, r9, -0x57dc
	ctx.r[7].s64 = ctx.r[9].s64 + -22492;
	// 82EF49AC: C00B9528  lfs f0, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF49B0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EF49B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF49B8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EF49BC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82EF49C0: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82EF49C4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EF49C8: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF49CC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF49F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF49F8 size=392
    let mut pc: u32 = 0x82EF49F8;
    'dispatch: loop {
        match pc {
            0x82EF49F8 => {
    //   block [0x82EF49F8..0x82EF4B80)
	// 82EF49F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF49FC: 482B3771  bl 0x831a816c
	ctx.lr = 0x82EF4A00;
	sub_831A8130(ctx, base);
	// 82EF4A00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF4A08: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF4A0C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EF4A10: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF4A14: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4A18: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF4A1C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF4A20: 409A0010  bne cr6, 0x82ef4a30
	if !ctx.cr[6].eq {
	pc = 0x82EF4A30; continue 'dispatch;
	}
	// 82EF4A24: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF4A28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF4A2C: 4BFB1E55  bl 0x82ea6880
	ctx.lr = 0x82EF4A30;
	sub_82EA6880(ctx, base);
	// 82EF4A30: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4A34: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4A38: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF4A3C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EF4A40: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF4A44: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4B80 size=212
    let mut pc: u32 = 0x82EF4B80;
    'dispatch: loop {
        match pc {
            0x82EF4B80 => {
    //   block [0x82EF4B80..0x82EF4C54)
	// 82EF4B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4B88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF4B8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF4B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4B94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF4B98: 81230024  lwz r9, 0x24(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4B9C: 3BC30020  addi r30, r3, 0x20
	ctx.r[30].s64 = ctx.r[3].s64 + 32;
	// 82EF4BA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4BA4: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF4BA8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF4BAC: 40980060  bge cr6, 0x82ef4c0c
	if !ctx.cr[6].lt {
	pc = 0x82EF4C0C; continue 'dispatch;
	}
	// 82EF4BB0: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF4BB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF4BB8: 409A0020  bne cr6, 0x82ef4bd8
	if !ctx.cr[6].eq {
	pc = 0x82EF4BD8; continue 'dispatch;
	}
	// 82EF4BBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4BC0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EF4BC4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EF4BC8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4BCC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF4BD0: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF4BD4: 4BFABBDD  bl 0x82ea07b0
	ctx.lr = 0x82EF4BD8;
	sub_82EA07B0(ctx, base);
	// 82EF4BD8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4BDC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF4BE0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4BE4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EF4BE8: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF4BEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF4BF0: 4BFABB41  bl 0x82ea0730
	ctx.lr = 0x82EF4BF4;
	sub_82EA0730(ctx, base);
	// 82EF4BF4: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4BF8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF4BFC: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF4C00: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4C04: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82EF4C08: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EF4C0C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4C10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4C14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF4C18: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF4C1C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4C20: 4099001C  ble cr6, 0x82ef4c3c
	if !ctx.cr[6].gt {
	pc = 0x82EF4C3C; continue 'dispatch;
	}
	// 82EF4C24: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4C58 size=20
    let mut pc: u32 = 0x82EF4C58;
    'dispatch: loop {
        match pc {
            0x82EF4C58 => {
    //   block [0x82EF4C58..0x82EF4C6C)
	// 82EF4C58: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF4C5C: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4C60: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF4C64: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF4C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4C70 size=192
    let mut pc: u32 = 0x82EF4C70;
    'dispatch: loop {
        match pc {
            0x82EF4C70 => {
    //   block [0x82EF4C70..0x82EF4D30)
	// 82EF4C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4C74: 482B34F9  bl 0x831a816c
	ctx.lr = 0x82EF4C78;
	sub_831A8130(ctx, base);
	// 82EF4C78: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82EF4C7C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82EF4C80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4C84: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82EF4C88: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF4C8C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EF4C90: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82EF4C94: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4D30 size=132
    let mut pc: u32 = 0x82EF4D30;
    'dispatch: loop {
        match pc {
            0x82EF4D30 => {
    //   block [0x82EF4D30..0x82EF4DB4)
	// 82EF4D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4D3C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82EF4D40: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4D44: 396BF330  addi r11, r11, -0xcd0
	ctx.r[11].s64 = ctx.r[11].s64 + -3280;
	// 82EF4D48: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82EF4D4C: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82EF4D50: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 82EF4D54: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EF4D58: 7D6A402E  lwzx r11, r10, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EF4D5C: 7D4A382E  lwzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EF4D60: 38AB0005  addi r5, r11, 5
	ctx.r[5].s64 = ctx.r[11].s64 + 5;
	// 82EF4D64: 386A0008  addi r3, r10, 8
	ctx.r[3].s64 = ctx.r[10].s64 + 8;
	// 82EF4D68: 54AA2036  slwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4D6C: 54682036  slwi r8, r3, 4
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF4D70: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82EF4D74: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 82EF4D78: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4DB8 size=8
    let mut pc: u32 = 0x82EF4DB8;
    'dispatch: loop {
        match pc {
            0x82EF4DB8 => {
    //   block [0x82EF4DB8..0x82EF4DC0)
	// 82EF4DB8: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 82EF4DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4DC0 size=12
    let mut pc: u32 = 0x82EF4DC0;
    'dispatch: loop {
        match pc {
            0x82EF4DC0 => {
    //   block [0x82EF4DC0..0x82EF4DCC)
	// 82EF4DC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF4DC4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF4DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4DD0 size=8
    let mut pc: u32 = 0x82EF4DD0;
    'dispatch: loop {
        match pc {
            0x82EF4DD0 => {
    //   block [0x82EF4DD0..0x82EF4DD8)
	// 82EF4DD0: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82EF4DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4DD8 size=76
    let mut pc: u32 = 0x82EF4DD8;
    'dispatch: loop {
        match pc {
            0x82EF4DD8 => {
    //   block [0x82EF4DD8..0x82EF4E24)
	// 82EF4DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF4DDC: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82EF4DE0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF4DE4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF4DE8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF4DEC: 39030018  addi r8, r3, 0x18
	ctx.r[8].s64 = ctx.r[3].s64 + 24;
	// 82EF4DF0: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF4DF4: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF4DF8: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EF4DFC: 91240014  stw r9, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF4E00: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF4E04: 90E40000  stw r7, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF4E08: 80C3001C  lwz r6, 0x1c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF4E0C: 90C40004  stw r6, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82EF4E10: 80A30020  lwz r5, 0x20(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF4E14: 90A40008  stw r5, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EF4E18: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4E1C: 9064000C  stw r3, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EF4E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4E28 size=16
    let mut pc: u32 = 0x82EF4E28;
    'dispatch: loop {
        match pc {
            0x82EF4E28 => {
    //   block [0x82EF4E28..0x82EF4E38)
	// 82EF4E28: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF4E2C: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4E30: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF4E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4E38 size=12
    let mut pc: u32 = 0x82EF4E38;
    'dispatch: loop {
        match pc {
            0x82EF4E38 => {
    //   block [0x82EF4E38..0x82EF4E44)
	// 82EF4E38: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 82EF4E3C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF4E40: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4E44 size=40
    let mut pc: u32 = 0x82EF4E44;
    'dispatch: loop {
        match pc {
            0x82EF4E44 => {
    //   block [0x82EF4E44..0x82EF4E6C)
	// 82EF4E44: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4E48: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82EF4E4C: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4E70 size=160
    let mut pc: u32 = 0x82EF4E70;
    'dispatch: loop {
        match pc {
            0x82EF4E70 => {
    //   block [0x82EF4E70..0x82EF4F10)
	// 82EF4E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4E74: 482B32F9  bl 0x831a816c
	ctx.lr = 0x82EF4E78;
	sub_831A8130(ctx, base);
	// 82EF4E78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4E7C: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82EF4E80: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4E84: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EF4E88: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF4E8C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF4E90: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82EF4E94: 8103004C  lwz r8, 0x4c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF4E98: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82EF4E9C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF4EA0: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 82EF4EA4: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4EA8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82EF4EAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4EB0: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4F10 size=92
    let mut pc: u32 = 0x82EF4F10;
    'dispatch: loop {
        match pc {
            0x82EF4F10 => {
    //   block [0x82EF4F10..0x82EF4F6C)
	// 82EF4F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4F14: 482B3259  bl 0x831a816c
	ctx.lr = 0x82EF4F18;
	sub_831A8130(ctx, base);
	// 82EF4F18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF4F20: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF4F24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF4F28: 392BB7BC  addi r9, r11, -0x4844
	ctx.r[9].s64 = ctx.r[11].s64 + -18500;
	// 82EF4F2C: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82EF4F30: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF4F34: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF4F38: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF4F3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF4F40: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4F44: 4BFF8495  bl 0x82eed3d8
	ctx.lr = 0x82EF4F48;
	sub_82EED3D8(ctx, base);
	// 82EF4F48: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82EF4F4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF4F50: 4BFFDE69  bl 0x82ef2db8
	ctx.lr = 0x82EF4F54;
	sub_82EF2DB8(ctx, base);
	// 82EF4F54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF4F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF4F5C: 4BFF847D  bl 0x82eed3d8
	ctx.lr = 0x82EF4F60;
	sub_82EED3D8(ctx, base);
	// 82EF4F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF4F64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF4F68: 482B3254  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4F70 size=160
    let mut pc: u32 = 0x82EF4F70;
    'dispatch: loop {
        match pc {
            0x82EF4F70 => {
    //   block [0x82EF4F70..0x82EF5010)
	// 82EF4F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4F74: 482B31F9  bl 0x831a816c
	ctx.lr = 0x82EF4F78;
	sub_831A8130(ctx, base);
	// 82EF4F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF4F80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF4F84: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF4F88: 392BB7BC  addi r9, r11, -0x4844
	ctx.r[9].s64 = ctx.r[11].s64 + -18500;
	// 82EF4F8C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4F90: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF4F94: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF4F98: 4099005C  ble cr6, 0x82ef4ff4
	if !ctx.cr[6].gt {
	pc = 0x82EF4FF4; continue 'dispatch;
	}
	// 82EF4F9C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF4FA0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF4FA4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EF4FA8: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4FAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF4FB0: 419A0030  beq cr6, 0x82ef4fe0
	if ctx.cr[6].eq {
	pc = 0x82EF4FE0; continue 'dispatch;
	}
	// 82EF4FB4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF4FB8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF4FBC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF4FC0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF4FC4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF4FC8: 409A0018  bne cr6, 0x82ef4fe0
	if !ctx.cr[6].eq {
	pc = 0x82EF4FE0; continue 'dispatch;
	}
	// 82EF4FCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4FD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF4FD4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4FD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF4FDC: 4E800421  bctrl
	ctx.lr = 0x82EF4FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF4FE0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4FE4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EF4FE8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EF4FEC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF4FF0: 4198FFB0  blt cr6, 0x82ef4fa0
	if ctx.cr[6].lt {
	pc = 0x82EF4FA0; continue 'dispatch;
	}
	// 82EF4FF4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82EF4FF8: 4BFE26B9  bl 0x82ed76b0
	ctx.lr = 0x82EF4FFC;
	sub_82ED76B0(ctx, base);
	// 82EF4FFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF5000: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF5004: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5008: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF500C: 482B31B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5010 size=1496
    let mut pc: u32 = 0x82EF5010;
    'dispatch: loop {
        match pc {
            0x82EF5010 => {
    //   block [0x82EF5010..0x82EF510C)
	// 82EF5010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5014: 482B3141  bl 0x831a8154
	ctx.lr = 0x82EF5018;
	sub_831A8130(ctx, base);
	// 82EF5018: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF501C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82EF5020: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF5024: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF5028: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EF502C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82EF5030: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82EF5034: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF5038: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF503C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EF5040: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF5044: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82EF5048: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF504C: 4200FFF0  bdnz 0x82ef503c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF503C; continue 'dispatch;
	}
	// 82EF5050: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF5054: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5058: 80810098  lwz r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EF505C: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82EF5060: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF5064: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 82EF5068: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF506C: 831E0028  lwz r24, 0x28(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF5070: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF5074: 90810170  stw r4, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[4].u32 ) };
	// 82EF5078: 91410174  stw r10, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[10].u32 ) };
	// 82EF507C: 91610178  stw r11, 0x178(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[11].u32 ) };
	// 82EF5080: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5084: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82EF5088: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82EF508C: 41990544  bgt cr6, 0x82ef55d0
	if ctx.cr[6].gt {
	pc = 0x82EF55D0; continue 'dispatch;
	}
	// 82EF5090: 3D8082EF  lis r12, -0x7d11
	ctx.r[12].s64 = -2098266112;
	// 82EF5094: 398C50A8  addi r12, r12, 0x50a8
	ctx.r[12].s64 = ctx.r[12].s64 + 20648;
	// 82EF5098: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82EF509C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82EF50A0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82EF50A4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82EF55E0; continue 'dispatch;
		},
		1 => {
	pc = 0x82EF5520; continue 'dispatch;
		},
		2 => {
	pc = 0x82EF5544; continue 'dispatch;
		},
		3 => {
	pc = 0x82EF510C; continue 'dispatch;
		},
		4 => {
	pc = 0x82EF5160; continue 'dispatch;
		},
		5 => {
	pc = 0x82EF51B4; continue 'dispatch;
		},
		6 => {
	pc = 0x82EF51E4; continue 'dispatch;
		},
		7 => {
	pc = 0x82EF5218; continue 'dispatch;
		},
		8 => {
	pc = 0x82EF5248; continue 'dispatch;
		},
		9 => {
	pc = 0x82EF526C; continue 'dispatch;
		},
		10 => {
	pc = 0x82EF5294; continue 'dispatch;
		},
		11 => {
	pc = 0x82EF52C8; continue 'dispatch;
		},
		12 => {
	pc = 0x82EF52FC; continue 'dispatch;
		},
		13 => {
	pc = 0x82EF5330; continue 'dispatch;
		},
		14 => {
	pc = 0x82EF5440; continue 'dispatch;
		},
		15 => {
	pc = 0x82EF5478; continue 'dispatch;
		},
		16 => {
	pc = 0x82EF5360; continue 'dispatch;
		},
		17 => {
	pc = 0x82EF5398; continue 'dispatch;
		},
		18 => {
	pc = 0x82EF53D0; continue 'dispatch;
		},
		19 => {
	pc = 0x82EF5408; continue 'dispatch;
		},
		20 => {
	pc = 0x82EF54B0; continue 'dispatch;
		},
		21 => {
	pc = 0x82EF54E8; continue 'dispatch;
		},
		22 => {
	pc = 0x82EF556C; continue 'dispatch;
		},
		23 => {
	pc = 0x82EF5594; continue 'dispatch;
		},
		24 => {
	pc = 0x82EF55AC; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82EF50A8: 82EF55E0  lwz r23, 0x55e0(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21984 as u32) ) } as u64;
	// 82EF50AC: 82EF5520  lwz r23, 0x5520(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21792 as u32) ) } as u64;
	// 82EF50B0: 82EF5544  lwz r23, 0x5544(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21828 as u32) ) } as u64;
	// 82EF50B4: 82EF510C  lwz r23, 0x510c(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(20748 as u32) ) } as u64;
	// 82EF50B8: 82EF5160  lwz r23, 0x5160(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(20832 as u32) ) } as u64;
	// 82EF50BC: 82EF51B4  lwz r23, 0x51b4(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(20916 as u32) ) } as u64;
	// 82EF50C0: 82EF51E4  lwz r23, 0x51e4(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(20964 as u32) ) } as u64;
	// 82EF50C4: 82EF5218  lwz r23, 0x5218(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21016 as u32) ) } as u64;
	// 82EF50C8: 82EF5248  lwz r23, 0x5248(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21064 as u32) ) } as u64;
	// 82EF50CC: 82EF526C  lwz r23, 0x526c(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21100 as u32) ) } as u64;
	// 82EF50D0: 82EF5294  lwz r23, 0x5294(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21140 as u32) ) } as u64;
	// 82EF50D4: 82EF52C8  lwz r23, 0x52c8(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21192 as u32) ) } as u64;
	// 82EF50D8: 82EF52FC  lwz r23, 0x52fc(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21244 as u32) ) } as u64;
	// 82EF50DC: 82EF5330  lwz r23, 0x5330(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21296 as u32) ) } as u64;
	// 82EF50E0: 82EF5440  lwz r23, 0x5440(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21568 as u32) ) } as u64;
	// 82EF50E4: 82EF5478  lwz r23, 0x5478(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21624 as u32) ) } as u64;
	// 82EF50E8: 82EF5360  lwz r23, 0x5360(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21344 as u32) ) } as u64;
	// 82EF50EC: 82EF5398  lwz r23, 0x5398(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21400 as u32) ) } as u64;
	// 82EF50F0: 82EF53D0  lwz r23, 0x53d0(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21456 as u32) ) } as u64;
	// 82EF50F4: 82EF5408  lwz r23, 0x5408(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21512 as u32) ) } as u64;
	// 82EF50F8: 82EF54B0  lwz r23, 0x54b0(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21680 as u32) ) } as u64;
	// 82EF50FC: 82EF54E8  lwz r23, 0x54e8(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21736 as u32) ) } as u64;
	// 82EF5100: 82EF556C  lwz r23, 0x556c(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21868 as u32) ) } as u64;
	// 82EF5104: 82EF5594  lwz r23, 0x5594(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21908 as u32) ) } as u64;
	// 82EF5108: 82EF55AC  lwz r23, 0x55ac(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(21932 as u32) ) } as u64;
            }
            0x82EF510C => {
    //   block [0x82EF510C..0x82EF5160)
	pc = 0x82EF5160; continue 'dispatch;
            }
            0x82EF5160 => {
    //   block [0x82EF5160..0x82EF51B4)
	pc = 0x82EF51B4; continue 'dispatch;
            }
            0x82EF51B4 => {
    //   block [0x82EF51B4..0x82EF51E4)
	// 82EF51B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	pc = 0x82EF51E4; continue 'dispatch;
            }
            0x82EF51E4 => {
    //   block [0x82EF51E4..0x82EF5218)
	// 82EF51E4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF51E8: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF51EC: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF51F0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF51F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF51F8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF51FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF5200: 480004E1  bl 0x82ef56e0
	ctx.lr = 0x82EF5204;
	sub_82EF56E0(ctx, base);
	// 82EF5204: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5208: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF520C: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5210: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5214: 4BFFFE6C  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5218 => {
    //   block [0x82EF5218..0x82EF5248)
	// 82EF5218: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF521C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5220: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF5224: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF5228: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF522C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5230: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF5234: 4BFFFA3D  bl 0x82ef4c70
	ctx.lr = 0x82EF5238;
	sub_82EF4C70(ctx, base);
	// 82EF5238: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF523C: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5240: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5244: 4BFFFE3C  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5248 => {
    //   block [0x82EF5248..0x82EF526C)
	// 82EF5248: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF524C: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82EF5250: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82EF5254: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF5258: 4BFB42F9  bl 0x82ea9550
	ctx.lr = 0x82EF525C;
	sub_82EA9550(ctx, base);
	// 82EF525C: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5260: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5264: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5268: 4BFFFE18  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF526C => {
    //   block [0x82EF526C..0x82EF5294)
	// 82EF526C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF5270: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82EF5274: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82EF5278: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82EF527C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF5280: 4BFB42D1  bl 0x82ea9550
	ctx.lr = 0x82EF5284;
	sub_82EA9550(ctx, base);
	// 82EF5284: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5288: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF528C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5290: 4BFFFDF0  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5294 => {
    //   block [0x82EF5294..0x82EF52C8)
	// 82EF5294: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	pc = 0x82EF52C8; continue 'dispatch;
            }
            0x82EF52C8 => {
    //   block [0x82EF52C8..0x82EF52FC)
	// 82EF52C8: 39610140  addi r11, r1, 0x140
	ctx.r[11].s64 = ctx.r[1].s64 + 320;
	pc = 0x82EF52FC; continue 'dispatch;
            }
            0x82EF52FC => {
    //   block [0x82EF52FC..0x82EF5330)
	// 82EF52FC: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF5300: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5304: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF5308: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF530C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF5310: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5314: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF5318: 48000451  bl 0x82ef5768
	ctx.lr = 0x82EF531C;
	sub_82EF5768(ctx, base);
	// 82EF531C: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5320: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5324: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5328: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF532C: 4BFFFD54  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5330 => {
    //   block [0x82EF5330..0x82EF5360)
	// 82EF5330: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF5334: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5338: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF533C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF5340: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF5344: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5348: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF534C: 48000505  bl 0x82ef5850
	ctx.lr = 0x82EF5350;
	sub_82EF5850(ctx, base);
	// 82EF5350: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5354: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5358: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF535C: 4BFFFD24  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5360 => {
    //   block [0x82EF5360..0x82EF5398)
	// 82EF5360: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF5364: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5368: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF536C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF5370: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF5374: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5378: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF537C: 480007ED  bl 0x82ef5b68
	ctx.lr = 0x82EF5380;
	sub_82EF5B68(ctx, base);
	// 82EF5380: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5384: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5388: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF538C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5390: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5394: 4BFFFCEC  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5398 => {
    //   block [0x82EF5398..0x82EF53D0)
	// 82EF5398: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF539C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF53A0: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF53A4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF53A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF53AC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF53B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF53B4: 4800084D  bl 0x82ef5c00
	ctx.lr = 0x82EF53B8;
	sub_82EF5C00(ctx, base);
	// 82EF53B8: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF53BC: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF53C0: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF53C4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF53C8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF53CC: 4BFFFCB4  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF53D0 => {
    //   block [0x82EF53D0..0x82EF5408)
	// 82EF53D0: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF53D4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF53D8: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF53DC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF53E0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF53E4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF53E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF53EC: 48000555  bl 0x82ef5940
	ctx.lr = 0x82EF53F0;
	sub_82EF5940(ctx, base);
	// 82EF53F0: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF53F4: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF53F8: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF53FC: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5400: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5404: 4BFFFC7C  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5408 => {
    //   block [0x82EF5408..0x82EF5440)
	// 82EF5408: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF540C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5410: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF5414: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF5418: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF541C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5420: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF5424: 48000685  bl 0x82ef5aa8
	ctx.lr = 0x82EF5428;
	sub_82EF5AA8(ctx, base);
	// 82EF5428: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF542C: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5430: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5434: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5438: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF543C: 4BFFFC44  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5440 => {
    //   block [0x82EF5440..0x82EF5478)
	// 82EF5440: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF5444: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5448: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF544C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF5450: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF5454: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5458: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF545C: 48000A7D  bl 0x82ef5ed8
	ctx.lr = 0x82EF5460;
	sub_82EF5ED8(ctx, base);
	// 82EF5460: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5464: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5468: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF546C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5470: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5474: 4BFFFC0C  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5478 => {
    //   block [0x82EF5478..0x82EF54B0)
	// 82EF5478: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF547C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5480: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF5484: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF5488: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF548C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5490: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF5494: 48000875  bl 0x82ef5d08
	ctx.lr = 0x82EF5498;
	sub_82EF5D08(ctx, base);
	// 82EF5498: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF549C: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF54A0: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF54A4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF54A8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF54AC: 4BFFFBD4  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF54B0 => {
    //   block [0x82EF54B0..0x82EF54E8)
	// 82EF54B0: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF54B4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF54B8: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF54BC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF54C0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF54C4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF54C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF54CC: 4800096D  bl 0x82ef5e38
	ctx.lr = 0x82EF54D0;
	sub_82EF5E38(ctx, base);
	// 82EF54D0: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF54D4: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF54D8: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF54DC: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF54E0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF54E4: 4BFFFB9C  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF54E8 => {
    //   block [0x82EF54E8..0x82EF5520)
	// 82EF54E8: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82EF54EC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF54F0: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82EF54F4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF54F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF54FC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF5500: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF5504: 4BFFF96D  bl 0x82ef4e70
	ctx.lr = 0x82EF5508;
	sub_82EF4E70(ctx, base);
	// 82EF5508: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF550C: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5510: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5514: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5518: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF551C: 4BFFFB64  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5520 => {
    //   block [0x82EF5520..0x82EF5544)
	// 82EF5520: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF5524: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82EF5528: 4BFB1941  bl 0x82ea6e68
	ctx.lr = 0x82EF552C;
	sub_82EA6E68(ctx, base);
	// 82EF552C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82EF5530: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF5534: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5538: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF553C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF5540: 4BFFFB40  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5544 => {
    //   block [0x82EF5544..0x82EF556C)
	// 82EF5544: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF5548: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82EF554C: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82EF5550: 4BFB1919  bl 0x82ea6e68
	ctx.lr = 0x82EF5554;
	sub_82EA6E68(ctx, base);
	// 82EF5554: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82EF5558: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF555C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5560: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5564: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF5568: 4BFFFB18  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF556C => {
    //   block [0x82EF556C..0x82EF5594)
	// 82EF556C: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5570: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82EF5574: C1A10080  lfs f13, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF5578: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF557C: ED8D0032  fmuls f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EF5580: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF5584: D1810080  stfs f12, 0x80(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82EF5588: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF558C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF5590: 4BFFFAF0  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF5594 => {
    //   block [0x82EF5594..0x82EF55AC)
	// 82EF5594: 81770020  lwz r11, 0x20(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF5598: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF559C: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF55A0: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF55A4: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82EF55A8: 4BFFFAD8  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF55AC => {
    //   block [0x82EF55AC..0x82EF55E0)
	// 82EF55AC: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF55B0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF55B4: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82EF55B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF55BC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF55C0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF55C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF55C8: 4E800421  bctrl
	ctx.lr = 0x82EF55CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF55CC: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 82EF55D0: 81410174  lwz r10, 0x174(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF55D4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF55D8: 80810170  lwz r4, 0x170(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF55DC: 4BFFFAA4  b 0x82ef5080
	pc = 0x82EF5080; continue 'dispatch;
            }
            0x82EF55E0 => {
    //   block [0x82EF55E0..0x82EF55E8)
	// 82EF55E0: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82EF55E4: 482B2BC0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF55E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF55E8 size=172
    let mut pc: u32 = 0x82EF55E8;
    'dispatch: loop {
        match pc {
            0x82EF55E8 => {
    //   block [0x82EF55E8..0x82EF5694)
	// 82EF55E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF55EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF55F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF55F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF55F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF55FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5600: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF5604: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF5608: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF560C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF5610: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EF5614: 38E8B7BC  addi r7, r8, -0x4844
	ctx.r[7].s64 = ctx.r[8].s64 + -18500;
	// 82EF5618: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EF561C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF5620: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF5624: B13F000C  sth r9, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 82EF5628: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EF562C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EF5630: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82EF5634: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82EF5638: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82EF563C: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82EF5640: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82EF5644: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82EF5648: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82EF564C: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82EF5650: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5654: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82EF5658: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EF565C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF5660: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EF5664: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EF5668: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82EF566C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF5670: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82EF5674: 90DF001C  stw r6, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82EF5678: 4BFF7D61  bl 0x82eed3d8
	ctx.lr = 0x82EF567C;
	sub_82EED3D8(ctx, base);
	// 82EF567C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF5680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF568C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF5690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5698 size=72
    let mut pc: u32 = 0x82EF5698;
    'dispatch: loop {
        match pc {
            0x82EF5698 => {
    //   block [0x82EF5698..0x82EF56E0)
	// 82EF5698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF569C: 482B2AD1  bl 0x831a816c
	ctx.lr = 0x82EF56A0;
	sub_831A8130(ctx, base);
	// 82EF56A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF56A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF56A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF56AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF56B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF56B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF56B8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82EF56BC: 80BE004C  lwz r5, 0x4c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF56C0: 480121D1  bl 0x82f07890
	ctx.lr = 0x82EF56C4;
	sub_82F07890(ctx, base);
	// 82EF56C4: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 82EF56C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EF56CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF56D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF56D4: 4BFFF93D  bl 0x82ef5010
	ctx.lr = 0x82EF56D8;
	sub_82EF5010(ctx, base);
	// 82EF56D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF56DC: 482B2AE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF56E0 size=132
    let mut pc: u32 = 0x82EF56E0;
    'dispatch: loop {
        match pc {
            0x82EF56E0 => {
    //   block [0x82EF56E0..0x82EF5764)
	// 82EF56E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF56E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF56E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF56EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF56F0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF56F4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF56F8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82EF56FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5700: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82EF5704: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5708: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5768 size=228
    let mut pc: u32 = 0x82EF5768;
    'dispatch: loop {
        match pc {
            0x82EF5768 => {
    //   block [0x82EF5768..0x82EF584C)
	// 82EF5768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF576C: 482B2A01  bl 0x831a816c
	ctx.lr = 0x82EF5770;
	sub_831A8130(ctx, base);
	// 82EF5770: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5774: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5778: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82EF577C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5780: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5784: 394AF330  addi r10, r10, -0xcd0
	ctx.r[10].s64 = ctx.r[10].s64 + -3280;
	// 82EF5788: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF578C: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82EF5790: 38EA0008  addi r7, r10, 8
	ctx.r[7].s64 = ctx.r[10].s64 + 8;
	// 82EF5794: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 82EF5798: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF579C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF57A0: 7D45382E  lwzx r10, r5, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EF57A4: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82EF57A8: 7D65302E  lwzx r11, r5, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82EF57AC: 386B0005  addi r3, r11, 5
	ctx.r[3].s64 = ctx.r[11].s64 + 5;
	// 82EF57B0: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF57B4: 394A0005  addi r10, r10, 5
	ctx.r[10].s64 = ctx.r[10].s64 + 5;
	// 82EF57B8: 54672036  slwi r7, r3, 4
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF57BC: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82EF57C0: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5850 size=236
    let mut pc: u32 = 0x82EF5850;
    'dispatch: loop {
        match pc {
            0x82EF5850 => {
    //   block [0x82EF5850..0x82EF593C)
	// 82EF5850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5854: 482B2919  bl 0x831a816c
	ctx.lr = 0x82EF5858;
	sub_831A8130(ctx, base);
	// 82EF5858: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF585C: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82EF5860: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5864: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EF5868: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 82EF586C: 39400090  li r10, 0x90
	ctx.r[10].s64 = 144;
	// 82EF5870: 39200070  li r9, 0x70
	ctx.r[9].s64 = 112;
	// 82EF5874: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82EF5878: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82EF587C: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5940 size=356
    let mut pc: u32 = 0x82EF5940;
    'dispatch: loop {
        match pc {
            0x82EF5940 => {
    //   block [0x82EF5940..0x82EF5AA4)
	// 82EF5940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5944: 482B281D  bl 0x831a8160
	ctx.lr = 0x82EF5948;
	sub_831A8130(ctx, base);
	// 82EF5948: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF594C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5950: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5954: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82EF5958: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF595C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EF5960: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5964: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82EF5968: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82EF596C: C0079450  lfs f0, -0x6bb0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5970: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82EF5974: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5978: 3BAB0005  addi r29, r11, 5
	ctx.r[29].s64 = ctx.r[11].s64 + 5;
	// 82EF597C: 3B8B0008  addi r28, r11, 8
	ctx.r[28].s64 = ctx.r[11].s64 + 8;
	// 82EF5980: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82EF5984: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5988: 57872036  slwi r7, r28, 4
	ctx.r[7].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF598C: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF5990: 7F87FA14  add r28, r7, r31
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82EF5994: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82EF5998: 7D6BF82A  ldx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	// 82EF599C: 3BC10090  addi r30, r1, 0x90
	ctx.r[30].s64 = ctx.r[1].s64 + 144;
	// 82EF59A0: 7CE7F82A  ldx r7, r7, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) };
	// 82EF59A4: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82EF59A8: EBBD0008  ld r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	// 82EF59AC: 3F408201  lis r26, -0x7dff
	ctx.r[26].s64 = -2113863680;
	// 82EF59B0: EB9C0008  ld r28, 8(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	// 82EF59B4: F9660000  std r11, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82EF59B8: F8E30000  std r7, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82EF59BC: FBA60008  std r29, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5AA8 size=192
    let mut pc: u32 = 0x82EF5AA8;
    'dispatch: loop {
        match pc {
            0x82EF5AA8 => {
    //   block [0x82EF5AA8..0x82EF5B68)
	// 82EF5AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5AAC: 482B26BD  bl 0x831a8168
	ctx.lr = 0x82EF5AB0;
	sub_831A8130(ctx, base);
	// 82EF5AB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5AB4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5AB8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EF5ABC: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82EF5AC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5AC4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF5AC8: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82EF5ACC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5AD0: C00A9450  lfs f0, -0x6bb0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5AD4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5AD8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82EF5ADC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5AE0: 38CA0008  addi r6, r10, 8
	ctx.r[6].s64 = ctx.r[10].s64 + 8;
	// 82EF5AE4: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF5AE8: 38AA0005  addi r5, r10, 5
	ctx.r[5].s64 = ctx.r[10].s64 + 5;
	// 82EF5AEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5AF0: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82EF5AF4: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EF5AF8: 388B0005  addi r4, r11, 5
	ctx.r[4].s64 = ctx.r[11].s64 + 5;
	// 82EF5AFC: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5B00: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF5B04: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF5B08: 54A32036  slwi r3, r5, 4
	ctx.r[3].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF5B0C: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 82EF5B10: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82EF5B14: 7CCBFA14  add r6, r11, r31
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF5B18: 7CAAFA14  add r5, r10, r31
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82EF5B1C: 7C89FA14  add r4, r9, r31
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82EF5B20: 7C63FA14  add r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EF5B24: 4801FA2D  bl 0x82f15550
	ctx.lr = 0x82EF5B28;
	sub_82F15550(ctx, base);
	// 82EF5B28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5B2C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF5B30: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82EF5B34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF5B38: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5B3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF5B40: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5B44: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82EF5B48: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF5B4C: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82EF5B50: 4800F7E1  bl 0x82f05330
	ctx.lr = 0x82EF5B54;
	sub_82F05330(ctx, base);
	// 82EF5B54: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82EF5B58: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EF5B5C: 913F00B8  stw r9, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[9].u32 ) };
	// 82EF5B60: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF5B64: 482B2654  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5B68 size=148
    let mut pc: u32 = 0x82EF5B68;
    'dispatch: loop {
        match pc {
            0x82EF5B68 => {
    //   block [0x82EF5B68..0x82EF5BFC)
	// 82EF5B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5B6C: 482B25FD  bl 0x831a8168
	ctx.lr = 0x82EF5B70;
	sub_831A8130(ctx, base);
	// 82EF5B70: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5B74: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5B78: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5B7C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EF5B80: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF5B84: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82EF5B88: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5B8C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5C00 size=260
    let mut pc: u32 = 0x82EF5C00;
    'dispatch: loop {
        match pc {
            0x82EF5C00 => {
    //   block [0x82EF5C00..0x82EF5D04)
	// 82EF5C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5C04: 482B2565  bl 0x831a8168
	ctx.lr = 0x82EF5C08;
	sub_831A8130(ctx, base);
	// 82EF5C08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5C0C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5C10: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EF5C14: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5C18: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5C1C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EF5C20: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5C24: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82EF5C28: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82EF5C2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF5C30: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5C34: 39030005  addi r8, r3, 5
	ctx.r[8].s64 = ctx.r[3].s64 + 5;
	// 82EF5C38: 55072036  slwi r7, r8, 4
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF5C3C: C00A9450  lfs f0, -0x6bb0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5C40: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82EF5C44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5D08 size=304
    let mut pc: u32 = 0x82EF5D08;
    'dispatch: loop {
        match pc {
            0x82EF5D08 => {
    //   block [0x82EF5D08..0x82EF5E38)
	// 82EF5D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5D0C: 482B2455  bl 0x831a8160
	ctx.lr = 0x82EF5D10;
	sub_831A8130(ctx, base);
	// 82EF5D10: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82EF5D14: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5D18: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82EF5D1C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5D20: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EF5D24: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EF5D28: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 82EF5D2C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5D30: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82EF5D34: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82EF5D38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5D3C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82EF5D40: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5D44: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5D48: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82EF5D4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5D50: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5D54: 837E004C  lwz r27, 0x4c(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF5D58: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF5D5C: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E38 size=156
    let mut pc: u32 = 0x82EF5E38;
    'dispatch: loop {
        match pc {
            0x82EF5E38 => {
    //   block [0x82EF5E38..0x82EF5ED4)
	// 82EF5E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF5E44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5E48: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82EF5E4C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82EF5E50: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EF5E54: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5E58: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82EF5E5C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5E60: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EF5E64: 8104004C  lwz r8, 0x4c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF5E68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF5E6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5E70: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5E74: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5E78: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82EF5E7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5E80: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF5E84: 80DF00B8  lwz r6, 0xb8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82EF5E88: 54C91838  slwi r9, r6, 3
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF5E8C: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82EF5E90: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82EF5E94: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5E98: 7D0BFA14  add r8, r11, r31
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF5E9C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82EF5EA0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5EA4: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82EF5EA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82EF5EAC: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82EF5EB0: 4801C821  bl 0x82f126d0
	ctx.lr = 0x82EF5EB4;
	sub_82F126D0(ctx, base);
	// 82EF5EB4: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82EF5EB8: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82EF5EBC: 90FF00B8  stw r7, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[7].u32 ) };
	// 82EF5EC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF5EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF5ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5ED8 size=488
    let mut pc: u32 = 0x82EF5ED8;
    'dispatch: loop {
        match pc {
            0x82EF5ED8 => {
    //   block [0x82EF5ED8..0x82EF60C0)
	// 82EF5ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5EDC: 482B2281  bl 0x831a815c
	ctx.lr = 0x82EF5EE0;
	sub_831A8130(ctx, base);
	// 82EF5EE0: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82EF5EE4: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82EF5EE8: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82EF5EEC: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5EF0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5EF4: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EF5EF8: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82EF5EFC: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF5F00: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82EF5F04: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5F08: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EF5F0C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F10: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82EF5F14: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F18: 3929F330  addi r9, r9, -0xcd0
	ctx.r[9].s64 = ctx.r[9].s64 + -3280;
	// 82EF5F1C: 837E004C  lwz r27, 0x4c(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF5F20: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82EF5F24: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF5F28: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF5F2C: 80C60034  lwz r6, 0x34(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EF5F30: 38690008  addi r3, r9, 8
	ctx.r[3].s64 = ctx.r[9].s64 + 8;
	// 82EF5F34: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82EF5F38: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF5F3C: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF5F40: C3EB0004  lfs f31, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF5F44: C3CB0008  lfs f30, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EF5F48: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82EF5F4C: D9A10060  stfd f13, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[13].u64 ) };
	// 82EF5F50: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EF5F54: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5F58: 7D05182E  lwzx r8, r5, r3
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EF5F5C: 38680008  addi r3, r8, 8
	ctx.r[3].s64 = ctx.r[8].s64 + 8;
	// 82EF5F60: 7D25482E  lwzx r9, r5, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EF5F64: 38A90005  addi r5, r9, 5
	ctx.r[5].s64 = ctx.r[9].s64 + 5;
	// 82EF5F68: 7F8B302E  lwzx r28, r11, r6
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82EF5F6C: 54AB2036  slwi r11, r5, 4
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5F70: 54682036  slwi r8, r3, 4
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF5F74: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF5F78: 38C90008  addi r6, r9, 8
	ctx.r[6].s64 = ctx.r[9].s64 + 8;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF60C0 size=100
    let mut pc: u32 = 0x82EF60C0;
    'dispatch: loop {
        match pc {
            0x82EF60C0 => {
    //   block [0x82EF60C0..0x82EF6124)
	// 82EF60C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF60C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF60C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF60CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF60D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF60D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF60D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF60DC: 4BFFEE95  bl 0x82ef4f70
	ctx.lr = 0x82EF60E0;
	sub_82EF4F70(ctx, base);
	// 82EF60E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EF60E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF60E8: 419A0020  beq cr6, 0x82ef6108
	if ctx.cr[6].eq {
	pc = 0x82EF6108; continue 'dispatch;
	}
	// 82EF60EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF60F0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF60F4: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EF60F8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF60FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF6100: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF6104: 4BFAA6AD  bl 0x82ea07b0
	ctx.lr = 0x82EF6108;
	sub_82EA07B0(ctx, base);
	// 82EF6108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF610C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6118: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF611C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6128 size=8
    let mut pc: u32 = 0x82EF6128;
    'dispatch: loop {
        match pc {
            0x82EF6128 => {
    //   block [0x82EF6128..0x82EF6130)
	// 82EF6128: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF612C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6130 size=12
    let mut pc: u32 = 0x82EF6130;
    'dispatch: loop {
        match pc {
            0x82EF6130 => {
    //   block [0x82EF6130..0x82EF613C)
	// 82EF6130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6134: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF6138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6140 size=12
    let mut pc: u32 = 0x82EF6140;
    'dispatch: loop {
        match pc {
            0x82EF6140 => {
    //   block [0x82EF6140..0x82EF614C)
	// 82EF6140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6144: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF6148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6150 size=4
    let mut pc: u32 = 0x82EF6150;
    'dispatch: loop {
        match pc {
            0x82EF6150 => {
    //   block [0x82EF6150..0x82EF6154)
	// 82EF6150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6158 size=8
    let mut pc: u32 = 0x82EF6158;
    'dispatch: loop {
        match pc {
            0x82EF6158 => {
    //   block [0x82EF6158..0x82EF6160)
	// 82EF6158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF615C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6160 size=20
    let mut pc: u32 = 0x82EF6160;
    'dispatch: loop {
        match pc {
            0x82EF6160 => {
    //   block [0x82EF6160..0x82EF6174)
	// 82EF6160: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6164: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6168: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF616C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF6170: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6178 size=216
    let mut pc: u32 = 0x82EF6178;
    'dispatch: loop {
        match pc {
            0x82EF6178 => {
    //   block [0x82EF6178..0x82EF6250)
	// 82EF6178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF617C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF6184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF618C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6190: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF6194: 3BCB002C  addi r30, r11, 0x2c
	ctx.r[30].s64 = ctx.r[11].s64 + 44;
	// 82EF6198: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF619C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF61A0: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF61A4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF61A8: 40980060  bge cr6, 0x82ef6208
	if !ctx.cr[6].lt {
	pc = 0x82EF6208; continue 'dispatch;
	}
	// 82EF61AC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF61B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF61B4: 409A0020  bne cr6, 0x82ef61d4
	if !ctx.cr[6].eq {
	pc = 0x82EF61D4; continue 'dispatch;
	}
	// 82EF61B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF61BC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EF61C0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EF61C4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF61C8: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF61CC: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF61D0: 4BFAA5E1  bl 0x82ea07b0
	ctx.lr = 0x82EF61D4;
	sub_82EA07B0(ctx, base);
	// 82EF61D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF61D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF61DC: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF61E0: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EF61E4: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF61E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF61EC: 4BFAA545  bl 0x82ea0730
	ctx.lr = 0x82EF61F0;
	sub_82EA0730(ctx, base);
	// 82EF61F0: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF61F4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF61F8: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF61FC: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6200: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82EF6204: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EF6208: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF620C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6210: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6214: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6218: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF621C: 4099001C  ble cr6, 0x82ef6238
	if !ctx.cr[6].gt {
	pc = 0x82EF6238; continue 'dispatch;
	}
	// 82EF6220: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82EF6224: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF6228: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF622C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF6230: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF6234: 4082FFF0  bne 0x82ef6224
	if !ctx.cr[0].eq {
	pc = 0x82EF6224; continue 'dispatch;
	}
	// 82EF6238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF623C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6244: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF624C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6250 size=136
    let mut pc: u32 = 0x82EF6250;
    'dispatch: loop {
        match pc {
            0x82EF6250 => {
    //   block [0x82EF6250..0x82EF62D8)
	// 82EF6250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF625C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6264: 83E40008  lwz r31, 8(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6268: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF626C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6270: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF6274: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF6278: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82EF627C: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EF6280: 4BFDBF79  bl 0x82ed21f8
	ctx.lr = 0x82EF6284;
	sub_82ED21F8(ctx, base);
	// 82EF6284: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF6288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF628C: 4BFE897D  bl 0x82edec08
	ctx.lr = 0x82EF6290;
	sub_82EDEC08(ctx, base);
	// 82EF6290: 815F0084  lwz r10, 0x84(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF6294: 356AFFFF  addic. r11, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF6298: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82EF629C: 40820024  bne 0x82ef62c0
	if !ctx.cr[0].eq {
	pc = 0x82EF62C0; continue 'dispatch;
	}
	// 82EF62A0: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF62A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF62A8: 409A0018  bne cr6, 0x82ef62c0
	if !ctx.cr[6].eq {
	pc = 0x82EF62C0; continue 'dispatch;
	}
	// 82EF62AC: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EF62B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF62B4: 419A000C  beq cr6, 0x82ef62c0
	if ctx.cr[6].eq {
	pc = 0x82EF62C0; continue 'dispatch;
	}
	// 82EF62B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF62BC: 4BFD6D2D  bl 0x82eccfe8
	ctx.lr = 0x82EF62C0;
	sub_82ECCFE8(ctx, base);
	// 82EF62C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF62C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF62C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF62CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF62D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF62D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF62D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF62D8 size=188
    let mut pc: u32 = 0x82EF62D8;
    'dispatch: loop {
        match pc {
            0x82EF62D8 => {
    //   block [0x82EF62D8..0x82EF6394)
	// 82EF62D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF62DC: 482B1E91  bl 0x831a816c
	ctx.lr = 0x82EF62E0;
	sub_831A8130(ctx, base);
	// 82EF62E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF62E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF62E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF62EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF62F0: 4BFE7219  bl 0x82edd508
	ctx.lr = 0x82EF62F4;
	sub_82EDD508(ctx, base);
	// 82EF62F4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF62F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF62FC: 394BAC60  addi r10, r11, -0x53a0
	ctx.r[10].s64 = ctx.r[11].s64 + -21408;
	// 82EF6300: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EF6304: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF6308: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82EF630C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82EF6310: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 82EF6314: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82EF6318: A11D0004  lhz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF631C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF6320: 419A0010  beq cr6, 0x82ef6330
	if ctx.cr[6].eq {
	pc = 0x82EF6330; continue 'dispatch;
	}
	// 82EF6324: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF6328: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF632C: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF6330: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EF6334: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82EF6338: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EF633C: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 82EF6340: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6344: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82EF6348: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EF634C: 4BFAA3E5  bl 0x82ea0730
	ctx.lr = 0x82EF6350;
	sub_82EA0730(ctx, base);
	// 82EF6350: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82EF6354: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF6358: 93E90018  stw r31, 0x18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82EF635C: 38E0001C  li r7, 0x1c
	ctx.r[7].s64 = 28;
	// 82EF6360: 93C90008  stw r30, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EF6364: 38C8ABD0  addi r6, r8, -0x5430
	ctx.r[6].s64 = ctx.r[8].s64 + -21552;
	// 82EF6368: 93C9000C  stw r30, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82EF636C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF6370: B0E90004  sth r7, 4(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82EF6374: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF6378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF637C: B0A90006  sth r5, 6(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82EF6380: 93C90010  stw r30, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EF6384: 93C90014  stw r30, 0x14(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EF6388: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82EF638C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6390: 482B1E2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6398 size=168
    let mut pc: u32 = 0x82EF6398;
    'dispatch: loop {
        match pc {
            0x82EF6398 => {
    //   block [0x82EF6398..0x82EF6440)
	// 82EF6398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF63A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF63A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF63A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF63AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF63B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF63B4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF63B8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82EF63BC: 40980024  bge cr6, 0x82ef63e0
	if !ctx.cr[6].lt {
	pc = 0x82EF63E0; continue 'dispatch;
	}
	// 82EF63C0: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82EF63C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF63C8: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF63CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF63D0: 409A0010  bne cr6, 0x82ef63e0
	if !ctx.cr[6].eq {
	pc = 0x82EF63E0; continue 'dispatch;
	}
	// 82EF63D4: 7FCBF92E  stwx r30, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 82EF63D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF63DC: 4BFDED0D  bl 0x82ed50e8
	ctx.lr = 0x82EF63E0;
	sub_82ED50E8(ctx, base);
	// 82EF63E0: 3BFF002C  addi r31, r31, 0x2c
	ctx.r[31].s64 = ctx.r[31].s64 + 44;
	// 82EF63E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF63E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF63EC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF63F0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF63F4: 409A0010  bne cr6, 0x82ef6404
	if !ctx.cr[6].eq {
	pc = 0x82EF6404; continue 'dispatch;
	}
	// 82EF63F8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EF63FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6400: 4BFB0481  bl 0x82ea6880
	ctx.lr = 0x82EF6404;
	sub_82EA6880(ctx, base);
	// 82EF6404: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF640C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6410: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF6414: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82EF6418: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF641C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EF6420: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF6424: 4BFDECC5  bl 0x82ed50e8
	ctx.lr = 0x82EF6428;
	sub_82ED50E8(ctx, base);
	// 82EF6428: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF642C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6434: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF643C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6440 size=200
    let mut pc: u32 = 0x82EF6440;
    'dispatch: loop {
        match pc {
            0x82EF6440 => {
    //   block [0x82EF6440..0x82EF6508)
	// 82EF6440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6444: 482B1D29  bl 0x831a816c
	ctx.lr = 0x82EF6448;
	sub_831A8130(ctx, base);
	// 82EF6448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF644C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6450: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF6454: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF6458: 392BAC60  addi r9, r11, -0x53a0
	ctx.r[9].s64 = ctx.r[11].s64 + -21408;
	// 82EF645C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF6460: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF6464: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6468: 40990028  ble cr6, 0x82ef6490
	if !ctx.cr[6].gt {
	pc = 0x82EF6490; continue 'dispatch;
	}
	// 82EF646C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF6470: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF6474: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF6478: 4BFDEC91  bl 0x82ed5108
	ctx.lr = 0x82EF647C;
	sub_82ED5108(ctx, base);
	// 82EF647C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF6480: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EF6484: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EF6488: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF648C: 4198FFE4  blt cr6, 0x82ef6470
	if ctx.cr[6].lt {
	pc = 0x82EF6470; continue 'dispatch;
	}
	// 82EF6490: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EF6494: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6498: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF649C: 419A0030  beq cr6, 0x82ef64cc
	if ctx.cr[6].eq {
	pc = 0x82EF64CC; continue 'dispatch;
	}
	// 82EF64A0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF64A4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF64A8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF64AC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF64B0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF64B4: 409A0018  bne cr6, 0x82ef64cc
	if !ctx.cr[6].eq {
	pc = 0x82EF64CC; continue 'dispatch;
	}
	// 82EF64B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF64BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF64C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF64C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF64C8: 4E800421  bctrl
	ctx.lr = 0x82EF64CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF64CC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EF64D0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF64D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF64D8: 409A0020  bne cr6, 0x82ef64f8
	if !ctx.cr[6].eq {
	pc = 0x82EF64F8; continue 'dispatch;
	}
	// 82EF64DC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF64E0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EF64E4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EF64E8: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF64EC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF64F0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EF64F4: 4BFAA2BD  bl 0x82ea07b0
	ctx.lr = 0x82EF64F8;
	sub_82EA07B0(ctx, base);
	// 82EF64F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF64FC: 4BFE719D  bl 0x82edd698
	ctx.lr = 0x82EF6500;
	sub_82EDD698(ctx, base);
	// 82EF6500: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6504: 482B1CB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF6508 size=72
    let mut pc: u32 = 0x82EF6508;
    'dispatch: loop {
        match pc {
            0x82EF6508 => {
    //   block [0x82EF6508..0x82EF6550)
	// 82EF6508: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF650C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF6510: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF6514: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82EF6518: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF651C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EF6520: 38CBADBC  addi r6, r11, -0x5244
	ctx.r[6].s64 = ctx.r[11].s64 + -21060;
	// 82EF6524: 98E30008  stb r7, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 82EF6528: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EF652C: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF6530: C009B7B4  lfs f0, -0x484c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-18508 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF6534: C1A89F64  lfs f13, -0x609c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24732 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF6538: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82EF653C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF6540: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82EF6544: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF6548: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82EF654C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF6550 size=180
    let mut pc: u32 = 0x82EF6550;
    'dispatch: loop {
        match pc {
            0x82EF6550 => {
    //   block [0x82EF6550..0x82EF6604)
	// 82EF6550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF655C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6560: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6564: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF6568: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF656C: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82EF6570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6574: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF6578: 4BFAA1B9  bl 0x82ea0730
	ctx.lr = 0x82EF657C;
	sub_82EA0730(ctx, base);
	// 82EF657C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF6580: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF6584: 38E99F74  addi r7, r9, -0x608c
	ctx.r[7].s64 = ctx.r[9].s64 + -24716;
	// 82EF6588: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82EF658C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF6590: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF6594: 38889F88  addi r4, r8, -0x6078
	ctx.r[4].s64 = ctx.r[8].s64 + -24696;
	// 82EF6598: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82EF659C: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82EF65A0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF65A4: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF65A8: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF65AC: 392BADBC  addi r9, r11, -0x5244
	ctx.r[9].s64 = ctx.r[11].s64 + -21060;
	// 82EF65B0: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82EF65B4: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF65B8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF65BC: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF65C0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF65C4: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF65C8: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF65CC: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EF65D0: 80FF0018  lwz r7, 0x18(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF65D4: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82EF65D8: 80DF001C  lwz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF65DC: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82EF65E0: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF65E4: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82EF65E8: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF65EC: 90830024  stw r4, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 82EF65F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF65F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF65F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF65FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6608 size=20
    let mut pc: u32 = 0x82EF6608;
    'dispatch: loop {
        match pc {
            0x82EF6608 => {
    //   block [0x82EF6608..0x82EF661C)
	// 82EF6608: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EF660C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF6610: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82EF6614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF6618: 48011278  b 0x82f07890
	sub_82F07890(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6620 size=88
    let mut pc: u32 = 0x82EF6620;
    'dispatch: loop {
        match pc {
            0x82EF6620 => {
    //   block [0x82EF6620..0x82EF6678)
	// 82EF6620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF662C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6638: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF663C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6640: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6644: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF6648: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF664C: 4E800421  bctrl
	ctx.lr = 0x82EF6650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6650: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 82EF6654: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82EF6658: 913E0010  stw r9, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82EF665C: 911E0014  stw r8, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EF6660: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF666C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6678 size=24
    let mut pc: u32 = 0x82EF6678;
    'dispatch: loop {
        match pc {
            0x82EF6678 => {
    //   block [0x82EF6678..0x82EF6690)
	// 82EF6678: A163001E  lhz r11, 0x1e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 82EF667C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6680: A163001C  lhz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF6684: 394B0034  addi r10, r11, 0x34
	ctx.r[10].s64 = ctx.r[11].s64 + 52;
	// 82EF6688: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF668C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6690 size=64
    let mut pc: u32 = 0x82EF6690;
    'dispatch: loop {
        match pc {
            0x82EF6690 => {
    //   block [0x82EF6690..0x82EF66D0)
	// 82EF6690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6698: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF669C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF66A0: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF66A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF66A8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF66AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF66B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF66B4: 4E800421  bctrl
	ctx.lr = 0x82EF66B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF66B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF66BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF66C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF66C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF66C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF66CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF66D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF66D0 size=8
    let mut pc: u32 = 0x82EF66D0;
    'dispatch: loop {
        match pc {
            0x82EF66D0 => {
    //   block [0x82EF66D0..0x82EF66D8)
	// 82EF66D0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82EF66D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF66D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF66D8 size=208
    let mut pc: u32 = 0x82EF66D8;
    'dispatch: loop {
        match pc {
            0x82EF66D8 => {
    //   block [0x82EF66D8..0x82EF67A8)
	// 82EF66D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF66DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF66E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF66E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF66E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF66EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF66F0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF66F4: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 82EF66F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF66FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF6700: 38E9B7F0  addi r7, r9, -0x4810
	ctx.r[7].s64 = ctx.r[9].s64 + -18448;
	// 82EF6704: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF6708: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF670C: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82EF6710: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF6714: C00889AC  lfs f0, -0x7654(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF6718: B15F000C  sth r10, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u16 ) };
	// 82EF671C: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82EF6720: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82EF6724: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82EF6728: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EF672C: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82EF6730: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6734: A0CB0004  lhz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6738: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EF673C: 419A0010  beq cr6, 0x82ef674c
	if ctx.cr[6].eq {
	pc = 0x82EF674C; continue 'dispatch;
	}
	// 82EF6740: A12B0006  lhz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF6744: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82EF6748: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82EF674C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6750: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF6754: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82EF6758: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82EF675C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6760: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6764: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6768: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF676C: 4E800421  bctrl
	ctx.lr = 0x82EF6770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6770: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF6774: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF6778: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF677C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6780: B11F001C  sth r8, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u16 ) };
	// 82EF6784: B0FF001E  sth r7, 0x1e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[7].u16 ) };
	// 82EF6788: 4BFF6C51  bl 0x82eed3d8
	ctx.lr = 0x82EF678C;
	sub_82EED3D8(ctx, base);
	// 82EF678C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF679C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF67A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF67A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF67A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF67A8 size=100
    let mut pc: u32 = 0x82EF67A8;
    'dispatch: loop {
        match pc {
            0x82EF67A8 => {
    //   block [0x82EF67A8..0x82EF680C)
	// 82EF67A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF67AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF67B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF67B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF67B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF67BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF67C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF67C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF67C8: 392BB7F0  addi r9, r11, -0x4810
	ctx.r[9].s64 = ctx.r[11].s64 + -18448;
	// 82EF67CC: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82EF67D0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF67D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF67D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF67DC: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF67E0: 4BFF6BF9  bl 0x82eed3d8
	ctx.lr = 0x82EF67E4;
	sub_82EED3D8(ctx, base);
	// 82EF67E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF67E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF67EC: 4BFF6BED  bl 0x82eed3d8
	ctx.lr = 0x82EF67F0;
	sub_82EED3D8(ctx, base);
	// 82EF67F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF67F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF67F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF67FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6800: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6810 size=124
    let mut pc: u32 = 0x82EF6810;
    'dispatch: loop {
        match pc {
            0x82EF6810 => {
    //   block [0x82EF6810..0x82EF688C)
	// 82EF6810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF681C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6824: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF6828: 394BB7F0  addi r10, r11, -0x4810
	ctx.r[10].s64 = ctx.r[11].s64 + -18448;
	// 82EF682C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6830: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF6834: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6838: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EF683C: 419A0030  beq cr6, 0x82ef686c
	if ctx.cr[6].eq {
	pc = 0x82EF686C; continue 'dispatch;
	}
	// 82EF6840: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF6844: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF6848: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF684C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF6850: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF6854: 409A0018  bne cr6, 0x82ef686c
	if !ctx.cr[6].eq {
	pc = 0x82EF686C; continue 'dispatch;
	}
	// 82EF6858: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF685C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF6860: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6864: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF6868: 4E800421  bctrl
	ctx.lr = 0x82EF686C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF686C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF6870: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EF6874: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF6878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF687C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF6890 size=668
    let mut pc: u32 = 0x82EF6890;
    'dispatch: loop {
        match pc {
            0x82EF6890 => {
    //   block [0x82EF6890..0x82EF6B2C)
	// 82EF6890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6894: 482B18D1  bl 0x831a8164
	ctx.lr = 0x82EF6898;
	sub_831A8130(ctx, base);
	// 82EF6898: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82EF689C: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82EF68A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF68A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF68A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF68AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF68B0: A17E001C  lhz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF68B4: 837D004C  lwz r27, 0x4c(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF68B8: A39E001E  lhz r28, 0x1e(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(30 as u32) ) } as u64;
	// 82EF68BC: C3CA08A4  lfs f30, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EF68C0: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EF68C4: 7D2BD8AE  lbzx r9, r11, r27
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82EF68C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EF68CC: 409A0230  bne cr6, 0x82ef6afc
	if !ctx.cr[6].eq {
	pc = 0x82EF6AFC; continue 'dispatch;
	}
	// 82EF68D0: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 82EF68D4: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82EF68D8: 41980044  blt cr6, 0x82ef691c
	if ctx.cr[6].lt {
	pc = 0x82EF691C; continue 'dispatch;
	}
	// 82EF68DC: 395CFFFC  addi r10, r28, -4
	ctx.r[10].s64 = ctx.r[28].s64 + -4;
	// 82EF68E0: 397B0010  addi r11, r27, 0x10
	ctx.r[11].s64 = ctx.r[27].s64 + 16;
	// 82EF68E4: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF68E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF68EC: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF68F0: C00BFFF0  lfs f0, -0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF68F4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF68F8: EDA0F83A  fmadds f13, f0, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 82EF68FC: C18BFFF8  lfs f12, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF6900: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF6904: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF6908: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EF690C: ED2C6B3A  fmadds f9, f12, f12, f13
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EF6910: ED0B4AFA  fmadds f8, f11, f11, f9
	ctx.f[8].f64 = (((ctx.f[11].f64 * ctx.f[11].f64 + ctx.f[9].f64) as f32) as f64);
	// 82EF6914: EFEA42BA  fmadds f31, f10, f10, f8
	ctx.f[31].f64 = (((ctx.f[10].f64 * ctx.f[10].f64 + ctx.f[8].f64) as f32) as f64);
	// 82EF6918: 4082FFD8  bne 0x82ef68f0
	if !ctx.cr[0].eq {
	pc = 0x82EF68F0; continue 'dispatch;
	}
	// 82EF691C: 7F09E000  cmpw cr6, r9, r28
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EF6920: 40980024  bge cr6, 0x82ef6944
	if !ctx.cr[6].lt {
	pc = 0x82EF6944; continue 'dispatch;
	}
	// 82EF6924: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF6928: 7D69E050  subf r11, r9, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[9].s64;
	// 82EF692C: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82EF6930: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF6934: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF6938: EFE0F83A  fmadds f31, f0, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 82EF693C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF6940: 4082FFF0  bne 0x82ef6930
	if !ctx.cr[0].eq {
	pc = 0x82EF6930; continue 'dispatch;
	}
	// 82EF6944: C01E0020  lfs f0, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF6948: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EF694C: FF1F6800  fcmpu cr6, f31, f13
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[13].f64);
	// 82EF6950: 409901AC  ble cr6, 0x82ef6afc
	if !ctx.cr[6].gt {
	pc = 0x82EF6AFC; continue 'dispatch;
	}
	// 82EF6954: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF6958: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF695C: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF6960: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF6964: 892B0012  lbz r9, 0x12(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EF6968: 5528063E  clrlwi r8, r9, 0x18
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82EF696C: 550807B8  rlwinm r8, r8, 0, 0x1e, 0x1c
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF6970: 990B0012  stb r8, 0x12(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[8].u8 ) };
	// 82EF6974: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF6978: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF697C: 419A0038  beq cr6, 0x82ef69b4
	if ctx.cr[6].eq {
	pc = 0x82EF69B4; continue 'dispatch;
	}
	// 82EF6980: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF6984: EC00F82C  fsqrts f0, f31
	ctx.f[0].f64 = ((ctx.f[31].f64).sqrt() as f32) as f64;
	// 82EF6988: 895E0024  lbz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF698C: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82EF6990: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82EF6994: 54E3003E  slwi r3, r7, 0
	ctx.r[3].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF6998: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EF699C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF69A0: 9941006C  stb r10, 0x6c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u8 ) };
	// 82EF69A4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF69A8: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF69AC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EF69B0: 4E800421  bctrl
	ctx.lr = 0x82EF69B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF69B4: 897E0025  lbz r11, 0x25(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(37 as u32) ) } as u64;
	// 82EF69B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF69BC: 419A0140  beq cr6, 0x82ef6afc
	if ctx.cr[6].eq {
	pc = 0x82EF6AFC; continue 'dispatch;
	}
	// 82EF69C0: EDA0F82C  fsqrts f13, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[31].f64).sqrt() as f32) as f64;
	// 82EF69C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF69C8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EF69CC: C15F0008  lfs f10, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF69D0: D1410064  stfs f10, 0x64(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82EF69D4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82EF69D8: C19E0020  lfs f12, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF69DC: 3CA08212  lis r5, -0x7dee
	ctx.r[5].s64 = -2112749568;
	// 82EF69E0: C13F000C  lfs f9, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EF69E4: 813D0030  lwz r9, 0x30(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF69E8: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF69EC: 3885FFA0  addi r4, r5, -0x60
	ctx.r[4].s64 = ctx.r[5].s64 + -96;
	// 82EF69F0: D1210068  stfs f9, 0x68(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82EF69F4: 39690010  addi r11, r9, 0x10
	ctx.r[11].s64 = ctx.r[9].s64 + 16;
	// 82EF69F8: C17F0004  lfs f11, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF69FC: 811D0034  lwz r8, 0x34(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EF6A00: D1610060  stfs f11, 0x60(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82EF6A04: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82EF6A08: D3C1006C  stfs f30, 0x6c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82EF6A0C: 39480010  addi r10, r8, 0x10
	ctx.r[10].s64 = ctx.r[8].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF6B30 size=300
    let mut pc: u32 = 0x82EF6B30;
    'dispatch: loop {
        match pc {
            0x82EF6B30 => {
    //   block [0x82EF6B30..0x82EF6C5C)
	// 82EF6B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6B34: 482B1639  bl 0x831a816c
	ctx.lr = 0x82EF6B38;
	sub_831A8130(ctx, base);
	// 82EF6B38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6B3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF6B40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF6B44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF6B48: A15E001C  lhz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF6B4C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EF6B50: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF6B54: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6B58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF6B5C: 409A00C0  bne cr6, 0x82ef6c1c
	if !ctx.cr[6].eq {
	pc = 0x82EF6C1C; continue 'dispatch;
	}
	// 82EF6B60: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF6B64: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EF6B68: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EF6B6C: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF6B70: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EF6B74: C1AA0014  lfs f13, 0x14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF6B78: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EF6B7C: C18A0018  lfs f12, 0x18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF6B80: D18B000C  stfs f12, 0xc(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EF6B84: C1690010  lfs f11, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF6B88: D16B0010  stfs f11, 0x10(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82EF6B8C: C1490014  lfs f10, 0x14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF6B90: D14B0014  stfs f10, 0x14(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82EF6B94: C1290018  lfs f9, 0x18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EF6B98: D12B0018  stfs f9, 0x18(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82EF6B9C: C10A0020  lfs f8, 0x20(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82EF6BA0: D10B001C  stfs f8, 0x1c(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82EF6BA4: C0EA0024  lfs f7, 0x24(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82EF6BA8: D0EB0020  stfs f7, 0x20(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82EF6BAC: C0CA0028  lfs f6, 0x28(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82EF6BB0: D0CB0024  stfs f6, 0x24(r11)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82EF6BB4: C0A90020  lfs f5, 0x20(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82EF6BB8: D0AB0028  stfs f5, 0x28(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82EF6BBC: C0890024  lfs f4, 0x24(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82EF6BC0: D08B002C  stfs f4, 0x2c(r11)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82EF6BC4: C0690028  lfs f3, 0x28(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82EF6BC8: D06B0030  stfs f3, 0x30(r11)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82EF6BCC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6BD0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6BD4: 80E8002C  lwz r7, 0x2c(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF6BD8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82EF6BDC: 4E800421  bctrl
	ctx.lr = 0x82EF6BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6BE0: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF6BE4: A0C30000  lhz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6BE8: 2B060016  cmplwi cr6, r6, 0x16
	ctx.cr[6].compare_u32(ctx.r[6].u32, 22 as u32, &mut ctx.xer);
	// 82EF6BEC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EF6BF0: 409A0018  bne cr6, 0x82ef6c08
	if !ctx.cr[6].eq {
	pc = 0x82EF6C08; continue 'dispatch;
	}
	// 82EF6BF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF6BF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF6BFC: 4801315D  bl 0x82f09d58
	ctx.lr = 0x82EF6C00;
	sub_82F09D58(ctx, base);
	// 82EF6C00: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6C04: 482B15B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EF6C08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF6C0C: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF6C10: 48010E31  bl 0x82f07a40
	ctx.lr = 0x82EF6C14;
	sub_82F07A40(ctx, base);
	// 82EF6C14: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6C18: 482B15A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EF6C1C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82EF6C20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF6C24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C2C: 48010C65  bl 0x82f07890
	ctx.lr = 0x82EF6C30;
	sub_82F07890(ctx, base);
	// 82EF6C30: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF6C34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6C38: 419A001C  beq cr6, 0x82ef6c54
	if ctx.cr[6].eq {
	pc = 0x82EF6C54; continue 'dispatch;
	}
	// 82EF6C3C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF6C40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF6C44: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82EF6C48: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6C4C: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6C50: 4BFDCED9  bl 0x82ed3b28
	ctx.lr = 0x82EF6C54;
	sub_82ED3B28(ctx, base);
	// 82EF6C54: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6C58: 482B1564  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6C60 size=100
    let mut pc: u32 = 0x82EF6C60;
    'dispatch: loop {
        match pc {
            0x82EF6C60 => {
    //   block [0x82EF6C60..0x82EF6CC4)
	// 82EF6C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6C68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF6C6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6C70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6C74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6C78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF6C7C: 4BFFFB95  bl 0x82ef6810
	ctx.lr = 0x82EF6C80;
	sub_82EF6810(ctx, base);
	// 82EF6C80: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EF6C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6C88: 419A0020  beq cr6, 0x82ef6ca8
	if ctx.cr[6].eq {
	pc = 0x82EF6CA8; continue 'dispatch;
	}
	// 82EF6C8C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6C90: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF6C94: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EF6C98: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6C9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF6CA0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF6CA4: 4BFA9B0D  bl 0x82ea07b0
	ctx.lr = 0x82EF6CA8;
	sub_82EA07B0(ctx, base);
	// 82EF6CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6CB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6CBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6CC8 size=16
    let mut pc: u32 = 0x82EF6CC8;
    'dispatch: loop {
        match pc {
            0x82EF6CC8 => {
    //   block [0x82EF6CC8..0x82EF6CD8)
	// 82EF6CC8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EF6CCC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EF6CD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF6CD4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6CD8 size=36
    let mut pc: u32 = 0x82EF6CD8;
    'dispatch: loop {
        match pc {
            0x82EF6CD8 => {
    //   block [0x82EF6CD8..0x82EF6CFC)
	// 82EF6CD8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EF6CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EF6CE0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6CE4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF6CE8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EF6CEC: 90E90030  stw r7, 0x30(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82EF6CF0: 91090040  stw r8, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82EF6CF4: 4082FFEC  bne 0x82ef6ce0
	if !ctx.cr[0].eq {
	pc = 0x82EF6CE0; continue 'dispatch;
	}
	// 82EF6CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF6D00 size=3096
    let mut pc: u32 = 0x82EF6D00;
    'dispatch: loop {
        match pc {
            0x82EF6D00 => {
    //   block [0x82EF6D00..0x82EF7918)
	// 82EF6D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6D04: 482B142D  bl 0x831a8130
	ctx.lr = 0x82EF6D08;
	sub_831A8130(ctx, base);
	// 82EF6D08: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82EF6D0C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82EF6D10: 3981FF50  addi r12, r1, -0xb0
	ctx.r[12].s64 = ctx.r[1].s64 + -176;
	// 82EF6D14: 482B3EF9  bl 0x831aac0c
	ctx.lr = 0x82EF6D18;
	sub_831AA9A0(ctx, base);
	// 82EF6D18: 9421F960  stwu r1, -0x6a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1696 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6D1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF6D20: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF6D24: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EF6D28: C3CB9450  lfs f30, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EF6D2C: EFE007B2  fmuls f31, f0, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82EF6D30: 40990BD0  ble cr6, 0x82ef7900
	if !ctx.cr[6].gt {
	pc = 0x82EF7900; continue 'dispatch;
	}
	// 82EF6D34: 39010180  addi r8, r1, 0x180
	ctx.r[8].s64 = ctx.r[1].s64 + 384;
	// 82EF6D38: 90810068  stw r4, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82EF6D3C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EF6D40: 90A1006C  stw r5, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[5].u32 ) };
	// 82EF6D44: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 82EF6D48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EF6D4C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82EF6D50: 38C9FFA0  addi r6, r9, -0x60
	ctx.r[6].s64 = ctx.r[9].s64 + -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7918 size=248
    let mut pc: u32 = 0x82EF7918;
    'dispatch: loop {
        match pc {
            0x82EF7918 => {
    //   block [0x82EF7918..0x82EF7A10)
	// 82EF7918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF791C: 482B084D  bl 0x831a8168
	ctx.lr = 0x82EF7920;
	sub_831A8130(ctx, base);
	// 82EF7920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7924: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF7928: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF792C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF7930: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82EF7934: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF7938: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82EF793C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF7940: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7944: 7D09F02A  ldx r8, r9, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32)) };
	// 82EF7948: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82EF794C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF7950: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7954: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF7958: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF795C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF7960: 40980018  bge cr6, 0x82ef7978
	if !ctx.cr[6].lt {
	pc = 0x82EF7978; continue 'dispatch;
	}
	// 82EF7964: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF7968: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF796C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7970: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF7974: 4198FFF0  blt cr6, 0x82ef7964
	if ctx.cr[6].lt {
	pc = 0x82EF7964; continue 'dispatch;
	}
	// 82EF7978: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF797C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF7980: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7984: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF7988: 40980018  bge cr6, 0x82ef79a0
	if !ctx.cr[6].lt {
	pc = 0x82EF79A0; continue 'dispatch;
	}
	// 82EF798C: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82EF7990: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82EF7994: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7998: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF799C: 4198FFF0  blt cr6, 0x82ef798c
	if ctx.cr[6].lt {
	pc = 0x82EF798C; continue 'dispatch;
	}
	// 82EF79A0: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EF79A4: 41980040  blt cr6, 0x82ef79e4
	if ctx.cr[6].lt {
	pc = 0x82EF79E4; continue 'dispatch;
	}
	// 82EF79A8: 419A002C  beq cr6, 0x82ef79d4
	if ctx.cr[6].eq {
	pc = 0x82EF79D4; continue 'dispatch;
	}
	// 82EF79AC: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF79B0: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF79B4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF79B8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82EF79BC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF79C0: E8EA0000  ld r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82EF79C4: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF79C8: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF79CC: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82EF79D0: F8EB0000  std r7, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82EF79D4: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82EF79D8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF79DC: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EF79E0: 4099FF70  ble cr6, 0x82ef7950
	if !ctx.cr[6].gt {
	pc = 0x82EF7950; continue 'dispatch;
	}
	// 82EF79E4: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82EF79E8: 40980010  bge cr6, 0x82ef79f8
	if !ctx.cr[6].lt {
	pc = 0x82EF79F8; continue 'dispatch;
	}
	// 82EF79EC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF79F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF79F4: 4BFFFF25  bl 0x82ef7918
	ctx.lr = 0x82EF79F8;
	sub_82EF7918(ctx, base);
	// 82EF79F8: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EF79FC: 4098000C  bge cr6, 0x82ef7a08
	if !ctx.cr[6].lt {
	pc = 0x82EF7A08; continue 'dispatch;
	}
	// 82EF7A00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF7A04: 4BFFFF2C  b 0x82ef7930
	pc = 0x82EF7930; continue 'dispatch;
	// 82EF7A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF7A0C: 482B07AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7A10 size=16
    let mut pc: u32 = 0x82EF7A10;
    'dispatch: loop {
        match pc {
            0x82EF7A10 => {
    //   block [0x82EF7A10..0x82EF7A20)
	// 82EF7A10: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF7A14: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EF7A18: A0830002  lhz r4, 2(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EF7A1C: 4BFF5E24  b 0x82eed840
	sub_82EED840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7A20 size=16
    let mut pc: u32 = 0x82EF7A20;
    'dispatch: loop {
        match pc {
            0x82EF7A20 => {
    //   block [0x82EF7A20..0x82EF7A30)
	// 82EF7A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF7A24: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7A28: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7A30 size=8
    let mut pc: u32 = 0x82EF7A30;
    'dispatch: loop {
        match pc {
            0x82EF7A30 => {
    //   block [0x82EF7A30..0x82EF7A38)
	// 82EF7A30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF7A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7A38 size=12
    let mut pc: u32 = 0x82EF7A38;
    'dispatch: loop {
        match pc {
            0x82EF7A38 => {
    //   block [0x82EF7A38..0x82EF7A44)
	// 82EF7A38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF7A3C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EF7A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7A48 size=8
    let mut pc: u32 = 0x82EF7A48;
    'dispatch: loop {
        match pc {
            0x82EF7A48 => {
    //   block [0x82EF7A48..0x82EF7A50)
	// 82EF7A48: 3860000B  li r3, 0xb
	ctx.r[3].s64 = 11;
	// 82EF7A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7A50 size=200
    let mut pc: u32 = 0x82EF7A50;
    'dispatch: loop {
        match pc {
            0x82EF7A50 => {
    //   block [0x82EF7A50..0x82EF7B18)
	// 82EF7A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7A54: 482B070D  bl 0x831a8160
	ctx.lr = 0x82EF7A58;
	sub_831A8130(ctx, base);
	// 82EF7A58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7A5C: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF7A60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF7A64: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF7A68: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF7A6C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF7A70: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF7A74: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7A78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7A7C: 419A0094  beq cr6, 0x82ef7b10
	if ctx.cr[6].eq {
	pc = 0x82EF7B10; continue 'dispatch;
	}
	// 82EF7A80: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A84: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82EF7A88: 41980088  blt cr6, 0x82ef7b10
	if ctx.cr[6].lt {
	pc = 0x82EF7B10; continue 'dispatch;
	}
	// 82EF7A8C: 2F0B0017  cmpwi cr6, r11, 0x17
	ctx.cr[6].compare_i32(ctx.r[11].s32, 23, &mut ctx.xer);
	// 82EF7A90: 419A005C  beq cr6, 0x82ef7aec
	if ctx.cr[6].eq {
	pc = 0x82EF7AEC; continue 'dispatch;
	}
	// 82EF7A94: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82EF7A98: 419A0030  beq cr6, 0x82ef7ac8
	if ctx.cr[6].eq {
	pc = 0x82EF7AC8; continue 'dispatch;
	}
	// 82EF7A9C: 2F0B001A  cmpwi cr6, r11, 0x1a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 26, &mut ctx.xer);
	// 82EF7AA0: 409A0068  bne cr6, 0x82ef7b08
	if !ctx.cr[6].eq {
	pc = 0x82EF7B08; continue 'dispatch;
	}
	// 82EF7AA4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF7AA8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF7AAC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF7AB0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF7AB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7ABC: 4BFF5B1D  bl 0x82eed5d8
	ctx.lr = 0x82EF7AC0;
	sub_82EED5D8(ctx, base);
	// 82EF7AC0: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7AC4: 4BFFFFBC  b 0x82ef7a80
	pc = 0x82EF7A80; continue 'dispatch;
	// 82EF7AC8: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF7ACC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF7AD0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF7AD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF7AD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7AE0: 4BFF5911  bl 0x82eed3f0
	ctx.lr = 0x82EF7AE4;
	sub_82EED3F0(ctx, base);
	// 82EF7AE4: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7AE8: 4BFFFF98  b 0x82ef7a80
	pc = 0x82EF7A80; continue 'dispatch;
	// 82EF7AEC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF7AF0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF7AF4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF7AF8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF7AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7B04: 4BFF5A95  bl 0x82eed598
	ctx.lr = 0x82EF7B08;
	sub_82EED598(ctx, base);
	// 82EF7B08: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7B0C: 4BFFFF74  b 0x82ef7a80
	pc = 0x82EF7A80; continue 'dispatch;
	// 82EF7B10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF7B14: 482B069C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7B18 size=224
    let mut pc: u32 = 0x82EF7B18;
    'dispatch: loop {
        match pc {
            0x82EF7B18 => {
    //   block [0x82EF7B18..0x82EF7BF8)
	// 82EF7B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7B1C: 482B0645  bl 0x831a8160
	ctx.lr = 0x82EF7B20;
	sub_831A8130(ctx, base);
	// 82EF7B20: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82EF7B24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7B28: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF7B2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF7B30: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EF7B34: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF7B38: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EF7B3C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EF7B40: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82EF7B44: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7B48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7B4C: 419A00A0  beq cr6, 0x82ef7bec
	if ctx.cr[6].eq {
	pc = 0x82EF7BEC; continue 'dispatch;
	}
	// 82EF7B50: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B54: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82EF7B58: 41980094  blt cr6, 0x82ef7bec
	if ctx.cr[6].lt {
	pc = 0x82EF7BEC; continue 'dispatch;
	}
	// 82EF7B5C: 2F0B0017  cmpwi cr6, r11, 0x17
	ctx.cr[6].compare_i32(ctx.r[11].s32, 23, &mut ctx.xer);
	// 82EF7B60: 419A0064  beq cr6, 0x82ef7bc4
	if ctx.cr[6].eq {
	pc = 0x82EF7BC4; continue 'dispatch;
	}
	// 82EF7B64: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82EF7B68: 419A0034  beq cr6, 0x82ef7b9c
	if ctx.cr[6].eq {
	pc = 0x82EF7B9C; continue 'dispatch;
	}
	// 82EF7B6C: 2F0B001A  cmpwi cr6, r11, 0x1a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 26, &mut ctx.xer);
	// 82EF7B70: 409A0074  bne cr6, 0x82ef7be4
	if !ctx.cr[6].eq {
	pc = 0x82EF7BE4; continue 'dispatch;
	}
	// 82EF7B74: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82EF7B78: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF7B7C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EF7B80: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EF7B84: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EF7B88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7B90: 4BFF5A91  bl 0x82eed620
	ctx.lr = 0x82EF7B94;
	sub_82EED620(ctx, base);
	// 82EF7B94: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7B98: 4BFFFFB8  b 0x82ef7b50
	pc = 0x82EF7B50; continue 'dispatch;
	// 82EF7B9C: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82EF7BA0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF7BA4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EF7BA8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EF7BAC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EF7BB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7BB8: 4BFF58D1  bl 0x82eed488
	ctx.lr = 0x82EF7BBC;
	sub_82EED488(ctx, base);
	// 82EF7BBC: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7BC0: 4BFFFF90  b 0x82ef7b50
	pc = 0x82EF7B50; continue 'dispatch;
	// 82EF7BC4: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82EF7BC8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EF7BCC: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EF7BD0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EF7BD4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EF7BD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7BDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7BE0: 4BFF58B1  bl 0x82eed490
	ctx.lr = 0x82EF7BE4;
	sub_82EED490(ctx, base);
	// 82EF7BE4: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7BE8: 4BFFFF68  b 0x82ef7b50
	pc = 0x82EF7B50; continue 'dispatch;
	// 82EF7BEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF7BF0: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82EF7BF4: 482B05BC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF7BF8 size=388
    let mut pc: u32 = 0x82EF7BF8;
    'dispatch: loop {
        match pc {
            0x82EF7BF8 => {
    //   block [0x82EF7BF8..0x82EF7D7C)
	// 82EF7BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7BFC: 482B0561  bl 0x831a815c
	ctx.lr = 0x82EF7C00;
	sub_831A8130(ctx, base);
	// 82EF7C00: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7C04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF7C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF7C0C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EF7C10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF7C14: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF7C18: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF7C1C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EF7C20: A12B0010  lhz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7C24: A3EB0004  lhz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7C28: 61280004  ori r8, r9, 4
	ctx.r[8].u64 = ctx.r[9].u64 | 4;
	// 82EF7C2C: B10B0010  sth r8, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u16 ) };
	// 82EF7C30: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82EF7C34: 835D0028  lwz r26, 0x28(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF7C38: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF7C3C: 4800131D  bl 0x82ef8f58
	ctx.lr = 0x82EF7C40;
	sub_82EF8F58(ctx, base);
	// 82EF7C40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF7C44: 57EB283E  rotlwi r11, r31, 5
	ctx.r[11].u64 = ((ctx.r[31].u32).rotate_left(5)) as u64;
	// 82EF7C48: 93DD0028  stw r30, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82EF7C4C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82EF7C50: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF7C54: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EF7C58: A0BE0006  lhz r5, 6(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF7C5C: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82EF7C60: 889E000A  lbz r4, 0xa(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82EF7C64: 7D44F9D6  mullw r10, r4, r31
	ctx.r[10].s64 = (ctx.r[4].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82EF7C68: 54A9283E  rotlwi r9, r5, 5
	ctx.r[9].u64 = ((ctx.r[5].u32).rotate_left(5)) as u64;
	// 82EF7C6C: C00708A4  lfs f0, 0x8a4(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF7C70: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EF7C74: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF7C78: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF7C7C: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82EF7C80: 7D445850  subf r10, r4, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82EF7C84: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EF7C88: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EF7C8C: 98CB000F  stb r6, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[6].u8 ) };
	// 82EF7C90: 40990028  ble cr6, 0x82ef7cb8
	if !ctx.cr[6].gt {
	pc = 0x82EF7CB8; continue 'dispatch;
	}
	// 82EF7C94: 892A000F  lbz r9, 0xf(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(15 as u32) ) } as u64;
	// 82EF7C98: 552707BC  rlwinm r7, r9, 0, 0x1e, 0x1e
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF7C9C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF7CA0: 409A0018  bne cr6, 0x82ef7cb8
	if !ctx.cr[6].eq {
	pc = 0x82EF7CB8; continue 'dispatch;
	}
	// 82EF7CA4: 894A000E  lbz r10, 0xe(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(14 as u32) ) } as u64;
	// 82EF7CA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF7CAC: 409A000C  bne cr6, 0x82ef7cb8
	if !ctx.cr[6].eq {
	pc = 0x82EF7CB8; continue 'dispatch;
	}
	// 82EF7CB0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82EF7CB4: 994B000F  stb r10, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82EF7CB8: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82EF7CBC: 911C0000  stw r8, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF7CC0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EF7CC8: 554B0FBC  rlwinm r11, r10, 1, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EF7CCC: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82EF7CD0: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82EF7CD4: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF7CD8: 1D6A0070  mulli r11, r10, 0x70
	ctx.r[11].s64 = ctx.r[10].s64 * 112;
	// 82EF7CDC: 54EA2036  slwi r10, r7, 4
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7CE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EF7CE4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF7CEC: 38CB00A0  addi r6, r11, 0xa0
	ctx.r[6].s64 = ctx.r[11].s64 + 160;
	// 82EF7CF0: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 82EF7CF4: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82EF7CF8: 409A0010  bne cr6, 0x82ef7d08
	if !ctx.cr[6].eq {
	pc = 0x82EF7D08; continue 'dispatch;
	}
	// 82EF7CFC: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82EF7D00: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EF7D04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF7D08: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF7D0C: 39270001  addi r9, r7, 1
	ctx.r[9].s64 = ctx.r[7].s64 + 1;
	// 82EF7D10: 39680030  addi r11, r8, 0x30
	ctx.r[11].s64 = ctx.r[8].s64 + 48;
	// 82EF7D14: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82EF7D18: 409A0008  bne cr6, 0x82ef7d20
	if !ctx.cr[6].eq {
	pc = 0x82EF7D20; continue 'dispatch;
	}
	// 82EF7D1C: 39680040  addi r11, r8, 0x40
	ctx.r[11].s64 = ctx.r[8].s64 + 64;
	// 82EF7D20: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82EF7D24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF7D28: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82EF7D2C: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF7D30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82EF7D34: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EF7D38: 4BFEB0F9  bl 0x82ee2e30
	ctx.lr = 0x82EF7D3C;
	sub_82EE2E30(ctx, base);
	// 82EF7D3C: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF7D40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7D44: 419A0020  beq cr6, 0x82ef7d64
	if ctx.cr[6].eq {
	pc = 0x82EF7D64; continue 'dispatch;
	}
	// 82EF7D48: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF7D4C: A0DE0002  lhz r6, 2(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EF7D50: 80BD0028  lwz r5, 0x28(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF7D54: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF7D58: 480004F9  bl 0x82ef8250
	ctx.lr = 0x82EF7D5C;
	sub_82EF8250(ctx, base);
	// 82EF7D5C: A17E0002  lhz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EF7D60: 917D002C  stw r11, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EF7D64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF7D68: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82EF7D6C: 4800044D  bl 0x82ef81b8
	ctx.lr = 0x82EF7D70;
	sub_82EF81B8(ctx, base);
	// 82EF7D70: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82EF7D74: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EF7D78: 482B0434  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF7D80 size=864
    let mut pc: u32 = 0x82EF7D80;
    'dispatch: loop {
        match pc {
            0x82EF7D80 => {
    //   block [0x82EF7D80..0x82EF80E0)
	// 82EF7D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7D84: 482B03AD  bl 0x831a8130
	ctx.lr = 0x82EF7D88;
	sub_831A8130(ctx, base);
	// 82EF7D88: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82EF7D8C: 482B0CDD  bl 0x831a8a68
	ctx.lr = 0x82EF7D90;
	sub_831A8A40(ctx, base);
	// 82EF7D90: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7D94: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82EF7D98: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF7D9C: 8925000A  lbz r9, 0xa(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(10 as u32) ) } as u64;
	// 82EF7DA0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82EF7DA4: 556B283E  rotlwi r11, r11, 5
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(5)) as u64;
	// 82EF7DA8: A1450004  lhz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7DAC: 3A370048  addi r17, r23, 0x48
	ctx.r[17].s64 = ctx.r[23].s64 + 72;
	// 82EF7DB0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82EF7DB4: 81F70048  lwz r15, 0x48(r23)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF7DB8: 3BC50030  addi r30, r5, 0x30
	ctx.r[30].s64 = ctx.r[5].s64 + 48;
	// 82EF7DBC: 364AFFFF  addic. r18, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[18].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[18].s32, 0, &mut ctx.xer);
	// 82EF7DC0: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82EF7DC4: 3BAB0030  addi r29, r11, 0x30
	ctx.r[29].s64 = ctx.r[11].s64 + 48;
	// 82EF7DC8: 83EF0014  lwz r31, 0x14(r15)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7DCC: 838F0018  lwz r28, 0x18(r15)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7DD0: 418002F0  blt 0x82ef80c0
	if ctx.cr[0].lt {
	pc = 0x82EF80C0; continue 'dispatch;
	}
	// 82EF7DD4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82EF7DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82EF7DDC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EF7DE0: 3D008207  lis r8, -0x7df9
	ctx.r[8].s64 = -2113470464;
	// 82EF7DE4: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82EF7DE8: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82EF7DEC: C34B95F8  lfs f26, -0x6a08(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27144 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 82EF7DF0: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82EF7DF4: C36A2534  lfs f27, 0x2534(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9524 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82EF7DF8: 3C808200  lis r4, -0x7e00
	ctx.r[4].s64 = -2113929216;
	// 82EF7DFC: C3C908A8  lfs f30, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EF7E00: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EF7E04: C3884D58  lfs f28, 0x4d58(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19800 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82EF7E08: C327BA78  lfs f25, -0x4588(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82EF7E0C: 3A800040  li r20, 0x40
	ctx.r[20].s64 = 64;
	// 82EF7E10: C3067BC8  lfs f24, 0x7bc8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(31688 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 82EF7E14: 3AA001A0  li r21, 0x1a0
	ctx.r[21].s64 = 416;
	// 82EF7E18: C3E5B184  lfs f31, -0x4e7c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-20092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF7E1C: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 82EF7E20: C3A408A4  lfs f29, 0x8a4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82EF7E24: 3A000190  li r16, 0x190
	ctx.r[16].s64 = 400;
	// 82EF7E28: 39CB0028  addi r14, r11, 0x28
	ctx.r[14].s64 = ctx.r[11].s64 + 40;
	// 82EF7E2C: 397D000F  addi r11, r29, 0xf
	ctx.r[11].s64 = ctx.r[29].s64 + 15;
	// 82EF7E30: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF7E34: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7E38: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82EF7E3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF7E40: 419A026C  beq cr6, 0x82ef80ac
	if ctx.cr[6].eq {
	pc = 0x82EF80AC; continue 'dispatch;
	}
	// 82EF7E44: 81370030  lwz r9, 0x30(r23)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(48 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF80E0 size=212
    let mut pc: u32 = 0x82EF80E0;
    'dispatch: loop {
        match pc {
            0x82EF80E0 => {
    //   block [0x82EF80E0..0x82EF81B4)
	// 82EF80E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF80E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF80E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF80EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF80F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF80F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF80F8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82EF80FC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82EF8100: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EF8104: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EF8108: 390AB1B8  addi r8, r10, -0x4e48
	ctx.r[8].s64 = ctx.r[10].s64 + -20040;
	// 82EF810C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF8110: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EF8114: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF8118: 612B0008  ori r11, r9, 8
	ctx.r[11].u64 = ctx.r[9].u64 | 8;
	// 82EF811C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EF8120: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82EF8124: 39401400  li r10, 0x1400
	ctx.r[10].s64 = 5120;
	// 82EF8128: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EF812C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF8130: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF8134: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EF8138: 909F0024  stw r4, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 82EF813C: 888500BD  lbz r4, 0xbd(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(189 as u32) ) } as u64;
	// 82EF8140: 88A600BD  lbz r5, 0xbd(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(189 as u32) ) } as u64;
	// 82EF8144: 7D652214  add r11, r5, r4
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[4].u64;
	// 82EF8148: 392B0005  addi r9, r11, 5
	ctx.r[9].s64 = ctx.r[11].s64 + 5;
	// 82EF814C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF8150: 7CEA43D6  divw r7, r10, r8
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[8].s32;
	// 82EF8154: 0CC80000  twi 6, r8, 0
	// 82EF8158: 54E6043E  clrlwi r6, r7, 0x10
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82EF815C: 48000C7D  bl 0x82ef8dd8
	ctx.lr = 0x82EF8160;
	sub_82EF8DD8(ctx, base);
	// 82EF8160: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82EF8164: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82EF8168: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82EF816C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82EF8170: B3C30010  sth r30, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[30].u16 ) };
	// 82EF8174: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF8178: B0C30012  sth r6, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[6].u16 ) };
	// 82EF817C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF8180: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EF8184: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF8188: 4200FFF8  bdnz 0x82ef8180
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF8180; continue 'dispatch;
	}
	// 82EF818C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF8190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8194: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EF8198: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82EF819C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF81A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF81A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF81A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF81AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF81B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF81B8 size=152
    let mut pc: u32 = 0x82EF81B8;
    'dispatch: loop {
        match pc {
            0x82EF81B8 => {
    //   block [0x82EF81B8..0x82EF8250)
	// 82EF81B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF81BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF81C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF81C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF81C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF81CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF81D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF81D4: 346AFFFF  addic. r3, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[3].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EF81D8: 4180001C  blt 0x82ef81f4
	if ctx.cr[0].lt {
	pc = 0x82EF81F4; continue 'dispatch;
	}
	// 82EF81DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF81E0: 7D2B18AE  lbzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82EF81E4: 2B0900FF  cmplwi cr6, r9, 0xff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 255 as u32, &mut ctx.xer);
	// 82EF81E8: 419A0060  beq cr6, 0x82ef8248
	if ctx.cr[6].eq {
	pc = 0x82EF8248; continue 'dispatch;
	}
	// 82EF81EC: 3463FFFF  addic. r3, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EF81F0: 4080FFF0  bge 0x82ef81e0
	if !ctx.cr[0].lt {
	pc = 0x82EF81E0; continue 'dispatch;
	}
	// 82EF81F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF81F8: 549E063E  clrlwi r30, r4, 0x18
	ctx.r[30].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82EF81FC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF8200: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF8204: 409A0010  bne cr6, 0x82ef8214
	if !ctx.cr[6].eq {
	pc = 0x82EF8214; continue 'dispatch;
	}
	// 82EF8208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF820C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8210: 4BFAE671  bl 0x82ea6880
	ctx.lr = 0x82EF8214;
	sub_82EA6880(ctx, base);
	// 82EF8214: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8218: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF821C: 7FCA59AE  stbx r30, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u8) };
	// 82EF8220: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8224: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF8228: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF822C: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82EF8230: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF823C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8244: 4E800020  blr
	return;
	// 82EF8248: 7C8B19AE  stbx r4, r11, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u8) };
	// 82EF824C: 4BFFFFE4  b 0x82ef8230
	pc = 0x82EF8230; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8250 size=36
    let mut pc: u32 = 0x82EF8250;
    'dispatch: loop {
        match pc {
            0x82EF8250 => {
    //   block [0x82EF8250..0x82EF8274)
	// 82EF8250: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8254: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF8258: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF825C: 419A0010  beq cr6, 0x82ef826c
	if ctx.cr[6].eq {
	pc = 0x82EF826C; continue 'dispatch;
	}
	// 82EF8260: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EF8264: 409A001C  bne cr6, 0x82ef8280
	if !ctx.cr[6].eq {
		sub_82EF8280(ctx, base);
		return;
	}
	// 82EF8268: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82EF826C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8270: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8274 size=12
    let mut pc: u32 = 0x82EF8274;
    'dispatch: loop {
        match pc {
            0x82EF8274 => {
    //   block [0x82EF8274..0x82EF8280)
	// 82EF8274: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82EF8278: B0CB0010  sth r6, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u16 ) };
	// 82EF827C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8280 size=40
    let mut pc: u32 = 0x82EF8280;
    'dispatch: loop {
        match pc {
            0x82EF8280 => {
    //   block [0x82EF8280..0x82EF82A8)
	// 82EF8280: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8284: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EF8288: 419A0014  beq cr6, 0x82ef829c
	if ctx.cr[6].eq {
	pc = 0x82EF829C; continue 'dispatch;
	}
	// 82EF828C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82EF8290: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8294: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EF8298: 409AFFF4  bne cr6, 0x82ef828c
	if !ctx.cr[6].eq {
	pc = 0x82EF828C; continue 'dispatch;
	}
	// 82EF829C: 90AA0014  stw r5, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82EF82A0: B0CA0012  sth r6, 0x12(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(18 as u32), ctx.r[6].u16 ) };
	// 82EF82A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF82A8 size=476
    let mut pc: u32 = 0x82EF82A8;
    'dispatch: loop {
        match pc {
            0x82EF82A8 => {
    //   block [0x82EF82A8..0x82EF8484)
	// 82EF82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF82AC: 482AFEB9  bl 0x831a8164
	ctx.lr = 0x82EF82B0;
	sub_831A8130(ctx, base);
	// 82EF82B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF82B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF82B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF82BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF82C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF82C4: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF82C8: 83FD0028  lwz r31, 0x28(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF82CC: 895E0025  lbz r10, 0x25(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(37 as u32) ) } as u64;
	// 82EF82D0: 516A3032  rlwimi r10, r11, 6, 0, 0x19
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[10].u64 & 0xFFFFFFFF0000003F);
	// 82EF82D4: 995E0025  stb r10, 0x25(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 82EF82D8: A13F0016  lhz r9, 0x16(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82EF82DC: A11F0018  lhz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF82E0: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF82E4: A0FF0014  lhz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF82E8: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82EF82EC: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82EF82F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82EF82F4: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82EF82F8: 4BFE3EC9  bl 0x82edc1c0
	ctx.lr = 0x82EF82FC;
	sub_82EDC1C0(ctx, base);
	// 82EF82FC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF8300: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF8304: 38CBFFFF  addi r6, r11, -1
	ctx.r[6].s64 = ctx.r[11].s64 + -1;
	// 82EF8308: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82EF830C: 90DE001C  stw r6, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82EF8310: 939D0008  stw r28, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82EF8314: 88BF001A  lbz r5, 0x1a(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EF8318: 20850002  subfic r4, r5, 2
	ctx.xer.ca = ctx.r[5].u32 <= 2 as u32;
	ctx.r[4].s64 = (2 as i64) - ctx.r[5].s64;
	// 82EF831C: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF8320: A11F0022  lhz r8, 0x22(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(34 as u32) ) } as u64;
	// 82EF8324: 5507103E  rotlwi r7, r8, 2
	ctx.r[7].u64 = ((ctx.r[8].u32).rotate_left(2)) as u64;
	// 82EF8328: 7D63F82E  lwzx r11, r3, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF832C: 814B00A0  lwz r10, 0xa0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EF8330: 80CB00A4  lwz r6, 0xa4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF8334: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF8338: 7CA95214  add r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF833C: 8085FFFC  lwz r4, -4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF8340: 7C87512E  stwx r4, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82EF8344: 814B00A4  lwz r10, 0xa4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF8348: 386AFFFF  addi r3, r10, -1
	ctx.r[3].s64 = ctx.r[10].s64 + -1;
	// 82EF834C: 906B00A4  stw r3, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF8350: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF8354: B10B0022  sth r8, 0x22(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(34 as u32), ctx.r[8].u16 ) };
	// 82EF8358: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF835C: 897F001A  lbz r11, 0x1a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EF8360: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF8364: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF8368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF836C: 7FC9F82E  lwzx r30, r9, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF8370: 419A003C  beq cr6, 0x82ef83ac
	if ctx.cr[6].eq {
	pc = 0x82EF83AC; continue 'dispatch;
	}
	// 82EF8374: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF8378: 395E00AC  addi r10, r30, 0xac
	ctx.r[10].s64 = ctx.r[30].s64 + 172;
	// 82EF837C: 813E00B0  lwz r9, 0xb0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF8380: 7C8B1A14  add r4, r11, r3
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EF8384: 815E00AC  lwz r10, 0xac(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF8388: 7F6B00D0  neg r27, r11
	ctx.r[27].s64 = -ctx.r[11].s64;
	// 82EF838C: 7D644850  subf r11, r4, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 82EF8390: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8394: 4BFB23BD  bl 0x82eaa750
	ctx.lr = 0x82EF8398;
	sub_82EAA750(ctx, base);
	// 82EF8398: 811E00B0  lwz r8, 0xb0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF839C: A0FF0020  lhz r7, 0x20(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF83A0: 7CC74050  subf r6, r7, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82EF83A4: 90DE00B0  stw r6, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[6].u32 ) };
	// 82EF83A8: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82EF83AC: 939D0028  stw r28, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82EF83B0: 38DE0098  addi r6, r30, 0x98
	ctx.r[6].s64 = ctx.r[30].s64 + 152;
	// 82EF83B4: 813E0098  lwz r9, 0x98(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EF83B8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EF83BC: A15E009C  lhz r10, 0x9c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF83C0: 5548083E  rotlwi r8, r10, 1
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82EF83C4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82EF83C8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF83CC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF83D0: 38AAFFD0  addi r5, r10, -0x30
	ctx.r[5].s64 = ctx.r[10].s64 + -48;
	// 82EF83D4: 7F1F2840  cmplw cr6, r31, r5
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF83D8: 40980058  bge cr6, 0x82ef8430
	if !ctx.cr[6].lt {
	pc = 0x82EF8430; continue 'dispatch;
	}
	// 82EF83DC: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 82EF83E0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EF83E4: 39470014  addi r10, r7, 0x14
	ctx.r[10].s64 = ctx.r[7].s64 + 20;
	// 82EF83E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82EF83EC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EF83F0: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82EF83F4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF83F8: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82EF83FC: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EF8400: 4200FFF0  bdnz 0x82ef83f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF83F0; continue 'dispatch;
	}
	// 82EF8404: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8408: 916A0028  stw r11, 0x28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EF840C: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8410: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF8414: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82EF8418: 409A0008  bne cr6, 0x82ef8420
	if !ctx.cr[6].eq {
	pc = 0x82EF8420; continue 'dispatch;
	}
	// 82EF841C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82EF8420: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82EF8424: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF8428: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF842C: 4198FFB0  blt cr6, 0x82ef83dc
	if ctx.cr[6].lt {
	pc = 0x82EF83DC; continue 'dispatch;
	}
	// 82EF8430: A1660004  lhz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8434: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 82EF8438: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EF843C: B1460004  sth r10, 4(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82EF8440: A11D0004  lhz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8444: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF8448: 419A0034  beq cr6, 0x82ef847c
	if ctx.cr[6].eq {
	pc = 0x82EF847C; continue 'dispatch;
	}
	// 82EF844C: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8450: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EF8454: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EF8458: B13D0006  sth r9, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EF845C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF8460: 409A001C  bne cr6, 0x82ef847c
	if !ctx.cr[6].eq {
	pc = 0x82EF847C; continue 'dispatch;
	}
	// 82EF8464: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8468: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF846C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8470: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8474: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF8478: 4E800421  bctrl
	ctx.lr = 0x82EF847C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF847C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8480: 482AFD34  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8488 size=156
    let mut pc: u32 = 0x82EF8488;
    'dispatch: loop {
        match pc {
            0x82EF8488 => {
    //   block [0x82EF8488..0x82EF8524)
	// 82EF8488: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EF848C: 9081FFF4  stw r4, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[4].u32 ) };
	// 82EF8490: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8494: 9061FFF0  stw r3, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[3].u32 ) };
	// 82EF8498: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82EF849C: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
	// 82EF84A0: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF84A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF84A8: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF84AC: A109009C  lhz r8, 0x9c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF84B0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EF84B4: 40990034  ble cr6, 0x82ef84e8
	if !ctx.cr[6].gt {
	pc = 0x82EF84E8; continue 'dispatch;
	}
	// 82EF84B8: 81690098  lwz r11, 0x98(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EF84BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF84C0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF84C4: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF84C8: 7C63FA78  xor r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 ^ ctx.r[31].u64;
	// 82EF84CC: 7C634A78  xor r3, r3, r9
	ctx.r[3].u64 = ctx.r[3].u64 ^ ctx.r[9].u64;
	// 82EF84D0: 7F033840  cmplw cr6, r3, r7
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF84D4: 419A0034  beq cr6, 0x82ef8508
	if ctx.cr[6].eq {
	pc = 0x82EF8508; continue 'dispatch;
	}
	// 82EF84D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF84DC: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82EF84E0: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EF84E4: 4198FFDC  blt cr6, 0x82ef84c0
	if ctx.cr[6].lt {
	pc = 0x82EF84C0; continue 'dispatch;
	}
	// 82EF84E8: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82EF84EC: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82EF84F0: 38A5FFFC  addi r5, r5, -4
	ctx.r[5].s64 = ctx.r[5].s64 + -4;
	// 82EF84F4: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82EF84F8: 4198FFA8  blt cr6, 0x82ef84a0
	if ctx.cr[6].lt {
	pc = 0x82EF84A0; continue 'dispatch;
	}
	// 82EF84FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF8500: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EF8504: 4E800020  blr
	return;
	// 82EF8508: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF850C: 81290098  lwz r9, 0x98(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EF8510: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF8514: 55072036  slwi r7, r8, 4
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF8518: 7C69382E  lwzx r3, r9, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82EF851C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EF8520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8528 size=272
    let mut pc: u32 = 0x82EF8528;
    'dispatch: loop {
        match pc {
            0x82EF8528 => {
    //   block [0x82EF8528..0x82EF8638)
	// 82EF8528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF852C: 482AFC39  bl 0x831a8164
	ctx.lr = 0x82EF8530;
	sub_831A8130(ctx, base);
	// 82EF8530: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8534: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EF8538: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF853C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF8540: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF8544: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8548: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF854C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF8550: 4E800421  bctrl
	ctx.lr = 0x82EF8554;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8554: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8558: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF855C: 419A00D4  beq cr6, 0x82ef8630
	if ctx.cr[6].eq {
	pc = 0x82EF8630; continue 'dispatch;
	}
	// 82EF8560: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EF8568: 839E0028  lwz r28, 0x28(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF856C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EF8570: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8574: 409A0034  bne cr6, 0x82ef85a8
	if !ctx.cr[6].eq {
	pc = 0x82EF85A8; continue 'dispatch;
	}
	// 82EF8578: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF857C: 419A0010  beq cr6, 0x82ef858c
	if ctx.cr[6].eq {
	pc = 0x82EF858C; continue 'dispatch;
	}
	// 82EF8580: 917C000C  stw r11, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF8584: A15F0012  lhz r10, 0x12(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EF8588: B15C0010  sth r10, 0x10(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 82EF858C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8590: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82EF8594: 4198000C  blt cr6, 0x82ef85a0
	if ctx.cr[6].lt {
	pc = 0x82EF85A0; continue 'dispatch;
	}
	// 82EF8598: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EF859C: 48000044  b 0x82ef85e0
	pc = 0x82EF85E0; continue 'dispatch;
	// 82EF85A0: 911E0010  stw r8, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82EF85A4: 4800003C  b 0x82ef85e0
	pc = 0x82EF85E0; continue 'dispatch;
	// 82EF85A8: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82EF85AC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF85B0: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82EF85B4: 4198007C  blt cr6, 0x82ef8630
	if ctx.cr[6].lt {
	pc = 0x82EF8630; continue 'dispatch;
	}
	// 82EF85B8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82EF85BC: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EF85C0: 419A0010  beq cr6, 0x82ef85d0
	if ctx.cr[6].eq {
	pc = 0x82EF85D0; continue 'dispatch;
	}
	// 82EF85C4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82EF85C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF85CC: 4BFFFFE0  b 0x82ef85ac
	pc = 0x82EF85AC; continue 'dispatch;
	// 82EF85D0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF85D4: 91490014  stw r10, 0x14(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF85D8: A0EB0012  lhz r7, 0x12(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EF85DC: B0E90012  sth r7, 0x12(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(18 as u32), ctx.r[7].u16 ) };
	// 82EF85E0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF85E4: 419A0030  beq cr6, 0x82ef8614
	if ctx.cr[6].eq {
	pc = 0x82EF8614; continue 'dispatch;
	}
	// 82EF85E8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82EF85EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF85F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82EF85F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF85F8: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82EF85FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82EF8600: 4BFF5101  bl 0x82eed700
	ctx.lr = 0x82EF8604;
	sub_82EED700(ctx, base);
	// 82EF8604: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF8608: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF860C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8610: 4BFE3BB1  bl 0x82edc1c0
	ctx.lr = 0x82EF8614;
	sub_82EDC1C0(ctx, base);
	// 82EF8614: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8618: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF861C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EF8620: A0BF0010  lhz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8624: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF8628: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF862C: 4BFA8185  bl 0x82ea07b0
	ctx.lr = 0x82EF8630;
	sub_82EA07B0(ctx, base);
	// 82EF8630: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8634: 482AFB80  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8638 size=60
    let mut pc: u32 = 0x82EF8638;
    'dispatch: loop {
        match pc {
            0x82EF8638 => {
    //   block [0x82EF8638..0x82EF8674)
	// 82EF8638: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF863C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8640: 419A0034  beq cr6, 0x82ef8674
	if ctx.cr[6].eq {
		sub_82EF8674(ctx, base);
		return;
	}
	// 82EF8644: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8648: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82EF864C: 419A0020  beq cr6, 0x82ef866c
	if ctx.cr[6].eq {
	pc = 0x82EF866C; continue 'dispatch;
	}
	// 82EF8650: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8654: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8658: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82EF865C: 41980018  blt cr6, 0x82ef8674
	if ctx.cr[6].lt {
		sub_82EF8674(ctx, base);
		return;
	}
	// 82EF8660: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82EF8664: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82EF8668: 409AFFE8  bne cr6, 0x82ef8650
	if !ctx.cr[6].eq {
	pc = 0x82EF8650; continue 'dispatch;
	}
	// 82EF866C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF8670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8674 size=8
    let mut pc: u32 = 0x82EF8674;
    'dispatch: loop {
        match pc {
            0x82EF8674 => {
    //   block [0x82EF8674..0x82EF867C)
	// 82EF8674: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF8678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8680 size=16
    let mut pc: u32 = 0x82EF8680;
    'dispatch: loop {
        match pc {
            0x82EF8680 => {
    //   block [0x82EF8680..0x82EF8690)
	// 82EF8680: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8684: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8688: 409A0008  bne cr6, 0x82ef8690
	if !ctx.cr[6].eq {
		sub_82EF8690(ctx, base);
		return;
	}
	// 82EF868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8690 size=16
    let mut pc: u32 = 0x82EF8690;
    'dispatch: loop {
        match pc {
            0x82EF8690 => {
    //   block [0x82EF8690..0x82EF86A0)
	// 82EF8690: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8694: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8698: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82EF869C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF86A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF86A0 size=8
    let mut pc: u32 = 0x82EF86A0;
    'dispatch: loop {
        match pc {
            0x82EF86A0 => {
    //   block [0x82EF86A0..0x82EF86A8)
	// 82EF86A0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF86A4: 4BFFFFEC  b 0x82ef8690
	sub_82EF8690(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF86A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF86A8 size=388
    let mut pc: u32 = 0x82EF86A8;
    'dispatch: loop {
        match pc {
            0x82EF86A8 => {
    //   block [0x82EF86A8..0x82EF882C)
	// 82EF86A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF86AC: 482AFAC1  bl 0x831a816c
	ctx.lr = 0x82EF86B0;
	sub_831A8130(ctx, base);
	// 82EF86B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF86B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF86B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF86BC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EF86C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF86C4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF86C8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF86CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF86D0: 4E800421  bctrl
	ctx.lr = 0x82EF86D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF86D4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF86D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF86DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF86E0: 409A0094  bne cr6, 0x82ef8774
	if !ctx.cr[6].eq {
	pc = 0x82EF8774; continue 'dispatch;
	}
	// 82EF86E4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EF86E8: A11F0000  lhz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF86EC: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF86F0: 2B08001A  cmplwi cr6, r8, 0x1a
	ctx.cr[6].compare_u32(ctx.r[8].u32, 26 as u32, &mut ctx.xer);
	// 82EF86F4: 409A0098  bne cr6, 0x82ef878c
	if !ctx.cr[6].eq {
	pc = 0x82EF878C; continue 'dispatch;
	}
	// 82EF86F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF86FC: 419A0090  beq cr6, 0x82ef878c
	if ctx.cr[6].eq {
	pc = 0x82EF878C; continue 'dispatch;
	}
	// 82EF8700: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8704: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EF8708: A0EB0012  lhz r7, 0x12(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EF870C: B0FF0012  sth r7, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[7].u16 ) };
	// 82EF8710: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82EF8714: A0DF0010  lhz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8718: B0CB0012  sth r6, 0x12(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[6].u16 ) };
	// 82EF871C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF8720: 419A004C  beq cr6, 0x82ef876c
	if ctx.cr[6].eq {
	pc = 0x82EF876C; continue 'dispatch;
	}
	// 82EF8724: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82EF8728: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF872C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82EF8730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8734: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82EF8738: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82EF873C: 4BFF4FC5  bl 0x82eed700
	ctx.lr = 0x82EF8740;
	sub_82EED700(ctx, base);
	// 82EF8740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8744: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF8748: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF874C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8750: 4BFEA6E1  bl 0x82ee2e30
	ctx.lr = 0x82EF8754;
	sub_82EE2E30(ctx, base);
	// 82EF8754: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF8758: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF875C: 419A0010  beq cr6, 0x82ef876c
	if ctx.cr[6].eq {
	pc = 0x82EF876C; continue 'dispatch;
	}
	// 82EF8760: 894B0012  lbz r10, 0x12(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EF8764: 7D49FB78  or r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 82EF8768: 992B0012  stb r9, 0x12(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[9].u8 ) };
	// 82EF876C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF8770: 482AFA4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EF8774: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8778: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF877C: 2B080017  cmplwi cr6, r8, 0x17
	ctx.cr[6].compare_u32(ctx.r[8].u32, 23 as u32, &mut ctx.xer);
	// 82EF8780: 4198FF68  blt cr6, 0x82ef86e8
	if ctx.cr[6].lt {
	pc = 0x82EF86E8; continue 'dispatch;
	}
	// 82EF8784: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF8788: 4BFFFFEC  b 0x82ef8774
	pc = 0x82EF8774; continue 'dispatch;
	// 82EF878C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8794: 419A003C  beq cr6, 0x82ef87d0
	if ctx.cr[6].eq {
	pc = 0x82EF87D0; continue 'dispatch;
	}
	// 82EF8798: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF879C: 2B08001B  cmplwi cr6, r8, 0x1b
	ctx.cr[6].compare_u32(ctx.r[8].u32, 27 as u32, &mut ctx.xer);
	// 82EF87A0: 409A0030  bne cr6, 0x82ef87d0
	if !ctx.cr[6].eq {
	pc = 0x82EF87D0; continue 'dispatch;
	}
	// 82EF87A4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF87A8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF87AC: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF87B0: A0E80012  lhz r7, 0x12(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EF87B4: B0FF0012  sth r7, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[7].u16 ) };
	// 82EF87B8: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF87BC: 93E60014  stw r31, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82EF87C0: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF87C4: A09F0010  lhz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF87C8: B0850012  sth r4, 0x12(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(18 as u32), ctx.r[4].u16 ) };
	// 82EF87CC: 4BFFFF50  b 0x82ef871c
	pc = 0x82EF871C; continue 'dispatch;
	// 82EF87D0: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82EF87D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF87D8: 419A0024  beq cr6, 0x82ef87fc
	if ctx.cr[6].eq {
	pc = 0x82EF87FC; continue 'dispatch;
	}
	// 82EF87DC: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF87E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF87E4: A10A0010  lhz r8, 0x10(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF87E8: B11F0012  sth r8, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[8].u16 ) };
	// 82EF87EC: 93EA000C  stw r31, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82EF87F0: A0FF0010  lhz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF87F4: B0EA0010  sth r7, 0x10(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[7].u16 ) };
	// 82EF87F8: 4BFFFF24  b 0x82ef871c
	pc = 0x82EF871C; continue 'dispatch;
	// 82EF87FC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8800: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EF8804: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8808: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF880C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF8810: 4E800421  bctrl
	ctx.lr = 0x82EF8814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8814: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF8818: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF881C: B13F0012  sth r9, 0x12(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[9].u16 ) };
	// 82EF8820: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EF8824: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF8828: 482AF994  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8830 size=1140
    let mut pc: u32 = 0x82EF8830;
    'dispatch: loop {
        match pc {
            0x82EF8830 => {
    //   block [0x82EF8830..0x82EF8CA4)
	// 82EF8830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8834: 482AF91D  bl 0x831a8150
	ctx.lr = 0x82EF8838;
	sub_831A8130(ctx, base);
	// 82EF8838: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF883C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82EF8840: A1770004  lhz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8844: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8848: 419A0010  beq cr6, 0x82ef8858
	if ctx.cr[6].eq {
	pc = 0x82EF8858; continue 'dispatch;
	}
	// 82EF884C: A1770006  lhz r11, 6(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8850: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EF8854: B1570006  sth r10, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EF8858: 8077000C  lwz r3, 0xc(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF885C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EF8860: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8864: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF8868: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF886C: 4E800421  bctrl
	ctx.lr = 0x82EF8870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8870: 80970014  lwz r4, 0x14(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8874: 80B70018  lwz r5, 0x18(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8878: 81770010  lwz r11, 0x10(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF887C: 92E10080  stw r23, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[23].u32 ) };
	// 82EF8880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8884: 90810084  stw r4, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 82EF8888: 90A10088  stw r5, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[5].u32 ) };
	// 82EF888C: 419A0014  beq cr6, 0x82ef88a0
	if ctx.cr[6].eq {
	pc = 0x82EF88A0; continue 'dispatch;
	}
	// 82EF8890: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82EF8894: A16B0010  lhz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8898: B1610090  sth r11, 0x90(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u16 ) };
	// 82EF889C: 48000014  b 0x82ef88b0
	pc = 0x82EF88B0; continue 'dispatch;
	// 82EF88A0: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF88A4: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82EF88A8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF88AC: B1610090  sth r11, 0x90(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u16 ) };
	// 82EF88B0: 8977001C  lbz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF88B4: 99610093  stb r11, 0x93(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(147 as u32), ctx.r[11].u8 ) };
	// 82EF88B8: 894400D8  lbz r10, 0xd8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 82EF88BC: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82EF88C0: 419A002C  beq cr6, 0x82ef88ec
	if ctx.cr[6].eq {
	pc = 0x82EF88EC; continue 'dispatch;
	}
	// 82EF88C4: 896500D8  lbz r11, 0xd8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82EF88C8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82EF88CC: 419A0020  beq cr6, 0x82ef88ec
	if ctx.cr[6].eq {
	pc = 0x82EF88EC; continue 'dispatch;
	}
	// 82EF88D0: 816400B8  lwz r11, 0xb8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) } as u64;
	// 82EF88D4: 814500B8  lwz r10, 0xb8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(184 as u32) ) } as u64;
	// 82EF88D8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF88DC: 419A0010  beq cr6, 0x82ef88ec
	if ctx.cr[6].eq {
	pc = 0x82EF88EC; continue 'dispatch;
	}
	// 82EF88E0: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF88E4: 4BFE9E3D  bl 0x82ee2720
	ctx.lr = 0x82EF88E8;
	sub_82EE2720(ctx, base);
	// 82EF88E8: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF88EC: 896400D8  lbz r11, 0xd8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 82EF88F0: 39410084  addi r10, r1, 0x84
	ctx.r[10].s64 = ctx.r[1].s64 + 132;
	// 82EF88F4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82EF88F8: 392BFFF9  addi r9, r11, -7
	ctx.r[9].s64 = ctx.r[11].s64 + -7;
	// 82EF88FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EF8900: 7D270034  cntlzw r7, r9
	ctx.r[7].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82EF8904: 54EBDFFE  rlwinm r11, r7, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82EF8908: 556615BA  rlwinm r6, r11, 2, 0x16, 0x1d
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF890C: 9961009A  stb r11, 0x9a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 82EF8910: 7FC6502E  lwzx r30, r6, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EF8914: 80BE00B8  lwz r5, 0xb8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 82EF8918: 90B70008  stw r5, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EF891C: 8165001C  lwz r11, 0x1c(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF8920: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82EF8924: 9085001C  stw r4, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF8928: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF892C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8930: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EF8934: 9901009B  stb r8, 0x9b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(155 as u32), ctx.r[8].u8 ) };
	// 82EF8938: 9AC10092  stb r22, 0x92(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(146 as u32), ctx.r[22].u8 ) };
	// 82EF893C: 409A0060  bne cr6, 0x82ef899c
	if !ctx.cr[6].eq {
	pc = 0x82EF899C; continue 'dispatch;
	}
	// 82EF8940: 8097000C  lwz r4, 0xc(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8944: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EF8948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF894C: 99610092  stb r11, 0x92(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(146 as u32), ctx.r[11].u8 ) };
	// 82EF8950: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8954: 812A0024  lwz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF8958: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF895C: 4E800421  bctrl
	ctx.lr = 0x82EF8960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8960: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8964: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF8968: 419A0010  beq cr6, 0x82ef8978
	if ctx.cr[6].eq {
	pc = 0x82EF8978; continue 'dispatch;
	}
	// 82EF896C: 89410092  lbz r10, 0x92(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(146 as u32) ) } as u64;
	// 82EF8970: 61490004  ori r9, r10, 4
	ctx.r[9].u64 = ctx.r[10].u64 | 4;
	// 82EF8974: 99210092  stb r9, 0x92(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(146 as u32), ctx.r[9].u8 ) };
	// 82EF8978: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF897C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82EF8980: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8984: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF8988: 4E800421  bctrl
	ctx.lr = 0x82EF898C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF898C: 7C690034  cntlzw r9, r3
	ctx.r[9].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EF8990: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82EF8994: 9901009B  stb r8, 0x9b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(155 as u32), ctx.r[8].u8 ) };
	// 82EF8998: 48000014  b 0x82ef89ac
	pc = 0x82EF89AC; continue 'dispatch;
	// 82EF899C: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82EF89A0: 409A000C  bne cr6, 0x82ef89ac
	if !ctx.cr[6].eq {
	pc = 0x82EF89AC; continue 'dispatch;
	}
	// 82EF89A4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EF89A8: 99610092  stb r11, 0x92(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(146 as u32), ctx.r[11].u8 ) };
	// 82EF89AC: 80770010  lwz r3, 0x10(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF89B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF89B4: 419A0018  beq cr6, 0x82ef89cc
	if ctx.cr[6].eq {
	pc = 0x82EF89CC; continue 'dispatch;
	}
	// 82EF89B8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EF89BC: 4BFF4DAD  bl 0x82eed768
	ctx.lr = 0x82EF89C0;
	sub_82EED768(ctx, base);
	// 82EF89C0: 89410092  lbz r10, 0x92(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(146 as u32) ) } as u64;
	// 82EF89C4: 7C695378  or r9, r3, r10
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[10].u64;
	// 82EF89C8: 99210092  stb r9, 0x92(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(146 as u32), ctx.r[9].u8 ) };
	// 82EF89CC: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82EF89D0: B2C10096  sth r22, 0x96(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(150 as u32), ctx.r[22].u16 ) };
	// 82EF89D4: B2C10098  sth r22, 0x98(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[22].u16 ) };
	// 82EF89D8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82EF89DC: B2C10094  sth r22, 0x94(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[22].u16 ) };
	// 82EF89E0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EF89E4: 91770028  stw r11, 0x28(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EF89E8: 80770008  lwz r3, 8(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF89EC: 4BFEA445  bl 0x82ee2e30
	ctx.lr = 0x82EF89F0;
	sub_82EE2E30(ctx, base);
	// 82EF89F0: A0FE009C  lhz r7, 0x9c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF89F4: 3B3E0098  addi r25, r30, 0x98
	ctx.r[25].s64 = ctx.r[30].s64 + 152;
	// 82EF89F8: 7ED8B378  mr r24, r22
	ctx.r[24].u64 = ctx.r[22].u64;
	// 82EF89FC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EF8A00: 40990064  ble cr6, 0x82ef8a64
	if !ctx.cr[6].gt {
	pc = 0x82EF8A64; continue 'dispatch;
	}
	// 82EF8A04: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8A08: 89010093  lbz r8, 0x93(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(147 as u32) ) } as u64;
	// 82EF8A0C: 80A10088  lwz r5, 0x88(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82EF8A10: 396B0013  addi r11, r11, 0x13
	ctx.r[11].s64 = ctx.r[11].s64 + 19;
	// 82EF8A14: 80C10084  lwz r6, 0x84(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF8A18: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8A1C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EF8A20: 41990044  bgt cr6, 0x82ef8a64
	if ctx.cr[6].gt {
	pc = 0x82EF8A64; continue 'dispatch;
	}
	// 82EF8A24: 409A0030  bne cr6, 0x82ef8a54
	if !ctx.cr[6].eq {
	pc = 0x82EF8A54; continue 'dispatch;
	}
	// 82EF8A28: 812BFFF1  lwz r9, -0xf(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15 as u32) ) } as u64;
	// 82EF8A2C: 814600C0  lwz r10, 0xc0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(192 as u32) ) } as u64;
	// 82EF8A30: 812900C0  lwz r9, 0xc0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(192 as u32) ) } as u64;
	// 82EF8A34: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF8A38: 4199002C  bgt cr6, 0x82ef8a64
	if ctx.cr[6].gt {
	pc = 0x82EF8A64; continue 'dispatch;
	}
	// 82EF8A3C: 409A0018  bne cr6, 0x82ef8a54
	if !ctx.cr[6].eq {
	pc = 0x82EF8A54; continue 'dispatch;
	}
	// 82EF8A40: 814BFFF5  lwz r10, -0xb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11 as u32) ) } as u64;
	// 82EF8A44: 812500C0  lwz r9, 0xc0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(192 as u32) ) } as u64;
	// 82EF8A48: 808A00C0  lwz r4, 0xc0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) } as u64;
	// 82EF8A4C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF8A50: 41990014  bgt cr6, 0x82ef8a64
	if ctx.cr[6].gt {
	pc = 0x82EF8A64; continue 'dispatch;
	}
	// 82EF8A54: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82EF8A58: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82EF8A5C: 7F183800  cmpw cr6, r24, r7
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EF8A60: 4198FFB8  blt cr6, 0x82ef8a18
	if ctx.cr[6].lt {
	pc = 0x82EF8A18; continue 'dispatch;
	}
	// 82EF8A64: A1790006  lhz r11, 6(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8A68: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82EF8A6C: 556A04BE  clrlwi r10, r11, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82EF8A70: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF8A74: 41980008  blt cr6, 0x82ef8a7c
	if ctx.cr[6].lt {
	pc = 0x82EF8A7C; continue 'dispatch;
	}
	// 82EF8A78: 7EDFB378  mr r31, r22
	ctx.r[31].u64 = ctx.r[22].u64;
	// 82EF8A7C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EF8A80: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF8A84: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EF8A88: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EF8A8C: 4800021D  bl 0x82ef8ca8
	ctx.lr = 0x82EF8A90;
	sub_82EF8CA8(ctx, base);
	// 82EF8A90: A1790004  lhz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8A94: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82EF8A98: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF8A9C: 40980034  bge cr6, 0x82ef8ad0
	if !ctx.cr[6].lt {
	pc = 0x82EF8AD0; continue 'dispatch;
	}
	// 82EF8AA0: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8AA4: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82EF8AA8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8AAC: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF8AB4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF8AB8: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82EF8ABC: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AC0: 91280028  stw r9, 0x28(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82EF8AC4: A0F90004  lhz r7, 4(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8AC8: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EF8ACC: 4198FFE0  blt cr6, 0x82ef8aac
	if ctx.cr[6].lt {
	pc = 0x82EF8AAC; continue 'dispatch;
	}
	// 82EF8AD0: 83570028  lwz r26, 0x28(r23)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF8AD4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82EF8AD8: 8897001D  lbz r4, 0x1d(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(29 as u32) ) } as u64;
	// 82EF8ADC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AE0: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8AE4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AE8: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8AEC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF8AF0: 4E800421  bctrl
	ctx.lr = 0x82EF8AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8AF4: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF8AF8: 38E8000F  addi r7, r8, 0xf
	ctx.r[7].s64 = ctx.r[8].s64 + 15;
	// 82EF8AFC: 54EB0036  rlwinm r11, r7, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF8B00: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF8B04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8B08: B17A0020  sth r11, 0x20(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 82EF8B0C: 419A0110  beq cr6, 0x82ef8c1c
	if ctx.cr[6].eq {
	pc = 0x82EF8C1C; continue 'dispatch;
	}
	// 82EF8B10: 3BDE00AC  addi r30, r30, 0xac
	ctx.r[30].s64 = ctx.r[30].s64 + 172;
	// 82EF8B14: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8B18: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8B1C: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF8B20: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8B24: 7C9D5A14  add r4, r29, r11
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82EF8B28: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82EF8B2C: 40980014  bge cr6, 0x82ef8b40
	if !ctx.cr[6].lt {
	pc = 0x82EF8B40; continue 'dispatch;
	}
	// 82EF8B30: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF8B34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8B38: 4BFADCC1  bl 0x82ea67f8
	ctx.lr = 0x82EF8B3C;
	sub_82EA67F8(ctx, base);
	// 82EF8B3C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF8B40: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8B44: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82EF8B48: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 82EF8B4C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8B50: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82EF8B54: 7FBCF850  subf r29, r28, r31
	ctx.r[29].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82EF8B58: 40990040  ble cr6, 0x82ef8b98
	if !ctx.cr[6].gt {
	pc = 0x82EF8B98; continue 'dispatch;
	}
	// 82EF8B5C: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 82EF8B60: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82EF8B64: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82EF8B68: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8B6C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF8B70: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF8B74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF8B78: 419A0014  beq cr6, 0x82ef8b8c
	if ctx.cr[6].eq {
	pc = 0x82EF8B8C; continue 'dispatch;
	}
	// 82EF8B7C: A12B0020  lhz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF8B80: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82EF8B84: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82EF8B88: 7FE95214  add r31, r9, r10
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF8B8C: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EF8B90: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 82EF8B94: 4082FFD4  bne 0x82ef8b68
	if !ctx.cr[0].eq {
	pc = 0x82EF8B68; continue 'dispatch;
	}
	// 82EF8B98: A17A0020  lhz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF8B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF8BA0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8BA4: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF8BA8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8BAC: 7D0B4850  subf r8, r11, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82EF8BB0: 7D7F4050  subf r11, r31, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[31].s64;
	// 82EF8BB4: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8BB8: 4BFB1B99  bl 0x82eaa750
	ctx.lr = 0x82EF8BBC;
	sub_82EAA750(ctx, base);
	// 82EF8BBC: A15A0020  lhz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF8BC0: 397B0001  addi r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 1;
	// 82EF8BC4: 93FA001C  stw r31, 0x1c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82EF8BC8: A0D90004  lhz r6, 4(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8BCC: 7CEAEA14  add r7, r10, r29
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82EF8BD0: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EF8BD4: 4098004C  bge cr6, 0x82ef8c20
	if !ctx.cr[6].lt {
	pc = 0x82EF8C20; continue 'dispatch;
	}
	// 82EF8BD8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8BDC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8BE0: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF8BE4: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8BE8: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF8BEC: 8148001C  lwz r10, 0x1c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF8BF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF8BF4: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82EF8BF8: 409A0008  bne cr6, 0x82ef8c00
	if !ctx.cr[6].eq {
	pc = 0x82EF8C00; continue 'dispatch;
	}
	// 82EF8BFC: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 82EF8C00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF8C04: 9148001C  stw r10, 0x1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82EF8C08: A1590004  lhz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8C0C: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 82EF8C10: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF8C14: 4198FFD0  blt cr6, 0x82ef8be4
	if ctx.cr[6].lt {
	pc = 0x82EF8BE4; continue 'dispatch;
	}
	// 82EF8C18: 48000008  b 0x82ef8c20
	pc = 0x82EF8C20; continue 'dispatch;
	// 82EF8C1C: 92DA001C  stw r22, 0x1c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(28 as u32), ctx.r[22].u32 ) };
	// 82EF8C20: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8C24: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82EF8C28: A0DA0020  lhz r6, 0x20(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF8C2C: 80BA001C  lwz r5, 0x1c(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF8C30: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8C34: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8C38: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF8C3C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EF8C40: 4E800421  bctrl
	ctx.lr = 0x82EF8C44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8C44: 891A001A  lbz r8, 0x1a(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EF8C48: 20E80002  subfic r7, r8, 2
	ctx.xer.ca = ctx.r[8].u32 <= 2 as u32;
	ctx.r[7].s64 = (2 as i64) - ctx.r[8].s64;
	// 82EF8C4C: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EF8C50: 7D66D02E  lwzx r11, r6, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82EF8C54: 3BEB00A0  addi r31, r11, 0xa0
	ctx.r[31].s64 = ctx.r[11].s64 + 160;
	// 82EF8C58: 80AB00A4  lwz r5, 0xa4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF8C5C: B0BA0022  sth r5, 0x22(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(34 as u32), ctx.r[5].u16 ) };
	// 82EF8C60: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF8C64: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82EF8C68: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF8C6C: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF8C70: 409A0010  bne cr6, 0x82ef8c80
	if !ctx.cr[6].eq {
	pc = 0x82EF8C80; continue 'dispatch;
	}
	// 82EF8C74: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EF8C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C7C: 4BFADC05  bl 0x82ea6880
	ctx.lr = 0x82EF8C80;
	sub_82EA6880(ctx, base);
	// 82EF8C80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8C84: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8C88: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF8C8C: 7EE9512E  stwx r23, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u32) };
	// 82EF8C90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8C94: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EF8C98: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF8C9C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82EF8CA0: 482AF500  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8CA8 size=300
    let mut pc: u32 = 0x82EF8CA8;
    'dispatch: loop {
        match pc {
            0x82EF8CA8 => {
    //   block [0x82EF8CA8..0x82EF8DD4)
	// 82EF8CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8CAC: 482AF4B5  bl 0x831a8160
	ctx.lr = 0x82EF8CB0;
	sub_831A8130(ctx, base);
	// 82EF8CB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8CB4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF8CB8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF8CBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF8CC0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF8CC4: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8CC8: A15C0006  lhz r10, 6(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8CCC: 7F4BEA14  add r26, r11, r29
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EF8CD0: 554A04BE  clrlwi r10, r10, 0x12
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82EF8CD4: 7FDF5850  subf r30, r31, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82EF8CD8: 7F0AD000  cmpw cr6, r10, r26
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82EF8CDC: 40980024  bge cr6, 0x82ef8d00
	if !ctx.cr[6].lt {
	pc = 0x82EF8D00; continue 'dispatch;
	}
	// 82EF8CE0: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8CE4: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF8CE8: 41980008  blt cr6, 0x82ef8cf0
	if ctx.cr[6].lt {
	pc = 0x82EF8CF0; continue 'dispatch;
	}
	// 82EF8CEC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82EF8CF0: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82EF8CF4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EF8CF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8CFC: 483065D5  bl 0x831ff2d0
	ctx.lr = 0x82EF8D00;
	sub_831FF2D0(ctx, base);
	// 82EF8D00: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82EF8D04: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8D08: 57E8083C  slwi r8, r31, 1
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF8D0C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF8D10: 7D1F4214  add r8, r31, r8
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 82EF8D14: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF8D18: 55052036  slwi r5, r8, 4
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF8D1C: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8D20: 353EFFFF  addic. r9, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF8D24: 7D0A2A14  add r8, r10, r5
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82EF8D28: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8D2C: 41800048  blt 0x82ef8d74
	if ctx.cr[0].lt {
	pc = 0x82EF8D74; continue 'dispatch;
	}
	// 82EF8D30: 552A083C  slwi r10, r9, 1
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8D34: 7CCB4050  subf r6, r11, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82EF8D38: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF8D3C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8D40: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF8D44: 7D664214  add r11, r6, r8
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 82EF8D48: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EF8D4C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82EF8D50: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82EF8D54: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EF8D58: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF8D5C: F8EA0000  std r7, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82EF8D60: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF8D64: 4200FFF0  bdnz 0x82ef8d54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF8D54; continue 'dispatch;
	}
	// 82EF8D68: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF8D6C: 3908FFD0  addi r8, r8, -0x30
	ctx.r[8].s64 = ctx.r[8].s64 + -48;
	// 82EF8D70: 4080FFD4  bge 0x82ef8d44
	if !ctx.cr[0].lt {
	pc = 0x82EF8D44; continue 'dispatch;
	}
	// 82EF8D74: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8D78: 34FDFFFF  addic. r7, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EF8D7C: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82EF8D80: 41800048  blt 0x82ef8dc8
	if ctx.cr[0].lt {
	pc = 0x82EF8DC8; continue 'dispatch;
	}
	// 82EF8D84: 54EA083C  slwi r10, r7, 1
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8D88: 7CCBD850  subf r6, r11, r27
	ctx.r[6].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 82EF8D8C: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EF8D90: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8D94: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF8D98: 7D664A14  add r11, r6, r9
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 82EF8D9C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82EF8DA0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82EF8DA4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EF8DA8: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EF8DAC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF8DB0: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82EF8DB4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EF8DB8: 4200FFF0  bdnz 0x82ef8da8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF8DA8; continue 'dispatch;
	}
	// 82EF8DBC: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EF8DC0: 3929FFD0  addi r9, r9, -0x30
	ctx.r[9].s64 = ctx.r[9].s64 + -48;
	// 82EF8DC4: 4080FFD4  bge 0x82ef8d98
	if !ctx.cr[0].lt {
	pc = 0x82EF8D98; continue 'dispatch;
	}
	// 82EF8DC8: B35C0004  sth r26, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82EF8DCC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8DD0: 482AF3E0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8DD8 size=168
    let mut pc: u32 = 0x82EF8DD8;
    'dispatch: loop {
        match pc {
            0x82EF8DD8 => {
    //   block [0x82EF8DD8..0x82EF8E80)
	// 82EF8DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8DDC: 482AF385  bl 0x831a8160
	ctx.lr = 0x82EF8DE0;
	sub_831A8130(ctx, base);
	// 82EF8DE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8DE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8DE8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8DEC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EF8DF0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF8DF4: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82EF8DF8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EF8DFC: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82EF8E00: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EF8E04: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EF8E08: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EF8E0C: 391D0020  addi r8, r29, 0x20
	ctx.r[8].s64 = ctx.r[29].s64 + 32;
	// 82EF8E10: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EF8E14: 7D68E1D6  mullw r11, r8, r28
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82EF8E18: 38EB003F  addi r7, r11, 0x3f
	ctx.r[7].s64 = ctx.r[11].s64 + 63;
	// 82EF8E1C: 54FA0036  rlwinm r26, r7, 0, 0, 0x1b
	ctx.r[26].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF8E20: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF8E24: 4BFA790D  bl 0x82ea0730
	ctx.lr = 0x82EF8E28;
	sub_82EA0730(ctx, base);
	// 82EF8E28: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82EF8E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF8E30: B3430002  sth r26, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 82EF8E34: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82EF8E38: B3830006  sth r28, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82EF8E3C: 9BC30008  stb r30, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82EF8E40: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF8E44: 9BE30009  stb r31, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[31].u8 ) };
	// 82EF8E48: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82EF8E4C: B363000C  sth r27, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[27].u16 ) };
	// 82EF8E50: 9BA3000A  stb r29, 0xa(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[29].u8 ) };
	// 82EF8E54: B1230010  sth r9, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u16 ) };
	// 82EF8E58: B0C30012  sth r6, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[6].u16 ) };
	// 82EF8E5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF8E60: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF8E64: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF8E68: 4200FFF8  bdnz 0x82ef8e60
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF8E60; continue 'dispatch;
	}
	// 82EF8E6C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 82EF8E70: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EF8E74: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82EF8E78: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8E7C: 482AF334  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF8E80 size=216
    let mut pc: u32 = 0x82EF8E80;
    'dispatch: loop {
        match pc {
            0x82EF8E80 => {
    //   block [0x82EF8E80..0x82EF8F58)
	// 82EF8E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8E84: 482AF2E5  bl 0x831a8168
	ctx.lr = 0x82EF8E88;
	sub_831A8130(ctx, base);
	// 82EF8E88: A0E30006  lhz r7, 6(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8E8C: 39240010  addi r9, r4, 0x10
	ctx.r[9].s64 = ctx.r[4].s64 + 16;
	// 82EF8E90: E9440010  ld r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	// 82EF8E94: 39040030  addi r8, r4, 0x30
	ctx.r[8].s64 = ctx.r[4].s64 + 48;
	// 82EF8E98: 54EB283E  rotlwi r11, r7, 5
	ctx.r[11].u64 = ((ctx.r[7].u32).rotate_left(5)) as u64;
	// 82EF8E9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF8EA0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EF8EA4: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82EF8EA8: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82EF8EAC: E8C40018  ld r6, 0x18(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	// 82EF8EB0: F8C30018  std r6, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[6].u64 ) };
	// 82EF8EB4: E9640020  ld r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	// 82EF8EB8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EF8EBC: E9440028  ld r10, 0x28(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	// 82EF8EC0: F9430028  std r10, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u64 ) };
	// 82EF8EC4: A1240004  lhz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8EC8: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EF8ECC: A0E40004  lhz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8ED0: A0C40006  lhz r6, 6(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8ED4: 54CB283E  rotlwi r11, r6, 5
	ctx.r[11].u64 = ((ctx.r[6].u32).rotate_left(5)) as u64;
	// 82EF8ED8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EF8EDC: 8BC4000A  lbz r30, 0xa(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(10 as u32) ) } as u64;
	// 82EF8EE0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF8EE4: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 82EF8EE8: 419A006C  beq cr6, 0x82ef8f54
	if ctx.cr[6].eq {
	pc = 0x82EF8F54; continue 'dispatch;
	}
	// 82EF8EEC: 7D644050  subf r11, r4, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 82EF8EF0: 7FDF1670  srawi r31, r30, 2
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[30].s32 >> 2) as i64;
	// 82EF8EF4: 7CCB1A14  add r6, r11, r3
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EF8EF8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8F58 size=236
    let mut pc: u32 = 0x82EF8F58;
    'dispatch: loop {
        match pc {
            0x82EF8F58 => {
    //   block [0x82EF8F58..0x82EF9044)
	// 82EF8F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8F5C: 482AF20D  bl 0x831a8168
	ctx.lr = 0x82EF8F60;
	sub_831A8130(ctx, base);
	// 82EF8F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8F68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF8F6C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82EF8F70: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EF8F74: A3DF0004  lhz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8F78: A15F0006  lhz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8F7C: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF8F80: 41980064  blt cr6, 0x82ef8fe4
	if ctx.cr[6].lt {
	pc = 0x82EF8FE4; continue 'dispatch;
	}
	// 82EF8F84: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82EF8F88: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EF8F8C: 41990008  bgt cr6, 0x82ef8f94
	if ctx.cr[6].gt {
	pc = 0x82EF8F94; continue 'dispatch;
	}
	// 82EF8F90: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EF8F94: A0DF000C  lhz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8F98: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF8F9C: 7F033000  cmpw cr6, r3, r6
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EF8FA0: 41980008  blt cr6, 0x82ef8fa8
	if ctx.cr[6].lt {
	pc = 0x82EF8FA8; continue 'dispatch;
	}
	// 82EF8FA4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82EF8FA8: 88BF0009  lbz r5, 9(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82EF8FAC: 889F0008  lbz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8FB0: 4BFFFE29  bl 0x82ef8dd8
	ctx.lr = 0x82EF8FB4;
	sub_82EF8DD8(ctx, base);
	// 82EF8FB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF8FB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF8FBC: 4BFFFEC5  bl 0x82ef8e80
	ctx.lr = 0x82EF8FC0;
	sub_82EF8E80(ctx, base);
	// 82EF8FC0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8FC4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF8FC8: A0BF0002  lhz r5, 2(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EF8FCC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82EF8FD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF8FD4: 4BFA77DD  bl 0x82ea07b0
	ctx.lr = 0x82EF8FD8;
	sub_82EA07B0(ctx, base);
	// 82EF8FD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF8FDC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EF8FE0: 993C0000  stb r9, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82EF8FE4: 890B000A  lbz r8, 0xa(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82EF8FE8: 7FC30734  extsh r3, r30
	ctx.r[3].s64 = ctx.r[30].s16 as i64;
	// 82EF8FEC: A0CB0006  lhz r6, 6(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF8FF0: 38BE0001  addi r5, r30, 1
	ctx.r[5].s64 = ctx.r[30].s64 + 1;
	// 82EF8FF4: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EF8FF8: 54C9283E  rotlwi r9, r6, 5
	ctx.r[9].u64 = ((ctx.r[6].u32).rotate_left(5)) as u64;
	// 82EF8FFC: B0AB0004  sth r5, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u16 ) };
	// 82EF9000: 7D4A19D6  mullw r10, r10, r3
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82EF9004: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF9008: 7D081670  srawi r8, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82EF900C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF9010: 3548FFFF  addic. r10, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF9014: 39690030  addi r11, r9, 0x30
	ctx.r[11].s64 = ctx.r[9].s64 + 48;
	// 82EF9018: 41800020  blt 0x82ef9038
	if ctx.cr[0].lt {
	pc = 0x82EF9038; continue 'dispatch;
	}
	// 82EF901C: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF9020: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF9024: 41820014  beq 0x82ef9038
	if ctx.cr[0].eq {
	pc = 0x82EF9038; continue 'dispatch;
	}
	// 82EF9028: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF902C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF9030: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF9034: 4200FFF8  bdnz 0x82ef902c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82EF902C; continue 'dispatch;
	}
	// 82EF9038: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF903C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9040: 482AF178  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9048 size=132
    let mut pc: u32 = 0x82EF9048;
    'dispatch: loop {
        match pc {
            0x82EF9048 => {
    //   block [0x82EF9048..0x82EF90CC)
	// 82EF9048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF904C: 482AF121  bl 0x831a816c
	ctx.lr = 0x82EF9050;
	sub_831A8130(ctx, base);
	// 82EF9050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9058: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF905C: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9060: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EF9064: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82EF9068: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF906C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF9070: 41990050  bgt cr6, 0x82ef90c0
	if ctx.cr[6].gt {
	pc = 0x82EF90C0; continue 'dispatch;
	}
	// 82EF9074: 5563F87E  srwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF9078: A0DF000C  lhz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF907C: 88BF0009  lbz r5, 9(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82EF9080: 889F0008  lbz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9084: 4BFFFD55  bl 0x82ef8dd8
	ctx.lr = 0x82EF9088;
	sub_82EF8DD8(ctx, base);
	// 82EF9088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF908C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF9090: 4BFFFDF1  bl 0x82ef8e80
	ctx.lr = 0x82EF9094;
	sub_82EF8E80(ctx, base);
	// 82EF9094: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9098: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF909C: A0BF0002  lhz r5, 2(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EF90A0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82EF90A4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF90A8: 4BFA7709  bl 0x82ea07b0
	ctx.lr = 0x82EF90AC;
	sub_82EA07B0(ctx, base);
	// 82EF90AC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF90B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF90B4: 993E0000  stb r9, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82EF90B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF90BC: 482AF100  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EF90C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF90C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF90C8: 482AF0F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF90D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF90D0 size=228
    let mut pc: u32 = 0x82EF90D0;
    'dispatch: loop {
        match pc {
            0x82EF90D0 => {
    //   block [0x82EF90D0..0x82EF91B4)
	// 82EF90D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF90D4: 482AF099  bl 0x831a816c
	ctx.lr = 0x82EF90D8;
	sub_831A8130(ctx, base);
	// 82EF90D8: A1240000  lhz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF90DC: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82EF90E0: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82EF90E4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82EF90E8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82EF90EC: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82EF90F0: B1230000  sth r9, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF91B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF91B8 size=224
    let mut pc: u32 = 0x82EF91B8;
    'dispatch: loop {
        match pc {
            0x82EF91B8 => {
    //   block [0x82EF91B8..0x82EF9298)
	// 82EF91B8: A1240000  lhz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF91BC: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82EF91C0: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82EF91C4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82EF91C8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82EF91CC: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82EF91D0: B1230000  sth r9, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9298 size=348
    let mut pc: u32 = 0x82EF9298;
    'dispatch: loop {
        match pc {
            0x82EF9298 => {
    //   block [0x82EF9298..0x82EF93F4)
	// 82EF9298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF929C: 482AEEC9  bl 0x831a8164
	ctx.lr = 0x82EF92A0;
	sub_831A8130(ctx, base);
	// 82EF92A0: A0E40000  lhz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF92A4: 39240010  addi r9, r4, 0x10
	ctx.r[9].s64 = ctx.r[4].s64 + 16;
	// 82EF92A8: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
	// 82EF92AC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82EF92B0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82EF92B4: 3BC00030  li r30, 0x30
	ctx.r[30].s64 = 48;
	// 82EF92B8: B0E30000  sth r7, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF93F8 size=400
    let mut pc: u32 = 0x82EF93F8;
    'dispatch: loop {
        match pc {
            0x82EF93F8 => {
    //   block [0x82EF93F8..0x82EF9588)
	// 82EF93F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF93FC: 482AED6D  bl 0x831a8168
	ctx.lr = 0x82EF9400;
	sub_831A8130(ctx, base);
	// 82EF9400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9408: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF940C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9410: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF9414: 4E800421  bctrl
	ctx.lr = 0x82EF9418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF9418: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82EF941C: 419A0104  beq cr6, 0x82ef9520
	if ctx.cr[6].eq {
	pc = 0x82EF9520; continue 'dispatch;
	}
	// 82EF9420: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82EF9424: 419A0094  beq cr6, 0x82ef94b8
	if ctx.cr[6].eq {
	pc = 0x82EF94B8; continue 'dispatch;
	}
	// 82EF9428: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82EF942C: 419A0010  beq cr6, 0x82ef943c
	if ctx.cr[6].eq {
	pc = 0x82EF943C; continue 'dispatch;
	}
	// 82EF9430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF9434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9438: 482AED80  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EF943C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9440: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF9444: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF9448: 38800140  li r4, 0x140
	ctx.r[4].s64 = 320;
	// 82EF944C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF9450: 4BFA72E1  bl 0x82ea0730
	ctx.lr = 0x82EF9454;
	sub_82EA0730(ctx, base);
	// 82EF9454: 39200140  li r9, 0x140
	ctx.r[9].s64 = 320;
	// 82EF9458: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EF945C: 4BFE2B3D  bl 0x82edbf98
	ctx.lr = 0x82EF9460;
	sub_82EDBF98(ctx, base);
	// 82EF9460: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF9464: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EF9468: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 82EF946C: 4BFFFE2D  bl 0x82ef9298
	ctx.lr = 0x82EF9470;
	sub_82EF9298(ctx, base);
	// 82EF9470: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9474: 3BDC00E0  addi r30, r28, 0xe0
	ctx.r[30].s64 = ctx.r[28].s64 + 224;
	// 82EF9478: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82EF947C: 911C0008  stw r8, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EF9480: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9484: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9488: 419A0018  beq cr6, 0x82ef94a0
	if ctx.cr[6].eq {
	pc = 0x82EF94A0; continue 'dispatch;
	}
	// 82EF948C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9490: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF9494: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF9498: 4E800421  bctrl
	ctx.lr = 0x82EF949C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF949C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF94A0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF94A4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EF94A8: 4082FFD8  bne 0x82ef9480
	if !ctx.cr[0].eq {
	pc = 0x82EF9480; continue 'dispatch;
	}
	// 82EF94AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF94B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF94B4: 482AED04  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EF94B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF94BC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF94C0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF94C4: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 82EF94C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF94CC: 4BFA7265  bl 0x82ea0730
	ctx.lr = 0x82EF94D0;
	sub_82EA0730(ctx, base);
	// 82EF94D0: 392000D0  li r9, 0xd0
	ctx.r[9].s64 = 208;
	// 82EF94D4: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EF94D8: 4BFF7011  bl 0x82ef04e8
	ctx.lr = 0x82EF94DC;
	sub_82EF04E8(ctx, base);
	// 82EF94DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF94E0: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EF94E4: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82EF94E8: 4BFFFCD1  bl 0x82ef91b8
	ctx.lr = 0x82EF94EC;
	sub_82EF91B8(ctx, base);
	// 82EF94EC: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF94F0: 911E0008  stw r8, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EF94F4: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF94F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF94FC: 419A0080  beq cr6, 0x82ef957c
	if ctx.cr[6].eq {
	pc = 0x82EF957C; continue 'dispatch;
	}
	// 82EF9500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9504: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF9508: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF950C: 4E800421  bctrl
	ctx.lr = 0x82EF9510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF9510: 907E00AC  stw r3, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[3].u32 ) };
	// 82EF9514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF951C: 482AEC9C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EF9520: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9524: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EF9528: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82EF952C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82EF9530: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF9534: 4BFA71FD  bl 0x82ea0730
	ctx.lr = 0x82EF9538;
	sub_82EA0730(ctx, base);
	// 82EF9538: 392000E0  li r9, 0xe0
	ctx.r[9].s64 = 224;
	// 82EF953C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EF9540: 4BFF8BB9  bl 0x82ef20f8
	ctx.lr = 0x82EF9544;
	sub_82EF20F8(ctx, base);
	// 82EF9544: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF9548: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EF954C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82EF9550: 4BFFFB81  bl 0x82ef90d0
	ctx.lr = 0x82EF9554;
	sub_82EF90D0(ctx, base);
	// 82EF9554: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9558: 911E0008  stw r8, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EF955C: 807E00B0  lwz r3, 0xb0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF9560: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9564: 419A0018  beq cr6, 0x82ef957c
	if ctx.cr[6].eq {
	pc = 0x82EF957C; continue 'dispatch;
	}
	// 82EF9568: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF956C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF9570: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF9574: 4E800421  bctrl
	ctx.lr = 0x82EF9578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF9578: 907E00B0  stw r3, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF957C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9584: 482AEC34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9588 size=60
    let mut pc: u32 = 0x82EF9588;
    'dispatch: loop {
        match pc {
            0x82EF9588 => {
    //   block [0x82EF9588..0x82EF95C4)
	// 82EF9588: 396300D0  addi r11, r3, 0xd0
	ctx.r[11].s64 = ctx.r[3].s64 + 208;
	// 82EF958C: 394300E0  addi r10, r3, 0xe0
	ctx.r[10].s64 = ctx.r[3].s64 + 224;
	// 82EF9590: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF95C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF95C8 size=144
    let mut pc: u32 = 0x82EF95C8;
    'dispatch: loop {
        match pc {
            0x82EF95C8 => {
    //   block [0x82EF95C8..0x82EF9658)
	// 82EF95C8: 3505FFFF  addic. r8, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EF95CC: 418000A4  blt 0x82ef9670
	if ctx.cr[0].lt {
		sub_82EF9658(ctx, base);
		return;
	}
	// 82EF95D0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82EF95D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EF95D8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EF95DC: C16B2780  lfs f11, 0x2780(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10112 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EF95E0: C18AD5B8  lfs f12, -0x2a48(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10824 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EF95E4: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF95E8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF95EC: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 82EF95F0: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 82EF95F4: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82EF95F8: 394B00D0  addi r10, r11, 0xd0
	ctx.r[10].s64 = ctx.r[11].s64 + 208;
	// 82EF95FC: 392B00E0  addi r9, r11, 0xe0
	ctx.r[9].s64 = ctx.r[11].s64 + 224;
	// 82EF9600: C1AB00B0  lfs f13, 0xb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF9604: ED406828  fsubs f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EF9658 size=32
    let mut pc: u32 = 0x82EF9658;
    'dispatch: loop {
        match pc {
            0x82EF9658 => {
    //   block [0x82EF9658..0x82EF9678)
	// 82EF9658: C14B010C  lfs f10, 0x10c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EF965C: FF0D5000  fcmpu cr6, f13, f10
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[10].f64);
	// 82EF9660: 41990018  bgt cr6, 0x82ef9678
	if ctx.cr[6].gt {
		sub_82EF9678(ctx, base);
		return;
	}
	// 82EF9664: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EF9668: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82EF966C: 4080FF7C  bge 0x82ef95e8
	if !ctx.cr[0].lt {
		sub_82EF95C8(ctx, base);
		return;
	}
	// 82EF9670: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF9674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9678 size=8
    let mut pc: u32 = 0x82EF9678;
    'dispatch: loop {
        match pc {
            0x82EF9678 => {
    //   block [0x82EF9678..0x82EF9680)
	// 82EF9678: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF967C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9680 size=1060
    let mut pc: u32 = 0x82EF9680;
    'dispatch: loop {
        match pc {
            0x82EF9680 => {
    //   block [0x82EF9680..0x82EF9AA4)
	// 82EF9680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9684: 482AEAB1  bl 0x831a8134
	ctx.lr = 0x82EF9688;
	sub_831A8130(ctx, base);
	// 82EF9688: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 82EF968C: 482AF3DD  bl 0x831a8a68
	ctx.lr = 0x82EF9690;
	sub_831A8A40(ctx, base);
	// 82EF9690: 3981FF30  addi r12, r1, -0xd0
	ctx.r[12].s64 = ctx.r[1].s64 + -208;
	// 82EF9694: 482B1561  bl 0x831aabf4
	ctx.lr = 0x82EF9698;
	sub_831AA9A0(ctx, base);
	// 82EF9698: 9421FDD0  stwu r1, -0x230(r1)
	ea = ctx.r[1].u32.wrapping_add(-560 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF969C: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 82EF96A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF96A4: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82EF96A8: 3765FFFF  addic. r27, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EF96AC: 418003E0  blt 0x82ef9a8c
	if ctx.cr[0].lt {
	pc = 0x82EF9A8C; continue 'dispatch;
	}
	// 82EF96B0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF96B4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82EF96B8: 392B6700  addi r9, r11, 0x6700
	ctx.r[9].s64 = ctx.r[11].s64 + 26368;
	// 82EF96BC: 390AB820  addi r8, r10, -0x47e0
	ctx.r[8].s64 = ctx.r[10].s64 + -18400;
	// 82EF96C0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82EF96C4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82EF96C8: 3CE08209  lis r7, -0x7df7
	ctx.r[7].s64 = -2113339392;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF9AA8 size=3416
    let mut pc: u32 = 0x82EF9AA8;
    'dispatch: loop {
        match pc {
            0x82EF9AA8 => {
    //   block [0x82EF9AA8..0x82EF9BE0)
	// 82EF9AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9AAC: 482AE685  bl 0x831a8130
	ctx.lr = 0x82EF9AB0;
	sub_831A8130(ctx, base);
	// 82EF9AB0: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82EF9AB4: 482AEFAD  bl 0x831a8a60
	ctx.lr = 0x82EF9AB8;
	sub_831A8A40(ctx, base);
	// 82EF9AB8: 9421FD00  stwu r1, -0x300(r1)
	ea = ctx.r[1].u32.wrapping_add(-768 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9ABC: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 82EF9AC0: 90C1032C  stw r6, 0x32c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(812 as u32), ctx.r[6].u32 ) };
	// 82EF9AC4: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 82EF9AC8: 9101033C  stw r8, 0x33c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(828 as u32), ctx.r[8].u32 ) };
	// 82EF9ACC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF9AD0: 3567FFFF  addic. r11, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9AD4: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 82EF9AD8: 93C10324  stw r30, 0x324(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(804 as u32), ctx.r[30].u32 ) };
	// 82EF9ADC: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82EF9AE0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82EF9AE4: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82EF9AE8: 41800D08  blt 0x82efa7f0
	if ctx.cr[0].lt {
	pc = 0x82EFA7F0; continue 'dispatch;
	}
	// 82EF9AEC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EF9AF0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EF9AF4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF9AF8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82EF9AFC: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82EF9B00: C389B834  lfs f28, -0x47cc(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-18380 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82EF9B04: 3CC08209  lis r6, -0x7df7
	ctx.r[6].s64 = -2113339392;
	// 82EF9B08: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82EF9B0C: C3A8B838  lfs f29, -0x47c8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-18376 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82EF9B10: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EF9B14: C2CBB830  lfs f22, -0x47d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18384 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82EF9B18: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 82EF9B1C: C30A7590  lfs f24, 0x7590(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30096 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 82EF9B20: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 82EF9B24: C3C7B83C  lfs f30, -0x47c4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-18372 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EF9B28: 3FE08200  lis r31, -0x7e00
	ctx.r[31].s64 = -2113929216;
	// 82EF9B2C: C366A100  lfs f27, -0x5f00(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-24320 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82EF9B30: 3CA08212  lis r5, -0x7dee
	ctx.r[5].s64 = -2112749568;
	// 82EF9B34: C324B840  lfs f25, -0x47c0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-18368 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82EF9B38: C2E908A4  lfs f23, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82EF9B3C: 3B6869F0  addi r27, r8, 0x69f0
	ctx.r[27].s64 = ctx.r[8].s64 + 27120;
	// 82EF9B40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EF9B44: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82EF9B48: C3439450  lfs f26, -0x6bb0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 82EF9B4C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82EF9B50: C3FF08A8  lfs f31, 0x8a8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EF9B54: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82EF9B58: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82EF9B5C: 3CC08332  lis r6, -0x7cce
	ctx.r[6].s64 = -2093875200;
	// 82EF9B60: 3C808212  lis r4, -0x7dee
	ctx.r[4].s64 = -2112749568;
	// 82EF9B64: 3905FFA0  addi r8, r5, -0x60
	ctx.r[8].s64 = ctx.r[5].s64 + -96;
	// 82EF9B68: 3A800020  li r20, 0x20
	ctx.r[20].s64 = 32;
	// 82EF9B6C: 3AA00010  li r21, 0x10
	ctx.r[21].s64 = 16;
	// 82EF9B70: 91010088  stw r8, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[8].u32 ) };
	// 82EF9B74: 39C00030  li r14, 0x30
	ctx.r[14].s64 = 48;
	// 82EF9B78: 3B0BB820  addi r24, r11, -0x47e0
	ctx.r[24].s64 = ctx.r[11].s64 + -18400;
	// 82EF9B7C: 3AEA6700  addi r23, r10, 0x6700
	ctx.r[23].s64 = ctx.r[10].s64 + 26368;
	// 82EF9B80: 3AC90028  addi r22, r9, 0x28
	ctx.r[22].s64 = ctx.r[9].s64 + 40;
	// 82EF9B84: 3A27BD40  addi r17, r7, -0x42c0
	ctx.r[17].s64 = ctx.r[7].s64 + -17088;
	// 82EF9B88: 3A06F3B0  addi r16, r6, -0xc50
	ctx.r[16].s64 = ctx.r[6].s64 + -3152;
	// 82EF9B8C: 3A44FF90  addi r18, r4, -0x70
	ctx.r[18].s64 = ctx.r[4].s64 + -112;
	// 82EF9B90: 48000008  b 0x82ef9b98
	pc = 0x82EF9B98; continue 'dispatch;
	// 82EF9B94: 83C10324  lwz r30, 0x324(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(804 as u32) ) } as u64;
	// 82EF9B98: 8161032C  lwz r11, 0x32c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(812 as u32) ) } as u64;
	// 82EF9B9C: 8141033C  lwz r10, 0x33c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(828 as u32) ) } as u64;
	// 82EF9BA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9BA4: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF9BA8: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9BAC: 396BFFFA  addi r11, r11, -6
	ctx.r[11].s64 = ctx.r[11].s64 + -6;
	// 82EF9BB0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82EF9BB4: 419905BC  bgt cr6, 0x82efa170
	if ctx.cr[6].gt {
	pc = 0x82EFA170; continue 'dispatch;
	}
	// 82EF9BB8: 3D8082F0  lis r12, -0x7d10
	ctx.r[12].s64 = -2098200576;
	// 82EF9BBC: 398C9BD0  addi r12, r12, -0x6430
	ctx.r[12].s64 = ctx.r[12].s64 + -25648;
	// 82EF9BC0: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82EF9BC4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82EF9BC8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82EF9BCC: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82EFA1E8; continue 'dispatch;
		},
		1 => {
	pc = 0x82EFA7D0; continue 'dispatch;
		},
		2 => {
	pc = 0x82EF9BE0; continue 'dispatch;
		},
		3 => {
	pc = 0x82EFA184; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82EF9BD0: 82EFA1E8  lwz r23, -0x5e18(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-24088 as u32) ) } as u64;
	// 82EF9BD4: 82EFA7D0  lwz r23, -0x5830(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-22576 as u32) ) } as u64;
	// 82EF9BD8: 82EF9BE0  lwz r23, -0x6420(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-25632 as u32) ) } as u64;
	// 82EF9BDC: 82EFA184  lwz r23, -0x5e7c(r15)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-24188 as u32) ) } as u64;
            }
            0x82EF9BE0 => {
    //   block [0x82EF9BE0..0x82EFA184)
	// 82EF9BE0: 3B9F00D0  addi r28, r31, 0xd0
	ctx.r[28].s64 = ctx.r[31].s64 + 208;
	pc = 0x82EFA184; continue 'dispatch;
            }
            0x82EFA184 => {
    //   block [0x82EFA184..0x82EFA1E8)
	// 82EFA184: C0130008  lfs f0, 8(r19)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFA188: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82EFA18C: C1BF00B4  lfs f13, 0xb4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFA190: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 82EFA194: ED8DF83C  fnmsubs f12, f13, f0, f31
	ctx.f[12].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82EFA198: 395F00E0  addi r10, r31, 0xe0
	ctx.r[10].s64 = ctx.r[31].s64 + 224;
	// 82EFA19C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	pc = 0x82EFA1E8; continue 'dispatch;
            }
            0x82EFA1E8 => {
    //   block [0x82EFA1E8..0x82EFA7D0)
	// 82EFA1E8: 3B9F00D0  addi r28, r31, 0xd0
	ctx.r[28].s64 = ctx.r[31].s64 + 208;
	pc = 0x82EFA7D0; continue 'dispatch;
            }
            0x82EFA7D0 => {
    //   block [0x82EFA7D0..0x82EFA800)
	// 82EFA7D0: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EFA7D4: 8141032C  lwz r10, 0x32c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(812 as u32) ) } as u64;
	// 82EFA7D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA7DC: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82EFA7E0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82EFA7E4: 9121032C  stw r9, 0x32c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(812 as u32), ctx.r[9].u32 ) };
	// 82EFA7E8: 4080F3AC  bge 0x82ef9b94
	if !ctx.cr[0].lt {
	pc = 0x82EF9B94; continue 'dispatch;
	}
	// 82EFA7EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFA7F0: 38210300  addi r1, r1, 0x300
	ctx.r[1].s64 = ctx.r[1].s64 + 768;
	// 82EFA7F4: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82EFA7F8: 482AE2B5  bl 0x831a8aac
	ctx.lr = 0x82EFA7FC;
	sub_831A8A8C(ctx, base);
	// 82EFA7FC: 482AD984  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EFA800 size=1100
    //   switch @ 0x82EFA8F8: r11 with 10 label(s)
    //       case  0 → 0x82EFA924
    //       case  1 → 0x82EFAC04
    //       case  2 → 0x82EFAA4C
    //       case  3 → 0x82EFAA4C
    //       case  4 → 0x82EFA924
    //       case  5 → 0x82EFA924
    //       case  6 → 0x82EFABA8
    //       case  7 → 0x82EFAB48
    //       case  8 → 0x82EFA924
    //       case  9 → 0x82EFAA40
    let mut pc: u32 = 0x82EFA800;
    'dispatch: loop {
        match pc {
            0x82EFA800 => {
    //   block [0x82EFA800..0x82EFA924)
	// 82EFA800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA804: 482AD92D  bl 0x831a8130
	ctx.lr = 0x82EFA808;
	sub_831A8130(ctx, base);
	// 82EFA808: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82EFA80C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82EFA810: 3980FF20  li r12, -0xe0
	ctx.r[12].s64 = -224;
	pc = 0x82EFA924; continue 'dispatch;
            }
            0x82EFA924 => {
    //   block [0x82EFA924..0x82EFAA40)
	// 82EFA924: C0170008  lfs f0, 8(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFA928: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82EFA92C: C1BE00B4  lfs f13, 0xb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFA930: 3B9E00D0  addi r28, r30, 0xd0
	ctx.r[28].s64 = ctx.r[30].s64 + 208;
	// 82EFA934: ED8DF83C  fnmsubs f12, f13, f0, f31
	ctx.f[12].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82EFA938: 3BBE00E0  addi r29, r30, 0xe0
	ctx.r[29].s64 = ctx.r[30].s64 + 224;
	// 82EFA93C: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 82EFA940: 3B7E0010  addi r27, r30, 0x10
	ctx.r[27].s64 = ctx.r[30].s64 + 16;
	// 82EFA944: 3B5FFFE0  addi r26, r31, -0x20
	ctx.r[26].s64 = ctx.r[31].s64 + -32;
	pc = 0x82EFAA40; continue 'dispatch;
            }
            0x82EFAA40 => {
    //   block [0x82EFAA40..0x82EFAA4C)
	// 82EFAA40: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EFAA44: 99730000  stb r11, 0(r19)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EFAA48: 48000008  b 0x82efaa50
	pc = 0x82EFAA50; continue 'dispatch;
            }
            0x82EFAA4C => {
    //   block [0x82EFAA4C..0x82EFAB48)
	// 82EFAA4C: 99D30000  stb r14, 0(r19)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[14].u8 ) };
	// 82EFAA50: C0170008  lfs f0, 8(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFAA54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82EFAA58: C1BE00B4  lfs f13, 0xb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFAA5C: 395E00D0  addi r10, r30, 0xd0
	ctx.r[10].s64 = ctx.r[30].s64 + 208;
	// 82EFAA60: ED8DF83C  fnmsubs f12, f13, f0, f31
	ctx.f[12].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82EFAA64: 3BA10054  addi r29, r1, 0x54
	ctx.r[29].s64 = ctx.r[1].s64 + 84;
	// 82EFAA68: 397E00E0  addi r11, r30, 0xe0
	ctx.r[11].s64 = ctx.r[30].s64 + 224;
	pc = 0x82EFAB48; continue 'dispatch;
            }
            0x82EFAB48 => {
    //   block [0x82EFAB48..0x82EFABA8)
	// 82EFAB48: 397FFFC0  addi r11, r31, -0x40
	ctx.r[11].s64 = ctx.r[31].s64 + -64;
	// 82EFAB4C: 9B130000  stb r24, 0(r19)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 82EFAB50: 9B1FFF91  stb r24, -0x6f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-111 as u32), ctx.r[24].u8 ) };
	// 82EFAB54: 395FFFA0  addi r10, r31, -0x60
	ctx.r[10].s64 = ctx.r[31].s64 + -96;
	// 82EFAB58: 393FFFB0  addi r9, r31, -0x50
	ctx.r[9].s64 = ctx.r[31].s64 + -80;
	pc = 0x82EFABA8; continue 'dispatch;
            }
            0x82EFABA8 => {
    //   block [0x82EFABA8..0x82EFAC04)
	// 82EFABA8: 397FFFC0  addi r11, r31, -0x40
	ctx.r[11].s64 = ctx.r[31].s64 + -64;
	// 82EFABAC: 9B130000  stb r24, 0(r19)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 82EFABB0: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	pc = 0x82EFAC04; continue 'dispatch;
            }
            0x82EFAC04 => {
    //   block [0x82EFAC04..0x82EFAC4C)
	// 82EFAC04: 3565FFFF  addic. r11, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFAC08: 39440004  addi r10, r4, 4
	ctx.r[10].s64 = ctx.r[4].s64 + 4;
	// 82EFAC0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFAC10: 3A730080  addi r19, r19, 0x80
	ctx.r[19].s64 = ctx.r[19].s64 + 128;
	// 82EFAC14: 9141016C  stw r10, 0x16c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), ctx.r[10].u32 ) };
	// 82EFAC18: 3BFF0080  addi r31, r31, 0x80
	ctx.r[31].s64 = ctx.r[31].s64 + 128;
	// 82EFAC1C: 4080FC88  bge 0x82efa8a4
	if !ctx.cr[0].lt {
	pc = 0x82EFA8A4; continue 'dispatch;
	}
	// 82EFAC20: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82EFAC24: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82EFAC28: 3800FF20  li r0, -0xe0
	ctx.r[0].s64 = -224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAC50 size=2636
    //   switch @ 0x82EFADA8: r11 with 10 label(s)
    //       case  0 → 0x82EFADD4
    //       case  1 → 0x82EFB254
    //       case  2 → 0x82EFAEE4
    //       case  3 → 0x82EFAEAC
    //       case  4 → 0x82EFAE28
    //       case  5 → 0x82EFAE7C
    //       case  6 → 0x82EFAED0
    //       case  7 → 0x82EFB254
    //       case  8 → 0x82EFADD4
    //       case  9 → 0x82EFAEE4
    let mut pc: u32 = 0x82EFAC50;
    'dispatch: loop {
        match pc {
            0x82EFAC50 => {
    //   block [0x82EFAC50..0x82EFADD4)
	// 82EFAC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAC54: 482AD4DD  bl 0x831a8130
	ctx.lr = 0x82EFAC58;
	sub_831A8130(ctx, base);
	// 82EFAC58: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82EFAC5C: 482ADE09  bl 0x831a8a64
	ctx.lr = 0x82EFAC60;
	sub_831A8A40(ctx, base);
	// 82EFAC60: 3980FF10  li r12, -0xf0
	ctx.r[12].s64 = -240;
	pc = 0x82EFADD4; continue 'dispatch;
            }
            0x82EFADD4 => {
    //   block [0x82EFADD4..0x82EFAE28)
	// 82EFADD4: E97CFFE0  ld r11, -0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(-32 as u32) ) };
	// 82EFADD8: 39410100  addi r10, r1, 0x100
	ctx.r[10].s64 = ctx.r[1].s64 + 256;
	// 82EFADDC: E93CFFE8  ld r9, -0x18(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(-24 as u32) ) };
	// 82EFADE0: 38E10100  addi r7, r1, 0x100
	ctx.r[7].s64 = ctx.r[1].s64 + 256;
	// 82EFADE4: 39010100  addi r8, r1, 0x100
	ctx.r[8].s64 = ctx.r[1].s64 + 256;
	// 82EFADE8: 80C100A0  lwz r6, 0xa0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFADEC: 38A10100  addi r5, r1, 0x100
	ctx.r[5].s64 = ctx.r[1].s64 + 256;
	// 82EFADF0: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EFADF4: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82EFADF8: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82EFADFC: F92A0008  std r9, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	pc = 0x82EFAE28; continue 'dispatch;
            }
            0x82EFAE28 => {
    //   block [0x82EFAE28..0x82EFAE7C)
	// 82EFAE28: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	pc = 0x82EFAE7C; continue 'dispatch;
            }
            0x82EFAE7C => {
    //   block [0x82EFAE7C..0x82EFAEAC)
	// 82EFAE7C: 38BC0010  addi r5, r28, 0x10
	ctx.r[5].s64 = ctx.r[28].s64 + 16;
	pc = 0x82EFAEAC; continue 'dispatch;
            }
            0x82EFAEAC => {
    //   block [0x82EFAEAC..0x82EFAED0)
	pc = 0x82EFAED0; continue 'dispatch;
            }
            0x82EFAED0 => {
    //   block [0x82EFAED0..0x82EFAEE4)
	// 82EFAED0: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	pc = 0x82EFAEE4; continue 'dispatch;
            }
            0x82EFAEE4 => {
    //   block [0x82EFAEE4..0x82EFB254)
	pc = 0x82EFB254; continue 'dispatch;
            }
            0x82EFB254 => {
    //   block [0x82EFB254..0x82EFB69C)
	// 82EFB254: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB258: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82EFB25C: 409A0064  bne cr6, 0x82efb2c0
	if !ctx.cr[6].eq {
	pc = 0x82EFB2C0; continue 'dispatch;
	}
	// 82EFB260: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB6A0 size=96
    let mut pc: u32 = 0x82EFB6A0;
    'dispatch: loop {
        match pc {
            0x82EFB6A0 => {
    //   block [0x82EFB6A0..0x82EFB700)
	// 82EFB6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB6B4: 390000D0  li r8, 0xd0
	ctx.r[8].s64 = 208;
	// 82EFB6B8: 38A50080  addi r5, r5, 0x80
	ctx.r[5].s64 = ctx.r[5].s64 + 128;
	// 82EFB6BC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82EFB6C0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82EFB6C4: 4BFFF58D  bl 0x82efac50
	ctx.lr = 0x82EFB6C8;
	sub_82EFAC50(ctx, base);
	// 82EFB6C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB6CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFB6D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFB6D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB6D8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB6DC: 806A006C  lwz r3, 0x6c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EFB6E0: 4BFFB621  bl 0x82ef6d00
	ctx.lr = 0x82EFB6E4;
	sub_82EF6D00(ctx, base);
	// 82EFB6E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB6E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB6F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB6F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EFB700 size=288
    let mut pc: u32 = 0x82EFB700;
    'dispatch: loop {
        match pc {
            0x82EFB700 => {
    //   block [0x82EFB700..0x82EFB820)
	// 82EFB700: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EFB704: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82EFB708: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82EFB70C: 7D6A2670  srawi r10, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82EFB710: 3926000F  addi r9, r6, 0xf
	ctx.r[9].s64 = ctx.r[6].s64 + 15;
	// 82EFB714: 7D0A0194  addze r8, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82EFB718: 552A0036  rlwinm r10, r9, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFB71C: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EFB720: 55052036  slwi r5, r8, 4
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFB724: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFB728: 7C655850  subf r3, r5, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82EFB72C: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82EFB730: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82EFB734: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EFB738: 7D633850  subf r11, r3, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[3].s64;
	// 82EFB73C: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82EFB740: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EFB744: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFB748: C1A4B848  lfs f13, -0x47b8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-18360 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFB74C: 7C8B4214  add r4, r11, r8
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EFB750: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EFB754: 5489083C  slwi r9, r4, 1
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFB758: C005B844  lfs f0, -0x47bc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-18364 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFB75C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EFB760: 7D273670  srawi r7, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 82EFB764: 54651838  slwi r5, r3, 3
	ctx.r[5].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFB768: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82EFB76C: 7D043670  srawi r4, r8, 6
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[8].s32 >> 6) as i64;
	// 82EFB770: 7CA807B4  extsw r8, r5
	ctx.r[8].s64 = ctx.r[5].s32 as i64;
	// 82EFB774: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFB778: 7C695214  add r3, r9, r10
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EFB77C: 90BF0014  stw r5, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82EFB780: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 82EFB784: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB788: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 82EFB78C: 546A0036  rlwinm r10, r3, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFB790: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82EFB794: 7D240194  addze r9, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82EFB798: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFB79C: 7C865A14  add r4, r6, r11
	ctx.r[4].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82EFB7A0: 7CE95214  add r7, r9, r10
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EFB7A4: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82EFB7A8: 909F0044  stw r4, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 82EFB7AC: 54E30036  rlwinm r3, r7, 0, 0, 0x1b
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFB7B0: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFB7B4: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82EFB7B8: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82EFB7BC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EFB7C0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EFB7C4: ED2A0372  fmuls f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 82EFB7C8: ED0A0032  fmuls f8, f10, f0
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82EFB7CC: FCE04E5E  fctidz f7, f9
	ctx.f[7].s64 = if ctx.f[9].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[9].f64.trunc() as i64 };
	// 82EFB7D0: D8E1FFF0  stfd f7, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[7].u64 ) };
	// 82EFB7D4: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EFB7D8: FCC0465E  fctidz f6, f8
	ctx.f[6].s64 = if ctx.f[8].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[8].f64.trunc() as i64 };
	// 82EFB7DC: D8C1FFF0  stfd f6, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[6].u64 ) };
	// 82EFB7E0: 80E1FFF4  lwz r7, -0xc(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EFB7E4: 7D2A1850  subf r9, r10, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 82EFB7E8: 55280036  rlwinm r8, r9, 0, 0, 0x1b
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFB7EC: 7CC74050  subf r6, r7, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82EFB7F0: 911F0030  stw r8, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82EFB7F4: 5504003E  slwi r4, r8, 0
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EFB7F8: 911F0024  stw r8, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 82EFB7FC: 54C50036  rlwinm r5, r6, 0, 0, 0x1b
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFB800: 909F0038  stw r4, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 82EFB804: 90BF0034  stw r5, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[5].u32 ) };
	// 82EFB808: 90BF0040  stw r5, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[5].u32 ) };
	// 82EFB80C: 90BF0048  stw r5, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[5].u32 ) };
	// 82EFB810: 90BF004C  stw r5, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 82EFB814: 909F003C  stw r4, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 82EFB818: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EFB81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB820 size=4
    let mut pc: u32 = 0x82EFB820;
    'dispatch: loop {
        match pc {
            0x82EFB820 => {
    //   block [0x82EFB820..0x82EFB824)
	// 82EFB820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB828 size=60
    let mut pc: u32 = 0x82EFB828;
    'dispatch: loop {
        match pc {
            0x82EFB828 => {
    //   block [0x82EFB828..0x82EFB864)
	// 82EFB828: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFB82C: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFB830: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFB834: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82EFB838: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EFB83C: 409A0008  bne cr6, 0x82efb844
	if !ctx.cr[6].eq {
	pc = 0x82EFB844; continue 'dispatch;
	}
	// 82EFB840: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82EFB844: 392BFFF0  addi r9, r11, -0x10
	ctx.r[9].s64 = ctx.r[11].s64 + -16;
	// 82EFB848: 54CA3830  slwi r10, r6, 7
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFB84C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB850: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB854: 41990008  bgt cr6, 0x82efb85c
	if ctx.cr[6].gt {
	pc = 0x82EFB85C; continue 'dispatch;
	}
	// 82EFB858: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB85C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EFB860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFB868 size=196
    let mut pc: u32 = 0x82EFB868;
    'dispatch: loop {
        match pc {
            0x82EFB868 => {
    //   block [0x82EFB868..0x82EFB92C)
	// 82EFB868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB87C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EFB880: 409A0014  bne cr6, 0x82efb894
	if !ctx.cr[6].eq {
	pc = 0x82EFB894; continue 'dispatch;
	}
	// 82EFB884: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFB888: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFB88C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EFB890: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82EFB894: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EFB898: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EFB89C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFB8A0: 40980020  bge cr6, 0x82efb8c0
	if !ctx.cr[6].lt {
	pc = 0x82EFB8C0; continue 'dispatch;
	}
	// 82EFB8A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EFB8A8: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFB8AC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EFB8B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EFB8B4: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EFB8B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFB8BC: 4198FFF0  blt cr6, 0x82efb8ac
	if ctx.cr[6].lt {
	pc = 0x82EFB8AC; continue 'dispatch;
	}
	// 82EFB8C0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EFB8C4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFB8C8: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFB8CC: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFB8D0: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82EFB8D4: 419A0014  beq cr6, 0x82efb8e8
	if ctx.cr[6].eq {
	pc = 0x82EFB8E8; continue 'dispatch;
	}
	// 82EFB8D8: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EFB8DC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFB8E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB8E4: 480108AD  bl 0x82f0c190
	ctx.lr = 0x82EFB8E8;
	sub_82F0C190(ctx, base);
	// 82EFB8E8: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EFB8EC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EFB8F0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFB8F4: 419A0014  beq cr6, 0x82efb908
	if ctx.cr[6].eq {
	pc = 0x82EFB908; continue 'dispatch;
	}
	// 82EFB8F8: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EFB8FC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFB900: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB904: 4801088D  bl 0x82f0c190
	ctx.lr = 0x82EFB908;
	sub_82F0C190(ctx, base);
	// 82EFB908: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFB90C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EFB910: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82EFB914: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82EFB918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB930 size=272
    let mut pc: u32 = 0x82EFB930;
    'dispatch: loop {
        match pc {
            0x82EFB930 => {
    //   block [0x82EFB930..0x82EFBA40)
	// 82EFB930: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EFB934: 90860014  stw r4, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82EFB938: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82EFB93C: 80A3004C  lwz r5, 0x4c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EFB940: 549F003E  slwi r31, r4, 0
	ctx.r[31].u32 = ctx.r[4].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EFB944: 90860030  stw r4, 0x30(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82EFB948: 8123004C  lwz r9, 0x4c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EFB94C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB950: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFB954: 7D485BD6  divw r10, r8, r11
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 82EFB958: 552B3830  slwi r11, r9, 7
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFB95C: 38EA0004  addi r7, r10, 4
	ctx.r[7].s64 = ctx.r[10].s64 + 4;
	// 82EFB960: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EFB964: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFB968: 394A0090  addi r10, r10, 0x90
	ctx.r[10].s64 = ctx.r[10].s64 + 144;
	// 82EFB96C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFB970: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82EFB974: 9146003C  stw r10, 0x3c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82EFB978: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EFB97C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFB980: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFB984: 54872036  slwi r7, r4, 4
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EFB988: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EFB98C: 7D674A14  add r11, r7, r9
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82EFB990: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFB994: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB998: 39490017  addi r10, r9, 0x17
	ctx.r[10].s64 = ctx.r[9].s64 + 23;
	// 82EFB99C: 91660040  stw r11, 0x40(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82EFB9A0: 55490036  rlwinm r9, r10, 0, 0, 0x1b
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFB9A4: 7D5F4850  subf r10, r31, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[31].s64;
	// 82EFB9A8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFB9AC: 90660018  stw r3, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82EFB9B0: 419A0088  beq cr6, 0x82efba38
	if ctx.cr[6].eq {
	pc = 0x82EFBA38; continue 'dispatch;
	}
	// 82EFB9B4: 81660030  lwz r11, 0x30(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFB9B8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EFB9BC: 3881FFE0  addi r4, r1, -0x20
	ctx.r[4].s64 = ctx.r[1].s64 + -32;
	// 82EFB9C0: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 82EFB9C4: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82EFB9C8: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82EFB9CC: 98CB0000  stb r6, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 82EFB9D0: 38EB0040  addi r7, r11, 0x40
	ctx.r[7].s64 = ctx.r[11].s64 + 64;
	// 82EFB9D4: 98CB0001  stb r6, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[6].u8 ) };
	// 82EFB9D8: 38C50001  addi r6, r5, 1
	ctx.r[6].s64 = ctx.r[5].s64 + 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBA40 size=400
    let mut pc: u32 = 0x82EFBA40;
    'dispatch: loop {
        match pc {
            0x82EFBA40 => {
    //   block [0x82EFBA40..0x82EFBBD0)
	// 82EFBA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBA44: 482AC721  bl 0x831a8164
	ctx.lr = 0x82EFBA48;
	sub_831A8130(ctx, base);
	// 82EFBA48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBA4C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EFBA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBA54: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EFBA58: 419A0170  beq cr6, 0x82efbbc8
	if ctx.cr[6].eq {
	pc = 0x82EFBBC8; continue 'dispatch;
	}
	// 82EFBA5C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFBA60: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82EFBA64: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFBA68: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFBA6C: 409A0098  bne cr6, 0x82efbb04
	if !ctx.cr[6].eq {
	pc = 0x82EFBB04; continue 'dispatch;
	}
	// 82EFBA70: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 82EFBA74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EFBA78: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82EFBA7C: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82EFBA80: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EFBA84: 990B0001  stb r8, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 82EFBA88: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EFBBD0 size=200
    let mut pc: u32 = 0x82EFBBD0;
    'dispatch: loop {
        match pc {
            0x82EFBBD0 => {
    //   block [0x82EFBBD0..0x82EFBC98)
	// 82EFBBD0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82EFBBD4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EFBBD8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82EFBBDC: 81240038  lwz r9, 0x38(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EFBBE0: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFBBE4: 81040048  lwz r8, 0x48(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EFBBE8: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFBBEC: 81440028  lwz r10, 0x28(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFBBF0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EFBBF4: 9121FFE4  stw r9, -0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), ctx.r[9].u32 ) };
	// 82EFBBF8: 9141FFE0  stw r10, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[10].u32 ) };
	// 82EFBBFC: 40980050  bge cr6, 0x82efbc4c
	if !ctx.cr[6].lt {
	pc = 0x82EFBC4C; continue 'dispatch;
	}
	// 82EFBC00: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EFBC04: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC08: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82EFBC0C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EFBC10: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EFBC14: 88E9001C  lbz r7, 0x1c(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBC18: 80C90028  lwz r6, 0x28(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFBC1C: 7D253810  subfc r9, r5, r7
	ctx.xer.ca = ctx.r[7].u32 >= ctx.r[5].u32;
	ctx.r[9].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 82EFBC20: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82EFBC24: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EFBC28: A3C60018  lhz r30, 0x18(r6)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBC2C: A0E60014  lhz r7, 0x14(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFBC30: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFBC34: 57C6103E  rotlwi r6, r30, 2
	ctx.r[6].u64 = ((ctx.r[30].u32).rotate_left(2)) as u64;
	// 82EFBC38: 7D064214  add r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 82EFBC3C: 7CC9502E  lwzx r6, r9, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EFBC40: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82EFBC44: 7CE9512E  stwx r7, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 82EFBC48: 4198FFBC  blt cr6, 0x82efbc04
	if ctx.cr[6].lt {
	pc = 0x82EFBC04; continue 'dispatch;
	}
	// 82EFBC4C: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EFBC50: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFBC54: 41990030  bgt cr6, 0x82efbc84
	if ctx.cr[6].gt {
	pc = 0x82EFBC84; continue 'dispatch;
	}
	// 82EFBC58: 8161FFE0  lwz r11, -0x20(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 82EFBC5C: 81440024  lwz r10, 0x24(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFBC60: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82EFBC64: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFBC68: 4199001C  bgt cr6, 0x82efbc84
	if ctx.cr[6].gt {
	pc = 0x82EFBC84; continue 'dispatch;
	}
	// 82EFBC6C: 8161FFE4  lwz r11, -0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82EFBC70: 81440034  lwz r10, 0x34(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFBC74: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82EFBC78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFBC7C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFBC80: 40990008  ble cr6, 0x82efbc88
	if !ctx.cr[6].gt {
	pc = 0x82EFBC88; continue 'dispatch;
	}
	// 82EFBC84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFBC88: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EFBC8C: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBC90: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EFBC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBC98 size=300
    let mut pc: u32 = 0x82EFBC98;
    'dispatch: loop {
        match pc {
            0x82EFBC98 => {
    //   block [0x82EFBC98..0x82EFBDC4)
	// 82EFBC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBC9C: 482AC49D  bl 0x831a8138
	ctx.lr = 0x82EFBCA0;
	sub_831A8130(ctx, base);
	// 82EFBCA0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBCA4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82EFBCA8: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82EFBCAC: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82EFBCB0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82EFBCB4: 40990108  ble cr6, 0x82efbdbc
	if !ctx.cr[6].gt {
	pc = 0x82EFBDBC; continue 'dispatch;
	}
	// 82EFBCB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EFBCBC: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82EFBCC0: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82EFBCC4: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82EFBCC8: 3B200030  li r25, 0x30
	ctx.r[25].s64 = 48;
	// 82EFBCCC: 3B400040  li r26, 0x40
	ctx.r[26].s64 = 64;
	// 82EFBCD0: 3B6000FF  li r27, 0xff
	ctx.r[27].s64 = 255;
	// 82EFBCD4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBCD8: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82EFBCDC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82EFBCE0: 38AA00D0  addi r5, r10, 0xd0
	ctx.r[5].s64 = ctx.r[10].s64 + 208;
	// 82EFBCE4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82EFBCE8: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82EFBCEC: 3AA100A0  addi r21, r1, 0xa0
	ctx.r[21].s64 = ctx.r[1].s64 + 160;
	// 82EFBCF0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82EFBCF4: 3A8100B0  addi r20, r1, 0xb0
	ctx.r[20].s64 = ctx.r[1].s64 + 176;
	// 82EFBCF8: 816A0090  lwz r11, 0x90(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFBCFC: 7D6BB214  add r11, r11, r22
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 82EFBD00: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBD04: 3A6100C0  addi r19, r1, 0xc0
	ctx.r[19].s64 = ctx.r[1].s64 + 192;
	// 82EFBD08: 98610060  stb r3, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u8 ) };
	// 82EFBD0C: 3A4100D0  addi r18, r1, 0xd0
	ctx.r[18].s64 = ctx.r[1].s64 + 208;
	// 82EFBD10: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82EFBD14: 3A2100A0  addi r17, r1, 0xa0
	ctx.r[17].s64 = ctx.r[1].s64 + 160;
	// 82EFBD18: 99210061  stb r9, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[9].u8 ) };
	// 82EFBD1C: 3A0100B0  addi r16, r1, 0xb0
	ctx.r[16].s64 = ctx.r[1].s64 + 176;
	// 82EFBD20: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBD24: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82EFBD28: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBDC8 size=656
    let mut pc: u32 = 0x82EFBDC8;
    'dispatch: loop {
        match pc {
            0x82EFBDC8 => {
    //   block [0x82EFBDC8..0x82EFC058)
	// 82EFBDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBDCC: 482AC379  bl 0x831a8144
	ctx.lr = 0x82EFBDD0;
	sub_831A8130(ctx, base);
	// 82EFBDD0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBDD4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFBDD8: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFBDDC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EFBDE0: 7E6B2214  add r19, r11, r4
	ctx.r[19].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EFBDE4: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82EFBDE8: 83FC0008  lwz r31, 8(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBDEC: 7F049840  cmplw cr6, r4, r19
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[19].u32, &mut ctx.xer);
	// 82EFBDF0: 4098024C  bge cr6, 0x82efc03c
	if !ctx.cr[6].lt {
	pc = 0x82EFC03C; continue 'dispatch;
	}
	// 82EFBDF4: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 82EFBDF8: 83B50000  lwz r29, 0(r21)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBDFC: 813C000C  lwz r9, 0xc(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFBE00: 815D0028  lwz r10, 0x28(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFBE04: 897D001C  lbz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBE08: 3BCA0004  addi r30, r10, 4
	ctx.r[30].s64 = ctx.r[10].s64 + 4;
	// 82EFBE0C: 390BFFFC  addi r8, r11, -4
	ctx.r[8].s64 = ctx.r[11].s64 + -4;
	// 82EFBE10: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBE14: 7D060034  cntlzw r6, r8
	ctx.r[6].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82EFBE18: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBE1C: 390B00D0  addi r8, r11, 0xd0
	ctx.r[8].s64 = ctx.r[11].s64 + 208;
	// 82EFBE20: 38EA00D0  addi r7, r10, 0xd0
	ctx.r[7].s64 = ctx.r[10].s64 + 208;
	// 82EFBE24: 38A80010  addi r5, r8, 0x10
	ctx.r[5].s64 = ctx.r[8].s64 + 16;
	// 82EFBE28: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFBE2C: 38870010  addi r4, r7, 0x10
	ctx.r[4].s64 = ctx.r[7].s64 + 16;
	// 82EFBE30: 814A0090  lwz r10, 0x90(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFBE34: 54D6DFFE  rlwinm r22, r6, 0x1b, 0x1f, 0x1f
	ctx.r[22].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 82EFBE38: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EFBE3C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFBE40: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EFBE44: 3B4B0040  addi r26, r11, 0x40
	ctx.r[26].s64 = ctx.r[11].s64 + 64;
	// 82EFBE48: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82EFBE4C: 3B0A0040  addi r24, r10, 0x40
	ctx.r[24].s64 = ctx.r[10].s64 + 64;
	// 82EFBE50: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBE54: 3AEA0050  addi r23, r10, 0x50
	ctx.r[23].s64 = ctx.r[10].s64 + 80;
	// 82EFBE58: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBE5C: 3B2B0050  addi r25, r11, 0x50
	ctx.r[25].s64 = ctx.r[11].s64 + 80;
	// 82EFBE60: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBE64: 81030090  lwz r8, 0x90(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFBE68: 80E90090  lwz r7, 0x90(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFBE6C: 54EBF0B4  rlwinm r11, r7, 0x1e, 2, 0x1a
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 82EFBE70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFBE74: 5509F0B4  rlwinm r9, r8, 0x1e, 2, 0x1a
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 82EFBE78: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC058 size=168
    let mut pc: u32 = 0x82EFC058;
    'dispatch: loop {
        match pc {
            0x82EFC058 => {
    //   block [0x82EFC058..0x82EFC100)
	// 82EFC058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC064: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC06C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFC070: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EFC074: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EFC078: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82EFC07C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EFC080: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82EFC084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFC088: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82EFC08C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC090: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82EFC094: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82EFC098: 912100B4  stw r9, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[9].u32 ) };
	// 82EFC09C: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82EFC0A0: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 82EFC0A4: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82EFC0A8: 916100C4  stw r11, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82EFC0AC: 916100C8  stw r11, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82EFC0B0: B16100CC  sth r11, 0xcc(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[11].u16 ) };
	// 82EFC0B4: 4BFFF87D  bl 0x82efb930
	ctx.lr = 0x82EFC0B8;
	sub_82EFB930(ctx, base);
	// 82EFC0B8: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EFC0BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC0C0: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFC0C4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EFC0C8: 409A0020  bne cr6, 0x82efc0e8
	if !ctx.cr[6].eq {
	pc = 0x82EFC0E8; continue 'dispatch;
	}
	// 82EFC0CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC0D0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EFC0D4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EFC0D8: 808100AC  lwz r4, 0xac(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EFC0DC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFC0E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EFC0E4: 4BFA46CD  bl 0x82ea07b0
	ctx.lr = 0x82EFC0E8;
	sub_82EA07B0(ctx, base);
	// 82EFC0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0EC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82EFC0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC100 size=2172
    let mut pc: u32 = 0x82EFC100;
    'dispatch: loop {
        match pc {
            0x82EFC100 => {
    //   block [0x82EFC100..0x82EFC97C)
	// 82EFC100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC104: 482AC02D  bl 0x831a8130
	ctx.lr = 0x82EFC108;
	sub_831A8130(ctx, base);
	// 82EFC108: 9421FAE0  stwu r1, -0x520(r1)
	ea = ctx.r[1].u32.wrapping_add(-1312 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC10C: 7CD23378  mr r18, r6
	ctx.r[18].u64 = ctx.r[6].u64;
	// 82EFC110: 820D0000  lwz r16, 0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC114: 39E00014  li r15, 0x14
	ctx.r[15].s64 = 20;
	// 82EFC118: 90610534  stw r3, 0x534(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1332 as u32), ctx.r[3].u32 ) };
	// 82EFC11C: 7D0E4378  mr r14, r8
	ctx.r[14].u64 = ctx.r[8].u64;
	// 82EFC120: 9081053C  stw r4, 0x53c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1340 as u32), ctx.r[4].u32 ) };
	// 82EFC124: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82EFC128: 90E10554  stw r7, 0x554(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1364 as u32), ctx.r[7].u32 ) };
	// 82EFC12C: 55CB3830  slwi r11, r14, 7
	ctx.r[11].u32 = ctx.r[14].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFC130: 81520014  lwz r10, 0x14(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC134: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFC138: 7C70782E  lwzx r3, r16, r15
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82EFC13C: 3B2B0090  addi r25, r11, 0x90
	ctx.r[25].s64 = ctx.r[11].s64 + 144;
	// 82EFC140: 8172000C  lwz r11, 0xc(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFC144: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82EFC148: 92810058  stw r20, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[20].u32 ) };
	// 82EFC14C: 7E93A378  mr r19, r20
	ctx.r[19].u64 = ctx.r[20].u64;
	// 82EFC150: 3B0B0010  addi r24, r11, 0x10
	ctx.r[24].s64 = ctx.r[11].s64 + 16;
	// 82EFC154: 555A103A  slwi r26, r10, 2
	ctx.r[26].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EFC158: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFC15C: 81030020  lwz r8, 0x20(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFC160: 7D7AC214  add r11, r26, r24
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[24].u64;
	// 82EFC164: 7D484850  subf r10, r8, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82EFC168: 7FEBCA14  add r31, r11, r25
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82EFC16C: 396AFFF0  addi r11, r10, -0x10
	ctx.r[11].s64 = ctx.r[10].s64 + -16;
	// 82EFC170: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EFC174: 41980008  blt cr6, 0x82efc17c
	if ctx.cr[6].lt {
	pc = 0x82EFC17C; continue 'dispatch;
	}
	// 82EFC178: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82EFC17C: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82EFC180: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFC184: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFC188: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFC18C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EFC190: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFC194: 41990010  bgt cr6, 0x82efc1a4
	if ctx.cr[6].gt {
	pc = 0x82EFC1A4; continue 'dispatch;
	}
	// 82EFC198: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFC19C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EFC1A0: 48000018  b 0x82efc1b8
	pc = 0x82EFC1B8; continue 'dispatch;
	// 82EFC1A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC1A8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC1AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFC1B0: 4E800421  bctrl
	ctx.lr = 0x82EFC1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC1B4: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EFC1B8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFC1BC: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFC1C0: 7E2BCA14  add r17, r11, r25
	ctx.r[17].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82EFC1C4: 7D755B78  mr r21, r11
	ctx.r[21].u64 = ctx.r[11].u64;
	// 82EFC1C8: 7D71C214  add r11, r17, r24
	ctx.r[11].u64 = ctx.r[17].u64 + ctx.r[24].u64;
	// 82EFC1CC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82EFC1D0: 7D765B78  mr r22, r11
	ctx.r[22].u64 = ctx.r[11].u64;
	// 82EFC1D4: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EFC1D8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFC1DC: 4099015C  ble cr6, 0x82efc338
	if !ctx.cr[6].gt {
	pc = 0x82EFC338; continue 'dispatch;
	}
	// 82EFC1E0: 7F115040  cmplw cr6, r17, r10
	ctx.cr[6].compare_u32(ctx.r[17].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFC1E4: 409800F8  bge cr6, 0x82efc2dc
	if !ctx.cr[6].lt {
	pc = 0x82EFC2DC; continue 'dispatch;
	}
	// 82EFC1E8: 81720008  lwz r11, 8(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC1EC: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFC1F0: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82EFC1F4: 41980098  blt cr6, 0x82efc28c
	if ctx.cr[6].lt {
	pc = 0x82EFC28C; continue 'dispatch;
	}
	// 82EFC1F8: 7F895050  subf r28, r9, r10
	ctx.r[28].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82EFC1FC: 7F71E051  subf. r27, r17, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[17].s64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EFC200: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82EFC204: 41810008  bgt 0x82efc20c
	if ctx.cr[0].gt {
	pc = 0x82EFC20C; continue 'dispatch;
	}
	// 82EFC208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFC20C: 8112000C  lwz r8, 0xc(r18)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFC210: 7D70782E  lwzx r11, r16, r15
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82EFC214: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82EFC218: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFC21C: 7FFDD214  add r31, r29, r26
	ctx.r[31].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 82EFC220: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFC224: 80EB002C  lwz r7, 0x2c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFC228: 38DF0010  addi r6, r31, 0x10
	ctx.r[6].s64 = ctx.r[31].s64 + 16;
	// 82EFC22C: 54C40036  rlwinm r4, r6, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFC230: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82EFC234: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EFC238: 4199000C  bgt cr6, 0x82efc244
	if ctx.cr[6].gt {
	pc = 0x82EFC244; continue 'dispatch;
	}
	// 82EFC23C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFC240: 48000018  b 0x82efc258
	pc = 0x82EFC258; continue 'dispatch;
	// 82EFC244: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC248: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFC24C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC250: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EFC254: 4E800421  bctrl
	ctx.lr = 0x82EFC258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC258: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82EFC25C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EFC260: 7D43FA14  add r10, r3, r31
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EFC264: 4098001C  bge cr6, 0x82efc280
	if !ctx.cr[6].lt {
	pc = 0x82EFC280; continue 'dispatch;
	}
	// 82EFC268: 8172000C  lwz r11, 0xc(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFC26C: 7C711B78  mr r17, r3
	ctx.r[17].u64 = ctx.r[3].u64;
	// 82EFC270: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82EFC274: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EFC278: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EFC27C: 4BFFFF50  b 0x82efc1cc
	pc = 0x82EFC1CC; continue 'dispatch;
	// 82EFC280: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82EFC284: 7D63EA14  add r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82EFC288: 4BFFFF48  b 0x82efc1d0
	pc = 0x82EFC1D0; continue 'dispatch;
	// 82EFC28C: 7C70782E  lwzx r3, r16, r15
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82EFC290: 397A0010  addi r11, r26, 0x10
	ctx.r[11].s64 = ctx.r[26].s64 + 16;
	// 82EFC294: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFC298: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFC29C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFC2A0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EFC2A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFC2A8: 41990014  bgt cr6, 0x82efc2bc
	if ctx.cr[6].gt {
	pc = 0x82EFC2BC; continue 'dispatch;
	}
	// 82EFC2AC: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFC2B0: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82EFC2B4: 7D4BD214  add r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EFC2B8: 4BFFFF18  b 0x82efc1d0
	pc = 0x82EFC1D0; continue 'dispatch;
	// 82EFC2BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC2C0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC2C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFC2C8: 4E800421  bctrl
	ctx.lr = 0x82EFC2CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC2CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFC2D0: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82EFC2D4: 7D4BD214  add r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EFC2D8: 4BFFFEF8  b 0x82efc1d0
	pc = 0x82EFC1D0; continue 'dispatch;
	// 82EFC2DC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFC2E0: 7C70782E  lwzx r3, r16, r15
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[15].u32)) } as u64;
	// 82EFC2E4: 7FEA5850  subf r31, r10, r11
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFC2E8: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 82EFC2EC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFC2F0: 55240036  rlwinm r4, r9, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFC2F4: 8103002C  lwz r8, 0x2c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFC2F8: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EFC2FC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EFC300: 41990018  bgt cr6, 0x82efc318
	if ctx.cr[6].gt {
	pc = 0x82EFC318; continue 'dispatch;
	}
	// 82EFC304: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFC308: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFC30C: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82EFC310: 7D43FA14  add r10, r3, r31
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EFC314: 4BFFFEAC  b 0x82efc1c0
	pc = 0x82EFC1C0; continue 'dispatch;
	// 82EFC318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC31C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC320: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFC324: 4E800421  bctrl
	ctx.lr = 0x82EFC328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC328: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82EFC32C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFC330: 7D43FA14  add r10, r3, r31
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EFC334: 4BFFFE8C  b 0x82efc1c0
	pc = 0x82EFC1C0; continue 'dispatch;
	// 82EFC338: 39750030  addi r11, r21, 0x30
	ctx.r[11].s64 = ctx.r[21].s64 + 48;
	// 82EFC33C: 83610554  lwz r27, 0x554(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1364 as u32) ) } as u64;
	// 82EFC340: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 82EFC344: 39550010  addi r10, r21, 0x10
	ctx.r[10].s64 = ctx.r[21].s64 + 16;
	// 82EFC348: 9B150000  stb r24, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 82EFC34C: 39350020  addi r9, r21, 0x20
	ctx.r[9].s64 = ctx.r[21].s64 + 32;
	// 82EFC350: 9B150001  stb r24, 1(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 82EFC354: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC980 size=108
    let mut pc: u32 = 0x82EFC980;
    'dispatch: loop {
        match pc {
            0x82EFC980 => {
    //   block [0x82EFC980..0x82EFC9EC)
	// 82EFC980: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC984: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82EFC988: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82EFC98C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82EFC990: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82EFC994: 39640050  addi r11, r4, 0x50
	ctx.r[11].s64 = ctx.r[4].s64 + 80;
	// 82EFC998: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EFC99C: 39430050  addi r10, r3, 0x50
	ctx.r[10].s64 = ctx.r[3].s64 + 80;
	// 82EFC9A0: 88A40001  lbz r5, 1(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 82EFC9A4: 98A30001  stb r5, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 82EFC9A8: 80A40004  lwz r5, 4(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC9AC: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFC9F0 size=500
    let mut pc: u32 = 0x82EFC9F0;
    'dispatch: loop {
        match pc {
            0x82EFC9F0 => {
    //   block [0x82EFC9F0..0x82EFCBE4)
	// 82EFC9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC9F4: 482AB75D  bl 0x831a8150
	ctx.lr = 0x82EFC9F8;
	sub_831A8130(ctx, base);
	// 82EFC9F8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC9FC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCA00: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82EFCA04: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFCA08: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82EFCA0C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFCA10: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFCA14: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCA18: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCA1C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFCA20: 4098002C  bge cr6, 0x82efca4c
	if !ctx.cr[6].lt {
	pc = 0x82EFCA4C; continue 'dispatch;
	}
	// 82EFCA24: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFCA28: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFCA2C: 38E9B888  addi r7, r9, -0x4778
	ctx.r[7].s64 = ctx.r[9].s64 + -18296;
	// 82EFCA30: 38C8B878  addi r6, r8, -0x4788
	ctx.r[6].s64 = ctx.r[8].s64 + -18312;
	// 82EFCA34: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFCA38: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFCA3C: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFCA40: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFCA44: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFCA48: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFCA4C: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCA50: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCA54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFCA58: 4BFD1DB1  bl 0x82ece808
	ctx.lr = 0x82EFCA5C;
	sub_82ECE808(ctx, base);
	// 82EFCA5C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCA60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFCA64: 830B0000  lwz r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCA68: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82EFCA6C: 409900FC  ble cr6, 0x82efcb68
	if !ctx.cr[6].gt {
	pc = 0x82EFCB68; continue 'dispatch;
	}
	// 82EFCA70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EFCA74: 3AF8FFFC  addi r23, r24, -4
	ctx.r[23].s64 = ctx.r[24].s64 + -4;
	// 82EFCA78: 3B2BB86C  addi r25, r11, -0x4794
	ctx.r[25].s64 = ctx.r[11].s64 + -18324;
	// 82EFCA7C: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82EFCA80: 40980020  bge cr6, 0x82efcaa0
	if !ctx.cr[6].lt {
	pc = 0x82EFCAA0; continue 'dispatch;
	}
	// 82EFCA84: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82EFCA88: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCA8C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82EFCA90: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82EFCA94: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EFCA98: 7CC7502E  lwzx r6, r7, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EFCA9C: 7C00322C  dcbt 0, r6
	// 82EFCAA0: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 82EFCAA4: 813D0010  lwz r9, 0x10(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCAA8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCAAC: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82EFCAB0: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EFCAB4: 7D67482E  lwzx r11, r7, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EFCAB8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCABC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFCAC0: 41990008  bgt cr6, 0x82efcac8
	if ctx.cr[6].gt {
	pc = 0x82EFCAC8; continue 'dispatch;
	}
	// 82EFCAC4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82EFCAC8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCACC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFCAD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCAD4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFCAD8: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCADC: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EFCAE0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCAE4: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82EFCAE8: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFCAEC: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82EFCAF0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCAF4: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFCAF8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82EFCAFC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFCB00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFCB04: 419A0058  beq cr6, 0x82efcb5c
	if ctx.cr[6].eq {
	pc = 0x82EFCB5C; continue 'dispatch;
	}
	// 82EFCB08: 7D3BE02E  lwzx r9, r27, r28
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFCB0C: 5548E13E  srwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EFCB10: 80E9000C  lwz r7, 0xc(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCB14: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCB18: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EFCB1C: 40980028  bge cr6, 0x82efcb44
	if !ctx.cr[6].lt {
	pc = 0x82EFCB44; continue 'dispatch;
	}
	// 82EFCB20: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82EFCB24: 932A0000  stw r25, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82EFCB28: 38EA0008  addi r7, r10, 8
	ctx.r[7].s64 = ctx.r[10].s64 + 8;
	// 82EFCB2C: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82EFCB30: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EFCB34: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82EFCB38: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82EFCB3C: D18A0004  stfs f12, 4(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EFCB40: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFCB44: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCB48: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82EFCB4C: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFCB50: 7C895214  add r4, r9, r10
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EFCB54: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB58: 48006809  bl 0x82f03360
	ctx.lr = 0x82EFCB5C;
	sub_82F03360(ctx, base);
	// 82EFCB5C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EFCB60: 7F1EC000  cmpw cr6, r30, r24
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82EFCB64: 4198FF18  blt cr6, 0x82efca7c
	if ctx.cr[6].lt {
	pc = 0x82EFCA7C; continue 'dispatch;
	}
	// 82EFCB68: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCB6C: 4800363D  bl 0x82f001a8
	ctx.lr = 0x82EFCB70;
	sub_82F001A8(ctx, base);
	// 82EFCB70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFCB74: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB78: 4BFD04D1  bl 0x82ecd048
	ctx.lr = 0x82EFCB7C;
	sub_82ECD048(ctx, base);
	// 82EFCB7C: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFCB80: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCB84: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCB88: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFCB8C: 40980020  bge cr6, 0x82efcbac
	if !ctx.cr[6].lt {
	pc = 0x82EFCBAC; continue 'dispatch;
	}
	// 82EFCB90: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFCB94: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFCB98: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFCB9C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFCBA0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFCBA4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFCBA8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFCBAC: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82EFCBB0: 419A0028  beq cr6, 0x82efcbd8
	if ctx.cr[6].eq {
	pc = 0x82EFCBD8; continue 'dispatch;
	}
	// 82EFCBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFCBB8: 809A0008  lwz r4, 8(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCBBC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82EFCBC0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EFCBC4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFCBC8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82EFCBCC: 4BFA4BED  bl 0x82ea17b8
	ctx.lr = 0x82EFCBD0;
	sub_82EA17B8(ctx, base);
	// 82EFCBD0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EFCBD4: 482AB5CC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 82EFCBD8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82EFCBDC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EFCBE0: 482AB5C0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCBE8 size=20
    let mut pc: u32 = 0x82EFCBE8;
    'dispatch: loop {
        match pc {
            0x82EFCBE8 => {
    //   block [0x82EFCBE8..0x82EFCBFC)
	// 82EFCBE8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFCBEC: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EFCBF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFCBF4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFCBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCC00 size=20
    let mut pc: u32 = 0x82EFCC00;
    'dispatch: loop {
        match pc {
            0x82EFCC00 => {
    //   block [0x82EFCC00..0x82EFCC14)
	// 82EFCC00: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFCC04: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EFCC08: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFCC0C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFCC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCC18 size=32
    let mut pc: u32 = 0x82EFCC18;
    'dispatch: loop {
        match pc {
            0x82EFCC18 => {
    //   block [0x82EFCC18..0x82EFCC38)
	// 82EFCC18: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFCC1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFCC20: 419A0018  beq cr6, 0x82efcc38
	if ctx.cr[6].eq {
		sub_82EFCC38(ctx, base);
		return;
	}
	// 82EFCC24: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFCC28: 894B0012  lbz r10, 0x12(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EFCC2C: 7D492B78  or r9, r10, r5
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 82EFCC30: 992B0012  stb r9, 0x12(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[9].u8 ) };
	// 82EFCC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCC38 size=16
    let mut pc: u32 = 0x82EFCC38;
    'dispatch: loop {
        match pc {
            0x82EFCC38 => {
    //   block [0x82EFCC38..0x82EFCC48)
	// 82EFCC38: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFCC3C: 7D6A2B78  or r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 82EFCC40: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFCC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCC48 size=96
    let mut pc: u32 = 0x82EFCC48;
    'dispatch: loop {
        match pc {
            0x82EFCC48 => {
    //   block [0x82EFCC48..0x82EFCCA8)
	// 82EFCC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCC50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFCC54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFCC5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EFCC60: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82EFCC64: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EFCC68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFCC6C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EFCC70: 419A0020  beq cr6, 0x82efcc90
	if ctx.cr[6].eq {
	pc = 0x82EFCC90; continue 'dispatch;
	}
	// 82EFCC74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCC78: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EFCC7C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82EFCC80: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCC84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFCC88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFCC8C: 4BFA3B25  bl 0x82ea07b0
	ctx.lr = 0x82EFCC90;
	sub_82EA07B0(ctx, base);
	// 82EFCC90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFCC94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCCA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFCCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFCCA8 size=2004
    let mut pc: u32 = 0x82EFCCA8;
    'dispatch: loop {
        match pc {
            0x82EFCCA8 => {
    //   block [0x82EFCCA8..0x82EFD47C)
	// 82EFCCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCCAC: 482AB485  bl 0x831a8130
	ctx.lr = 0x82EFCCB0;
	sub_831A8130(ctx, base);
	// 82EFCCB0: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82EFCCB4: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82EFCCB8: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82EFCCBC: E981D000  ld r12, -0x3000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-12288 as u32) ) };
	// 82EFCCC0: 9421CD20  stwu r1, -0x32e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-13024 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCCC4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFCCC8: 908132FC  stw r4, 0x32fc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(13052 as u32), ctx.r[4].u32 ) };
	// 82EFCCCC: 7C701B78  mr r16, r3
	ctx.r[16].u64 = ctx.r[3].u64;
	// 82EFCCD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EFCCD4: 93C13304  stw r30, 0x3304(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(13060 as u32), ctx.r[30].u32 ) };
	// 82EFCCD8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EFCCDC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFCCE0: E97E0020  ld r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	// 82EFCCE4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82EFCCE8: 38E8B89C  addi r7, r8, -0x4764
	ctx.r[7].s64 = ctx.r[8].s64 + -18276;
	// 82EFCCEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82EFCCF0: C3EABA78  lfs f31, -0x4588(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EFCCF4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82EFCCF8: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFCCFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFCD00: D3E131E0  stfs f31, 0x31e0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12768 as u32), tmp.u32 ) };
	// 82EFCD04: F9700060  std r11, 0x60(r16)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[16].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82EFCD08: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 82EFCD0C: E8BE0028  ld r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	// 82EFCD10: D0013200  stfs f0, 0x3200(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12800 as u32), tmp.u32 ) };
	// 82EFCD14: F8B00068  std r5, 0x68(r16)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[16].u32.wrapping_add(104 as u32), ctx.r[5].u64 ) };
	// 82EFCD18: D0013204  stfs f0, 0x3204(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12804 as u32), tmp.u32 ) };
	// 82EFCD1C: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCD20: 393E0020  addi r9, r30, 0x20
	ctx.r[9].s64 = ctx.r[30].s64 + 32;
	// 82EFCD24: 39100060  addi r8, r16, 0x60
	ctx.r[8].s64 = ctx.r[16].s64 + 96;
	// 82EFCD28: B0610076  sth r3, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[3].u16 ) };
	// 82EFCD2C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 82EFCD30: 2B0A000D  cmplwi cr6, r10, 0xd
	ctx.cr[6].compare_u32(ctx.r[10].u32, 13 as u32, &mut ctx.xer);
	// 82EFCD34: 90C100A0  stw r6, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[6].u32 ) };
	// 82EFCD38: 932100A4  stw r25, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[25].u32 ) };
	// 82EFCD3C: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82EFCD40: 93210078  stw r25, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[25].u32 ) };
	// 82EFCD44: 9321007C  stw r25, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[25].u32 ) };
	// 82EFCD48: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82EFCD4C: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 82EFCD50: 93210088  stw r25, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[25].u32 ) };
	// 82EFCD54: 93210090  stw r25, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[25].u32 ) };
	// 82EFCD58: 908101B4  stw r4, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[4].u32 ) };
	// 82EFCD5C: 409A0450  bne cr6, 0x82efd1ac
	if !ctx.cr[6].eq {
	pc = 0x82EFD1AC; continue 'dispatch;
	}
	// 82EFCD60: 81CD0000  lwz r14, 0(r13)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCD64: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82EFCD68: 831E0004  lwz r24, 4(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCD6C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EFCD70: 7D4B702E  lwzx r10, r11, r14
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82EFCD74: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCD78: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCD7C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFCD80: 40980020  bge cr6, 0x82efcda0
	if !ctx.cr[6].lt {
	pc = 0x82EFCDA0; continue 'dispatch;
	}
	// 82EFCD84: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFCD88: 3909B3C8  addi r8, r9, -0x4c38
	ctx.r[8].s64 = ctx.r[9].s64 + -19512;
	// 82EFCD8C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFCD90: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFCD94: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFCD98: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFCD9C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFCDA0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFCDA4: 3AD80058  addi r22, r24, 0x58
	ctx.r[22].s64 = ctx.r[24].s64 + 88;
	// 82EFCDA8: 8118005C  lwz r8, 0x5c(r24)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFCDAC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFCDB0: A15E0018  lhz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFCDB4: 80F80060  lwz r7, 0x60(r24)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EFCDB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFCDBC: 3A8AFFFF  addi r20, r10, -1
	ctx.r[20].s64 = ctx.r[10].s64 + -1;
	// 82EFCDC0: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EFCDC4: 7FE8482E  lwzx r31, r8, r9
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EFCDC8: 409A000C  bne cr6, 0x82efcdd4
	if !ctx.cr[6].eq {
	pc = 0x82EFCDD4; continue 'dispatch;
	}
	// 82EFCDCC: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCDD0: 48000008  b 0x82efcdd8
	pc = 0x82EFCDD8; continue 'dispatch;
	// 82EFCDD4: 39400200  li r10, 0x200
	ctx.r[10].s64 = 512;
	// 82EFCDD8: 3A5F0080  addi r18, r31, 0x80
	ctx.r[18].s64 = ctx.r[31].s64 + 128;
	// 82EFCDDC: 7EEAFA14  add r23, r10, r31
	ctx.r[23].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82EFCDE0: 3B520080  addi r26, r18, 0x80
	ctx.r[26].s64 = ctx.r[18].s64 + 128;
	// 82EFCDE4: 7F12B840  cmplw cr6, r18, r23
	ctx.cr[6].compare_u32(ctx.r[18].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82EFCDE8: 4198000C  blt cr6, 0x82efcdf4
	if ctx.cr[6].lt {
	pc = 0x82EFCDF4; continue 'dispatch;
	}
	// 82EFCDEC: 7F32CB78  mr r18, r25
	ctx.r[18].u64 = ctx.r[25].u64;
	// 82EFCDF0: 4800000C  b 0x82efcdfc
	pc = 0x82EFCDFC; continue 'dispatch;
	// 82EFCDF4: 7F1AB840  cmplw cr6, r26, r23
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82EFCDF8: 41980008  blt cr6, 0x82efce00
	if ctx.cr[6].lt {
	pc = 0x82EFCE00; continue 'dispatch;
	}
	// 82EFCDFC: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 82EFCE00: 3B700010  addi r27, r16, 0x10
	ctx.r[27].s64 = ctx.r[16].s64 + 16;
	// 82EFCE04: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82EFCE08: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 82EFCE0C: 39E0FFFF  li r15, -1
	ctx.r[15].s64 = -1;
	// 82EFCE10: 3A2000C0  li r17, 0xc0
	ctx.r[17].s64 = 192;
	// 82EFCE14: 895F000C  lbz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCE18: 392101C0  addi r9, r1, 0x1c0
	ctx.r[9].s64 = ctx.r[1].s64 + 448;
	// 82EFCE1C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCE20: 38A101B0  addi r5, r1, 0x1b0
	ctx.r[5].s64 = ctx.r[1].s64 + 432;
	// 82EFCE24: 7D480774  extsb r8, r10
	ctx.r[8].s64 = ctx.r[10].s8 as i64;
	// 82EFCE28: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCE2C: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFCE30: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFCE34: 550A3032  slwi r10, r8, 6
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFCE38: 83900004  lwz r28, 4(r16)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFCE40: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFCE44: 394B1C20  addi r10, r11, 0x1c20
	ctx.r[10].s64 = ctx.r[11].s64 + 7200;
	// 82EFCE48: 915B0060  stw r10, 0x60(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82EFCE4C: 80EB1C30  lwz r7, 0x1c30(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7216 as u32) ) } as u64;
	// 82EFCE50: D3E131E0  stfs f31, 0x31e0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12768 as u32), tmp.u32 ) };
	// 82EFCE54: 912101B0  stw r9, 0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[9].u32 ) };
	// 82EFCE58: 90FB0010  stw r7, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82EFCE5C: 93213230  stw r25, 0x3230(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12848 as u32), ctx.r[25].u32 ) };
	// 82EFCE60: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCE64: 48071FED  bl 0x82f6ee50
	ctx.lr = 0x82EFCE68;
	sub_82F6EE50(ctx, base);
	// 82EFCE68: 80A101B0  lwz r5, 0x1b0(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(432 as u32) ) } as u64;
	// 82EFCE6C: 38C101C0  addi r6, r1, 0x1c0
	ctx.r[6].s64 = ctx.r[1].s64 + 448;
	// 82EFCE70: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EFCE74: 419A0028  beq cr6, 0x82efce9c
	if ctx.cr[6].eq {
	pc = 0x82EFCE9C; continue 'dispatch;
	}
	// 82EFCE78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCE7C: 38E101B0  addi r7, r1, 0x1b0
	ctx.r[7].s64 = ctx.r[1].s64 + 432;
	// 82EFCE80: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EFCE84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFCE88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFCE8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCE90: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFCE94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFCE98: 4E800421  bctrl
	ctx.lr = 0x82EFCE9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCE9C: C00131E0  lfs f0, 0x31e0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(12768 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFCEA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFCEA4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82EFCEA8: 409A0008  bne cr6, 0x82efceb0
	if !ctx.cr[6].eq {
	pc = 0x82EFCEB0; continue 'dispatch;
	}
	// 82EFCEAC: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82EFCEB0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EFCEB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFCEB8: 419A002C  beq cr6, 0x82efcee4
	if ctx.cr[6].eq {
	pc = 0x82EFCEE4; continue 'dispatch;
	}
	// 82EFCEBC: 3BDC0340  addi r30, r28, 0x340
	ctx.r[30].s64 = ctx.r[28].s64 + 832;
	// 82EFCEC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFCEC4: 4BFA51ED  bl 0x82ea20b0
	ctx.lr = 0x82EFCEC8;
	sub_82EA20B0(ctx, base);
	// 82EFCEC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFCECC: 388101B0  addi r4, r1, 0x1b0
	ctx.r[4].s64 = ctx.r[1].s64 + 432;
	// 82EFCED0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFCED4: 480646BD  bl 0x82f61590
	ctx.lr = 0x82EFCED8;
	sub_82F61590(ctx, base);
	// 82EFCED8: F9FC0360  std r15, 0x360(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(864 as u32), ctx.r[15].u64 ) };
	// 82EFCEDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFCEE0: 48345A7D  bl 0x8324295c
	ctx.lr = 0x82EFCEE4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFCEE4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82EFCEE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFCEEC: 409A000C  bne cr6, 0x82efcef8
	if !ctx.cr[6].eq {
	pc = 0x82EFCEF8; continue 'dispatch;
	}
	// 82EFCEF0: 93210090  stw r25, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[25].u32 ) };
	// 82EFCEF4: 48000078  b 0x82efcf6c
	pc = 0x82EFCF6C; continue 'dispatch;
	// 82EFCEF8: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFCEFC: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFCF00: 40990040  ble cr6, 0x82efcf40
	if !ctx.cr[6].gt {
	pc = 0x82EFCF40; continue 'dispatch;
	}
	// 82EFCF04: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFCF08: 9B210060  stb r25, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u8 ) };
	// 82EFCF0C: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82EFCF10: 81410090  lwz r10, 0x90(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFCF14: 8101008C  lwz r8, 0x8c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EFCF18: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EFCF1C: 7CCB4A14  add r6, r11, r9
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EFCF20: 91460008  stw r10, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFCF24: 91060004  stw r8, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EFCF28: 7CEB492E  stwx r7, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 82EFCF2C: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFCF30: 93210090  stw r25, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[25].u32 ) };
	// 82EFCF34: 38850010  addi r4, r5, 0x10
	ctx.r[4].s64 = ctx.r[5].s64 + 16;
	// 82EFCF38: 908100A4  stw r4, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 82EFCF3C: 48000030  b 0x82efcf6c
	pc = 0x82EFCF6C; continue 'dispatch;
	// 82EFCF40: 8121008C  lwz r9, 0x8c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EFCF44: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82EFCF48: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFCF4C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFCF50: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82EFCF54: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82EFCF58: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EFCF5C: 7CA6412A  stdx r5, r6, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u64) };
	// 82EFCF60: 808100A4  lwz r4, 0xa4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFCF64: 38640010  addi r3, r4, 0x10
	ctx.r[3].s64 = ctx.r[4].s64 + 16;
	// 82EFCF68: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EFCF6C: 7E5F9378  mr r31, r18
	ctx.r[31].u64 = ctx.r[18].u64;
	// 82EFCF70: 93210088  stw r25, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[25].u32 ) };
	// 82EFCF74: 7F52D378  mr r18, r26
	ctx.r[18].u64 = ctx.r[26].u64;
	// 82EFCF78: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EFCF7C: 419A0090  beq cr6, 0x82efd00c
	if ctx.cr[6].eq {
	pc = 0x82EFD00C; continue 'dispatch;
	}
	// 82EFCF80: 3B5A0080  addi r26, r26, 0x80
	ctx.r[26].s64 = ctx.r[26].s64 + 128;
	// 82EFCF84: 7F1AB840  cmplw cr6, r26, r23
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82EFCF88: 41980064  blt cr6, 0x82efcfec
	if ctx.cr[6].lt {
	pc = 0x82EFCFEC; continue 'dispatch;
	}
	// 82EFCF8C: 3694FFFF  addic. r20, r20, -1
	ctx.xer.ca = (ctx.r[20].u32 > (!(-1 as u32)));
	ctx.r[20].s64 = ctx.r[20].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 82EFCF90: 3AB50004  addi r21, r21, 4
	ctx.r[21].s64 = ctx.r[21].s64 + 4;
	// 82EFCF94: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 82EFCF98: 41800110  blt 0x82efd0a8
	if ctx.cr[0].lt {
	pc = 0x82EFD0A8; continue 'dispatch;
	}
	// 82EFCF9C: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCFA0: 2F140000  cmpwi cr6, r20, 0
	ctx.cr[6].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 82EFCFA4: 7D6BAA14  add r11, r11, r21
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 82EFCFA8: 834B0000  lwz r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCFAC: 40990024  ble cr6, 0x82efcfd0
	if !ctx.cr[6].gt {
	pc = 0x82EFCFD0; continue 'dispatch;
	}
	// 82EFCFB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCFB4: 7C005A2C  dcbt 0, r11
	// 82EFCFB8: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 82EFCFBC: 7C0A5A2C  dcbt r10, r11
	// 82EFCFC0: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 82EFCFC4: 7C095A2C  dcbt r9, r11
	// 82EFCFC8: 39000180  li r8, 0x180
	ctx.r[8].s64 = 384;
	// 82EFCFCC: 7C085A2C  dcbt r8, r11
	// 82EFCFD0: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCFD4: 7F135800  cmpw cr6, r19, r11
	ctx.cr[6].compare_i32(ctx.r[19].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EFCFD8: 409A000C  bne cr6, 0x82efcfe4
	if !ctx.cr[6].eq {
	pc = 0x82EFCFE4; continue 'dispatch;
	}
	// 82EFCFDC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCFE0: 48000008  b 0x82efcfe8
	pc = 0x82EFCFE8; continue 'dispatch;
	// 82EFCFE4: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82EFCFE8: 7EFA5A14  add r23, r26, r11
	ctx.r[23].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82EFCFEC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCFF0: 7C005A2C  dcbt 0, r11
	// 82EFCFF4: 815A0010  lwz r10, 0x10(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCFF8: 7C00522C  dcbt 0, r10
	// 82EFCFFC: 7C0A8A2C  dcbt r10, r17
	// 82EFD000: 813A0014  lwz r9, 0x14(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD004: 7C004A2C  dcbt 0, r9
	// 82EFD008: 7C098A2C  dcbt r9, r17
	// 82EFD00C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFD010: 409AFE04  bne cr6, 0x82efce14
	if !ctx.cr[6].eq {
	pc = 0x82EFCE14; continue 'dispatch;
	}
	// 82EFD014: 81613304  lwz r11, 0x3304(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(13060 as u32) ) } as u64;
	// 82EFD018: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD01C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFD020: 409A0090  bne cr6, 0x82efd0b0
	if !ctx.cr[6].eq {
	pc = 0x82EFD0B0; continue 'dispatch;
	}
	// 82EFD024: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EFD028: 80700000  lwz r3, 0(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD02C: 4BFD17DD  bl 0x82ece808
	ctx.lr = 0x82EFD030;
	sub_82ECE808(ctx, base);
	// 82EFD030: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD034: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD038: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EFD03C: 41990008  bgt cr6, 0x82efd044
	if ctx.cr[6].gt {
	pc = 0x82EFD044; continue 'dispatch;
	}
	// 82EFD040: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFD044: 91780008  stw r11, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFD048: 8158000C  lwz r10, 0xc(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD04C: 81380010  lwz r9, 0x10(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD050: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EFD054: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFD058: 81580014  lwz r10, 0x14(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD05C: 9178000C  stw r11, 0xc(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EFD060: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EFD064: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFD068: 91380010  stw r9, 0x10(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82EFD06C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EFD070: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFD074: 91180014  stw r8, 0x14(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82EFD078: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFD07C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EFD080: 419A0018  beq cr6, 0x82efd098
	if ctx.cr[6].eq {
	pc = 0x82EFD098; continue 'dispatch;
	}
	// 82EFD084: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFD088: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82EFD08C: 80700000  lwz r3, 0(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD090: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFD094: 480062CD  bl 0x82f03360
	ctx.lr = 0x82EFD098;
	sub_82F03360(ctx, base);
	// 82EFD098: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EFD09C: 80700000  lwz r3, 0(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD0A0: 4BFCFFA9  bl 0x82ecd048
	ctx.lr = 0x82EFD0A4;
	sub_82ECD048(ctx, base);
	// 82EFD0A4: 480000A8  b 0x82efd14c
	pc = 0x82EFD14C; continue 'dispatch;
	// 82EFD0A8: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 82EFD0AC: 4BFFFF60  b 0x82efd00c
	pc = 0x82EFD00C; continue 'dispatch;
	// 82EFD0B0: A16B0014  lhz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD0B4: 39210078  addi r9, r1, 0x78
	ctx.r[9].s64 = ctx.r[1].s64 + 120;
	// 82EFD0B8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82EFD0BC: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82EFD0C0: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD0C4: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EFD0C8: 7D66502E  lwzx r11, r6, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EFD0CC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFD0D0: 80A90004  lwz r5, 4(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD0D4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD0D8: 80890008  lwz r4, 8(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD0DC: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EFD0E0: 8069000C  lwz r3, 0xc(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD0E4: 906B000C  stw r3, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EFD0E8: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFD0EC: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82EFD0F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD0F4: 419A0058  beq cr6, 0x82efd14c
	if ctx.cr[6].eq {
	pc = 0x82EFD14C; continue 'dispatch;
	}
	// 82EFD0F8: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD0FC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82EFD100: 80C100A0  lwz r6, 0xa0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFD104: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82EFD108: 7D482671  srawi. r8, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 4) as i64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EFD10C: 7D292A14  add r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 82EFD110: 7D463A14  add r10, r6, r7
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 82EFD114: 40810034  ble 0x82efd148
	if !ctx.cr[0].gt {
	pc = 0x82EFD148; continue 'dispatch;
	}
	// 82EFD118: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD11C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EFD120: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFD124: 80CA0004  lwz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD128: 90C90004  stw r6, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82EFD12C: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD130: 90A90008  stw r5, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EFD134: 806A000C  lwz r3, 0xc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD138: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82EFD13C: 9069000C  stw r3, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EFD140: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82EFD144: 4082FFD4  bne 0x82efd118
	if !ctx.cr[0].eq {
	pc = 0x82EFD118; continue 'dispatch;
	}
	// 82EFD148: 908B0014  stw r4, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82EFD14C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFD150: 7D4B702E  lwzx r10, r11, r14
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82EFD154: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD158: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD15C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD160: 40980020  bge cr6, 0x82efd180
	if !ctx.cr[6].lt {
	pc = 0x82EFD180; continue 'dispatch;
	}
	// 82EFD164: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFD168: 3909B8E0  addi r8, r9, -0x4720
	ctx.r[8].s64 = ctx.r[9].s64 + -18208;
	// 82EFD16C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFD170: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFD174: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFD178: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD17C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD180: 83C13304  lwz r30, 0x3304(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(13060 as u32) ) } as u64;
	// 82EFD184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFD188: 80900008  lwz r4, 8(r16)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD18C: 7E078378  mr r7, r16
	ctx.r[7].u64 = ctx.r[16].u64;
	// 82EFD190: 806132FC  lwz r3, 0x32fc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(13052 as u32) ) } as u64;
	// 82EFD194: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EFD198: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFD19C: 4BFA461D  bl 0x82ea17b8
	ctx.lr = 0x82EFD1A0;
	sub_82EA17B8(ctx, base);
	// 82EFD1A0: 382132E0  addi r1, r1, 0x32e0
	ctx.r[1].s64 = ctx.r[1].s64 + 13024;
	// 82EFD1A4: CBE1FF60  lfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) };
	// 82EFD1A8: 482AAFD8  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 82EFD1AC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD1B0: 3B400018  li r26, 0x18
	ctx.r[26].s64 = 24;
	// 82EFD1B4: 7D5AC02E  lwzx r10, r26, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EFD1B8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD1BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD1C0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD1C4: 40980020  bge cr6, 0x82efd1e4
	if !ctx.cr[6].lt {
	pc = 0x82EFD1E4; continue 'dispatch;
	}
	// 82EFD1C8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFD1CC: 3909B8CC  addi r8, r9, -0x4734
	ctx.r[8].s64 = ctx.r[9].s64 + -18228;
	// 82EFD1D0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFD1D4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFD1D8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFD1DC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD1E0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD1E4: A17E001A  lhz r11, 0x1a(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EFD1E8: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 82EFD1EC: A15E0014  lhz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD1F0: A13E0018  lhz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD1F4: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82EFD1F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EFD1FC: 419A01AC  beq cr6, 0x82efd3a8
	if ctx.cr[6].eq {
	pc = 0x82EFD3A8; continue 'dispatch;
	}
	// 82EFD200: 3BB00010  addi r29, r16, 0x10
	ctx.r[29].s64 = ctx.r[16].s64 + 16;
	// 82EFD204: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82EFD208: 39E0FFFF  li r15, -1
	ctx.r[15].s64 = -1;
	// 82EFD20C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFD210: 895E0030  lbz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFD214: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFD218: 7FFC582E  lwzx r31, r28, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFD21C: 419A002C  beq cr6, 0x82efd248
	if ctx.cr[6].eq {
	pc = 0x82EFD248; continue 'dispatch;
	}
	// 82EFD220: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD224: 394B1DA0  addi r10, r11, 0x1da0
	ctx.r[10].s64 = ctx.r[11].s64 + 7584;
	// 82EFD228: 91500070  stw r10, 0x70(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82EFD22C: 893F000C  lbz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD230: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 82EFD234: 550A3032  slwi r10, r8, 6
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFD238: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFD23C: 80C71C30  lwz r6, 0x1c30(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(7216 as u32) ) } as u64;
	// 82EFD240: 90D00020  stw r6, 0x20(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82EFD244: 48000028  b 0x82efd26c
	pc = 0x82EFD26C; continue 'dispatch;
	// 82EFD248: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD24C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD250: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82EFD254: 552B3032  slwi r11, r9, 6
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFD258: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFD25C: 394B1C20  addi r10, r11, 0x1c20
	ctx.r[10].s64 = ctx.r[11].s64 + 7200;
	// 82EFD260: 91500070  stw r10, 0x70(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82EFD264: 810B1C30  lwz r8, 0x1c30(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7216 as u32) ) } as u64;
	// 82EFD268: 91100020  stw r8, 0x20(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82EFD26C: 396101C0  addi r11, r1, 0x1c0
	ctx.r[11].s64 = ctx.r[1].s64 + 448;
	// 82EFD270: D3E131E0  stfs f31, 0x31e0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12768 as u32), tmp.u32 ) };
	// 82EFD274: 93213230  stw r25, 0x3230(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(12848 as u32), ctx.r[25].u32 ) };
	// 82EFD278: 38A101B0  addi r5, r1, 0x1b0
	ctx.r[5].s64 = ctx.r[1].s64 + 432;
	// 82EFD27C: 916101B0  stw r11, 0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 82EFD280: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFD284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD288: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD28C: 48071BC5  bl 0x82f6ee50
	ctx.lr = 0x82EFD290;
	sub_82F6EE50(ctx, base);
	// 82EFD290: 814101B0  lwz r10, 0x1b0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(432 as u32) ) } as u64;
	// 82EFD294: 392101C0  addi r9, r1, 0x1c0
	ctx.r[9].s64 = ctx.r[1].s64 + 448;
	// 82EFD298: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD29C: 419A0028  beq cr6, 0x82efd2c4
	if ctx.cr[6].eq {
	pc = 0x82EFD2C4; continue 'dispatch;
	}
	// 82EFD2A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD2A4: 38E101B0  addi r7, r1, 0x1b0
	ctx.r[7].s64 = ctx.r[1].s64 + 432;
	// 82EFD2A8: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD2AC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EFD2B0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD2B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD2B8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD2BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFD2C0: 4E800421  bctrl
	ctx.lr = 0x82EFD2C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD2C4: C00131E0  lfs f0, 0x31e0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(12768 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFD2C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFD2CC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82EFD2D0: 409A0008  bne cr6, 0x82efd2d8
	if !ctx.cr[6].eq {
	pc = 0x82EFD2D8; continue 'dispatch;
	}
	// 82EFD2D4: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82EFD2D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EFD2DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD2E0: 419A0034  beq cr6, 0x82efd314
	if ctx.cr[6].eq {
	pc = 0x82EFD314; continue 'dispatch;
	}
	// 82EFD2E4: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD2E8: 3BCB0340  addi r30, r11, 0x340
	ctx.r[30].s64 = ctx.r[11].s64 + 832;
	// 82EFD2EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD2F0: 4BFA4DC1  bl 0x82ea20b0
	ctx.lr = 0x82EFD2F4;
	sub_82EA20B0(ctx, base);
	// 82EFD2F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFD2F8: 388101B0  addi r4, r1, 0x1b0
	ctx.r[4].s64 = ctx.r[1].s64 + 432;
	// 82EFD2FC: 80700004  lwz r3, 4(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD300: 48064291  bl 0x82f61590
	ctx.lr = 0x82EFD304;
	sub_82F61590(ctx, base);
	// 82EFD304: F9FE0020  std r15, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[15].u64 ) };
	// 82EFD308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD30C: 48345651  bl 0x8324295c
	ctx.lr = 0x82EFD310;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFD310: 83C13304  lwz r30, 0x3304(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(13060 as u32) ) } as u64;
	// 82EFD314: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82EFD318: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD31C: 409A000C  bne cr6, 0x82efd328
	if !ctx.cr[6].eq {
	pc = 0x82EFD328; continue 'dispatch;
	}
	// 82EFD320: 93210090  stw r25, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[25].u32 ) };
	// 82EFD324: 4800006C  b 0x82efd390
	pc = 0x82EFD390; continue 'dispatch;
	// 82EFD328: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFD32C: 80E1008C  lwz r7, 0x8c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EFD330: 40990034  ble cr6, 0x82efd364
	if !ctx.cr[6].gt {
	pc = 0x82EFD364; continue 'dispatch;
	}
	// 82EFD334: 812100A4  lwz r9, 0xa4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFD338: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82EFD33C: 81010090  lwz r8, 0x90(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFD340: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EFD344: 9B210060  stb r25, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u8 ) };
	// 82EFD348: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EFD34C: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFD350: 7CAB512E  stwx r5, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82EFD354: 90E60004  stw r7, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD358: 91060008  stw r8, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EFD35C: 93210090  stw r25, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[25].u32 ) };
	// 82EFD360: 48000024  b 0x82efd384
	pc = 0x82EFD384; continue 'dispatch;
	// 82EFD364: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EFD368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82EFD36C: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFD370: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82EFD374: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82EFD378: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFD37C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EFD380: 7CA6412A  stdx r5, r6, r8
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32), ctx.r[5].u64) };
	// 82EFD384: 808100A4  lwz r4, 0xa4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFD388: 38640010  addi r3, r4, 0x10
	ctx.r[3].s64 = ctx.r[4].s64 + 16;
	// 82EFD38C: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EFD390: A17E0018  lhz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD394: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82EFD398: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82EFD39C: 93210088  stw r25, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[25].u32 ) };
	// 82EFD3A0: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EFD3A4: 4198FE68  blt cr6, 0x82efd20c
	if ctx.cr[6].lt {
	pc = 0x82EFD20C; continue 'dispatch;
	}
	// 82EFD3A8: A17E0014  lhz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD3AC: 39410078  addi r10, r1, 0x78
	ctx.r[10].s64 = ctx.r[1].s64 + 120;
	// 82EFD3B0: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD3B4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82EFD3B8: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82EFD3BC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD3C0: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EFD3C4: 7D26482E  lwzx r9, r6, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EFD3C8: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFD3CC: 80AA0004  lwz r5, 4(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD3D0: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD3D4: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD3D8: 90890008  stw r4, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EFD3DC: 806A000C  lwz r3, 0xc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD3E0: 9069000C  stw r3, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EFD3E4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFD3E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EFD3EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD3F0: 419A0058  beq cr6, 0x82efd448
	if ctx.cr[6].eq {
	pc = 0x82EFD448; continue 'dispatch;
	}
	// 82EFD3F4: 80E90010  lwz r7, 0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD3F8: 39490010  addi r10, r9, 0x10
	ctx.r[10].s64 = ctx.r[9].s64 + 16;
	// 82EFD3FC: 80C100A0  lwz r6, 0xa0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFD400: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 82EFD404: 7D682671  srawi. r8, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 4) as i64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EFD408: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFD40C: 7D662A14  add r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82EFD410: 40810034  ble 0x82efd444
	if !ctx.cr[0].gt {
	pc = 0x82EFD444; continue 'dispatch;
	}
	// 82EFD414: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD418: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82EFD41C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFD420: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD424: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82EFD428: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD42C: 90AA0008  stw r5, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EFD430: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD434: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EFD438: 906A000C  stw r3, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EFD43C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82EFD440: 4082FFD4  bne 0x82efd414
	if !ctx.cr[0].eq {
	pc = 0x82EFD414; continue 'dispatch;
	}
	// 82EFD444: 90890014  stw r4, 0x14(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82EFD448: 7D5AC02E  lwzx r10, r26, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EFD44C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD450: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD454: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD458: 4098FD2C  bge cr6, 0x82efd184
	if !ctx.cr[6].lt {
	pc = 0x82EFD184; continue 'dispatch;
	}
	// 82EFD45C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFD460: 3909B8B8  addi r8, r9, -0x4748
	ctx.r[8].s64 = ctx.r[9].s64 + -18248;
	// 82EFD464: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFD468: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFD46C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFD470: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD474: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD478: 4BFFFD0C  b 0x82efd184
	pc = 0x82EFD184; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD480 size=200
    let mut pc: u32 = 0x82EFD480;
    'dispatch: loop {
        match pc {
            0x82EFD480 => {
    //   block [0x82EFD480..0x82EFD548)
	// 82EFD480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFD488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFD48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFD490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD498: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFD49C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EFD4A0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFD4A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD4A8: 409A0020  bne cr6, 0x82efd4c8
	if !ctx.cr[6].eq {
	pc = 0x82EFD4C8; continue 'dispatch;
	}
	// 82EFD4AC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD4B0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EFD4B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EFD4B8: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFD4BC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFD4C0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EFD4C4: 4BFA32ED  bl 0x82ea07b0
	ctx.lr = 0x82EFD4C8;
	sub_82EA07B0(ctx, base);
	// 82EFD4C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EFD4CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD4D0: 419A005C  beq cr6, 0x82efd52c
	if ctx.cr[6].eq {
	pc = 0x82EFD52C; continue 'dispatch;
	}
	// 82EFD4D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFD4D8: 419A0054  beq cr6, 0x82efd52c
	if ctx.cr[6].eq {
	pc = 0x82EFD52C; continue 'dispatch;
	}
	// 82EFD4DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD4E0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EFD4E4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFD4E8: 812B0074  lwz r9, 0x74(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EFD4EC: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFD4F0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EFD4F4: 4198001C  blt cr6, 0x82efd510
	if ctx.cr[6].lt {
	pc = 0x82EFD510; continue 'dispatch;
	}
	// 82EFD4F8: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFD4FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFD500: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82EFD504: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFD508: 4BFA2B59  bl 0x82ea0060
	ctx.lr = 0x82EFD50C;
	sub_82EA0060(ctx, base);
	// 82EFD50C: 48000020  b 0x82efd52c
	pc = 0x82EFD52C; continue 'dispatch;
	// 82EFD510: 812B0074  lwz r9, 0x74(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EFD514: 394B0070  addi r10, r11, 0x70
	ctx.r[10].s64 = ctx.r[11].s64 + 112;
	// 82EFD518: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EFD51C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EFD520: 912B0074  stw r9, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82EFD524: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFD528: 93EB0070  stw r31, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82EFD52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFD538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFD53C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFD540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFD544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD548 size=352
    let mut pc: u32 = 0x82EFD548;
    'dispatch: loop {
        match pc {
            0x82EFD548 => {
    //   block [0x82EFD548..0x82EFD6A8)
	// 82EFD548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD54C: 482AAC21  bl 0x831a816c
	ctx.lr = 0x82EFD550;
	sub_831A8130(ctx, base);
	// 82EFD550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD558: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EFD55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD560: 419A0064  beq cr6, 0x82efd5c4
	if ctx.cr[6].eq {
	pc = 0x82EFD5C4; continue 'dispatch;
	}
	// 82EFD564: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD568: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82EFD56C: 80BF0068  lwz r5, 0x68(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EFD570: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD574: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82EFD578: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EFD57C: 814300B4  lwz r10, 0xb4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EFD580: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFD584: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFD588: 41980014  blt cr6, 0x82efd59c
	if ctx.cr[6].lt {
	pc = 0x82EFD59C; continue 'dispatch;
	}
	// 82EFD58C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFD590: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82EFD594: 4BFA2ACD  bl 0x82ea0060
	ctx.lr = 0x82EFD598;
	sub_82EA0060(ctx, base);
	// 82EFD598: 48000020  b 0x82efd5b8
	pc = 0x82EFD5B8; continue 'dispatch;
	// 82EFD59C: 814300B4  lwz r10, 0xb4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EFD5A0: 396300B0  addi r11, r3, 0xb0
	ctx.r[11].s64 = ctx.r[3].s64 + 176;
	// 82EFD5A4: 816300B0  lwz r11, 0xb0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EFD5A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EFD5AC: 914300B4  stw r10, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82EFD5B0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD5B4: 90A300B0  stw r5, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 82EFD5B8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EFD5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD5C0: 409AFFAC  bne cr6, 0x82efd56c
	if !ctx.cr[6].eq {
	pc = 0x82EFD56C; continue 'dispatch;
	}
	// 82EFD5C4: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD5C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD5CC: 419A0064  beq cr6, 0x82efd630
	if ctx.cr[6].eq {
	pc = 0x82EFD630; continue 'dispatch;
	}
	// 82EFD5D0: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD5D4: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82EFD5D8: 80BF0078  lwz r5, 0x78(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD5DC: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD5E0: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82EFD5E4: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EFD5E8: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFD5EC: 8123005C  lwz r9, 0x5c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFD5F0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EFD5F4: 41980014  blt cr6, 0x82efd608
	if ctx.cr[6].lt {
	pc = 0x82EFD608; continue 'dispatch;
	}
	// 82EFD5F8: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFD5FC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EFD600: 4BFA2A61  bl 0x82ea0060
	ctx.lr = 0x82EFD604;
	sub_82EA0060(ctx, base);
	// 82EFD604: 48000020  b 0x82efd624
	pc = 0x82EFD624; continue 'dispatch;
	// 82EFD608: 8143005C  lwz r10, 0x5c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFD60C: 39630058  addi r11, r3, 0x58
	ctx.r[11].s64 = ctx.r[3].s64 + 88;
	// 82EFD610: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFD614: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EFD618: 9143005C  stw r10, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82EFD61C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD620: 90A30058  stw r5, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82EFD624: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD628: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD62C: 409AFFAC  bne cr6, 0x82efd5d8
	if !ctx.cr[6].eq {
	pc = 0x82EFD5D8; continue 'dispatch;
	}
	// 82EFD630: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD634: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFD638: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFD63C: 419A0028  beq cr6, 0x82efd664
	if ctx.cr[6].eq {
	pc = 0x82EFD664; continue 'dispatch;
	}
	// 82EFD640: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EFD644: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD648: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFD64C: 806B6A30  lwz r3, 0x6a30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EFD650: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD654: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD658: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EFD65C: 4E800421  bctrl
	ctx.lr = 0x82EFD660;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD660: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EFD664: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD668: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFD66C: 40990028  ble cr6, 0x82efd694
	if !ctx.cr[6].gt {
	pc = 0x82EFD694; continue 'dispatch;
	}
	// 82EFD670: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD674: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EFD678: A13F0056  lhz r9, 0x56(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(86 as u32) ) } as u64;
	// 82EFD67C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EFD680: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFD684: 5525103E  rotlwi r5, r9, 2
	ctx.r[5].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82EFD688: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFD68C: 4BFA3125  bl 0x82ea07b0
	ctx.lr = 0x82EFD690;
	sub_82EA07B0(ctx, base);
	// 82EFD690: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EFD694: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFD698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD69C: 4BFFFDE5  bl 0x82efd480
	ctx.lr = 0x82EFD6A0;
	sub_82EFD480(ctx, base);
	// 82EFD6A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD6A4: 482AAB18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD6A8 size=1264
    let mut pc: u32 = 0x82EFD6A8;
    'dispatch: loop {
        match pc {
            0x82EFD6A8 => {
    //   block [0x82EFD6A8..0x82EFDB98)
	// 82EFD6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD6AC: 482AAA99  bl 0x831a8144
	ctx.lr = 0x82EFD6B0;
	sub_831A8130(ctx, base);
	// 82EFD6B0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD6B4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD6B8: 3A800018  li r20, 0x18
	ctx.r[20].s64 = 24;
	// 82EFD6BC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EFD6C0: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82EFD6C4: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82EFD6C8: 7D78A02E  lwzx r11, r24, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82EFD6CC: 82DC0004  lwz r22, 4(r28)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD6D0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD6D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD6D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD6DC: 40980020  bge cr6, 0x82efd6fc
	if !ctx.cr[6].lt {
	pc = 0x82EFD6FC; continue 'dispatch;
	}
	// 82EFD6E0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFD6E4: 3909B90C  addi r8, r9, -0x46f4
	ctx.r[8].s64 = ctx.r[9].s64 + -18164;
	// 82EFD6E8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFD6EC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFD6F0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFD6F4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD6F8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD6FC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD700: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD704: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD708: 419A003C  beq cr6, 0x82efd744
	if ctx.cr[6].eq {
	pc = 0x82EFD744; continue 'dispatch;
	}
	// 82EFD70C: 80750000  lwz r3, 0(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD710: 896300C5  lbz r11, 0xc5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(197 as u32) ) } as u64;
	// 82EFD714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD718: 419A002C  beq cr6, 0x82efd744
	if ctx.cr[6].eq {
	pc = 0x82EFD744; continue 'dispatch;
	}
	// 82EFD71C: 89760026  lbz r11, 0x26(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(38 as u32) ) } as u64;
	// 82EFD720: 556A06B6  rlwinm r10, r11, 0, 0x1a, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFD724: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFD728: 419A001C  beq cr6, 0x82efd744
	if ctx.cr[6].eq {
	pc = 0x82EFD744; continue 'dispatch;
	}
	// 82EFD72C: 89760025  lbz r11, 0x25(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(37 as u32) ) } as u64;
	// 82EFD730: 556A06B6  rlwinm r10, r11, 0, 0x1a, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFD734: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFD738: 409A000C  bne cr6, 0x82efd744
	if !ctx.cr[6].eq {
	pc = 0x82EFD744; continue 'dispatch;
	}
	// 82EFD73C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82EFD740: 4BFE2371  bl 0x82edfab0
	ctx.lr = 0x82EFD744;
	sub_82EDFAB0(ctx, base);
	// 82EFD744: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD748: 80D50000  lwz r6, 0(r21)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD74C: 3B4B001C  addi r26, r11, 0x1c
	ctx.r[26].s64 = ctx.r[11].s64 + 28;
	// 82EFD750: 80B6004C  lwz r5, 0x4c(r22)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EFD754: 80960048  lwz r4, 0x48(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EFD758: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82EFD75C: 80750004  lwz r3, 4(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD760: 480657E9  bl 0x82f62f48
	ctx.lr = 0x82EFD764;
	sub_82F62F48(ctx, base);
	// 82EFD764: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFD768: 83DC0008  lwz r30, 8(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD76C: 38FE000C  addi r7, r30, 0xc
	ctx.r[7].s64 = ctx.r[30].s64 + 12;
	// 82EFD770: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFD774: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFD778: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFD77C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFD780: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFD784: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFD788: 4082FFE8  bne 0x82efd770
	if !ctx.cr[0].eq {
	pc = 0x82EFD770; continue 'dispatch;
	}
	// 82EFD78C: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD790: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82EFD794: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82EFD798: 7F193000  cmpw cr6, r25, r6
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EFD79C: 409A0104  bne cr6, 0x82efd8a0
	if !ctx.cr[6].eq {
	pc = 0x82EFD8A0; continue 'dispatch;
	}
	// 82EFD7A0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFD7A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD7A8: 419A00F0  beq cr6, 0x82efd898
	if ctx.cr[6].eq {
	pc = 0x82EFD898; continue 'dispatch;
	}
	// 82EFD7AC: 7EFDBB78  mr r29, r23
	ctx.r[29].u64 = ctx.r[23].u64;
	// 82EFD7B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD7B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD7B8: 7FAAEA14  add r29, r10, r29
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82EFD7BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD7C0: 409AFFF0  bne cr6, 0x82efd7b0
	if !ctx.cr[6].eq {
	pc = 0x82EFD7B0; continue 'dispatch;
	}
	// 82EFD7C4: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82EFD7C8: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82EFD7CC: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EFD7D0: 7C78D82E  lwzx r3, r24, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82EFD7D4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFD7D8: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFD7DC: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EFD7E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD7E4: 41990010  bgt cr6, 0x82efd7f4
	if ctx.cr[6].gt {
	pc = 0x82EFD7F4; continue 'dispatch;
	}
	// 82EFD7E8: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFD7EC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82EFD7F0: 48000018  b 0x82efd808
	pc = 0x82EFD808; continue 'dispatch;
	// 82EFD7F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD7F8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD7FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFD800: 4E800421  bctrl
	ctx.lr = 0x82EFD804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD808: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFD80C: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82EFD810: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EFD814: 419A004C  beq cr6, 0x82efd860
	if ctx.cr[6].eq {
	pc = 0x82EFD860; continue 'dispatch;
	}
	// 82EFD818: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD81C: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFD820: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFD824: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFD828: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD82C: 40990020  ble cr6, 0x82efd84c
	if !ctx.cr[6].gt {
	pc = 0x82EFD84C; continue 'dispatch;
	}
	// 82EFD830: 7D0B4850  subf r8, r11, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82EFD834: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 82EFD838: 7CC8582E  lwzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFD83C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD840: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EFD844: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EFD848: 4082FFF0  bne 0x82efd838
	if !ctx.cr[0].eq {
	pc = 0x82EFD838; continue 'dispatch;
	}
	// 82EFD84C: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD850: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD854: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EFD858: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EFD85C: 409AFFBC  bne cr6, 0x82efd818
	if !ctx.cr[6].eq {
	pc = 0x82EFD818; continue 'dispatch;
	}
	// 82EFD860: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFD864: 80750000  lwz r3, 0(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFD86C: 4BFE75AD  bl 0x82ee4e18
	ctx.lr = 0x82EFD870;
	sub_82EE4E18(ctx, base);
	// 82EFD870: 7C78D82E  lwzx r3, r24, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82EFD874: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFD878: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82EFD87C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFD880: 409A0018  bne cr6, 0x82efd898
	if !ctx.cr[6].eq {
	pc = 0x82EFD898; continue 'dispatch;
	}
	// 82EFD884: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD888: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFD88C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD890: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EFD894: 4E800421  bctrl
	ctx.lr = 0x82EFD898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD898: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EFD89C: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82EFD8A0: 895A0000  lbz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD8A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EFD8A8: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82EFD8AC: 3BABBADC  addi r29, r11, -0x4524
	ctx.r[29].s64 = ctx.r[11].s64 + -17700;
	// 82EFD8B0: 419A0068  beq cr6, 0x82efd918
	if ctx.cr[6].eq {
	pc = 0x82EFD918; continue 'dispatch;
	}
	// 82EFD8B4: 7D58A02E  lwzx r10, r24, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82EFD8B8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD8BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD8C0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD8C4: 40980020  bge cr6, 0x82efd8e4
	if !ctx.cr[6].lt {
	pc = 0x82EFD8E4; continue 'dispatch;
	}
	// 82EFD8C8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFD8CC: 3909B8F0  addi r8, r9, -0x4710
	ctx.r[8].s64 = ctx.r[9].s64 + -18192;
	// 82EFD8D0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFD8D4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFD8D8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFD8DC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD8E0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFD8E4: 897A0000  lbz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD8E8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82EFD8EC: 409AFFF8  bne cr6, 0x82efd8e4
	if !ctx.cr[6].eq {
	pc = 0x82EFD8E4; continue 'dispatch;
	}
	// 82EFD8F0: 7D58A02E  lwzx r10, r24, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82EFD8F4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD8F8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD8FC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFD900: 40980018  bge cr6, 0x82efd918
	if !ctx.cr[6].lt {
	pc = 0x82EFD918; continue 'dispatch;
	}
	// 82EFD904: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EFD908: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EFD90C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EFD910: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EFD914: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFD918: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82EFD91C: 409A0140  bne cr6, 0x82efda5c
	if !ctx.cr[6].eq {
	pc = 0x82EFDA5C; continue 'dispatch;
	}
	// 82EFD920: 83FC0008  lwz r31, 8(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD924: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EFD928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD92C: 419A0060  beq cr6, 0x82efd98c
	if ctx.cr[6].eq {
	pc = 0x82EFD98C; continue 'dispatch;
	}
	// 82EFD930: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82EFD934: 80BF0068  lwz r5, 0x68(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EFD938: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD93C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82EFD940: 7C78F02E  lwzx r3, r24, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFD944: 814300B4  lwz r10, 0xb4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EFD948: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFD94C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFD950: 41980014  blt cr6, 0x82efd964
	if ctx.cr[6].lt {
	pc = 0x82EFD964; continue 'dispatch;
	}
	// 82EFD954: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFD958: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82EFD95C: 4BFA2705  bl 0x82ea0060
	ctx.lr = 0x82EFD960;
	sub_82EA0060(ctx, base);
	// 82EFD960: 48000020  b 0x82efd980
	pc = 0x82EFD980; continue 'dispatch;
	// 82EFD964: 814300B4  lwz r10, 0xb4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EFD968: 396300B0  addi r11, r3, 0xb0
	ctx.r[11].s64 = ctx.r[3].s64 + 176;
	// 82EFD96C: 816300B0  lwz r11, 0xb0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EFD970: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EFD974: 914300B4  stw r10, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82EFD978: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD97C: 90A300B0  stw r5, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 82EFD980: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EFD984: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD988: 409AFFAC  bne cr6, 0x82efd934
	if !ctx.cr[6].eq {
	pc = 0x82EFD934; continue 'dispatch;
	}
	// 82EFD98C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD994: 419A0060  beq cr6, 0x82efd9f4
	if ctx.cr[6].eq {
	pc = 0x82EFD9F4; continue 'dispatch;
	}
	// 82EFD998: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82EFD99C: 80BF0078  lwz r5, 0x78(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD9A0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD9A4: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82EFD9A8: 7C78F02E  lwzx r3, r24, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFD9AC: 8143005C  lwz r10, 0x5c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFD9B0: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFD9B4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFD9B8: 41980014  blt cr6, 0x82efd9cc
	if ctx.cr[6].lt {
	pc = 0x82EFD9CC; continue 'dispatch;
	}
	// 82EFD9BC: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFD9C0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EFD9C4: 4BFA269D  bl 0x82ea0060
	ctx.lr = 0x82EFD9C8;
	sub_82EA0060(ctx, base);
	// 82EFD9C8: 48000020  b 0x82efd9e8
	pc = 0x82EFD9E8; continue 'dispatch;
	// 82EFD9CC: 8143005C  lwz r10, 0x5c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFD9D0: 39630058  addi r11, r3, 0x58
	ctx.r[11].s64 = ctx.r[3].s64 + 88;
	// 82EFD9D4: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFD9D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EFD9DC: 9143005C  stw r10, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82EFD9E0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD9E4: 90A30058  stw r5, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82EFD9E8: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFD9EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD9F0: 409AFFAC  bne cr6, 0x82efd99c
	if !ctx.cr[6].eq {
	pc = 0x82EFD99C; continue 'dispatch;
	}
	// 82EFD9F4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD9F8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFD9FC: 419A0028  beq cr6, 0x82efda24
	if ctx.cr[6].eq {
	pc = 0x82EFDA24; continue 'dispatch;
	}
	// 82EFDA00: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EFDA04: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFDA08: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82EFDA0C: 806B6A30  lwz r3, 0x6a30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82EFDA10: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDA14: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFDA18: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EFDA1C: 4E800421  bctrl
	ctx.lr = 0x82EFDA20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFDA20: 92FF0014  stw r23, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[23].u32 ) };
	// 82EFDA24: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFDA28: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFDA2C: 40990024  ble cr6, 0x82efda50
	if !ctx.cr[6].gt {
	pc = 0x82EFDA50; continue 'dispatch;
	}
	// 82EFDA30: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82EFDA34: A15F0056  lhz r10, 0x56(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(86 as u32) ) } as u64;
	// 82EFDA38: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EFDA3C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFDA40: 5545103E  rotlwi r5, r10, 2
	ctx.r[5].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82EFDA44: 7C78582E  lwzx r3, r24, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFDA48: 4BFA2D69  bl 0x82ea07b0
	ctx.lr = 0x82EFDA4C;
	sub_82EA07B0(ctx, base);
	// 82EFDA4C: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 82EFDA50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFDA54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFDA58: 4BFFFA29  bl 0x82efd480
	ctx.lr = 0x82EFDA5C;
	sub_82EFD480(ctx, base);
	// 82EFDA5C: 81360060  lwz r9, 0x60(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EFDA60: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFDA64: 409900E8  ble cr6, 0x82efdb4c
	if !ctx.cr[6].gt {
	pc = 0x82EFDB4C; continue 'dispatch;
	}
	// 82EFDA68: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDA6C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82EFDA70: 80D60058  lwz r6, 0x58(r22)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFDA74: 387C0020  addi r3, r28, 0x20
	ctx.r[3].s64 = ctx.r[28].s64 + 32;
	// 82EFDA78: 8096005C  lwz r4, 0x5c(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EFDA7C: 390B0190  addi r8, r11, 0x190
	ctx.r[8].s64 = ctx.r[11].s64 + 400;
	// 82EFDA80: 814B00D4  lwz r10, 0xd4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 82EFDA84: B0FC0000  sth r7, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 82EFDA88: 810B0190  lwz r8, 0x190(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(400 as u32) ) } as u64;
	// 82EFDA8C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EFDA90: 911C0020  stw r8, 0x20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82EFDA94: 80EB0194  lwz r7, 0x194(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(404 as u32) ) } as u64;
	// 82EFDA98: 90FC0024  stw r7, 0x24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82EFDA9C: 810B0198  lwz r8, 0x198(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(408 as u32) ) } as u64;
	// 82EFDAA0: 911C0028  stw r8, 0x28(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 82EFDAA4: 80EB019C  lwz r7, 0x19c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 82EFDAA8: 90FC002C  stw r7, 0x2c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82EFDAAC: B2FC0014  sth r23, 0x14(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[23].u16 ) };
	// 82EFDAB0: 909C001C  stw r4, 0x1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EFDAB4: B13C0018  sth r9, 0x18(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[9].u16 ) };
	// 82EFDAB8: B15C001A  sth r10, 0x1a(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(26 as u32), ctx.r[10].u16 ) };
	// 82EFDABC: 92FC0010  stw r23, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[23].u32 ) };
	// 82EFDAC0: 92FC0030  stw r23, 0x30(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(48 as u32), ctx.r[23].u32 ) };
	// 82EFDAC4: B0DC0034  sth r6, 0x34(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(52 as u32), ctx.r[6].u16 ) };
	// 82EFDAC8: 40990038  ble cr6, 0x82efdb00
	if !ctx.cr[6].gt {
	pc = 0x82EFDB00; continue 'dispatch;
	}
	// 82EFDACC: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 82EFDAD0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EFDAD4: 5569083E  rotlwi r9, r11, 1
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82EFDAD8: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82EFDADC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82EFDAE0: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82EFDAE4: 7D484878  andc r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82EFDAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFDAEC: 0CCA0000  twi 6, r10, 0
	// 82EFDAF0: 0CA8FFFF  twi 5, r8, -1
	// 82EFDAF4: 48002595  bl 0x82f00088
	ctx.lr = 0x82EFDAF8;
	sub_82F00088(ctx, base);
	// 82EFDAF8: 907C0010  stw r3, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EFDAFC: B3FC0016  sth r31, 0x16(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(22 as u32), ctx.r[31].u16 ) };
	// 82EFDB00: 7D58A02E  lwzx r10, r24, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82EFDB04: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDB08: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDB0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDB10: 40980018  bge cr6, 0x82efdb28
	if !ctx.cr[6].lt {
	pc = 0x82EFDB28; continue 'dispatch;
	}
	// 82EFDB14: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EFDB18: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EFDB1C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EFDB20: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EFDB24: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDB28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFDB2C: 80D50008  lwz r6, 8(r21)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDB30: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EFDB34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EFDB38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFDB3C: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82EFDB40: 4BFA4101  bl 0x82ea1c40
	ctx.lr = 0x82EFDB44;
	sub_82EA1C40(ctx, base);
	// 82EFDB44: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82EFDB48: 482AA64C  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 82EFDB4C: 7D58A02E  lwzx r10, r24, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82EFDB50: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDB54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDB58: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDB5C: 40980018  bge cr6, 0x82efdb74
	if !ctx.cr[6].lt {
	pc = 0x82EFDB74; continue 'dispatch;
	}
	// 82EFDB60: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EFDB64: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82EFDB68: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82EFDB6C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EFDB70: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFDB78: 80950008  lwz r4, 8(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDB7C: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82EFDB80: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EFDB84: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFDB88: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82EFDB8C: 4BFA3C2D  bl 0x82ea17b8
	ctx.lr = 0x82EFDB90;
	sub_82EA17B8(ctx, base);
	// 82EFDB90: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82EFDB94: 482AA600  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDB98 size=108
    let mut pc: u32 = 0x82EFDB98;
    'dispatch: loop {
        match pc {
            0x82EFDB98 => {
    //   block [0x82EFDB98..0x82EFDC04)
	// 82EFDB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFDBA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFDBA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFDBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDBAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFDBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFDBB4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EFDBB8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFDBBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFDBC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EFDBC4: 895F0025  lbz r10, 0x25(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 82EFDBC8: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82EFDBCC: 554806BE  clrlwi r8, r10, 0x1a
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82EFDBD0: 991F0025  stb r8, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[8].u8 ) };
	// 82EFDBD4: 4BFEE37D  bl 0x82eebf50
	ctx.lr = 0x82EFDBD8;
	sub_82EEBF50(ctx, base);
	// 82EFDBD8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EFDBDC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFDBE0: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFDBE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFDBE8: 4BFE3B69  bl 0x82ee1750
	ctx.lr = 0x82EFDBEC;
	sub_82EE1750(ctx, base);
	// 82EFDBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFDBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFDBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFDBF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFDBFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFDC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDC08 size=236
    let mut pc: u32 = 0x82EFDC08;
    'dispatch: loop {
        match pc {
            0x82EFDC08 => {
    //   block [0x82EFDC08..0x82EFDCF4)
	// 82EFDC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDC0C: 482AA551  bl 0x831a815c
	ctx.lr = 0x82EFDC10;
	sub_831A8130(ctx, base);
	// 82EFDC10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDC14: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDC18: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82EFDC1C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EFDC20: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EFDC24: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFDC28: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFDC2C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDC30: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDC34: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDC38: 40980020  bge cr6, 0x82efdc58
	if !ctx.cr[6].lt {
	pc = 0x82EFDC58; continue 'dispatch;
	}
	// 82EFDC3C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFDC40: 3909B91C  addi r8, r9, -0x46e4
	ctx.r[8].s64 = ctx.r[9].s64 + -18148;
	// 82EFDC44: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFDC48: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFDC4C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFDC50: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDC54: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDC58: 833F0004  lwz r25, 4(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDC5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFDC60: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EFDC64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDC68: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EFDC6C: 3B8B005C  addi r28, r11, 0x5c
	ctx.r[28].s64 = ctx.r[11].s64 + 92;
	// 82EFDC70: 89190025  lbz r8, 0x25(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(37 as u32) ) } as u64;
	// 82EFDC74: 550706BE  clrlwi r7, r8, 0x1a
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000003Fu64;
	// 82EFDC78: 98F90025  stb r7, 0x25(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(37 as u32), ctx.r[7].u8 ) };
	// 82EFDC7C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFDC80: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82EFDC84: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82EFDC88: 4BFEE2C9  bl 0x82eebf50
	ctx.lr = 0x82EFDC8C;
	sub_82EEBF50(ctx, base);
	// 82EFDC8C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EFDC90: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFDC94: 80990018  lwz r4, 0x18(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFDC98: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EFDC9C: 4BFE3AB5  bl 0x82ee1750
	ctx.lr = 0x82EFDCA0;
	sub_82EE1750(ctx, base);
	// 82EFDCA0: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFDCA4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDCA8: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDCAC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EFDCB0: 40980020  bge cr6, 0x82efdcd0
	if !ctx.cr[6].lt {
	pc = 0x82EFDCD0; continue 'dispatch;
	}
	// 82EFDCB4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82EFDCB8: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82EFDCBC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFDCC0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFDCC4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFDCC8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDCCC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDCD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFDCD4: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDCD8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EFDCDC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFDCE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFDCE4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFDCE8: 4BFA3AD1  bl 0x82ea17b8
	ctx.lr = 0x82EFDCEC;
	sub_82EA17B8(ctx, base);
	// 82EFDCEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EFDCF0: 482AA4BC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDCF8 size=268
    let mut pc: u32 = 0x82EFDCF8;
    'dispatch: loop {
        match pc {
            0x82EFDCF8 => {
    //   block [0x82EFDCF8..0x82EFDE04)
	// 82EFDCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDCFC: 482AA461  bl 0x831a815c
	ctx.lr = 0x82EFDD00;
	sub_831A8130(ctx, base);
	// 82EFDD00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDD04: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDD08: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82EFDD0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFDD10: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EFDD14: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFDD18: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFDD1C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDD20: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDD24: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDD28: 4098002C  bge cr6, 0x82efdd54
	if !ctx.cr[6].lt {
	pc = 0x82EFDD54; continue 'dispatch;
	}
	// 82EFDD2C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFDD30: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFDD34: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFDD38: 38C8B928  addi r6, r8, -0x46d8
	ctx.r[6].s64 = ctx.r[8].s64 + -18136;
	// 82EFDD3C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFDD40: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFDD44: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFDD48: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFDD4C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDD50: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFDD54: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDD58: 390000D0  li r8, 0xd0
	ctx.r[8].s64 = 208;
	// 82EFDD5C: A17F0026  lhz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 82EFDD60: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDD64: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82EFDD68: A33F0024  lhz r25, 0x24(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFDD6C: 5566103E  rotlwi r6, r11, 2
	ctx.r[6].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82EFDD70: 83C50050  lwz r30, 0x50(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFDD74: 54893830  slwi r9, r4, 7
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(7);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFDD78: 81650030  lwz r11, 0x30(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFDD7C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EFDD80: 7FDE3214  add r30, r30, r6
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[6].u64;
	// 82EFDD84: 388A0190  addi r4, r10, 0x190
	ctx.r[4].s64 = ctx.r[10].s64 + 400;
	// 82EFDD88: 386A01A0  addi r3, r10, 0x1a0
	ctx.r[3].s64 = ctx.r[10].s64 + 416;
	// 82EFDD8C: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFDD90: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EFDD94: 4BFFCEBD  bl 0x82efac50
	ctx.lr = 0x82EFDD98;
	sub_82EFAC50(ctx, base);
	// 82EFDD98: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82EFDD9C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EFDDA0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDDA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFDDA8: 8063006C  lwz r3, 0x6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EFDDAC: 4BFF8F55  bl 0x82ef6d00
	ctx.lr = 0x82EFDDB0;
	sub_82EF6D00(ctx, base);
	// 82EFDDB0: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFDDB4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDDB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDDBC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDDC0: 40980020  bge cr6, 0x82efdde0
	if !ctx.cr[6].lt {
	pc = 0x82EFDDE0; continue 'dispatch;
	}
	// 82EFDDC4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFDDC8: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFDDCC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFDDD0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFDDD4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFDDD8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDDDC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDDE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFDDE4: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDDE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFDDEC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFDDF0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFDDF4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFDDF8: 4BFA39C1  bl 0x82ea17b8
	ctx.lr = 0x82EFDDFC;
	sub_82EA17B8(ctx, base);
	// 82EFDDFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFDE00: 482AA3AC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDE08 size=196
    let mut pc: u32 = 0x82EFDE08;
    'dispatch: loop {
        match pc {
            0x82EFDE08 => {
    //   block [0x82EFDE08..0x82EFDECC)
	// 82EFDE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDE0C: 482AA359  bl 0x831a8164
	ctx.lr = 0x82EFDE10;
	sub_831A8130(ctx, base);
	// 82EFDE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDE14: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDE18: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82EFDE1C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFDE20: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EFDE24: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFDE28: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFDE2C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDE30: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDE34: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDE38: 4098002C  bge cr6, 0x82efde64
	if !ctx.cr[6].lt {
	pc = 0x82EFDE64; continue 'dispatch;
	}
	// 82EFDE3C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFDE40: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFDE44: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFDE48: 38C8B93C  addi r6, r8, -0x46c4
	ctx.r[6].s64 = ctx.r[8].s64 + -18116;
	// 82EFDE4C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFDE50: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFDE54: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFDE58: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFDE5C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDE60: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFDE64: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDE68: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFDE6C: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 82EFDE70: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFDE74: 4800D635  bl 0x82f0b4a8
	ctx.lr = 0x82EFDE78;
	sub_82F0B4A8(ctx, base);
	// 82EFDE78: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFDE7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDE80: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDE84: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDE88: 40980020  bge cr6, 0x82efdea8
	if !ctx.cr[6].lt {
	pc = 0x82EFDEA8; continue 'dispatch;
	}
	// 82EFDE8C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFDE90: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFDE94: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFDE98: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFDE9C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFDEA0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDEA4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDEA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFDEAC: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDEB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFDEB4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFDEB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFDEBC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFDEC0: 4BFA38F9  bl 0x82ea17b8
	ctx.lr = 0x82EFDEC4;
	sub_82EA17B8(ctx, base);
	// 82EFDEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFDEC8: 482AA2EC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDED0 size=292
    let mut pc: u32 = 0x82EFDED0;
    'dispatch: loop {
        match pc {
            0x82EFDED0 => {
    //   block [0x82EFDED0..0x82EFDFF4)
	// 82EFDED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDED4: 482AA291  bl 0x831a8164
	ctx.lr = 0x82EFDED8;
	sub_831A8130(ctx, base);
	// 82EFDED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDEDC: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDEE0: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82EFDEE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFDEE8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EFDEEC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFDEF0: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EFDEF4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDEF8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDEFC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDF00: 4098002C  bge cr6, 0x82efdf2c
	if !ctx.cr[6].lt {
	pc = 0x82EFDF2C; continue 'dispatch;
	}
	// 82EFDF04: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFDF08: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFDF0C: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFDF10: 38C8B948  addi r6, r8, -0x46b8
	ctx.r[6].s64 = ctx.r[8].s64 + -18104;
	// 82EFDF14: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFDF18: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFDF1C: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFDF20: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFDF24: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDF28: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFDF2C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDF30: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFDF34: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFDF38: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EFDF3C: 80AA02B8  lwz r5, 0x2b8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(696 as u32) ) } as u64;
	// 82EFDF40: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDF44: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDF48: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFDF4C: 90BF0014  stw r5, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82EFDF50: 409A003C  bne cr6, 0x82efdf8c
	if !ctx.cr[6].eq {
	pc = 0x82EFDF8C; continue 'dispatch;
	}
	// 82EFDF54: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFDF58: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82EFDF5C: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EFDF60: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFDF64: 40990028  ble cr6, 0x82efdf8c
	if !ctx.cr[6].gt {
	pc = 0x82EFDF8C; continue 'dispatch;
	}
	// 82EFDF68: 39680008  addi r11, r8, 8
	ctx.r[11].s64 = ctx.r[8].s64 + 8;
	// 82EFDF6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFDF70: 914BFFF8  stw r10, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 82EFDF74: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFDF78: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 82EFDF7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFDF80: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFDF84: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EFDF88: 4082FFE8  bne 0x82efdf70
	if !ctx.cr[0].eq {
	pc = 0x82EFDF70; continue 'dispatch;
	}
	// 82EFDF8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDF90: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFDF94: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 82EFDF98: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFDF9C: 48011E6D  bl 0x82f0fe08
	ctx.lr = 0x82EFDFA0;
	sub_82F0FE08(ctx, base);
	// 82EFDFA0: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EFDFA4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDFA8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDFAC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFDFB0: 40980020  bge cr6, 0x82efdfd0
	if !ctx.cr[6].lt {
	pc = 0x82EFDFD0; continue 'dispatch;
	}
	// 82EFDFB4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFDFB8: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFDFBC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFDFC0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFDFC4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFDFC8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFDFCC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFDFD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFDFD4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDFD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFDFDC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFDFE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFDFE4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFDFE8: 4BFA37D1  bl 0x82ea17b8
	ctx.lr = 0x82EFDFEC;
	sub_82EA17B8(ctx, base);
	// 82EFDFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFDFF0: 482AA1C4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDFF8 size=220
    let mut pc: u32 = 0x82EFDFF8;
    'dispatch: loop {
        match pc {
            0x82EFDFF8 => {
    //   block [0x82EFDFF8..0x82EFE0D4)
	// 82EFDFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDFFC: 482AA169  bl 0x831a8164
	ctx.lr = 0x82EFE000;
	sub_831A8130(ctx, base);
	// 82EFE000: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE004: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE008: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82EFE00C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFE010: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EFE014: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFE018: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFE01C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE020: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE024: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE028: 4098002C  bge cr6, 0x82efe054
	if !ctx.cr[6].lt {
	pc = 0x82EFE054; continue 'dispatch;
	}
	// 82EFE02C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFE030: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFE034: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFE038: 38C8B950  addi r6, r8, -0x46b0
	ctx.r[6].s64 = ctx.r[8].s64 + -18096;
	// 82EFE03C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFE040: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFE044: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFE048: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFE04C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE050: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFE054: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFE058: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE05C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82EFE060: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE064: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82EFE068: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 82EFE06C: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE070: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE074: 4800D52D  bl 0x82f0b5a0
	ctx.lr = 0x82EFE078;
	sub_82F0B5A0(ctx, base);
	// 82EFE078: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFE07C: 993F001C  stb r9, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u8 ) };
	// 82EFE080: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EFE084: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE088: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE08C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EFE090: 40980020  bge cr6, 0x82efe0b0
	if !ctx.cr[6].lt {
	pc = 0x82EFE0B0; continue 'dispatch;
	}
	// 82EFE094: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFE098: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFE09C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFE0A0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFE0A4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFE0A8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFE0AC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE0B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFE0B4: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE0B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFE0BC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFE0C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFE0C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE0C8: 4BFA36F1  bl 0x82ea17b8
	ctx.lr = 0x82EFE0CC;
	sub_82EA17B8(ctx, base);
	// 82EFE0CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFE0D0: 482AA0E4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE0D8 size=204
    let mut pc: u32 = 0x82EFE0D8;
    'dispatch: loop {
        match pc {
            0x82EFE0D8 => {
    //   block [0x82EFE0D8..0x82EFE1A4)
	// 82EFE0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE0DC: 482AA089  bl 0x831a8164
	ctx.lr = 0x82EFE0E0;
	sub_831A8130(ctx, base);
	// 82EFE0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE0E4: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE0E8: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82EFE0EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFE0F0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EFE0F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFE0F8: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EFE0FC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE100: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE104: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE108: 4098002C  bge cr6, 0x82efe134
	if !ctx.cr[6].lt {
	pc = 0x82EFE134; continue 'dispatch;
	}
	// 82EFE10C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFE110: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFE114: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFE118: 38C8B95C  addi r6, r8, -0x46a4
	ctx.r[6].s64 = ctx.r[8].s64 + -18084;
	// 82EFE11C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFE120: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFE124: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFE128: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFE12C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE130: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFE134: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE138: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE13C: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 82EFE140: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE144: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE148: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE14C: 4801824D  bl 0x82f16398
	ctx.lr = 0x82EFE150;
	sub_82F16398(ctx, base);
	// 82EFE150: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EFE154: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE158: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE15C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE160: 40980020  bge cr6, 0x82efe180
	if !ctx.cr[6].lt {
	pc = 0x82EFE180; continue 'dispatch;
	}
	// 82EFE164: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFE168: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFE16C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFE170: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFE174: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFE178: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFE17C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE180: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFE184: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE188: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFE18C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EFE190: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE194: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE198: 4BFA3621  bl 0x82ea17b8
	ctx.lr = 0x82EFE19C;
	sub_82EA17B8(ctx, base);
	// 82EFE19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFE1A0: 482AA014  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE1A8 size=428
    let mut pc: u32 = 0x82EFE1A8;
    'dispatch: loop {
        match pc {
            0x82EFE1A8 => {
    //   block [0x82EFE1A8..0x82EFE354)
	// 82EFE1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE1AC: 482A9FB1  bl 0x831a815c
	ctx.lr = 0x82EFE1B0;
	sub_831A8130(ctx, base);
	// 82EFE1B0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE1B4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE1B8: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82EFE1BC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFE1C0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EFE1C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFE1C8: 7D5BE02E  lwzx r10, r27, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFE1CC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE1D0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE1D4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE1D8: 4098002C  bge cr6, 0x82efe204
	if !ctx.cr[6].lt {
	pc = 0x82EFE204; continue 'dispatch;
	}
	// 82EFE1DC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFE1E0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFE1E4: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFE1E8: 38C8B948  addi r6, r8, -0x46b8
	ctx.r[6].s64 = ctx.r[8].s64 + -18104;
	// 82EFE1EC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFE1F0: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFE1F4: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFE1F8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFE1FC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE200: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFE204: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFE208: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFE20C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE210: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFE214: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE218: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFE21C: 38CA000F  addi r6, r10, 0xf
	ctx.r[6].s64 = ctx.r[10].s64 + 15;
	// 82EFE220: 80FE0020  lwz r7, 0x20(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFE224: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFE228: 54CAE13E  srwi r10, r6, 4
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFE22C: 7C885A14  add r4, r8, r11
	ctx.r[4].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82EFE230: 7CC75A14  add r6, r7, r11
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EFE234: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFE238: 40990024  ble cr6, 0x82efe25c
	if !ctx.cr[6].gt {
	pc = 0x82EFE25C; continue 'dispatch;
	}
	// 82EFE23C: 39660008  addi r11, r6, 8
	ctx.r[11].s64 = ctx.r[6].s64 + 8;
	// 82EFE240: 93ABFFF8  stw r29, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[29].u32 ) };
	// 82EFE244: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFE248: 93ABFFFC  stw r29, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 82EFE24C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EFE250: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFE254: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EFE258: 4082FFE8  bne 0x82efe240
	if !ctx.cr[0].eq {
	pc = 0x82EFE240; continue 'dispatch;
	}
	// 82EFE25C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE260: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 82EFE264: 4800F9F5  bl 0x82f0dc58
	ctx.lr = 0x82EFE268;
	sub_82F0DC58(ctx, base);
	// 82EFE268: A15E0002  lhz r10, 2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EFE26C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE270: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82EFE274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EFE278: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82EFE27C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE280: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFE284: B1410052  sth r10, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u16 ) };
	// 82EFE288: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EFE28C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82EFE290: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE294: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82EFE298: 811F0078  lwz r8, 0x78(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EFE29C: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82EFE2A0: B3A10076  sth r29, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[29].u16 ) };
	// 82EFE2A4: A0FF0054  lhz r7, 0x54(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFE2A8: B0E10074  sth r7, 0x74(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u16 ) };
	// 82EFE2AC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE2B0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82EFE2B4: 4BFA3365  bl 0x82ea1618
	ctx.lr = 0x82EFE2B8;
	sub_82EA1618(ctx, base);
	// 82EFE2B8: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFE2BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE2C0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE2C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE2C8: 40980020  bge cr6, 0x82efe2e8
	if !ctx.cr[6].lt {
	pc = 0x82EFE2E8; continue 'dispatch;
	}
	// 82EFE2CC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFE2D0: 3909B95C  addi r8, r9, -0x46a4
	ctx.r[8].s64 = ctx.r[9].s64 + -18084;
	// 82EFE2D4: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFE2D8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFE2DC: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFE2E0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFE2E4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE2E8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE2EC: 80DF0030  lwz r6, 0x30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFE2F0: 386B01A0  addi r3, r11, 0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + 416;
	// 82EFE2F4: 80BF003C  lwz r5, 0x3c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EFE2F8: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EFE2FC: 4801809D  bl 0x82f16398
	ctx.lr = 0x82EFE300;
	sub_82F16398(ctx, base);
	// 82EFE300: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFE304: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE308: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE30C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE310: 40980020  bge cr6, 0x82efe330
	if !ctx.cr[6].lt {
	pc = 0x82EFE330; continue 'dispatch;
	}
	// 82EFE314: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFE318: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFE31C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFE320: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFE324: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82EFE328: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFE32C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE330: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFE334: 809A0008  lwz r4, 8(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE338: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFE33C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EFE340: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFE344: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EFE348: 4BFA3471  bl 0x82ea17b8
	ctx.lr = 0x82EFE34C;
	sub_82EA17B8(ctx, base);
	// 82EFE34C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82EFE350: 482A9E5C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE358 size=316
    let mut pc: u32 = 0x82EFE358;
    'dispatch: loop {
        match pc {
            0x82EFE358 => {
    //   block [0x82EFE358..0x82EFE494)
	// 82EFE358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE35C: 482A9E05  bl 0x831a8160
	ctx.lr = 0x82EFE360;
	sub_831A8130(ctx, base);
	// 82EFE360: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFE368: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFE36C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EFE370: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE374: 837E0004  lwz r27, 4(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE378: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE37C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EFE380: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFE384: 409900D8  ble cr6, 0x82efe45c
	if !ctx.cr[6].gt {
	pc = 0x82EFE45C; continue 'dispatch;
	}
	// 82EFE388: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82EFE38C: 3B400200  li r26, 0x200
	ctx.r[26].s64 = 512;
	// 82EFE390: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82EFE394: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EFE398: 3BBD001C  addi r29, r29, 0x1c
	ctx.r[29].s64 = ctx.r[29].s64 + 28;
	// 82EFE39C: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EFE3A0: 40980020  bge cr6, 0x82efe3c0
	if !ctx.cr[6].lt {
	pc = 0x82EFE3C0; continue 'dispatch;
	}
	// 82EFE3A4: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE3A8: 7C004A2C  dcbt 0, r9
	// 82EFE3AC: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82EFE3B0: 7C084A2C  dcbt r8, r9
	// 82EFE3B4: 80FD0008  lwz r7, 8(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE3B8: 7C003A2C  dcbt 0, r7
	// 82EFE3BC: 7C0AD22C  dcbt r10, r26
	// 82EFE3C0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE3C4: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82EFE3C8: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE3CC: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82EFE3D0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE3D4: A10B0018  lhz r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE3D8: 5509383E  rotlwi r9, r8, 7
	ctx.r[9].u64 = ((ctx.r[8].u32).rotate_left(7)) as u64;
	// 82EFE3DC: 7CE95214  add r7, r9, r10
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EFE3E0: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82EFE3E4: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE3E8: A0CB001A  lhz r6, 0x1a(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EFE3EC: 54CA383E  rotlwi r10, r6, 7
	ctx.r[10].u64 = ((ctx.r[6].u32).rotate_left(7)) as u64;
	// 82EFE3F0: 7CAA4A14  add r5, r10, r9
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFE3F4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EFE3F8: 90BF0034  stw r5, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[5].u32 ) };
	// 82EFE3FC: A08B0018  lhz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE400: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82EFE404: A06B001A  lhz r3, 0x1a(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EFE408: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EFE40C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE410: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82EFE414: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE418: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EFE41C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE420: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82EFE424: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE428: A08B0014  lhz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE42C: A0E30000  lhz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE430: 2B070016  cmplwi cr6, r7, 0x16
	ctx.cr[6].compare_u32(ctx.r[7].u32, 22 as u32, &mut ctx.xer);
	// 82EFE434: 409A0014  bne cr6, 0x82efe448
	if !ctx.cr[6].eq {
	pc = 0x82EFE448; continue 'dispatch;
	}
	// 82EFE438: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EFE43C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFE440: 4800B919  bl 0x82f09d58
	ctx.lr = 0x82EFE444;
	sub_82F09D58(ctx, base);
	// 82EFE444: 4800000C  b 0x82efe450
	pc = 0x82EFE450; continue 'dispatch;
	// 82EFE448: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFE44C: 480095F5  bl 0x82f07a40
	ctx.lr = 0x82EFE450;
	sub_82F07A40(ctx, base);
	// 82EFE450: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFE454: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EFE458: 4198FF38  blt cr6, 0x82efe390
	if ctx.cr[6].lt {
	pc = 0x82EFE390; continue 'dispatch;
	}
	// 82EFE45C: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFE464: 409A0010  bne cr6, 0x82efe474
	if !ctx.cr[6].eq {
	pc = 0x82EFE474; continue 'dispatch;
	}
	// 82EFE468: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFE46C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFE470: 482A9D40  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82EFE474: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE478: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE47C: 40810010  ble 0x82efe48c
	if !ctx.cr[0].gt {
	pc = 0x82EFE48C; continue 'dispatch;
	}
	// 82EFE480: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82EFE484: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFE488: 992A0003  stb r9, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 82EFE48C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFE490: 482A9D20  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE498 size=508
    let mut pc: u32 = 0x82EFE498;
    'dispatch: loop {
        match pc {
            0x82EFE498 => {
    //   block [0x82EFE498..0x82EFE694)
	// 82EFE498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE49C: 482A9CA9  bl 0x831a8144
	ctx.lr = 0x82EFE4A0;
	sub_831A8130(ctx, base);
	// 82EFE4A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE4A4: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE4A8: 3A800018  li r20, 0x18
	ctx.r[20].s64 = 24;
	// 82EFE4AC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82EFE4B0: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82EFE4B4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82EFE4B8: 7D54A82E  lwzx r10, r20, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82EFE4BC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE4C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE4C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE4C8: 4098002C  bge cr6, 0x82efe4f4
	if !ctx.cr[6].lt {
	pc = 0x82EFE4F4; continue 'dispatch;
	}
	// 82EFE4CC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFE4D0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFE4D4: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFE4D8: 38C8B96C  addi r6, r8, -0x4694
	ctx.r[6].s64 = ctx.r[8].s64 + -18068;
	// 82EFE4DC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFE4E0: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFE4E4: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFE4E8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFE4EC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE4F0: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFE4F4: 81790010  lwz r11, 0x10(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE4F8: 3BF60080  addi r31, r22, 0x80
	ctx.r[31].s64 = ctx.r[22].s64 + 128;
	// 82EFE4FC: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE500: 3BCB0018  addi r30, r11, 0x18
	ctx.r[30].s64 = ctx.r[11].s64 + 24;
	// 82EFE504: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE508: 836B0014  lwz r27, 0x14(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE50C: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE510: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE514: 830B0010  lwz r24, 0x10(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE518: 419A0014  beq cr6, 0x82efe52c
	if ctx.cr[6].eq {
	pc = 0x82EFE52C; continue 'dispatch;
	}
	// 82EFE51C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE520: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82EFE524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFE528: 409A0008  bne cr6, 0x82efe530
	if !ctx.cr[6].eq {
	pc = 0x82EFE530; continue 'dispatch;
	}
	// 82EFE52C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82EFE530: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFE534: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFE538: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EFE53C: 409900CC  ble cr6, 0x82efe608
	if !ctx.cr[6].gt {
	pc = 0x82EFE608; continue 'dispatch;
	}
	// 82EFE540: 3B400200  li r26, 0x200
	ctx.r[26].s64 = 512;
	// 82EFE544: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EFE548: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EFE54C: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 82EFE550: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EFE554: 40980020  bge cr6, 0x82efe574
	if !ctx.cr[6].lt {
	pc = 0x82EFE574; continue 'dispatch;
	}
	// 82EFE558: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE55C: 7C004A2C  dcbt 0, r9
	// 82EFE560: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82EFE564: 7C084A2C  dcbt r8, r9
	// 82EFE568: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE56C: 7C003A2C  dcbt 0, r7
	// 82EFE570: 7C0AD22C  dcbt r10, r26
	// 82EFE574: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE578: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82EFE57C: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFE580: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82EFE584: A10B0018  lhz r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE588: 550A383E  rotlwi r10, r8, 7
	ctx.r[10].u64 = ((ctx.r[8].u32).rotate_left(7)) as u64;
	// 82EFE58C: 7CEAE214  add r7, r10, r28
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82EFE590: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82EFE594: A0CB001A  lhz r6, 0x1a(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EFE598: 54CA383E  rotlwi r10, r6, 7
	ctx.r[10].u64 = ((ctx.r[6].u32).rotate_left(7)) as u64;
	// 82EFE59C: 7CAAE214  add r5, r10, r28
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82EFE5A0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EFE5A4: 90BF0034  stw r5, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[5].u32 ) };
	// 82EFE5A8: A08B0018  lhz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE5AC: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82EFE5B0: A06B001A  lhz r3, 0x1a(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(26 as u32) ) } as u64;
	// 82EFE5B4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EFE5B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE5BC: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82EFE5C0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE5C4: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EFE5C8: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE5CC: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82EFE5D0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE5D4: A08B0014  lhz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE5D8: A0E30000  lhz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE5DC: 2B070016  cmplwi cr6, r7, 0x16
	ctx.cr[6].compare_u32(ctx.r[7].u32, 22 as u32, &mut ctx.xer);
	// 82EFE5E0: 409A0014  bne cr6, 0x82efe5f4
	if !ctx.cr[6].eq {
	pc = 0x82EFE5F4; continue 'dispatch;
	}
	// 82EFE5E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EFE5E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFE5EC: 4800B76D  bl 0x82f09d58
	ctx.lr = 0x82EFE5F0;
	sub_82F09D58(ctx, base);
	// 82EFE5F0: 4800000C  b 0x82efe5fc
	pc = 0x82EFE5FC; continue 'dispatch;
	// 82EFE5F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFE5F8: 48009449  bl 0x82f07a40
	ctx.lr = 0x82EFE5FC;
	sub_82F07A40(ctx, base);
	// 82EFE5FC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFE600: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EFE604: 4198FF40  blt cr6, 0x82efe544
	if ctx.cr[6].lt {
	pc = 0x82EFE544; continue 'dispatch;
	}
	// 82EFE608: 7EEB0774  extsb r11, r23
	ctx.r[11].s64 = ctx.r[23].s8 as i64;
	// 82EFE60C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE610: 409A0014  bne cr6, 0x82efe624
	if !ctx.cr[6].eq {
	pc = 0x82EFE624; continue 'dispatch;
	}
	// 82EFE614: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82EFE618: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFE61C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82EFE620: 48000020  b 0x82efe640
	pc = 0x82EFE640; continue 'dispatch;
	// 82EFE624: 7D6AC051  subf. r11, r10, r24
	ctx.r[11].s64 = ctx.r[24].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE628: 40810018  ble 0x82efe640
	if !ctx.cr[0].gt {
	pc = 0x82EFE640; continue 'dispatch;
	}
	// 82EFE62C: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82EFE630: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFE634: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 82EFE638: 992A0003  stb r9, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 82EFE63C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82EFE640: 7D54A82E  lwzx r10, r20, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82EFE644: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE648: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE64C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE650: 40980020  bge cr6, 0x82efe670
	if !ctx.cr[6].lt {
	pc = 0x82EFE670; continue 'dispatch;
	}
	// 82EFE654: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFE658: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFE65C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFE660: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFE664: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFE668: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFE66C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE670: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFE674: 80960008  lwz r4, 8(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFE67C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82EFE680: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EFE684: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82EFE688: 4BFA3131  bl 0x82ea17b8
	ctx.lr = 0x82EFE68C;
	sub_82EA17B8(ctx, base);
	// 82EFE68C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82EFE690: 482A9B04  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE698 size=472
    let mut pc: u32 = 0x82EFE698;
    'dispatch: loop {
        match pc {
            0x82EFE698 => {
    //   block [0x82EFE698..0x82EFE870)
	// 82EFE698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE69C: 482A9AB1  bl 0x831a814c
	ctx.lr = 0x82EFE6A0;
	sub_831A8130(ctx, base);
	// 82EFE6A0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE6A4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82EFE6A8: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82EFE6AC: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82EFE6B0: 3BD60080  addi r30, r22, 0x80
	ctx.r[30].s64 = ctx.r[22].s64 + 128;
	// 82EFE6B4: 83B70008  lwz r29, 8(r23)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE6B8: 817D0074  lwz r11, 0x74(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EFE6BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE6C0: 4099018C  ble cr6, 0x82efe84c
	if !ctx.cr[6].gt {
	pc = 0x82EFE84C; continue 'dispatch;
	}
	// 82EFE6C4: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE6C8: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82EFE6CC: 7D5AC02E  lwzx r10, r26, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EFE6D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE6D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE6D8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE6DC: 4098002C  bge cr6, 0x82efe708
	if !ctx.cr[6].lt {
	pc = 0x82EFE708; continue 'dispatch;
	}
	// 82EFE6E0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82EFE6E4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82EFE6E8: 38E9B440  addi r7, r9, -0x4bc0
	ctx.r[7].s64 = ctx.r[9].s64 + -19392;
	// 82EFE6EC: 38C8B980  addi r6, r8, -0x4680
	ctx.r[6].s64 = ctx.r[8].s64 + -18048;
	// 82EFE6F0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EFE6F4: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82EFE6F8: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82EFE6FC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFE700: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE704: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFE708: 817D0074  lwz r11, 0x74(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EFE70C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82EFE710: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 82EFE714: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE718: 409900E0  ble cr6, 0x82efe7f8
	if !ctx.cr[6].gt {
	pc = 0x82EFE7F8; continue 'dispatch;
	}
	// 82EFE71C: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82EFE720: 817D0070  lwz r11, 0x70(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EFE724: 7FEBE02E  lwzx r31, r11, r28
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EFE728: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE72C: 915E0048  stw r10, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82EFE730: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFE734: 913E004C  stw r9, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EFE738: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE73C: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFE740: 81480090  lwz r10, 0x90(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFE744: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFE748: 90FE0030  stw r7, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82EFE74C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE750: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFE754: 81460090  lwz r10, 0x90(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 82EFE758: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFE75C: 90BE0034  stw r5, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[5].u32 ) };
	// 82EFE760: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE764: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE768: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE76C: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82EFE770: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE774: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE778: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFE77C: 911E003C  stw r8, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[8].u32 ) };
	// 82EFE780: 88FF0012  lbz r7, 0x12(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EFE784: 54E607FE  clrlwi r6, r7, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 82EFE788: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EFE78C: 419A002C  beq cr6, 0x82efe7b8
	if ctx.cr[6].eq {
	pc = 0x82EFE7B8; continue 'dispatch;
	}
	// 82EFE790: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE794: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE798: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE79C: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE7A0: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 82EFE7A4: 4198000C  blt cr6, 0x82efe7b0
	if ctx.cr[6].lt {
	pc = 0x82EFE7B0; continue 'dispatch;
	}
	// 82EFE7A8: 80A50014  lwz r5, 0x14(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFE7AC: 4BFFFFF0  b 0x82efe79c
	pc = 0x82EFE79C; continue 'dispatch;
	// 82EFE7B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFE7B4: 4BFF95CD  bl 0x82ef7d80
	ctx.lr = 0x82EFE7B8;
	sub_82EF7D80(ctx, base);
	// 82EFE7B8: 897F0012  lbz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82EFE7BC: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFE7C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE7C4: 419A0020  beq cr6, 0x82efe7e4
	if ctx.cr[6].eq {
	pc = 0x82EFE7E4; continue 'dispatch;
	}
	// 82EFE7C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE7CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFE7D0: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE7D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE7D8: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFE7DC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EFE7E0: 4E800421  bctrl
	ctx.lr = 0x82EFE7E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFE7E4: 817D0074  lwz r11, 0x74(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EFE7E8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82EFE7EC: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82EFE7F0: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EFE7F4: 4198FF2C  blt cr6, 0x82efe720
	if ctx.cr[6].lt {
	pc = 0x82EFE720; continue 'dispatch;
	}
	// 82EFE7F8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82EFE7FC: 815D0074  lwz r10, 0x74(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EFE800: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82EFE804: 809D0070  lwz r4, 0x70(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EFE808: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFE80C: 7C7A582E  lwzx r3, r26, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFE810: 4BFA1FA1  bl 0x82ea07b0
	ctx.lr = 0x82EFE814;
	sub_82EA07B0(ctx, base);
	// 82EFE814: 933D0074  stw r25, 0x74(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82EFE818: 933D0070  stw r25, 0x70(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82EFE81C: 7D5AC02E  lwzx r10, r26, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82EFE820: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFE824: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE828: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EFE82C: 40980020  bge cr6, 0x82efe84c
	if !ctx.cr[6].lt {
	pc = 0x82EFE84C; continue 'dispatch;
	}
	// 82EFE830: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82EFE834: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82EFE838: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EFE83C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82EFE840: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82EFE844: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EFE848: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82EFE84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFE850: 80960008  lwz r4, 8(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFE858: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82EFE85C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82EFE860: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82EFE864: 4BFA2F55  bl 0x82ea17b8
	ctx.lr = 0x82EFE868;
	sub_82EA17B8(ctx, base);
	// 82EFE868: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EFE86C: 482A9930  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


