pub fn sub_82412960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412960 size=4
    let mut pc: u32 = 0x82412960;
    'dispatch: loop {
        match pc {
            0x82412960 => {
    //   block [0x82412960..0x82412964)
	// 82412960: 4BFFFBC8  b 0x82412528
	sub_82412528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82412968 size=388
    let mut pc: u32 = 0x82412968;
    'dispatch: loop {
        match pc {
            0x82412968 => {
    //   block [0x82412968..0x82412AEC)
	// 82412968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241296C: 4812274D  bl 0x825350b8
	ctx.lr = 0x82412970;
	sub_82535080(ctx, base);
	// 82412970: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82412974: 48123671  bl 0x82535fe4
	ctx.lr = 0x82412978;
	sub_82535FB0(ctx, base);
	// 82412978: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241297C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412980: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 82412984: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82412988: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8241298C: 4BFFFAFD  bl 0x82412488
	ctx.lr = 0x82412990;
	sub_82412488(ctx, base);
	// 82412990: 48000F69  bl 0x824138f8
	ctx.lr = 0x82412994;
	sub_824138F8(ctx, base);
	// 82412994: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412998: 4082000C  bne 0x824129a4
	if !ctx.cr[0].eq {
	pc = 0x824129A4; continue 'dispatch;
	}
	// 8241299C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824129A0: 4800013C  b 0x82412adc
	pc = 0x82412ADC; continue 'dispatch;
	// 824129A4: C01C0000  lfs f0, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824129A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824129AC: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824129B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824129B4: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824129B8: 4BFFFAD1  bl 0x82412488
	ctx.lr = 0x824129BC;
	sub_82412488(ctx, base);
	// 824129BC: 4800269D  bl 0x82415058
	ctx.lr = 0x824129C0;
	sub_82415058(ctx, base);
	// 824129C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824129C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824129C8: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 824129CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824129D0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824129D4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824129D8: FFC00018  frsp f30, f0
	ctx.f[30].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824129DC: 4BFFFAAD  bl 0x82412488
	ctx.lr = 0x824129E0;
	sub_82412488(ctx, base);
	// 824129E0: 48000F41  bl 0x82413920
	ctx.lr = 0x824129E4;
	sub_82413920(ctx, base);
	// 824129E4: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 824129E8: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 824129EC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824129F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824129F4: C38B1850  lfs f28, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 824129F8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824129FC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82412A00: FFA00018  frsp f29, f0
	ctx.f[29].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82412A04: 41980010  blt cr6, 0x82412a14
	if ctx.cr[6].lt {
	pc = 0x82412A14; continue 'dispatch;
	}
	// 82412A08: EC1DE02A  fadds f0, f29, f28
	ctx.f[0].f64 = ((ctx.f[29].f64 + ctx.f[28].f64) as f32) as f64;
	// 82412A0C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82412A10: 419900C4  bgt cr6, 0x82412ad4
	if ctx.cr[6].gt {
	pc = 0x82412AD4; continue 'dispatch;
	}
	// 82412A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A18: 4BFFFAC9  bl 0x824124e0
	ctx.lr = 0x82412A1C;
	sub_824124E0(ctx, base);
	// 82412A1C: 48000EDD  bl 0x824138f8
	ctx.lr = 0x82412A20;
	sub_824138F8(ctx, base);
	// 82412A20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412A24: 4182FF78  beq 0x8241299c
	if ctx.cr[0].eq {
	pc = 0x8241299C; continue 'dispatch;
	}
	// 82412A28: 389F004C  addi r4, r31, 0x4c
	ctx.r[4].s64 = ctx.r[31].s64 + 76;
	// 82412A2C: 83DF004C  lwz r30, 0x4c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A34: 4BFFF5CD  bl 0x82412000
	ctx.lr = 0x82412A38;
	sub_82412000(ctx, base);
	// 82412A38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412A3C: 41820074  beq 0x82412ab0
	if ctx.cr[0].eq {
	pc = 0x82412AB0; continue 'dispatch;
	}
	// 82412A40: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412A44: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82412A48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82412A4C: 4800113D  bl 0x82413b88
	ctx.lr = 0x82412A50;
	sub_82413B88(ctx, base);
	// 82412A50: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82412A54: 40980024  bge cr6, 0x82412a78
	if !ctx.cr[6].lt {
	pc = 0x82412A78; continue 'dispatch;
	}
	// 82412A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A5C: 4BFFFA2D  bl 0x82412488
	ctx.lr = 0x82412A60;
	sub_82412488(ctx, base);
	// 82412A60: 48000EC1  bl 0x82413920
	ctx.lr = 0x82412A64;
	sub_82413920(ctx, base);
	// 82412A64: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82412A68: EC1EF828  fsubs f0, f30, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 82412A6C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82412A70: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82412A74: 4800002C  b 0x82412aa0
	pc = 0x82412AA0; continue 'dispatch;
	// 82412A78: EFDDE02A  fadds f30, f29, f28
	ctx.f[30].f64 = ((ctx.f[29].f64 + ctx.f[28].f64) as f32) as f64;
	// 82412A7C: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82412A80: 41990054  bgt cr6, 0x82412ad4
	if ctx.cr[6].gt {
	pc = 0x82412AD4; continue 'dispatch;
	}
	// 82412A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412A88: 4BFFFA01  bl 0x82412488
	ctx.lr = 0x82412A8C;
	sub_82412488(ctx, base);
	// 82412A8C: 480025CD  bl 0x82415058
	ctx.lr = 0x82412A90;
	sub_82415058(ctx, base);
	// 82412A90: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82412A94: EC1FF028  fsubs f0, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 82412A98: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82412A9C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82412AA0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82412AA4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82412AA8: EFED002A  fadds f31, f13, f0
	ctx.f[31].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82412AAC: 48000028  b 0x82412ad4
	pc = 0x82412AD4; continue 'dispatch;
	// 82412AB0: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82412AB4: 4098000C  bge cr6, 0x82412ac0
	if !ctx.cr[6].lt {
	pc = 0x82412AC0; continue 'dispatch;
	}
	// 82412AB8: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 82412ABC: 48000014  b 0x82412ad0
	pc = 0x82412AD0; continue 'dispatch;
	// 82412AC0: EC1DE02A  fadds f0, f29, f28
	ctx.f[0].f64 = ((ctx.f[29].f64 + ctx.f[28].f64) as f32) as f64;
	// 82412AC4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82412AC8: 4199000C  bgt cr6, 0x82412ad4
	if ctx.cr[6].gt {
	pc = 0x82412AD4; continue 'dispatch;
	}
	// 82412ACC: EFFDD82A  fadds f31, f29, f27
	ctx.f[31].f64 = ((ctx.f[29].f64 + ctx.f[27].f64) as f32) as f64;
	// 82412AD0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82412AD4: D3FC0000  stfs f31, 0(r28)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82412AD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82412ADC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82412AE0: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82412AE4: 4812354D  bl 0x82536030
	ctx.lr = 0x82412AE8;
	sub_82535FFC(ctx, base);
	// 82412AE8: 48122620  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412AF0 size=36
    let mut pc: u32 = 0x82412AF0;
    'dispatch: loop {
        match pc {
            0x82412AF0 => {
    //   block [0x82412AF0..0x82412B14)
	// 82412AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412AFC: 4BFFF98D  bl 0x82412488
	ctx.lr = 0x82412B00;
	sub_82412488(ctx, base);
	// 82412B00: 4BEE96D1  bl 0x822fc1d0
	ctx.lr = 0x82412B04;
	sub_822FC1D0(ctx, base);
	// 82412B04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412B18 size=8
    let mut pc: u32 = 0x82412B18;
    'dispatch: loop {
        match pc {
            0x82412B18 => {
    //   block [0x82412B18..0x82412B20)
	// 82412B18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82412B1C: 4BFFF8EC  b 0x82412408
	sub_82412408(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412B20 size=36
    let mut pc: u32 = 0x82412B20;
    'dispatch: loop {
        match pc {
            0x82412B20 => {
    //   block [0x82412B20..0x82412B44)
	// 82412B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412B2C: 4BFFF95D  bl 0x82412488
	ctx.lr = 0x82412B30;
	sub_82412488(ctx, base);
	// 82412B30: 48002529  bl 0x82415058
	ctx.lr = 0x82412B34;
	sub_82415058(ctx, base);
	// 82412B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412B48 size=36
    let mut pc: u32 = 0x82412B48;
    'dispatch: loop {
        match pc {
            0x82412B48 => {
    //   block [0x82412B48..0x82412B6C)
	// 82412B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412B54: 4BFFF935  bl 0x82412488
	ctx.lr = 0x82412B58;
	sub_82412488(ctx, base);
	// 82412B58: 48000DC9  bl 0x82413920
	ctx.lr = 0x82412B5C;
	sub_82413920(ctx, base);
	// 82412B5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412B70 size=68
    let mut pc: u32 = 0x82412B70;
    'dispatch: loop {
        match pc {
            0x82412B70 => {
    //   block [0x82412B70..0x82412BB4)
	// 82412B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412B78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412B7C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82412B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412B84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82412B88: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82412B8C: 4BFFF8FD  bl 0x82412488
	ctx.lr = 0x82412B90;
	sub_82412488(ctx, base);
	// 82412B90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82412B94: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82412B98: 48001101  bl 0x82413c98
	ctx.lr = 0x82412B9C;
	sub_82413C98(ctx, base);
	// 82412B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412BA8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82412BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412BB8 size=720
    let mut pc: u32 = 0x82412BB8;
    'dispatch: loop {
        match pc {
            0x82412BB8 => {
    //   block [0x82412BB8..0x82412E88)
	// 82412BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412BBC: 481224E9  bl 0x825350a4
	ctx.lr = 0x82412BC0;
	sub_82535080(ctx, base);
	// 82412BC0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412BC4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82412BC8: 3BF80008  addi r31, r24, 8
	ctx.r[31].s64 = ctx.r[24].s64 + 8;
	// 82412BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412BD0: 48055EF1  bl 0x82468ac0
	ctx.lr = 0x82412BD4;
	sub_82468AC0(ctx, base);
	// 82412BD4: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82412BD8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82412BDC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82412BE0: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82412BE4: 40990008  ble cr6, 0x82412bec
	if !ctx.cr[6].gt {
	pc = 0x82412BEC; continue 'dispatch;
	}
	// 82412BE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82412BEC: 4BFF21DD  bl 0x82404dc8
	ctx.lr = 0x82412BF0;
	sub_82404DC8(ctx, base);
	// 82412BF0: 3B580048  addi r26, r24, 0x48
	ctx.r[26].s64 = ctx.r[24].s64 + 72;
	// 82412BF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412BF8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82412BFC: 4800141D  bl 0x82414018
	ctx.lr = 0x82412C00;
	sub_82414018(ctx, base);
	// 82412C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C04: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412C08: 48055EB9  bl 0x82468ac0
	ctx.lr = 0x82412C0C;
	sub_82468AC0(ctx, base);
	// 82412C0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412C10: 418201B0  beq 0x82412dc0
	if ctx.cr[0].eq {
	pc = 0x82412DC0; continue 'dispatch;
	}
	// 82412C14: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82412C18: 4BFFDBA9  bl 0x824107c0
	ctx.lr = 0x82412C1C;
	sub_824107C0(ctx, base);
	// 82412C1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C20: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82412C24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82412C28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412C2C: 4E800421  bctrl
	ctx.lr = 0x82412C30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412C30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412C34: 41820018  beq 0x82412c4c
	if ctx.cr[0].eq {
	pc = 0x82412C4C; continue 'dispatch;
	}
	// 82412C38: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 82412C3C: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C40: 48001569  bl 0x824141a8
	ctx.lr = 0x82412C44;
	sub_824141A8(ctx, base);
	// 82412C44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82412C48: 48000008  b 0x82412c50
	pc = 0x82412C50; continue 'dispatch;
	// 82412C4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82412C50: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C58: 7D6AE12E  stwx r11, r10, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 82412C5C: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C60: 480018E1  bl 0x82414540
	ctx.lr = 0x82412C64;
	sub_82414540(ctx, base);
	// 82412C64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82412C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C6C: 4BFFE8AD  bl 0x82411518
	ctx.lr = 0x82412C70;
	sub_82411518(ctx, base);
	// 82412C70: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82412C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C78: 480018C1  bl 0x82414538
	ctx.lr = 0x82412C7C;
	sub_82414538(ctx, base);
	// 82412C7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412C80: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82412C84: 7C7EE02E  lwzx r3, r30, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412C88: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82412C8C: 48000C3D  bl 0x824138c8
	ctx.lr = 0x82412C90;
	sub_824138C8(ctx, base);
	// 82412C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412C94: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412C98: 48001899  bl 0x82414530
	ctx.lr = 0x82412C9C;
	sub_82414530(ctx, base);
	// 82412C9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412CA0: 7C7EE02E  lwzx r3, r30, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412CA4: 480015C5  bl 0x82414268
	ctx.lr = 0x82412CA8;
	sub_82414268(ctx, base);
	// 82412CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412CAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82412CB0: 48001881  bl 0x82414530
	ctx.lr = 0x82412CB4;
	sub_82414530(ctx, base);
	// 82412CB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412CB8: 418200F0  beq 0x82412da8
	if ctx.cr[0].eq {
	pc = 0x82412DA8; continue 'dispatch;
	}
	// 82412CBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CC0: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412CC8: 48001901  bl 0x824145c8
	ctx.lr = 0x82412CCC;
	sub_824145C8(ctx, base);
	// 82412CCC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82412CD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412CD8: 480018C9  bl 0x824145a0
	ctx.lr = 0x82412CDC;
	sub_824145A0(ctx, base);
	// 82412CDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82412CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CE4: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412CE8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82412CEC: 48000C6D  bl 0x82413958
	ctx.lr = 0x82412CF0;
	sub_82413958(ctx, base);
	// 82412CF0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412CF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412CF8: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D00: 480018F1  bl 0x824145f0
	ctx.lr = 0x82412D04;
	sub_824145F0(ctx, base);
	// 82412D04: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82412D08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D0C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412D10: 48000C19  bl 0x82413928
	ctx.lr = 0x82412D14;
	sub_82413928(ctx, base);
	// 82412D14: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D20: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412D24: 480018CD  bl 0x824145f0
	ctx.lr = 0x82412D28;
	sub_824145F0(ctx, base);
	// 82412D28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412D2C: 41820068  beq 0x82412d94
	if ctx.cr[0].eq {
	pc = 0x82412D94; continue 'dispatch;
	}
	// 82412D30: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82412D34: 833A0000  lwz r25, 0(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412D38: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D44: 48001945  bl 0x82414688
	ctx.lr = 0x82412D48;
	sub_82414688(ctx, base);
	// 82412D48: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82412D4C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82412D50: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D5C: 480018D5  bl 0x82414630
	ctx.lr = 0x82412D60;
	sub_82414630(ctx, base);
	// 82412D60: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82412D64: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82412D68: 7C79E02E  lwzx r3, r25, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82412D6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D70: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82412D74: 48000C45  bl 0x824139b8
	ctx.lr = 0x82412D78;
	sub_824139B8(ctx, base);
	// 82412D78: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412D7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D84: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412D88: 48001869  bl 0x824145f0
	ctx.lr = 0x82412D8C;
	sub_824145F0(ctx, base);
	// 82412D8C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412D90: 4198FFA0  blt cr6, 0x82412d30
	if ctx.cr[6].lt {
	pc = 0x82412D30; continue 'dispatch;
	}
	// 82412D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412D98: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82412D9C: 48001795  bl 0x82414530
	ctx.lr = 0x82412DA0;
	sub_82414530(ctx, base);
	// 82412DA0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412DA4: 4198FF18  blt cr6, 0x82412cbc
	if ctx.cr[6].lt {
	pc = 0x82412CBC; continue 'dispatch;
	}
	// 82412DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412DAC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82412DB0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82412DB4: 48055D0D  bl 0x82468ac0
	ctx.lr = 0x82412DB8;
	sub_82468AC0(ctx, base);
	// 82412DB8: 7F1B1840  cmplw cr6, r27, r3
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412DBC: 4198FE5C  blt cr6, 0x82412c18
	if ctx.cr[6].lt {
	pc = 0x82412C18; continue 'dispatch;
	}
	// 82412DC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412DC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412DC8: 48001769  bl 0x82414530
	ctx.lr = 0x82412DCC;
	sub_82414530(ctx, base);
	// 82412DCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412DD0: 418200B0  beq 0x82412e80
	if ctx.cr[0].eq {
	pc = 0x82412E80; continue 'dispatch;
	}
	// 82412DD4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412DD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82412DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412DE0: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412DE4: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412DE8: 480017B9  bl 0x824145a0
	ctx.lr = 0x82412DEC;
	sub_824145A0(ctx, base);
	// 82412DEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412DF0: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82412DF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412DFC: 4E800421  bctrl
	ctx.lr = 0x82412E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412E00: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82412E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E08: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412E0C: 48055CB5  bl 0x82468ac0
	ctx.lr = 0x82412E10;
	sub_82468AC0(ctx, base);
	// 82412E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412E14: 41820058  beq 0x82412e6c
	if ctx.cr[0].eq {
	pc = 0x82412E6C; continue 'dispatch;
	}
	// 82412E18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82412E1C: 839A0000  lwz r28, 0(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412E20: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82412E24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82412E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E2C: 419A0018  beq cr6, 0x82412e44
	if ctx.cr[6].eq {
	pc = 0x82412E44; continue 'dispatch;
	}
	// 82412E30: 48001771  bl 0x824145a0
	ctx.lr = 0x82412E34;
	sub_824145A0(ctx, base);
	// 82412E34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412E38: 7C7CF02E  lwzx r3, r28, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82412E3C: 48000ED5  bl 0x82413d10
	ctx.lr = 0x82412E40;
	sub_82413D10(ctx, base);
	// 82412E40: 48000014  b 0x82412e54
	pc = 0x82412E54; continue 'dispatch;
	// 82412E44: 4800175D  bl 0x824145a0
	ctx.lr = 0x82412E48;
	sub_824145A0(ctx, base);
	// 82412E48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412E4C: 7C7CF02E  lwzx r3, r28, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82412E50: 48000EF1  bl 0x82413d40
	ctx.lr = 0x82412E54;
	sub_82413D40(ctx, base);
	// 82412E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E58: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82412E5C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82412E60: 48055C61  bl 0x82468ac0
	ctx.lr = 0x82412E64;
	sub_82468AC0(ctx, base);
	// 82412E64: 7F1B1840  cmplw cr6, r27, r3
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412E68: 4198FFB4  blt cr6, 0x82412e1c
	if ctx.cr[6].lt {
	pc = 0x82412E1C; continue 'dispatch;
	}
	// 82412E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412E70: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412E74: 480016BD  bl 0x82414530
	ctx.lr = 0x82412E78;
	sub_82414530(ctx, base);
	// 82412E78: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412E7C: 4198FF58  blt cr6, 0x82412dd4
	if ctx.cr[6].lt {
	pc = 0x82412DD4; continue 'dispatch;
	}
	// 82412E80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82412E84: 48122270  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412E88 size=4
    let mut pc: u32 = 0x82412E88;
    'dispatch: loop {
        match pc {
            0x82412E88 => {
    //   block [0x82412E88..0x82412E8C)
	// 82412E88: 4BFFFD30  b 0x82412bb8
	sub_82412BB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412E90 size=88
    let mut pc: u32 = 0x82412E90;
    'dispatch: loop {
        match pc {
            0x82412E90 => {
    //   block [0x82412E90..0x82412EE8)
	// 82412E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412E98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412E9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82412EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412EA8: 396BEC38  addi r11, r11, -0x13c8
	ctx.r[11].s64 = ctx.r[11].s64 + -5064;
	// 82412EAC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82412EB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82412EB4: 4182001C  beq 0x82412ed0
	if ctx.cr[0].eq {
	pc = 0x82412ED0; continue 'dispatch;
	}
	// 82412EB8: 4BFFD909  bl 0x824107c0
	ctx.lr = 0x82412EBC;
	sub_824107C0(ctx, base);
	// 82412EBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412EC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82412EC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412EC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412ECC: 4E800421  bctrl
	ctx.lr = 0x82412ED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412ED4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412EE8 size=8
    let mut pc: u32 = 0x82412EE8;
    'dispatch: loop {
        match pc {
            0x82412EE8 => {
    //   block [0x82412EE8..0x82412EF0)
	// 82412EE8: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82412EEC: 4800228C  b 0x82415178
	sub_82415178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412EF0 size=8
    let mut pc: u32 = 0x82412EF0;
    'dispatch: loop {
        match pc {
            0x82412EF0 => {
    //   block [0x82412EF0..0x82412EF8)
	// 82412EF0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82412EF4: 480022D4  b 0x824151c8
	sub_824151C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412EF8 size=8
    let mut pc: u32 = 0x82412EF8;
    'dispatch: loop {
        match pc {
            0x82412EF8 => {
    //   block [0x82412EF8..0x82412F00)
	// 82412EF8: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82412EFC: 48002454  b 0x82415350
	sub_82415350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412F00 size=8
    let mut pc: u32 = 0x82412F00;
    'dispatch: loop {
        match pc {
            0x82412F00 => {
    //   block [0x82412F00..0x82412F08)
	// 82412F00: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82412F04: 4800261C  b 0x82415520
	sub_82415520(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412F08 size=504
    let mut pc: u32 = 0x82412F08;
    'dispatch: loop {
        match pc {
            0x82412F08 => {
    //   block [0x82412F08..0x82413100)
	// 82412F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412F0C: 4812218D  bl 0x82535098
	ctx.lr = 0x82412F10;
	sub_82535080(ctx, base);
	// 82412F10: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412F14: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82412F18: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82412F1C: 3BFA0008  addi r31, r26, 8
	ctx.r[31].s64 = ctx.r[26].s64 + 8;
	// 82412F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F24: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412F28: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412F2C: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412F30: 480020E9  bl 0x82415018
	ctx.lr = 0x82412F34;
	sub_82415018(ctx, base);
	// 82412F34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82412F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F3C: 4800210D  bl 0x82415048
	ctx.lr = 0x82412F40;
	sub_82415048(ctx, base);
	// 82412F40: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82412F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F48: 480020F9  bl 0x82415040
	ctx.lr = 0x82412F4C;
	sub_82415040(ctx, base);
	// 82412F4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412F50: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82412F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412F58: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82412F5C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82412F60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412F64: 4E800421  bctrl
	ctx.lr = 0x82412F68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412F68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412F6C: 40820008  bne 0x82412f74
	if !ctx.cr[0].eq {
	pc = 0x82412F74; continue 'dispatch;
	}
	// 82412F70: 48000000  b 0x82412f70
	pc = 0x82412F70; continue 'dispatch;
	// 82412F74: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412F78: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82412F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412F80: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82412F84: 832B0010  lwz r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82412F88: 480015A9  bl 0x82414530
	ctx.lr = 0x82412F8C;
	sub_82414530(ctx, base);
	// 82412F8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412F90: 418200A4  beq 0x82413034
	if ctx.cr[0].eq {
	pc = 0x82413034; continue 'dispatch;
	}
	// 82412F94: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82412F98: B3810052  sth r28, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[28].u16 ) };
	// 82412F9C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82412FA0: B3810050  sth r28, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u16 ) };
	// 82412FA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82412FA8: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82412FAC: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 82412FB0: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82412FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412FB8: 48002261  bl 0x82415218
	ctx.lr = 0x82412FBC;
	sub_82415218(ctx, base);
	// 82412FBC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412FC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412FC4: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82412FC8: 40820008  bne 0x82412fd0
	if !ctx.cr[0].eq {
	pc = 0x82412FD0; continue 'dispatch;
	}
	// 82412FCC: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82412FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412FD4: 83B90000  lwz r29, 0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412FD8: 82E10054  lwz r23, 0x54(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82412FDC: 82C10058  lwz r22, 0x58(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82412FE0: A2A10050  lhz r21, 0x50(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82412FE4: A2810052  lhz r20, 0x52(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 82412FE8: 48001541  bl 0x82414528
	ctx.lr = 0x82412FEC;
	sub_82414528(ctx, base);
	// 82412FEC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82412FF0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82412FF4: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82412FF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82412FFC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82413000: 7E87A378  mr r7, r20
	ctx.r[7].u64 = ctx.r[20].u64;
	// 82413004: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82413008: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 8241300C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82413010: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413014: 4E800421  bctrl
	ctx.lr = 0x82413018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82413018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241301C: 48002255  bl 0x82415270
	ctx.lr = 0x82413020;
	sub_82415270(ctx, base);
	// 82413020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413024: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82413028: 48001509  bl 0x82414530
	ctx.lr = 0x8241302C;
	sub_82414530(ctx, base);
	// 8241302C: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82413030: 4198FF64  blt cr6, 0x82412f94
	if ctx.cr[6].lt {
	pc = 0x82412F94; continue 'dispatch;
	}
	// 82413034: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241303C: 48002285  bl 0x824152c0
	ctx.lr = 0x82413040;
	sub_824152C0(ctx, base);
	// 82413040: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413048: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8241304C: 832B0010  lwz r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82413050: 48002001  bl 0x82415050
	ctx.lr = 0x82413054;
	sub_82415050(ctx, base);
	// 82413054: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413058: 41820094  beq 0x824130ec
	if ctx.cr[0].eq {
	pc = 0x824130EC; continue 'dispatch;
	}
	// 8241305C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82413060: B3810052  sth r28, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[28].u16 ) };
	// 82413064: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82413068: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8241306C: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 82413070: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82413074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413078: 48002329  bl 0x824153a0
	ctx.lr = 0x8241307C;
	sub_824153A0(ctx, base);
	// 8241307C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413080: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413084: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82413088: 40820008  bne 0x82413090
	if !ctx.cr[0].eq {
	pc = 0x82413090; continue 'dispatch;
	}
	// 8241308C: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82413090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413094: 83B90000  lwz r29, 0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413098: 82E10058  lwz r23, 0x58(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8241309C: 82C10054  lwz r22, 0x54(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824130A0: A2A10052  lhz r21, 0x52(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 824130A4: 48001485  bl 0x82414528
	ctx.lr = 0x824130A8;
	sub_82414528(ctx, base);
	// 824130A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824130AC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824130B0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 824130B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824130B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824130BC: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 824130C0: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 824130C4: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 824130C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824130CC: 4E800421  bctrl
	ctx.lr = 0x824130D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824130D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824130D4: 48002315  bl 0x824153e8
	ctx.lr = 0x824130D8;
	sub_824153E8(ctx, base);
	// 824130D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824130DC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824130E0: 48001F71  bl 0x82415050
	ctx.lr = 0x824130E4;
	sub_82415050(ctx, base);
	// 824130E4: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824130E8: 4198FF74  blt cr6, 0x8241305c
	if ctx.cr[6].lt {
	pc = 0x8241305C; continue 'dispatch;
	}
	// 824130EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824130F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824130F4: 48002345  bl 0x82415438
	ctx.lr = 0x824130F8;
	sub_82415438(ctx, base);
	// 824130F8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824130FC: 48121FEC  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413100 size=8
    let mut pc: u32 = 0x82413100;
    'dispatch: loop {
        match pc {
            0x82413100 => {
    //   block [0x82413100..0x82413108)
	// 82413100: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82413104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413108 size=8
    let mut pc: u32 = 0x82413108;
    'dispatch: loop {
        match pc {
            0x82413108 => {
    //   block [0x82413108..0x82413110)
	// 82413108: 80630094  lwz r3, 0x94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8241310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413110 size=20
    let mut pc: u32 = 0x82413110;
    'dispatch: loop {
        match pc {
            0x82413110 => {
    //   block [0x82413110..0x82413124)
	// 82413110: 80630098  lwz r3, 0x98(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82413114: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413118: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241311C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413120: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413128 size=172
    let mut pc: u32 = 0x82413128;
    'dispatch: loop {
        match pc {
            0x82413128 => {
    //   block [0x82413128..0x824131D4)
	// 82413128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413134: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82413138: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8241313C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82413140: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82413144: 81630088  lwz r11, 0x88(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413148: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 8241314C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413150: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82413154: 2B0A8002  cmplwi cr6, r10, 0x8002
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32770 as u32, &mut ctx.xer);
	// 82413158: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8241315C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413160: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82413164: 419A005C  beq cr6, 0x824131c0
	if ctx.cr[6].eq {
	pc = 0x824131C0; continue 'dispatch;
	}
	// 82413168: 2B0A8003  cmplwi cr6, r10, 0x8003
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32771 as u32, &mut ctx.xer);
	// 8241316C: 419A0054  beq cr6, 0x824131c0
	if ctx.cr[6].eq {
	pc = 0x824131C0; continue 'dispatch;
	}
	// 82413170: 2B0A8001  cmplwi cr6, r10, 0x8001
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32769 as u32, &mut ctx.xer);
	// 82413174: 409A0044  bne cr6, 0x824131b8
	if !ctx.cr[6].eq {
	pc = 0x824131B8; continue 'dispatch;
	}
	// 82413178: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8241317C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82413180: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82413184: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82413188: 48002341  bl 0x824154c8
	ctx.lr = 0x8241318C;
	sub_824154C8(ctx, base);
	// 8241318C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413190: 40820008  bne 0x82413198
	if !ctx.cr[0].eq {
	pc = 0x82413198; continue 'dispatch;
	}
	// 82413194: 48000000  b 0x82413194
	pc = 0x82413194; continue 'dispatch;
	// 82413198: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241319C: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 824131A0: 419A000C  beq cr6, 0x824131ac
	if ctx.cr[6].eq {
	pc = 0x824131AC; continue 'dispatch;
	}
	// 824131A4: 2B0B8003  cmplwi cr6, r11, 0x8003
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32771 as u32, &mut ctx.xer);
	// 824131A8: 409A0010  bne cr6, 0x824131b8
	if !ctx.cr[6].eq {
	pc = 0x824131B8; continue 'dispatch;
	}
	// 824131AC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824131B0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824131B4: 48000010  b 0x824131c4
	pc = 0x824131C4; continue 'dispatch;
	// 824131B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824131BC: 48000008  b 0x824131c4
	pc = 0x824131C4; continue 'dispatch;
	// 824131C0: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824131C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824131C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824131CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824131D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824131D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824131D8 size=256
    let mut pc: u32 = 0x824131D8;
    'dispatch: loop {
        match pc {
            0x824131D8 => {
    //   block [0x824131D8..0x824132D8)
	// 824131D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824131DC: 48121EE1  bl 0x825350bc
	ctx.lr = 0x824131E0;
	sub_82535080(ctx, base);
	// 824131E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824131E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824131E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824131EC: 817D0090  lwz r11, 0x90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 824131F0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824131F4: 41980010  blt cr6, 0x82413204
	if ctx.cr[6].lt {
	pc = 0x82413204; continue 'dispatch;
	}
	// 824131F8: 817D0094  lwz r11, 0x94(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(148 as u32) ) } as u64;
	// 824131FC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82413200: 4098000C  bge cr6, 0x8241320c
	if !ctx.cr[6].lt {
	pc = 0x8241320C; continue 'dispatch;
	}
	// 82413204: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413208: 480000C8  b 0x824132d0
	pc = 0x824132D0; continue 'dispatch;
	// 8241320C: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 82413210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413214: 48001E7D  bl 0x82415090
	ctx.lr = 0x82413218;
	sub_82415090(ctx, base);
	// 82413218: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241321C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413220: 48002779  bl 0x82415998
	ctx.lr = 0x82413224;
	sub_82415998(ctx, base);
	// 82413224: 817D0090  lwz r11, 0x90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82413228: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241322C: 409A0018  bne cr6, 0x82413244
	if !ctx.cr[6].eq {
	pc = 0x82413244; continue 'dispatch;
	}
	// 82413230: 817D0088  lwz r11, 0x88(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413234: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413238: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 8241323C: 419A0090  beq cr6, 0x824132cc
	if ctx.cr[6].eq {
	pc = 0x824132CC; continue 'dispatch;
	}
	// 82413240: 48000000  b 0x82413240
	pc = 0x82413240; continue 'dispatch;
	// 82413244: 815D009C  lwz r10, 0x9c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(156 as u32) ) } as u64;
	// 82413248: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241324C: 41820020  beq 0x8241326c
	if ctx.cr[0].eq {
	pc = 0x8241326C; continue 'dispatch;
	}
	// 82413250: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 82413254: 41980018  blt cr6, 0x8241326c
	if ctx.cr[6].lt {
	pc = 0x8241326C; continue 'dispatch;
	}
	// 82413258: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241325C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413260: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82413264: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 82413268: 48001E11  bl 0x82415078
	ctx.lr = 0x8241326C;
	sub_82415078(ctx, base);
	// 8241326C: 817D0088  lwz r11, 0x88(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413270: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413274: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 82413278: 409A0014  bne cr6, 0x8241328c
	if !ctx.cr[6].eq {
	pc = 0x8241328C; continue 'dispatch;
	}
	// 8241327C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82413280: 4BFFFEA9  bl 0x82413128
	ctx.lr = 0x82413284;
	sub_82413128(ctx, base);
	// 82413284: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82413288: 419A0044  beq cr6, 0x824132cc
	if ctx.cr[6].eq {
	pc = 0x824132CC; continue 'dispatch;
	}
	// 8241328C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413290: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413294: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413298: 48001E51  bl 0x824150e8
	ctx.lr = 0x8241329C;
	sub_824150E8(ctx, base);
	// 8241329C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824132A0: 40820028  bne 0x824132c8
	if !ctx.cr[0].eq {
	pc = 0x824132C8; continue 'dispatch;
	}
	// 824132A4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824132A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824132AC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824132B0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824132B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824132B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824132BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824132C0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824132C4: 4BFFFFA8  b 0x8241326c
	pc = 0x8241326C; continue 'dispatch;
	// 824132C8: 48000000  b 0x824132c8
	pc = 0x824132C8; continue 'dispatch;
	// 824132CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824132D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824132D4: 48121E38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824132D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824132D8 size=608
    let mut pc: u32 = 0x824132D8;
    'dispatch: loop {
        match pc {
            0x824132D8 => {
    //   block [0x824132D8..0x82413538)
	// 824132D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824132DC: 48121DD1  bl 0x825350ac
	ctx.lr = 0x824132E0;
	sub_82535080(ctx, base);
	// 824132E0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 824132E4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824132E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824132EC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824132F0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 824132F4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 824132F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824132FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413300: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413304: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82413308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241330C: 83AA0010  lwz r29, 0x10(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82413310: 419A0014  beq cr6, 0x82413324
	if ctx.cr[6].eq {
	pc = 0x82413324; continue 'dispatch;
	}
	// 82413314: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413318: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241331C: 3B6B0004  addi r27, r11, 4
	ctx.r[27].s64 = ctx.r[11].s64 + 4;
	// 82413320: 48000008  b 0x82413328
	pc = 0x82413328; continue 'dispatch;
	// 82413324: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82413328: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241332C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82413330: 480011F9  bl 0x82414528
	ctx.lr = 0x82413334;
	sub_82414528(ctx, base);
	// 82413334: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413338: B3E10060  sth r31, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u16 ) };
	// 8241333C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82413340: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82413344: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82413348: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241334C: A14A0000  lhz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413350: 2B0A8002  cmplwi cr6, r10, 0x8002
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32770 as u32, &mut ctx.xer);
	// 82413354: 419A0010  beq cr6, 0x82413364
	if ctx.cr[6].eq {
	pc = 0x82413364; continue 'dispatch;
	}
	// 82413358: 2B0A8001  cmplwi cr6, r10, 0x8001
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32769 as u32, &mut ctx.xer);
	// 8241335C: 419A0008  beq cr6, 0x82413364
	if ctx.cr[6].eq {
	pc = 0x82413364; continue 'dispatch;
	}
	// 82413360: 48000000  b 0x82413360
	pc = 0x82413360; continue 'dispatch;
	// 82413364: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413368: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8241336C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82413370: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82413374: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82413378: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8241337C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82413384: B1210060  sth r9, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u16 ) };
	// 82413388: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241338C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82413390: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82413394: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413398: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241339C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824133A0: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824133A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824133A8: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 824133AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824133B0: 4E800421  bctrl
	ctx.lr = 0x824133B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824133B4: 48000094  b 0x82413448
	pc = 0x82413448; continue 'dispatch;
	// 824133B8: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 824133BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824133C0: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824133C4: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824133C8: 48001D21  bl 0x824150e8
	ctx.lr = 0x824133CC;
	sub_824150E8(ctx, base);
	// 824133CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824133D0: 40820024  bne 0x824133f4
	if !ctx.cr[0].eq {
	pc = 0x824133F4; continue 'dispatch;
	}
	// 824133D4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824133D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824133DC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824133E0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824133E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824133E8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824133EC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824133F0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824133F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824133F8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824133FC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82413400: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82413404: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82413408: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8241340C: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413410: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82413414: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413418: B1210060  sth r9, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u16 ) };
	// 8241341C: 814B0088  lwz r10, 0x88(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413420: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82413424: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82413428: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241342C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413430: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82413434: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8241343C: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82413440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413444: 4E800421  bctrl
	ctx.lr = 0x82413448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82413448: A1610060  lhz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241344C: 2B0B8003  cmplwi cr6, r11, 0x8003
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32771 as u32, &mut ctx.xer);
	// 82413450: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413454: 409AFF64  bne cr6, 0x824133b8
	if !ctx.cr[6].eq {
	pc = 0x824133B8; continue 'dispatch;
	}
	// 82413458: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 8241345C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82413460: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82413464: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82413468: 48002061  bl 0x824154c8
	ctx.lr = 0x8241346C;
	sub_824154C8(ctx, base);
	// 8241346C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413470: 40820008  bne 0x82413478
	if !ctx.cr[0].eq {
	pc = 0x82413478; continue 'dispatch;
	}
	// 82413474: 48000000  b 0x82413474
	pc = 0x82413474; continue 'dispatch;
	// 82413478: A1610060  lhz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241347C: 2B0B8004  cmplwi cr6, r11, 0x8004
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32772 as u32, &mut ctx.xer);
	// 82413480: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413484: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82413488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241348C: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413490: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413494: 409A006C  bne cr6, 0x82413500
	if !ctx.cr[6].eq {
	pc = 0x82413500; continue 'dispatch;
	}
	// 82413498: 48001C51  bl 0x824150e8
	ctx.lr = 0x8241349C;
	sub_824150E8(ctx, base);
	// 8241349C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824134A0: 40820024  bne 0x824134c4
	if !ctx.cr[0].eq {
	pc = 0x824134C4; continue 'dispatch;
	}
	// 824134A4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824134A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824134AC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824134B0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824134B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824134B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824134BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824134C0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824134C4: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 824134C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824134CC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824134D0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 824134D4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824134D8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 824134DC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824134E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824134E4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 824134E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824134EC: A1210060  lhz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824134F0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824134F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824134F8: 4E800421  bctrl
	ctx.lr = 0x824134FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824134FC: 48000030  b 0x8241352c
	pc = 0x8241352C; continue 'dispatch;
	// 82413500: 48001BE9  bl 0x824150e8
	ctx.lr = 0x82413504;
	sub_824150E8(ctx, base);
	// 82413504: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413508: 40820024  bne 0x8241352c
	if !ctx.cr[0].eq {
	pc = 0x8241352C; continue 'dispatch;
	}
	// 8241350C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413510: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82413514: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82413518: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241351C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413520: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82413524: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82413528: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8241352C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82413530: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82413534: 48121BC8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82413538 size=96
    let mut pc: u32 = 0x82413538;
    'dispatch: loop {
        match pc {
            0x82413538 => {
    //   block [0x82413538..0x82413598)
	// 82413538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241353C: 48121B81  bl 0x825350bc
	ctx.lr = 0x82413540;
	sub_82535080(ctx, base);
	// 82413540: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82413544: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413548: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241354C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82413550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413554: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82413558: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8241355C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413560: FC00FE5E  fctidz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82413564: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82413568: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241356C: 4BFFFC6D  bl 0x824131d8
	ctx.lr = 0x82413570;
	sub_824131D8(ctx, base);
	// 82413570: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413574: 41820018  beq 0x8241358c
	if ctx.cr[0].eq {
	pc = 0x8241358C; continue 'dispatch;
	}
	// 82413578: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241357C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413580: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82413584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413588: 4BFFFD51  bl 0x824132d8
	ctx.lr = 0x8241358C;
	sub_824132D8(ctx, base);
	// 8241358C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413590: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82413594: 48121B78  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413598 size=652
    let mut pc: u32 = 0x82413598;
    'dispatch: loop {
        match pc {
            0x82413598 => {
    //   block [0x82413598..0x82413824)
	// 82413598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241359C: 48121B11  bl 0x825350ac
	ctx.lr = 0x824135A0;
	sub_82535080(ctx, base);
	// 824135A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824135A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824135A8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824135AC: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 824135B0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824135B4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824135B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824135BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824135C0: 480023E9  bl 0x824159a8
	ctx.lr = 0x824135C4;
	sub_824159A8(ctx, base);
	// 824135C4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824135C8: 3B9F0098  addi r28, r31, 0x98
	ctx.r[28].s64 = ctx.r[31].s64 + 152;
	// 824135CC: 3B3F009C  addi r25, r31, 0x9c
	ctx.r[25].s64 = ctx.r[31].s64 + 156;
	// 824135D0: 935F0090  stw r26, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[26].u32 ) };
	// 824135D4: 935F0094  stw r26, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[26].u32 ) };
	// 824135D8: 935F0098  stw r26, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[26].u32 ) };
	// 824135DC: 935F009C  stw r26, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[26].u32 ) };
	// 824135E0: 4BFFD1E1  bl 0x824107c0
	ctx.lr = 0x824135E4;
	sub_824107C0(ctx, base);
	// 824135E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824135E8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824135EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824135F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824135F4: 4E800421  bctrl
	ctx.lr = 0x824135F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824135F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824135FC: 4182001C  beq 0x82413618
	if ctx.cr[0].eq {
	pc = 0x82413618; continue 'dispatch;
	}
	// 82413600: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82413604: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82413608: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8241360C: 396BEC40  addi r11, r11, -0x13c0
	ctx.r[11].s64 = ctx.r[11].s64 + -5056;
	// 82413610: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82413614: 48000008  b 0x8241361c
	pc = 0x8241361C; continue 'dispatch;
	// 82413618: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8241361C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82413620: 4BFFD1B9  bl 0x824107d8
	ctx.lr = 0x82413624;
	sub_824107D8(ctx, base);
	// 82413624: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241362C: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82413630: 48002369  bl 0x82415998
	ctx.lr = 0x82413634;
	sub_82415998(ctx, base);
	// 82413634: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413638: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241363C: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 82413640: 419A0008  beq cr6, 0x82413648
	if ctx.cr[6].eq {
	pc = 0x82413648; continue 'dispatch;
	}
	// 82413644: 48000000  b 0x82413644
	pc = 0x82413644; continue 'dispatch;
	// 82413648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241364C: 80BE0084  lwz r5, 0x84(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413650: 809E0080  lwz r4, 0x80(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413654: 48001A95  bl 0x824150e8
	ctx.lr = 0x82413658;
	sub_824150E8(ctx, base);
	// 82413658: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241365C: 40820024  bne 0x82413680
	if !ctx.cr[0].eq {
	pc = 0x82413680; continue 'dispatch;
	}
	// 82413660: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82413664: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82413668: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8241366C: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82413670: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413674: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82413678: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8241367C: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82413680: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82413684: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413688: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 8241368C: 419A0008  beq cr6, 0x82413694
	if ctx.cr[6].eq {
	pc = 0x82413694; continue 'dispatch;
	}
	// 82413690: 48000000  b 0x82413690
	pc = 0x82413690; continue 'dispatch;
	// 82413694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82413698: 4BFFFA91  bl 0x82413128
	ctx.lr = 0x8241369C;
	sub_82413128(ctx, base);
	// 8241369C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824136A0: 907F0090  stw r3, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 824136A4: 40820008  bne 0x824136ac
	if !ctx.cr[0].eq {
	pc = 0x824136AC; continue 'dispatch;
	}
	// 824136A8: 48000000  b 0x824136a8
	pc = 0x824136A8; continue 'dispatch;
	// 824136AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824136B0: 480019A9  bl 0x82415058
	ctx.lr = 0x824136B4;
	sub_82415058(ctx, base);
	// 824136B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824136B8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824136BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824136C0: 419A0024  beq cr6, 0x824136e4
	if ctx.cr[6].eq {
	pc = 0x824136E4; continue 'dispatch;
	}
	// 824136C4: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 824136C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824136CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824136D0: 480022C9  bl 0x82415998
	ctx.lr = 0x824136D4;
	sub_82415998(ctx, base);
	// 824136D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824136D8: 4BFFFA51  bl 0x82413128
	ctx.lr = 0x824136DC;
	sub_82413128(ctx, base);
	// 824136DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824136E0: 4182FFDC  beq 0x824136bc
	if ctx.cr[0].eq {
	pc = 0x824136BC; continue 'dispatch;
	}
	// 824136E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824136E8: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 824136EC: 409A0008  bne cr6, 0x824136f4
	if !ctx.cr[6].eq {
	pc = 0x824136F4; continue 'dispatch;
	}
	// 824136F0: 48000000  b 0x824136f0
	pc = 0x824136F0; continue 'dispatch;
	// 824136F4: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 824136F8: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824136FC: 40990008  ble cr6, 0x82413704
	if !ctx.cr[6].gt {
	pc = 0x82413704; continue 'dispatch;
	}
	// 82413700: 48000000  b 0x82413700
	pc = 0x82413700; continue 'dispatch;
	// 82413704: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241370C: 4800228D  bl 0x82415998
	ctx.lr = 0x82413710;
	sub_82415998(ctx, base);
	// 82413710: 83BF0094  lwz r29, 0x94(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82413714: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413718: 93BF00A0  stw r29, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[29].u32 ) };
	// 8241371C: 418200FC  beq 0x82413818
	if ctx.cr[0].eq {
	pc = 0x82413818; continue 'dispatch;
	}
	// 82413720: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82413724: 57BC1838  slwi r28, r29, 3
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82413728: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8241372C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413730: 40990008  ble cr6, 0x82413738
	if !ctx.cr[6].gt {
	pc = 0x82413738; continue 'dispatch;
	}
	// 82413734: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82413738: 4BFFD089  bl 0x824107c0
	ctx.lr = 0x8241373C;
	sub_824107C0(ctx, base);
	// 8241373C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413740: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413744: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241374C: 4E800421  bctrl
	ctx.lr = 0x82413750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82413750: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413754: 4182002C  beq 0x82413780
	if ctx.cr[0].eq {
	pc = 0x82413780; continue 'dispatch;
	}
	// 82413758: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241375C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82413760: 41800018  blt 0x82413778
	if ctx.cr[0].lt {
	pc = 0x82413778; continue 'dispatch;
	}
	// 82413764: 934A0000  stw r26, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82413768: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241376C: 934A0004  stw r26, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82413770: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82413774: 4080FFF0  bge 0x82413764
	if !ctx.cr[0].lt {
	pc = 0x82413764; continue 'dispatch;
	}
	// 82413778: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241377C: 48000008  b 0x82413784
	pc = 0x82413784; continue 'dispatch;
	// 82413780: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82413784: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82413788: 48001991  bl 0x82415118
	ctx.lr = 0x8241378C;
	sub_82415118(ctx, base);
	// 8241378C: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82413790: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82413794: 93790000  stw r27, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82413798: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8241379C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824137A0: 2B0B8002  cmplwi cr6, r11, 0x8002
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32770 as u32, &mut ctx.xer);
	// 824137A4: 409A001C  bne cr6, 0x824137c0
	if !ctx.cr[6].eq {
	pc = 0x824137C0; continue 'dispatch;
	}
	// 824137A8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824137AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824137B0: 7C9D5A14  add r4, r29, r11
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 824137B4: 480018AD  bl 0x82415060
	ctx.lr = 0x824137B8;
	sub_82415060(ctx, base);
	// 824137B8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824137BC: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824137C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824137C4: 80BE0084  lwz r5, 0x84(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 824137C8: 809E0080  lwz r4, 0x80(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 824137CC: 4800191D  bl 0x824150e8
	ctx.lr = 0x824137D0;
	sub_824150E8(ctx, base);
	// 824137D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824137D4: 40820028  bne 0x824137fc
	if !ctx.cr[0].eq {
	pc = 0x824137FC; continue 'dispatch;
	}
	// 824137D8: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 824137DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824137E0: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824137E4: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 824137E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824137EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824137F0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824137F4: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824137F8: 4BFFFFA0  b 0x82413798
	pc = 0x82413798; continue 'dispatch;
	// 824137FC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82413800: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413804: 419A0008  beq cr6, 0x8241380c
	if ctx.cr[6].eq {
	pc = 0x8241380C; continue 'dispatch;
	}
	// 82413808: 48000000  b 0x82413808
	pc = 0x82413808; continue 'dispatch;
	// 8241380C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82413810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82413814: 48002185  bl 0x82415998
	ctx.lr = 0x82413818;
	sub_82415998(ctx, base);
	// 82413818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241381C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82413820: 481218DC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413828 size=64
    let mut pc: u32 = 0x82413828;
    'dispatch: loop {
        match pc {
            0x82413828 => {
    //   block [0x82413828..0x82413868)
	// 82413828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82413834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241383C: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 82413840: 480018D9  bl 0x82415118
	ctx.lr = 0x82413844;
	sub_82415118(ctx, base);
	// 82413844: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82413848: 4BFFCF91  bl 0x824107d8
	ctx.lr = 0x8241384C;
	sub_824107D8(ctx, base);
	// 8241384C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82413850: 480020F9  bl 0x82415948
	ctx.lr = 0x82413854;
	sub_82415948(ctx, base);
	// 82413854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82413858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241385C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82413860: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82413864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413868 size=92
    let mut pc: u32 = 0x82413868;
    'dispatch: loop {
        match pc {
            0x82413868 => {
    //   block [0x82413868..0x824138C4)
	// 82413868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82413874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82413878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241387C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82413884: 48002825  bl 0x824160a8
	ctx.lr = 0x82413888;
	sub_824160A8(ctx, base);
	// 82413888: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241388C: 4182001C  beq 0x824138a8
	if ctx.cr[0].eq {
	pc = 0x824138A8; continue 'dispatch;
	}
	// 82413890: 4BFFCF31  bl 0x824107c0
	ctx.lr = 0x82413894;
	sub_824107C0(ctx, base);
	// 82413894: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413898: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241389C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824138A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824138A4: 4E800421  bctrl
	ctx.lr = 0x824138A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824138A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824138AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824138B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824138B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824138B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824138BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824138C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824138C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824138C8 size=16
    let mut pc: u32 = 0x824138C8;
    'dispatch: loop {
        match pc {
            0x824138C8 => {
    //   block [0x824138C8..0x824138D8)
	// 824138C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824138CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824138D0: 419A0008  beq cr6, 0x824138d8
	if ctx.cr[6].eq {
		sub_824138D8(ctx, base);
		return;
	}
	// 824138D4: 48000000  b 0x824138d4
	pc = 0x824138D4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824138D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824138D8 size=12
    let mut pc: u32 = 0x824138D8;
    'dispatch: loop {
        match pc {
            0x824138D8 => {
    //   block [0x824138D8..0x824138E4)
	// 824138D8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824138DC: 409A0008  bne cr6, 0x824138e4
	if !ctx.cr[6].eq {
		sub_824138E4(ctx, base);
		return;
	}
	// 824138E0: 48000000  b 0x824138e0
	pc = 0x824138E0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824138E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824138E4 size=16
    let mut pc: u32 = 0x824138E4;
    'dispatch: loop {
        match pc {
            0x824138E4 => {
    //   block [0x824138E4..0x824138F4)
	// 824138E4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 824138E8: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 824138EC: 90C30010  stw r6, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 824138F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824138F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824138F8 size=28
    let mut pc: u32 = 0x824138F8;
    'dispatch: loop {
        match pc {
            0x824138F8 => {
    //   block [0x824138F8..0x82413914)
	// 824138F8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824138FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413900: 419A0014  beq cr6, 0x82413914
	if ctx.cr[6].eq {
		sub_82413914(ctx, base);
		return;
	}
	// 82413904: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82413908: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241390C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413910: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413914 size=8
    let mut pc: u32 = 0x82413914;
    'dispatch: loop {
        match pc {
            0x82413914 => {
    //   block [0x82413914..0x8241391C)
	// 82413914: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413920 size=8
    let mut pc: u32 = 0x82413920;
    'dispatch: loop {
        match pc {
            0x82413920 => {
    //   block [0x82413920..0x82413928)
	// 82413920: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413928 size=28
    let mut pc: u32 = 0x82413928;
    'dispatch: loop {
        match pc {
            0x82413928 => {
    //   block [0x82413928..0x82413944)
	// 82413928: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241392C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82413930: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413934: 4182000C  beq 0x82413940
	if ctx.cr[0].eq {
	pc = 0x82413940; continue 'dispatch;
	}
	// 82413938: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241393C: 41980008  blt cr6, 0x82413944
	if ctx.cr[6].lt {
		sub_82413944(ctx, base);
		return;
	}
	// 82413940: 48000000  b 0x82413940
	pc = 0x82413940; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413944 size=20
    let mut pc: u32 = 0x82413944;
    'dispatch: loop {
        match pc {
            0x82413944 => {
    //   block [0x82413944..0x82413958)
	// 82413944: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413948: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241394C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82413950: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413954: 480027E4  b 0x82416138
	sub_82416138(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413958 size=92
    let mut pc: u32 = 0x82413958;
    'dispatch: loop {
        match pc {
            0x82413958 => {
    //   block [0x82413958..0x824139B4)
	// 82413958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241395C: 48121761  bl 0x825350bc
	ctx.lr = 0x82413960;
	sub_82535080(ctx, base);
	// 82413960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413968: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8241396C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82413970: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413974: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413978: 4182000C  beq 0x82413984
	if ctx.cr[0].eq {
	pc = 0x82413984; continue 'dispatch;
	}
	// 8241397C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82413980: 41980008  blt cr6, 0x82413988
	if ctx.cr[6].lt {
	pc = 0x82413988; continue 'dispatch;
	}
	// 82413984: 48000000  b 0x82413984
	pc = 0x82413984; continue 'dispatch;
	// 82413988: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8241398C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413990: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82413994: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413998: 480020E1  bl 0x82415a78
	ctx.lr = 0x8241399C;
	sub_82415A78(ctx, base);
	// 8241399C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824139A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824139A4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824139A8: 480020E9  bl 0x82415a90
	ctx.lr = 0x824139AC;
	sub_82415A90(ctx, base);
	// 824139AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824139B0: 4812175C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824139B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824139B8 size=36
    let mut pc: u32 = 0x824139B8;
    'dispatch: loop {
        match pc {
            0x824139B8 => {
    //   block [0x824139B8..0x824139DC)
	// 824139B8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824139BC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 824139C0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824139C4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824139C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824139CC: 4182000C  beq 0x824139d8
	if ctx.cr[0].eq {
	pc = 0x824139D8; continue 'dispatch;
	}
	// 824139D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824139D4: 41980008  blt cr6, 0x824139dc
	if ctx.cr[6].lt {
		sub_824139DC(ctx, base);
		return;
	}
	// 824139D8: 48000000  b 0x824139d8
	pc = 0x824139D8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824139DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824139DC size=20
    let mut pc: u32 = 0x824139DC;
    'dispatch: loop {
        match pc {
            0x824139DC => {
    //   block [0x824139DC..0x824139F0)
	// 824139DC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824139E0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824139E4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 824139E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824139EC: 4800211C  b 0x82415b08
	sub_82415B08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824139F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824139F0 size=408
    let mut pc: u32 = 0x824139F0;
    'dispatch: loop {
        match pc {
            0x824139F0 => {
    //   block [0x824139F0..0x82413B88)
	// 824139F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824139F4: 481216C9  bl 0x825350bc
	ctx.lr = 0x824139F8;
	sub_82535080(ctx, base);
	// 824139F8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 824139FC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82413A00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413A08: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82413A0C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82413A10: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413A14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82413A18: 419A0118  beq cr6, 0x82413b30
	if ctx.cr[6].eq {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A1C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82413A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413A24: 419A010C  beq cr6, 0x82413b30
	if ctx.cr[6].eq {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A28: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82413A2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413A30: 419A0100  beq cr6, 0x82413b30
	if ctx.cr[6].eq {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A34: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82413A38: FC00FE5E  fctidz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82413A3C: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82413A40: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82413A44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82413A48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82413A4C: 419800E4  blt cr6, 0x82413b30
	if ctx.cr[6].lt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A50: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413A54: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413A58: 419800D8  blt cr6, 0x82413b30
	if ctx.cr[6].lt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413A5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82413A60: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82413A64: 40810120  ble 0x82413b84
	if !ctx.cr[0].gt {
	pc = 0x82413B84; continue 'dispatch;
	}
	// 82413A68: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82413A6C: 409900D4  ble cr6, 0x82413b40
	if !ctx.cr[6].gt {
	pc = 0x82413B40; continue 'dispatch;
	}
	// 82413A70: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82413A74: 409A0110  bne cr6, 0x82413b84
	if !ctx.cr[6].eq {
	pc = 0x82413B84; continue 'dispatch;
	}
	// 82413A78: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413A7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413A84: 40990054  ble cr6, 0x82413ad8
	if !ctx.cr[6].gt {
	pc = 0x82413AD8; continue 'dispatch;
	}
	// 82413A88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82413A8C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413A90: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413A94: 48000A9D  bl 0x82414530
	ctx.lr = 0x82413A98;
	sub_82414530(ctx, base);
	// 82413A98: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413A9C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413AA0: 419A001C  beq cr6, 0x82413abc
	if ctx.cr[6].eq {
	pc = 0x82413ABC; continue 'dispatch;
	}
	// 82413AA4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413AA8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413AAC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413AB0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413AB4: 4198FFD8  blt cr6, 0x82413a8c
	if ctx.cr[6].lt {
	pc = 0x82413A8C; continue 'dispatch;
	}
	// 82413AB8: 48000020  b 0x82413ad8
	pc = 0x82413AD8; continue 'dispatch;
	// 82413ABC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413AC0: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82413AC4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413AC8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82413ACC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413AD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413AD4: 48002765  bl 0x82416238
	ctx.lr = 0x82413AD8;
	sub_82416238(ctx, base);
	// 82413AD8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413ADC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413AE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413AE4: 4099004C  ble cr6, 0x82413b30
	if !ctx.cr[6].gt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413AE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82413AEC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413AF0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413AF4: 48000A3D  bl 0x82414530
	ctx.lr = 0x82413AF8;
	sub_82414530(ctx, base);
	// 82413AF8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413AFC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413B00: 419A001C  beq cr6, 0x82413b1c
	if ctx.cr[6].eq {
	pc = 0x82413B1C; continue 'dispatch;
	}
	// 82413B04: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413B08: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82413B0C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413B10: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413B14: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413B18: 48002721  bl 0x82416238
	ctx.lr = 0x82413B1C;
	sub_82416238(ctx, base);
	// 82413B1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413B20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413B24: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413B28: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413B2C: 4198FFC0  blt cr6, 0x82413aec
	if ctx.cr[6].lt {
	pc = 0x82413AEC; continue 'dispatch;
	}
	// 82413B30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82413B34: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82413B38: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82413B3C: 481215D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82413B40: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413B44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413B4C: 4099FFE4  ble cr6, 0x82413b30
	if !ctx.cr[6].gt {
	pc = 0x82413B30; continue 'dispatch;
	}
	// 82413B50: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82413B54: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413B58: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82413B5C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413B60: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82413B64: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413B68: 480026D1  bl 0x82416238
	ctx.lr = 0x82413B6C;
	sub_82416238(ctx, base);
	// 82413B6C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413B70: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413B74: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413B78: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413B7C: 4198FFD8  blt cr6, 0x82413b54
	if ctx.cr[6].lt {
	pc = 0x82413B54; continue 'dispatch;
	}
	// 82413B80: 4BFFFFB0  b 0x82413b30
	pc = 0x82413B30; continue 'dispatch;
	// 82413B84: 48000000  b 0x82413b84
	pc = 0x82413B84; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413B88 size=152
    let mut pc: u32 = 0x82413B88;
    'dispatch: loop {
        match pc {
            0x82413B88 => {
    //   block [0x82413B88..0x82413C20)
	// 82413B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413B8C: 4812152D  bl 0x825350b8
	ctx.lr = 0x82413B90;
	sub_82535080(ctx, base);
	// 82413B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413B94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413B98: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82413B9C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82413BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413BA4: 419A0038  beq cr6, 0x82413bdc
	if ctx.cr[6].eq {
	pc = 0x82413BDC; continue 'dispatch;
	}
	// 82413BA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413BAC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82413BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413BB4: 40990028  ble cr6, 0x82413bdc
	if !ctx.cr[6].gt {
	pc = 0x82413BDC; continue 'dispatch;
	}
	// 82413BB8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82413BBC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413BC0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413BC4: 4800213D  bl 0x82415d00
	ctx.lr = 0x82413BC8;
	sub_82415D00(ctx, base);
	// 82413BC8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413BCC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413BD0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413BD4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413BD8: 4198FFE4  blt cr6, 0x82413bbc
	if ctx.cr[6].lt {
	pc = 0x82413BBC; continue 'dispatch;
	}
	// 82413BDC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82413BE0: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82413BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413BE8: 419A0020  beq cr6, 0x82413c08
	if ctx.cr[6].eq {
	pc = 0x82413C08; continue 'dispatch;
	}
	// 82413BEC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413BF0: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82413BF4: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413BF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413BFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82413C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82413C04: 4E800421  bctrl
	ctx.lr = 0x82413C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82413C08: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82413C0C: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82413C10: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82413C14: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82413C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413C1C: 481214EC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413C20 size=120
    let mut pc: u32 = 0x82413C20;
    'dispatch: loop {
        match pc {
            0x82413C20 => {
    //   block [0x82413C20..0x82413C98)
	// 82413C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413C24: 48121495  bl 0x825350b8
	ctx.lr = 0x82413C28;
	sub_82535080(ctx, base);
	// 82413C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413C30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82413C34: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82413C38: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413C3C: 41980010  blt cr6, 0x82413c4c
	if ctx.cr[6].lt {
	pc = 0x82413C4C; continue 'dispatch;
	}
	// 82413C40: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413C44: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82413C48: 4098000C  bge cr6, 0x82413c54
	if !ctx.cr[6].lt {
	pc = 0x82413C54; continue 'dispatch;
	}
	// 82413C4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413C50: 48000040  b 0x82413c90
	pc = 0x82413C90; continue 'dispatch;
	// 82413C54: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413C58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413C5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413C60: 4099002C  ble cr6, 0x82413c8c
	if !ctx.cr[6].gt {
	pc = 0x82413C8C; continue 'dispatch;
	}
	// 82413C64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82413C68: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413C6C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413C70: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413C74: 48002115  bl 0x82415d88
	ctx.lr = 0x82413C78;
	sub_82415D88(ctx, base);
	// 82413C78: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413C7C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413C80: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413C84: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413C88: 4198FFE0  blt cr6, 0x82413c68
	if ctx.cr[6].lt {
	pc = 0x82413C68; continue 'dispatch;
	}
	// 82413C8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82413C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413C94: 48121474  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82413C98 size=116
    let mut pc: u32 = 0x82413C98;
    'dispatch: loop {
        match pc {
            0x82413C98 => {
    //   block [0x82413C98..0x82413D0C)
	// 82413C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82413CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413CA4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413CA8: 3924FFFF  addi r9, r4, -1
	ctx.r[9].s64 = ctx.r[4].s64 + -1;
	// 82413CAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413CB0: 4182000C  beq 0x82413cbc
	if ctx.cr[0].eq {
	pc = 0x82413CBC; continue 'dispatch;
	}
	// 82413CB4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413CB8: 4198000C  blt cr6, 0x82413cc4
	if ctx.cr[6].lt {
	pc = 0x82413CC4; continue 'dispatch;
	}
	// 82413CBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413CC0: 4800003C  b 0x82413cfc
	pc = 0x82413CFC; continue 'dispatch;
	// 82413CC4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82413CC8: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82413CCC: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82413CD0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82413CD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82413CD8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82413CDC: 4198FFE0  blt cr6, 0x82413cbc
	if ctx.cr[6].lt {
	pc = 0x82413CBC; continue 'dispatch;
	}
	// 82413CE0: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413CE4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413CE8: 4198FFD4  blt cr6, 0x82413cbc
	if ctx.cr[6].lt {
	pc = 0x82413CBC; continue 'dispatch;
	}
	// 82413CEC: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413CF0: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82413CF4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82413CF8: 48002669  bl 0x82416360
	ctx.lr = 0x82413CFC;
	sub_82416360(ctx, base);
	// 82413CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82413D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82413D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82413D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413D10 size=28
    let mut pc: u32 = 0x82413D10;
    'dispatch: loop {
        match pc {
            0x82413D10 => {
    //   block [0x82413D10..0x82413D2C)
	// 82413D10: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413D14: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82413D18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413D1C: 4182000C  beq 0x82413d28
	if ctx.cr[0].eq {
	pc = 0x82413D28; continue 'dispatch;
	}
	// 82413D20: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413D24: 41980008  blt cr6, 0x82413d2c
	if ctx.cr[6].lt {
		sub_82413D2C(ctx, base);
		return;
	}
	// 82413D28: 48000000  b 0x82413d28
	pc = 0x82413D28; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413D2C size=16
    let mut pc: u32 = 0x82413D2C;
    'dispatch: loop {
        match pc {
            0x82413D2C => {
    //   block [0x82413D2C..0x82413D3C)
	// 82413D2C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413D30: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82413D34: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413D38: 48001DB0  b 0x82415ae8
	sub_82415AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413D40 size=28
    let mut pc: u32 = 0x82413D40;
    'dispatch: loop {
        match pc {
            0x82413D40 => {
    //   block [0x82413D40..0x82413D5C)
	// 82413D40: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413D44: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82413D48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413D4C: 4182000C  beq 0x82413d58
	if ctx.cr[0].eq {
	pc = 0x82413D58; continue 'dispatch;
	}
	// 82413D50: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413D54: 41980008  blt cr6, 0x82413d5c
	if ctx.cr[6].lt {
		sub_82413D5C(ctx, base);
		return;
	}
	// 82413D58: 48000000  b 0x82413d58
	pc = 0x82413D58; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82413D5C size=16
    let mut pc: u32 = 0x82413D5C;
    'dispatch: loop {
        match pc {
            0x82413D5C => {
    //   block [0x82413D5C..0x82413D6C)
	// 82413D5C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413D60: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82413D64: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413D68: 48001D90  b 0x82415af8
	sub_82415AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413D70 size=112
    let mut pc: u32 = 0x82413D70;
    'dispatch: loop {
        match pc {
            0x82413D70 => {
    //   block [0x82413D70..0x82413DE0)
	// 82413D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413D74: 48121341  bl 0x825350b4
	ctx.lr = 0x82413D78;
	sub_82535080(ctx, base);
	// 82413D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413D80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82413D84: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82413D88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413D8C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413D94: 40990034  ble cr6, 0x82413dc8
	if !ctx.cr[6].gt {
	pc = 0x82413DC8; continue 'dispatch;
	}
	// 82413D98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82413D9C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413DA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413DA4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413DA8: 48002551  bl 0x824162f8
	ctx.lr = 0x82413DAC;
	sub_824162F8(ctx, base);
	// 82413DAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413DB0: 40820024  bne 0x82413dd4
	if !ctx.cr[0].eq {
	pc = 0x82413DD4; continue 'dispatch;
	}
	// 82413DB4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413DB8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413DBC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413DC0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413DC4: 4198FFD8  blt cr6, 0x82413d9c
	if ctx.cr[6].lt {
	pc = 0x82413D9C; continue 'dispatch;
	}
	// 82413DC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413DD0: 48121334  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82413DD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82413DD8: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82413DDC: 4BFFFFF0  b 0x82413dcc
	pc = 0x82413DCC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413DE0 size=112
    let mut pc: u32 = 0x82413DE0;
    'dispatch: loop {
        match pc {
            0x82413DE0 => {
    //   block [0x82413DE0..0x82413E50)
	// 82413DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413DE4: 481212D1  bl 0x825350b4
	ctx.lr = 0x82413DE8;
	sub_82535080(ctx, base);
	// 82413DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413DF0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82413DF4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82413DF8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413DFC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413E04: 40990034  ble cr6, 0x82413e38
	if !ctx.cr[6].gt {
	pc = 0x82413E38; continue 'dispatch;
	}
	// 82413E08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82413E0C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413E10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82413E14: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82413E18: 48002521  bl 0x82416338
	ctx.lr = 0x82413E1C;
	sub_82416338(ctx, base);
	// 82413E1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82413E20: 40820024  bne 0x82413e44
	if !ctx.cr[0].eq {
	pc = 0x82413E44; continue 'dispatch;
	}
	// 82413E24: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413E28: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413E2C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413E30: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413E34: 4198FFD8  blt cr6, 0x82413e0c
	if ctx.cr[6].lt {
	pc = 0x82413E0C; continue 'dispatch;
	}
	// 82413E38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82413E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82413E40: 481212C4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82413E44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82413E48: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82413E4C: 4BFFFFF0  b 0x82413e3c
	pc = 0x82413E3C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413E50 size=124
    let mut pc: u32 = 0x82413E50;
    'dispatch: loop {
        match pc {
            0x82413E50 => {
    //   block [0x82413E50..0x82413ECC)
	// 82413E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413E54: 48121269  bl 0x825350bc
	ctx.lr = 0x82413E58;
	sub_82535080(ctx, base);
	// 82413E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413E60: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413E64: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413E68: 48001C41  bl 0x82415aa8
	ctx.lr = 0x82413E6C;
	sub_82415AA8(ctx, base);
	// 82413E6C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413E70: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413E74: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82413E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413E7C: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82413E80: 40990044  ble cr6, 0x82413ec4
	if !ctx.cr[6].gt {
	pc = 0x82413EC4; continue 'dispatch;
	}
	// 82413E84: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82413E88: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413E8C: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413E90: 48001C39  bl 0x82415ac8
	ctx.lr = 0x82413E94;
	sub_82415AC8(ctx, base);
	// 82413E94: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413E98: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82413E9C: 41990014  bgt cr6, 0x82413eb0
	if ctx.cr[6].gt {
	pc = 0x82413EB0; continue 'dispatch;
	}
	// 82413EA0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413EA4: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413EA8: 48001C21  bl 0x82415ac8
	ctx.lr = 0x82413EAC;
	sub_82415AC8(ctx, base);
	// 82413EAC: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82413EB0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413EB4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413EB8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413EBC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413EC0: 4198FFC8  blt cr6, 0x82413e88
	if ctx.cr[6].lt {
	pc = 0x82413E88; continue 'dispatch;
	}
	// 82413EC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82413EC8: 48121244  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413ED0 size=136
    let mut pc: u32 = 0x82413ED0;
    'dispatch: loop {
        match pc {
            0x82413ED0 => {
    //   block [0x82413ED0..0x82413F58)
	// 82413ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413ED4: 481211E9  bl 0x825350bc
	ctx.lr = 0x82413ED8;
	sub_82535080(ctx, base);
	// 82413ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413EDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413EE0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413EE4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413EE8: 48001BC1  bl 0x82415aa8
	ctx.lr = 0x82413EEC;
	sub_82415AA8(ctx, base);
	// 82413EEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413EF0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413EF4: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82413EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413EFC: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82413F00: 40990050  ble cr6, 0x82413f50
	if !ctx.cr[6].gt {
	pc = 0x82413F50; continue 'dispatch;
	}
	// 82413F04: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82413F08: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413F0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413F10: 419A001C  beq cr6, 0x82413f2c
	if ctx.cr[6].eq {
	pc = 0x82413F2C; continue 'dispatch;
	}
	// 82413F14: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F18: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413F1C: 48001BAD  bl 0x82415ac8
	ctx.lr = 0x82413F20;
	sub_82415AC8(ctx, base);
	// 82413F20: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82413F24: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413F28: 40980014  bge cr6, 0x82413f3c
	if !ctx.cr[6].lt {
	pc = 0x82413F3C; continue 'dispatch;
	}
	// 82413F2C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F30: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413F34: 48001B95  bl 0x82415ac8
	ctx.lr = 0x82413F38;
	sub_82415AC8(ctx, base);
	// 82413F38: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82413F3C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413F40: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82413F44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82413F48: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413F4C: 4198FFBC  blt cr6, 0x82413f08
	if ctx.cr[6].lt {
	pc = 0x82413F08; continue 'dispatch;
	}
	// 82413F50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82413F54: 481211B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82413F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82413F58 size=188
    let mut pc: u32 = 0x82413F58;
    'dispatch: loop {
        match pc {
            0x82413F58 => {
    //   block [0x82413F58..0x82414014)
	// 82413F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82413F5C: 4812115D  bl 0x825350b8
	ctx.lr = 0x82413F60;
	sub_82535080(ctx, base);
	// 82413F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82413F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82413F68: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82413F6C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413F70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82413F74: 40990070  ble cr6, 0x82413fe4
	if !ctx.cr[6].gt {
	pc = 0x82413FE4; continue 'dispatch;
	}
	// 82413F78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82413F7C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F80: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413F84: 480005AD  bl 0x82414530
	ctx.lr = 0x82413F88;
	sub_82414530(ctx, base);
	// 82413F88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82413F8C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413F90: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413F94: 419A0058  beq cr6, 0x82413fec
	if ctx.cr[6].eq {
	pc = 0x82413FEC; continue 'dispatch;
	}
	// 82413F98: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82413F9C: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413FA0: 48054B21  bl 0x82468ac0
	ctx.lr = 0x82413FA4;
	sub_82468AC0(ctx, base);
	// 82413FA4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413FA8: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82413FAC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82413FB0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82413FB4: 41820014  beq 0x82413fc8
	if ctx.cr[0].eq {
	pc = 0x82413FC8; continue 'dispatch;
	}
	// 82413FB8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82413FBC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82413FC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82413FC4: 419AFFE0  beq cr6, 0x82413fa4
	if ctx.cr[6].eq {
	pc = 0x82413FA4; continue 'dispatch;
	}
	// 82413FC8: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82413FCC: 4182001C  beq 0x82413fe8
	if ctx.cr[0].eq {
	pc = 0x82413FE8; continue 'dispatch;
	}
	// 82413FD0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82413FD4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82413FD8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82413FDC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82413FE0: 4198FF9C  blt cr6, 0x82413f7c
	if ctx.cr[6].lt {
	pc = 0x82413F7C; continue 'dispatch;
	}
	// 82413FE4: 48000000  b 0x82413fe4
	pc = 0x82413FE4; continue 'dispatch;
	// 82413FE8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413FEC: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82413FF0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82413FF4: 48001AB5  bl 0x82415aa8
	ctx.lr = 0x82413FF8;
	sub_82415AA8(ctx, base);
	// 82413FF8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82413FFC: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82414000: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82414004: 48001AC5  bl 0x82415ac8
	ctx.lr = 0x82414008;
	sub_82415AC8(ctx, base);
	// 82414008: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8241400C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414010: 481210F8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414018 size=80
    let mut pc: u32 = 0x82414018;
    'dispatch: loop {
        match pc {
            0x82414018 => {
    //   block [0x82414018..0x82414068)
	// 82414018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241401C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82414024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241402C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414034: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414038: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241403C: 41820010  beq 0x8241404c
	if ctx.cr[0].eq {
	pc = 0x8241404C; continue 'dispatch;
	}
	// 82414040: 4811EB79  bl 0x82532bb8
	ctx.lr = 0x82414044;
	sub_82532BB8(ctx, base);
	// 82414044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414048: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241404C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414050: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241405C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414068 size=76
    let mut pc: u32 = 0x82414068;
    'dispatch: loop {
        match pc {
            0x82414068 => {
    //   block [0x82414068..0x824140B4)
	// 82414068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241406C: 48121051  bl 0x825350bc
	ctx.lr = 0x82414070;
	sub_82535080(ctx, base);
	// 82414070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414074: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82414078: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241407C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414084: 4BFFFCED  bl 0x82413d70
	ctx.lr = 0x82414088;
	sub_82413D70(ctx, base);
	// 82414088: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241408C: 41820020  beq 0x824140ac
	if ctx.cr[0].eq {
	pc = 0x824140AC; continue 'dispatch;
	}
	// 82414090: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82414094: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82414098: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241409C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824140A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824140A4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824140A8: 48001E51  bl 0x82415ef8
	ctx.lr = 0x824140AC;
	sub_82415EF8(ctx, base);
	// 824140AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824140B0: 4812105C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824140B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824140B8 size=76
    let mut pc: u32 = 0x824140B8;
    'dispatch: loop {
        match pc {
            0x824140B8 => {
    //   block [0x824140B8..0x82414104)
	// 824140B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824140BC: 48121001  bl 0x825350bc
	ctx.lr = 0x824140C0;
	sub_82535080(ctx, base);
	// 824140C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824140C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824140C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824140CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824140D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824140D4: 4BFFFC9D  bl 0x82413d70
	ctx.lr = 0x824140D8;
	sub_82413D70(ctx, base);
	// 824140D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824140DC: 41820020  beq 0x824140fc
	if ctx.cr[0].eq {
	pc = 0x824140FC; continue 'dispatch;
	}
	// 824140E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824140E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824140E8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824140EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824140F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824140F4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824140F8: 48001E61  bl 0x82415f58
	ctx.lr = 0x824140FC;
	sub_82415F58(ctx, base);
	// 824140FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414100: 4812100C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414108 size=76
    let mut pc: u32 = 0x82414108;
    'dispatch: loop {
        match pc {
            0x82414108 => {
    //   block [0x82414108..0x82414154)
	// 82414108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241410C: 48120FB1  bl 0x825350bc
	ctx.lr = 0x82414110;
	sub_82535080(ctx, base);
	// 82414110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414114: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82414118: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241411C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414120: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414124: 4BFFFC4D  bl 0x82413d70
	ctx.lr = 0x82414128;
	sub_82413D70(ctx, base);
	// 82414128: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241412C: 41820020  beq 0x8241414c
	if ctx.cr[0].eq {
	pc = 0x8241414C; continue 'dispatch;
	}
	// 82414130: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82414134: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82414138: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241413C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414140: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82414144: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414148: 48001E71  bl 0x82415fb8
	ctx.lr = 0x8241414C;
	sub_82415FB8(ctx, base);
	// 8241414C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414150: 48120FBC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414158 size=76
    let mut pc: u32 = 0x82414158;
    'dispatch: loop {
        match pc {
            0x82414158 => {
    //   block [0x82414158..0x824141A4)
	// 82414158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241415C: 48120F61  bl 0x825350bc
	ctx.lr = 0x82414160;
	sub_82535080(ctx, base);
	// 82414160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414164: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82414168: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8241416C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414174: 4BFFFBFD  bl 0x82413d70
	ctx.lr = 0x82414178;
	sub_82413D70(ctx, base);
	// 82414178: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241417C: 41820020  beq 0x8241419c
	if ctx.cr[0].eq {
	pc = 0x8241419C; continue 'dispatch;
	}
	// 82414180: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82414184: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82414188: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8241418C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414190: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82414194: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414198: 48001E81  bl 0x82416018
	ctx.lr = 0x8241419C;
	sub_82416018(ctx, base);
	// 8241419C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824141A0: 48120F6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824141A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824141A8 size=56
    let mut pc: u32 = 0x824141A8;
    'dispatch: loop {
        match pc {
            0x824141A8 => {
    //   block [0x824141A8..0x824141E0)
	// 824141A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824141AC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824141B0: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824141B4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824141B8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824141BC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824141C0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824141C4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824141C8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824141CC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824141D0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824141D4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824141D8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824141DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824141E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824141E0 size=136
    let mut pc: u32 = 0x824141E0;
    'dispatch: loop {
        match pc {
            0x824141E0 => {
    //   block [0x824141E0..0x82414268)
	// 824141E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824141E4: 48120ED5  bl 0x825350b8
	ctx.lr = 0x824141E8;
	sub_82535080(ctx, base);
	// 824141E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824141EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824141F0: 4BFFF999  bl 0x82413b88
	ctx.lr = 0x824141F4;
	sub_82413B88(ctx, base);
	// 824141F4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824141F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824141FC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82414200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82414204: 40990048  ble cr6, 0x8241424c
	if !ctx.cr[6].gt {
	pc = 0x8241424C; continue 'dispatch;
	}
	// 82414208: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8241420C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414210: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82414214: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82414218: 419A0020  beq cr6, 0x82414238
	if ctx.cr[6].eq {
	pc = 0x82414238; continue 'dispatch;
	}
	// 8241421C: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82414220: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414224: 4182000C  beq 0x82414230
	if ctx.cr[0].eq {
	pc = 0x82414230; continue 'dispatch;
	}
	// 82414228: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241422C: 4BFFF63D  bl 0x82413868
	ctx.lr = 0x82414230;
	sub_82413868(ctx, base);
	// 82414230: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414234: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82414238: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241423C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82414240: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82414244: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414248: 4198FFC4  blt cr6, 0x8241420c
	if ctx.cr[6].lt {
	pc = 0x8241420C; continue 'dispatch;
	}
	// 8241424C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414250: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414254: 4182000C  beq 0x82414260
	if ctx.cr[0].eq {
	pc = 0x82414260; continue 'dispatch;
	}
	// 82414258: 4811E961  bl 0x82532bb8
	ctx.lr = 0x8241425C;
	sub_82532BB8(ctx, base);
	// 8241425C: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82414260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414264: 48120EA4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414268 size=184
    let mut pc: u32 = 0x82414268;
    'dispatch: loop {
        match pc {
            0x82414268 => {
    //   block [0x82414268..0x82414320)
	// 82414268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241426C: 48120E4D  bl 0x825350b8
	ctx.lr = 0x82414270;
	sub_82535080(ctx, base);
	// 82414270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414278: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8241427C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414284: 419A0008  beq cr6, 0x8241428c
	if ctx.cr[6].eq {
	pc = 0x8241428C; continue 'dispatch;
	}
	// 82414288: 48000000  b 0x82414288
	pc = 0x82414288; continue 'dispatch;
	// 8241428C: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82414290: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82414294: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82414298: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8241429C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824142A0: 40990008  ble cr6, 0x824142a8
	if !ctx.cr[6].gt {
	pc = 0x824142A8; continue 'dispatch;
	}
	// 824142A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824142A8: 4BFF0B21  bl 0x82404dc8
	ctx.lr = 0x824142AC;
	sub_82404DC8(ctx, base);
	// 824142AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824142B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824142B4: 4BFFFD65  bl 0x82414018
	ctx.lr = 0x824142B8;
	sub_82414018(ctx, base);
	// 824142B8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824142BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824142C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824142C4: 40990054  ble cr6, 0x82414318
	if !ctx.cr[6].gt {
	pc = 0x82414318; continue 'dispatch;
	}
	// 824142C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824142CC: 4BFFC4F5  bl 0x824107c0
	ctx.lr = 0x824142D0;
	sub_824107C0(ctx, base);
	// 824142D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824142D4: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 824142D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824142DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824142E0: 4E800421  bctrl
	ctx.lr = 0x824142E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824142E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824142E8: 41820010  beq 0x824142f8
	if ctx.cr[0].eq {
	pc = 0x824142F8; continue 'dispatch;
	}
	// 824142EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824142F0: 48001D89  bl 0x82416078
	ctx.lr = 0x824142F4;
	sub_82416078(ctx, base);
	// 824142F4: 48000008  b 0x824142fc
	pc = 0x824142FC; continue 'dispatch;
	// 824142F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824142FC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414300: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82414304: 7C7D592E  stwx r3, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82414308: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8241430C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82414310: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414314: 4198FFB8  blt cr6, 0x824142cc
	if ctx.cr[6].lt {
	pc = 0x824142CC; continue 'dispatch;
	}
	// 82414318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8241431C: 48120DEC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414320 size=380
    let mut pc: u32 = 0x82414320;
    'dispatch: loop {
        match pc {
            0x82414320 => {
    //   block [0x82414320..0x8241449C)
	// 82414320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414324: 48120D91  bl 0x825350b4
	ctx.lr = 0x82414328;
	sub_82535080(ctx, base);
	// 82414328: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241432C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414330: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82414334: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82414338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241433C: 409A0038  bne cr6, 0x82414374
	if !ctx.cr[6].eq {
	pc = 0x82414374; continue 'dispatch;
	}
	// 82414340: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414344: 937F001C  stw r27, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[27].u32 ) };
	// 82414348: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241434C: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414354: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241435C: 4E800421  bctrl
	ctx.lr = 0x82414360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414364: 4082000C  bne 0x82414370
	if !ctx.cr[0].eq {
	pc = 0x82414370; continue 'dispatch;
	}
	// 82414368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241436C: 48000128  b 0x82414494
	pc = 0x82414494; continue 'dispatch;
	// 82414370: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82414374: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82414378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241437C: 409A0114  bne cr6, 0x82414490
	if !ctx.cr[6].eq {
	pc = 0x82414490; continue 'dispatch;
	}
	// 82414380: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82414384: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82414388: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8241438C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82414390: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82414394: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414398: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8241439C: 40990008  ble cr6, 0x824143a4
	if !ctx.cr[6].gt {
	pc = 0x824143A4; continue 'dispatch;
	}
	// 824143A0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824143A4: 4BFF0A25  bl 0x82404dc8
	ctx.lr = 0x824143A8;
	sub_82404DC8(ctx, base);
	// 824143A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824143AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824143B0: 4BFFFC69  bl 0x82414018
	ctx.lr = 0x824143B4;
	sub_82414018(ctx, base);
	// 824143B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824143B8: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824143BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824143C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824143C4: 40990030  ble cr6, 0x824143f4
	if !ctx.cr[6].gt {
	pc = 0x824143F4; continue 'dispatch;
	}
	// 824143C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824143CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824143D0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824143D4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824143D8: 48001771  bl 0x82415b48
	ctx.lr = 0x824143DC;
	sub_82415B48(ctx, base);
	// 824143DC: 7C7EE12E  stwx r3, r30, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 824143E0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824143E4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824143E8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824143EC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824143F0: 4198FFDC  blt cr6, 0x824143cc
	if ctx.cr[6].lt {
	pc = 0x824143CC; continue 'dispatch;
	}
	// 824143F4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824143F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824143FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82414400: 40990028  ble cr6, 0x82414428
	if !ctx.cr[6].gt {
	pc = 0x82414428; continue 'dispatch;
	}
	// 82414404: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82414408: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241440C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82414410: 419A0038  beq cr6, 0x82414448
	if ctx.cr[6].eq {
	pc = 0x82414448; continue 'dispatch;
	}
	// 82414414: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82414418: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241441C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82414420: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82414424: 4198FFE4  blt cr6, 0x82414408
	if ctx.cr[6].lt {
	pc = 0x82414408; continue 'dispatch;
	}
	// 82414428: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241442C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82414430: 419A0044  beq cr6, 0x82414474
	if ctx.cr[6].eq {
	pc = 0x82414474; continue 'dispatch;
	}
	// 82414434: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82414438: 419A0030  beq cr6, 0x82414468
	if ctx.cr[6].eq {
	pc = 0x82414468; continue 'dispatch;
	}
	// 8241443C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82414440: 419A001C  beq cr6, 0x8241445c
	if ctx.cr[6].eq {
	pc = 0x8241445C; continue 'dispatch;
	}
	// 82414444: 48000000  b 0x82414444
	pc = 0x82414444; continue 'dispatch;
	// 82414448: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8241444C: 419AFF1C  beq cr6, 0x82414368
	if ctx.cr[6].eq {
	pc = 0x82414368; continue 'dispatch;
	}
	// 82414450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82414454: 4811E765  bl 0x82532bb8
	ctx.lr = 0x82414458;
	sub_82532BB8(ctx, base);
	// 82414458: 4BFFFF10  b 0x82414368
	pc = 0x82414368; continue 'dispatch;
	// 8241445C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414460: 4BFFFAF9  bl 0x82413f58
	ctx.lr = 0x82414464;
	sub_82413F58(ctx, base);
	// 82414464: 48000018  b 0x8241447c
	pc = 0x8241447C; continue 'dispatch;
	// 82414468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241446C: 4BFFFA65  bl 0x82413ed0
	ctx.lr = 0x82414470;
	sub_82413ED0(ctx, base);
	// 82414470: 4800000C  b 0x8241447c
	pc = 0x8241447C; continue 'dispatch;
	// 82414474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414478: 4BFFF9D9  bl 0x82413e50
	ctx.lr = 0x8241447C;
	sub_82413E50(ctx, base);
	// 8241447C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82414480: 937F0024  stw r27, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82414484: 419A000C  beq cr6, 0x82414490
	if ctx.cr[6].eq {
	pc = 0x82414490; continue 'dispatch;
	}
	// 82414488: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241448C: 4811E72D  bl 0x82532bb8
	ctx.lr = 0x82414490;
	sub_82532BB8(ctx, base);
	// 82414490: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82414494: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82414498: 48120C6C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824144A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824144A0 size=36
    let mut pc: u32 = 0x824144A0;
    'dispatch: loop {
        match pc {
            0x824144A0 => {
    //   block [0x824144A0..0x824144C4)
	// 824144A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824144A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824144A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824144AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824144B0: 4BFFF8C1  bl 0x82413d70
	ctx.lr = 0x824144B4;
	sub_82413D70(ctx, base);
	// 824144B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824144B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824144BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824144C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824144C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824144C8 size=36
    let mut pc: u32 = 0x824144C8;
    'dispatch: loop {
        match pc {
            0x824144C8 => {
    //   block [0x824144C8..0x824144EC)
	// 824144C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824144CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824144D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824144D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824144D8: 4BFFF909  bl 0x82413de0
	ctx.lr = 0x824144DC;
	sub_82413DE0(ctx, base);
	// 824144DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824144E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824144E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824144E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824144F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824144F0 size=40
    let mut pc: u32 = 0x824144F0;
    'dispatch: loop {
        match pc {
            0x824144F0 => {
    //   block [0x824144F0..0x82414518)
	// 824144F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824144F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824144F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824144FC: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82414500: 48001F21  bl 0x82416420
	ctx.lr = 0x82414504;
	sub_82416420(ctx, base);
	// 82414504: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82414508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241450C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414518 size=8
    let mut pc: u32 = 0x82414518;
    'dispatch: loop {
        match pc {
            0x82414518 => {
    //   block [0x82414518..0x82414520)
	// 82414518: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 8241451C: 48001F3C  b 0x82416458
	sub_82416458(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414520 size=8
    let mut pc: u32 = 0x82414520;
    'dispatch: loop {
        match pc {
            0x82414520 => {
    //   block [0x82414520..0x82414528)
	// 82414520: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82414524: 48001F54  b 0x82416478
	sub_82416478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414528 size=8
    let mut pc: u32 = 0x82414528;
    'dispatch: loop {
        match pc {
            0x82414528 => {
    //   block [0x82414528..0x82414530)
	// 82414528: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414530 size=8
    let mut pc: u32 = 0x82414530;
    'dispatch: loop {
        match pc {
            0x82414530 => {
    //   block [0x82414530..0x82414538)
	// 82414530: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414538 size=8
    let mut pc: u32 = 0x82414538;
    'dispatch: loop {
        match pc {
            0x82414538 => {
    //   block [0x82414538..0x82414540)
	// 82414538: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241453C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414540 size=8
    let mut pc: u32 = 0x82414540;
    'dispatch: loop {
        match pc {
            0x82414540 => {
    //   block [0x82414540..0x82414548)
	// 82414540: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82414544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414548 size=88
    let mut pc: u32 = 0x82414548;
    'dispatch: loop {
        match pc {
            0x82414548 => {
    //   block [0x82414548..0x824145A0)
	// 82414548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241454C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241455C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82414560: 38801001  li r4, 0x1001
	ctx.r[4].s64 = 4097;
	// 82414564: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82414568: 48001FA9  bl 0x82416510
	ctx.lr = 0x8241456C;
	sub_82416510(ctx, base);
	// 8241456C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414570: 4182001C  beq 0x8241458c
	if ctx.cr[0].eq {
	pc = 0x8241458C; continue 'dispatch;
	}
	// 82414574: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414578: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241457C: 40820008  bne 0x82414584
	if !ctx.cr[0].eq {
	pc = 0x82414584; continue 'dispatch;
	}
	// 82414580: 48000000  b 0x82414580
	pc = 0x82414580; continue 'dispatch;
	// 82414584: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82414588: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241458C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82414590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241459C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145A0 size=20
    let mut pc: u32 = 0x824145A0;
    'dispatch: loop {
        match pc {
            0x824145A0 => {
    //   block [0x824145A0..0x824145B4)
	// 824145A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824145A4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824145A8: 4199000C  bgt cr6, 0x824145b4
	if ctx.cr[6].gt {
		sub_824145B4(ctx, base);
		return;
	}
	// 824145AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824145B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145B4 size=16
    let mut pc: u32 = 0x824145B4;
    'dispatch: loop {
        match pc {
            0x824145B4 => {
    //   block [0x824145B4..0x824145C4)
	// 824145B4: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824145B8: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 824145BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824145C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145C8 size=20
    let mut pc: u32 = 0x824145C8;
    'dispatch: loop {
        match pc {
            0x824145C8 => {
    //   block [0x824145C8..0x824145DC)
	// 824145C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824145CC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824145D0: 4199000C  bgt cr6, 0x824145dc
	if ctx.cr[6].gt {
		sub_824145DC(ctx, base);
		return;
	}
	// 824145D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824145D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145DC size=20
    let mut pc: u32 = 0x824145DC;
    'dispatch: loop {
        match pc {
            0x824145DC => {
    //   block [0x824145DC..0x824145F0)
	// 824145DC: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824145E0: 1D64000C  mulli r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 * 12;
	// 824145E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824145E8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824145EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824145F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824145F0 size=20
    let mut pc: u32 = 0x824145F0;
    'dispatch: loop {
        match pc {
            0x824145F0 => {
    //   block [0x824145F0..0x82414604)
	// 824145F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824145F4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824145F8: 4199000C  bgt cr6, 0x82414604
	if ctx.cr[6].gt {
		sub_82414604(ctx, base);
		return;
	}
	// 824145FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82414600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414604(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414604 size=40
    let mut pc: u32 = 0x82414604;
    'dispatch: loop {
        match pc {
            0x82414604 => {
    //   block [0x82414604..0x8241462C)
	// 82414604: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414608: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8241460C: 4099FFF0  ble cr6, 0x824145fc
	if !ctx.cr[6].gt {
		sub_824145F0(ctx, base);
		return;
	}
	// 82414610: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414614: 1D64000C  mulli r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 * 12;
	// 82414618: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241461C: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82414620: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414624: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414630 size=20
    let mut pc: u32 = 0x82414630;
    'dispatch: loop {
        match pc {
            0x82414630 => {
    //   block [0x82414630..0x82414644)
	// 82414630: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414634: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82414638: 4199000C  bgt cr6, 0x82414644
	if ctx.cr[6].gt {
		sub_82414644(ctx, base);
		return;
	}
	// 8241463C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82414640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414644 size=64
    let mut pc: u32 = 0x82414644;
    'dispatch: loop {
        match pc {
            0x82414644 => {
    //   block [0x82414644..0x82414684)
	// 82414644: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414648: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8241464C: 4099FFF0  ble cr6, 0x8241463c
	if !ctx.cr[6].gt {
		sub_82414630(ctx, base);
		return;
	}
	// 82414650: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414654: 1D64000C  mulli r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 * 12;
	// 82414658: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241465C: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82414660: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414664: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82414668: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241466C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82414670: 4099FFCC  ble cr6, 0x8241463c
	if !ctx.cr[6].gt {
		sub_82414630(ctx, base);
		return;
	}
	// 82414674: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414678: 54CA1838  slwi r10, r6, 3
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241467C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82414680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82414688 size=20
    let mut pc: u32 = 0x82414688;
    'dispatch: loop {
        match pc {
            0x82414688 => {
    //   block [0x82414688..0x8241469C)
	// 82414688: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241468C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82414690: 4199000C  bgt cr6, 0x8241469c
	if ctx.cr[6].gt {
		sub_8241469C(ctx, base);
		return;
	}
	// 82414694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82414698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241469C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241469C size=68
    let mut pc: u32 = 0x8241469C;
    'dispatch: loop {
        match pc {
            0x8241469C => {
    //   block [0x8241469C..0x824146E0)
	// 8241469C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824146A0: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 824146A4: 4099FFF0  ble cr6, 0x82414694
	if !ctx.cr[6].gt {
		sub_82414688(ctx, base);
		return;
	}
	// 824146A8: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824146AC: 1D64000C  mulli r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 * 12;
	// 824146B0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824146B4: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824146B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824146BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824146C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824146C4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 824146C8: 4099FFCC  ble cr6, 0x82414694
	if !ctx.cr[6].gt {
		sub_82414688(ctx, base);
		return;
	}
	// 824146CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824146D0: 54CA1838  slwi r10, r6, 3
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824146D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824146D8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824146DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824146E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824146E0 size=112
    let mut pc: u32 = 0x824146E0;
    'dispatch: loop {
        match pc {
            0x824146E0 => {
    //   block [0x824146E0..0x82414750)
	// 824146E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824146E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824146E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824146EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824146F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824146F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824146F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824146FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82414700: 38802001  li r4, 0x2001
	ctx.r[4].s64 = 8193;
	// 82414704: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82414708: 48001E09  bl 0x82416510
	ctx.lr = 0x8241470C;
	sub_82416510(ctx, base);
	// 8241470C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414710: 41820028  beq 0x82414738
	if ctx.cr[0].eq {
	pc = 0x82414738; continue 'dispatch;
	}
	// 82414714: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414718: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241471C: 40820008  bne 0x82414724
	if !ctx.cr[0].eq {
	pc = 0x82414724; continue 'dispatch;
	}
	// 82414720: 48000000  b 0x82414720
	pc = 0x82414720; continue 'dispatch;
	// 82414724: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414728: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 * 12;
	// 8241472C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82414730: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 82414734: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82414738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241473C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414744: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414748: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241474C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414750 size=380
    let mut pc: u32 = 0x82414750;
    'dispatch: loop {
        match pc {
            0x82414750 => {
    //   block [0x82414750..0x824148CC)
	// 82414750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414754: 48120955  bl 0x825350a8
	ctx.lr = 0x82414758;
	sub_82535080(ctx, base);
	// 82414758: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241475C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82414760: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82414764: 3BBC0020  addi r29, r28, 0x20
	ctx.r[29].s64 = ctx.r[28].s64 + 32;
	// 82414768: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8241476C: 38802002  li r4, 0x2002
	ctx.r[4].s64 = 8194;
	// 82414770: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414774: 48001D9D  bl 0x82416510
	ctx.lr = 0x82414778;
	sub_82416510(ctx, base);
	// 82414778: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241477C: 40820020  bne 0x8241479c
	if !ctx.cr[0].eq {
	pc = 0x8241479C; continue 'dispatch;
	}
	// 82414780: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82414784: 38802003  li r4, 0x2003
	ctx.r[4].s64 = 8195;
	// 82414788: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241478C: 48001D85  bl 0x82416510
	ctx.lr = 0x82414790;
	sub_82416510(ctx, base);
	// 82414790: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414794: 4082012C  bne 0x824148c0
	if !ctx.cr[0].eq {
	pc = 0x824148C0; continue 'dispatch;
	}
	// 82414798: 48000000  b 0x82414798
	pc = 0x82414798; continue 'dispatch;
	// 8241479C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 824147A0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824147A4: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 824147A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824147AC: 40990114  ble cr6, 0x824148c0
	if !ctx.cr[6].gt {
	pc = 0x824148C0; continue 'dispatch;
	}
	// 824147B0: 1F5F000C  mulli r26, r31, 0xc
	ctx.r[26].s64 = ctx.r[31].s64 * 12;
	// 824147B4: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 824147B8: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 824147BC: 3D401FFF  lis r10, 0x1fff
	ctx.r[10].s64 = 536805376;
	// 824147C0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824147C4: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 824147C8: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 824147CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824147D0: 7D2BE92E  stwx r9, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 824147D4: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824147D8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824147DC: 57FB1838  slwi r27, r31, 3
	ctx.r[27].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 824147E0: 40990008  ble cr6, 0x824147e8
	if !ctx.cr[6].gt {
	pc = 0x824147E8; continue 'dispatch;
	}
	// 824147E4: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 824147E8: 4BFFBFD9  bl 0x824107c0
	ctx.lr = 0x824147EC;
	sub_824107C0(ctx, base);
	// 824147EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824147F0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824147F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824147F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824147FC: 4E800421  bctrl
	ctx.lr = 0x82414800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414800: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414804: 4182002C  beq 0x82414830
	if ctx.cr[0].eq {
	pc = 0x82414830; continue 'dispatch;
	}
	// 82414808: 357FFFFF  addic. r11, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241480C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82414810: 41800018  blt 0x82414828
	if ctx.cr[0].lt {
	pc = 0x82414828; continue 'dispatch;
	}
	// 82414814: 932A0000  stw r25, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82414818: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241481C: 932A0004  stw r25, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82414820: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82414824: 4080FFF0  bge 0x82414814
	if !ctx.cr[0].lt {
	pc = 0x82414814; continue 'dispatch;
	}
	// 82414828: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241482C: 48000008  b 0x82414834
	pc = 0x82414834; continue 'dispatch;
	// 82414830: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 82414834: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414838: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 8241483C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414840: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82414844: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82414848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241484C: 480008CD  bl 0x82415118
	ctx.lr = 0x82414850;
	sub_82415118(ctx, base);
	// 82414850: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82414854: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414858: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 8241485C: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82414860: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82414864: 40990044  ble cr6, 0x824148a8
	if !ctx.cr[6].gt {
	pc = 0x824148A8; continue 'dispatch;
	}
	// 82414868: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 8241486C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414870: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414874: 41820054  beq 0x824148c8
	if ctx.cr[0].eq {
	pc = 0x824148C8; continue 'dispatch;
	}
	// 82414878: 811C001C  lwz r8, 0x1c(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241487C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82414880: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82414884: 7D1A4214  add r8, r26, r8
	ctx.r[8].u64 = ctx.r[26].u64 + ctx.r[8].u64;
	// 82414888: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241488C: 7D08EA14  add r8, r8, r29
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[29].u64;
	// 82414890: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414894: 7CE8492E  stwx r7, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 82414898: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8241489C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824148A0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824148A4: 4198FFC8  blt cr6, 0x8241486c
	if ctx.cr[6].lt {
	pc = 0x8241486C; continue 'dispatch;
	}
	// 824148A8: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 824148AC: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 824148B0: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 824148B4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824148B8: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824148BC: 4198FEFC  blt cr6, 0x824147b8
	if ctx.cr[6].lt {
	pc = 0x824147B8; continue 'dispatch;
	}
	// 824148C0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824148C4: 48120834  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 824148C8: 48000000  b 0x824148c8
	pc = 0x824148C8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824148D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824148D0 size=464
    let mut pc: u32 = 0x824148D0;
    'dispatch: loop {
        match pc {
            0x824148D0 => {
    //   block [0x824148D0..0x82414AA0)
	// 824148D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824148D4: 481207D5  bl 0x825350a8
	ctx.lr = 0x824148D8;
	sub_82535080(ctx, base);
	// 824148D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824148DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824148E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824148E4: 3BDD0020  addi r30, r29, 0x20
	ctx.r[30].s64 = ctx.r[29].s64 + 32;
	// 824148E8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824148EC: 38802003  li r4, 0x2003
	ctx.r[4].s64 = 8195;
	// 824148F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824148F4: 48001C1D  bl 0x82416510
	ctx.lr = 0x824148F8;
	sub_82416510(ctx, base);
	// 824148F8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824148FC: 40820020  bne 0x8241491c
	if !ctx.cr[0].eq {
	pc = 0x8241491C; continue 'dispatch;
	}
	// 82414900: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82414904: 38802002  li r4, 0x2002
	ctx.r[4].s64 = 8194;
	// 82414908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241490C: 48001C05  bl 0x82416510
	ctx.lr = 0x82414910;
	sub_82416510(ctx, base);
	// 82414910: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414914: 4082017C  bne 0x82414a90
	if !ctx.cr[0].eq {
	pc = 0x82414A90; continue 'dispatch;
	}
	// 82414918: 48000000  b 0x82414918
	pc = 0x82414918; continue 'dispatch;
	// 8241491C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414920: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82414924: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82414928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241492C: 40990164  ble cr6, 0x82414a90
	if !ctx.cr[6].gt {
	pc = 0x82414A90; continue 'dispatch;
	}
	// 82414930: 1F5F000C  mulli r26, r31, 0xc
	ctx.r[26].s64 = ctx.r[31].s64 * 12;
	// 82414934: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82414938: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241493C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82414940: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414944: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82414948: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241494C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414950: 409A00A4  bne cr6, 0x824149f4
	if !ctx.cr[6].eq {
	pc = 0x824149F4; continue 'dispatch;
	}
	// 82414954: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82414958: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241495C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82414960: 577F1838  slwi r31, r27, 3
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82414964: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414968: 40990008  ble cr6, 0x82414970
	if !ctx.cr[6].gt {
	pc = 0x82414970; continue 'dispatch;
	}
	// 8241496C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82414970: 4BFFBE51  bl 0x824107c0
	ctx.lr = 0x82414974;
	sub_824107C0(ctx, base);
	// 82414974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414978: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241497C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414984: 4E800421  bctrl
	ctx.lr = 0x82414988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414988: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241498C: 4182002C  beq 0x824149b8
	if ctx.cr[0].eq {
	pc = 0x824149B8; continue 'dispatch;
	}
	// 82414990: 357BFFFF  addic. r11, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414994: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82414998: 41800018  blt 0x824149b0
	if ctx.cr[0].lt {
	pc = 0x824149B0; continue 'dispatch;
	}
	// 8241499C: 930A0000  stw r24, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 824149A0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824149A4: 930A0004  stw r24, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 824149A8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824149AC: 4080FFF0  bge 0x8241499c
	if !ctx.cr[0].lt {
	pc = 0x8241499C; continue 'dispatch;
	}
	// 824149B0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824149B4: 48000008  b 0x824149bc
	pc = 0x824149BC; continue 'dispatch;
	// 824149B8: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 824149BC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824149C0: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 824149C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824149C8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824149CC: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 824149D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824149D4: 48000745  bl 0x82415118
	ctx.lr = 0x824149D8;
	sub_82415118(ctx, base);
	// 824149D8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824149DC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 824149E0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824149E4: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 824149E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824149EC: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 824149F0: 48000014  b 0x82414a04
	pc = 0x82414A04; continue 'dispatch;
	// 824149F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824149F8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824149FC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414A00: 409A009C  bne cr6, 0x82414a9c
	if !ctx.cr[6].eq {
	pc = 0x82414A9C; continue 'dispatch;
	}
	// 82414A04: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A08: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82414A0C: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82414A10: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82414A14: 40990054  ble cr6, 0x82414a68
	if !ctx.cr[6].gt {
	pc = 0x82414A68; continue 'dispatch;
	}
	// 82414A18: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82414A1C: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A20: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414A24: 41820074  beq 0x82414a98
	if ctx.cr[0].eq {
	pc = 0x82414A98; continue 'dispatch;
	}
	// 82414A28: 811D001C  lwz r8, 0x1c(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414A2C: 38EB0002  addi r7, r11, 2
	ctx.r[7].s64 = ctx.r[11].s64 + 2;
	// 82414A30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82414A34: 7D08D214  add r8, r8, r26
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[26].u64;
	// 82414A38: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414A3C: 7D08F214  add r8, r8, r30
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 82414A40: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414A44: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82414A48: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82414A4C: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82414A50: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A54: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414A58: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82414A5C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82414A60: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82414A64: 4198FFB8  blt cr6, 0x82414a1c
	if ctx.cr[6].lt {
	pc = 0x82414A1C; continue 'dispatch;
	}
	// 82414A68: 556A07BF  clrlwi. r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414A6C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82414A70: 4182000C  beq 0x82414a7c
	if ctx.cr[0].eq {
	pc = 0x82414A7C; continue 'dispatch;
	}
	// 82414A74: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82414A78: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 82414A7C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414A80: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82414A84: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82414A88: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414A8C: 4198FEAC  blt cr6, 0x82414938
	if ctx.cr[6].lt {
	pc = 0x82414938; continue 'dispatch;
	}
	// 82414A90: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82414A94: 48120664  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82414A98: 48000000  b 0x82414a98
	pc = 0x82414A98; continue 'dispatch;
	// 82414A9C: 48000000  b 0x82414a9c
	pc = 0x82414A9C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414AA0 size=168
    let mut pc: u32 = 0x82414AA0;
    'dispatch: loop {
        match pc {
            0x82414AA0 => {
    //   block [0x82414AA0..0x82414B48)
	// 82414AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414AA4: 48120615  bl 0x825350b8
	ctx.lr = 0x82414AA8;
	sub_82535080(ctx, base);
	// 82414AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414AAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82414AB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414AB4: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414AB8: 4182005C  beq 0x82414b14
	if ctx.cr[0].eq {
	pc = 0x82414B14; continue 'dispatch;
	}
	// 82414ABC: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82414AC0: 3BBFFFFC  addi r29, r31, -4
	ctx.r[29].s64 = ctx.r[31].s64 + -4;
	// 82414AC4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82414AC8: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414ACC: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82414AD0: 4180001C  blt 0x82414aec
	if ctx.cr[0].lt {
	pc = 0x82414AEC; continue 'dispatch;
	}
	// 82414AD4: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82414AD8: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82414ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414AE0: 48000639  bl 0x82415118
	ctx.lr = 0x82414AE4;
	sub_82415118(ctx, base);
	// 82414AE4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414AE8: 4080FFF0  bge 0x82414ad8
	if !ctx.cr[0].lt {
	pc = 0x82414AD8; continue 'dispatch;
	}
	// 82414AEC: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414AF0: 4182001C  beq 0x82414b0c
	if ctx.cr[0].eq {
	pc = 0x82414B0C; continue 'dispatch;
	}
	// 82414AF4: 4BFFBCCD  bl 0x824107c0
	ctx.lr = 0x82414AF8;
	sub_824107C0(ctx, base);
	// 82414AF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414AFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82414B00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414B04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414B08: 4E800421  bctrl
	ctx.lr = 0x82414B0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414B0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414B10: 48000030  b 0x82414b40
	pc = 0x82414B40; continue 'dispatch;
	// 82414B14: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82414B18: 48000601  bl 0x82415118
	ctx.lr = 0x82414B1C;
	sub_82415118(ctx, base);
	// 82414B1C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414B20: 4182001C  beq 0x82414b3c
	if ctx.cr[0].eq {
	pc = 0x82414B3C; continue 'dispatch;
	}
	// 82414B24: 4BFFBC9D  bl 0x824107c0
	ctx.lr = 0x82414B28;
	sub_824107C0(ctx, base);
	// 82414B28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414B2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82414B30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414B38: 4E800421  bctrl
	ctx.lr = 0x82414B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414B44: 481205C4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414B48 size=84
    let mut pc: u32 = 0x82414B48;
    'dispatch: loop {
        match pc {
            0x82414B48 => {
    //   block [0x82414B48..0x82414B9C)
	// 82414B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82414B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414B60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414B64: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414B68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414B6C: 41820014  beq 0x82414b80
	if ctx.cr[0].eq {
	pc = 0x82414B80; continue 'dispatch;
	}
	// 82414B70: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414B74: 4BFFFF2D  bl 0x82414aa0
	ctx.lr = 0x82414B78;
	sub_82414AA0(ctx, base);
	// 82414B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414B7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82414B80: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414BA0 size=232
    let mut pc: u32 = 0x82414BA0;
    'dispatch: loop {
        match pc {
            0x82414BA0 => {
    //   block [0x82414BA0..0x82414C88)
	// 82414BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414BA4: 48120515  bl 0x825350b8
	ctx.lr = 0x82414BA8;
	sub_82535080(ctx, base);
	// 82414BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414BAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82414BB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82414BB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82414BB8: 38802000  li r4, 0x2000
	ctx.r[4].s64 = 8192;
	// 82414BBC: 387D0020  addi r3, r29, 0x20
	ctx.r[3].s64 = ctx.r[29].s64 + 32;
	// 82414BC0: 48001951  bl 0x82416510
	ctx.lr = 0x82414BC4;
	sub_82416510(ctx, base);
	// 82414BC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414BC8: 40820008  bne 0x82414bd0
	if !ctx.cr[0].eq {
	pc = 0x82414BD0; continue 'dispatch;
	}
	// 82414BCC: 48000000  b 0x82414bcc
	pc = 0x82414BCC; continue 'dispatch;
	// 82414BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414BD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414BD8: 40820008  bne 0x82414be0
	if !ctx.cr[0].eq {
	pc = 0x82414BE0; continue 'dispatch;
	}
	// 82414BDC: 48000000  b 0x82414bdc
	pc = 0x82414BDC; continue 'dispatch;
	// 82414BE0: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414BE4: 1F9F000C  mulli r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 * 12;
	// 82414BE8: 3D201FFF  lis r9, 0x1fff
	ctx.r[9].s64 = 536805376;
	// 82414BEC: 7D6AE12E  stwx r11, r10, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u32) };
	// 82414BF0: 83FD000C  lwz r31, 0xc(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414BF4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82414BF8: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82414BFC: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82414C00: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82414C04: 40990008  ble cr6, 0x82414c0c
	if !ctx.cr[6].gt {
	pc = 0x82414C0C; continue 'dispatch;
	}
	// 82414C08: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82414C0C: 3940FFFB  li r10, -5
	ctx.r[10].s64 = -5;
	// 82414C10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414C14: 41990008  bgt cr6, 0x82414c1c
	if ctx.cr[6].gt {
	pc = 0x82414C1C; continue 'dispatch;
	}
	// 82414C18: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82414C1C: 4BFFBBA5  bl 0x824107c0
	ctx.lr = 0x82414C20;
	sub_824107C0(ctx, base);
	// 82414C20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414C24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414C28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414C30: 4E800421  bctrl
	ctx.lr = 0x82414C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414C34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414C38: 41820034  beq 0x82414c6c
	if ctx.cr[0].eq {
	pc = 0x82414C6C; continue 'dispatch;
	}
	// 82414C3C: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82414C40: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82414C44: 357FFFFF  addic. r11, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414C48: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82414C4C: 41800024  blt 0x82414c70
	if ctx.cr[0].lt {
	pc = 0x82414C70; continue 'dispatch;
	}
	// 82414C50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82414C54: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82414C58: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414C5C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82414C60: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82414C64: 4080FFF0  bge 0x82414c54
	if !ctx.cr[0].lt {
	pc = 0x82414C54; continue 'dispatch;
	}
	// 82414C68: 48000008  b 0x82414c70
	pc = 0x82414C70; continue 'dispatch;
	// 82414C6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82414C70: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414C74: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82414C78: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82414C7C: 4BFFFECD  bl 0x82414b48
	ctx.lr = 0x82414C80;
	sub_82414B48(ctx, base);
	// 82414C80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414C84: 48120484  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414C88 size=208
    let mut pc: u32 = 0x82414C88;
    'dispatch: loop {
        match pc {
            0x82414C88 => {
    //   block [0x82414C88..0x82414D58)
	// 82414C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414C8C: 4812042D  bl 0x825350b8
	ctx.lr = 0x82414C90;
	sub_82535080(ctx, base);
	// 82414C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414C94: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82414C98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414C9C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414CA0: 41820070  beq 0x82414d10
	if ctx.cr[0].eq {
	pc = 0x82414D10; continue 'dispatch;
	}
	// 82414CA4: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82414CA8: 3BBFFFFC  addi r29, r31, -4
	ctx.r[29].s64 = ctx.r[31].s64 + -4;
	// 82414CAC: 1D4B000C  mulli r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 * 12;
	// 82414CB0: 37CBFFFF  addic. r30, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414CB4: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82414CB8: 41800030  blt 0x82414ce8
	if ctx.cr[0].lt {
	pc = 0x82414CE8; continue 'dispatch;
	}
	// 82414CBC: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82414CC0: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 82414CC4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414CC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414CCC: 41820014  beq 0x82414ce0
	if ctx.cr[0].eq {
	pc = 0x82414CE0; continue 'dispatch;
	}
	// 82414CD0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414CD4: 4BFFFDCD  bl 0x82414aa0
	ctx.lr = 0x82414CD8;
	sub_82414AA0(ctx, base);
	// 82414CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414CDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82414CE0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82414CE4: 4080FFDC  bge 0x82414cc0
	if !ctx.cr[0].lt {
	pc = 0x82414CC0; continue 'dispatch;
	}
	// 82414CE8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414CEC: 4182001C  beq 0x82414d08
	if ctx.cr[0].eq {
	pc = 0x82414D08; continue 'dispatch;
	}
	// 82414CF0: 4BFFBAD1  bl 0x824107c0
	ctx.lr = 0x82414CF4;
	sub_824107C0(ctx, base);
	// 82414CF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414CF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82414CFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414D04: 4E800421  bctrl
	ctx.lr = 0x82414D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414D08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414D0C: 48000044  b 0x82414d50
	pc = 0x82414D50; continue 'dispatch;
	// 82414D10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414D14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414D18: 41820014  beq 0x82414d2c
	if ctx.cr[0].eq {
	pc = 0x82414D2C; continue 'dispatch;
	}
	// 82414D1C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414D20: 4BFFFD81  bl 0x82414aa0
	ctx.lr = 0x82414D24;
	sub_82414AA0(ctx, base);
	// 82414D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414D28: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82414D2C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82414D30: 4182001C  beq 0x82414d4c
	if ctx.cr[0].eq {
	pc = 0x82414D4C; continue 'dispatch;
	}
	// 82414D34: 4BFFBA8D  bl 0x824107c0
	ctx.lr = 0x82414D38;
	sub_824107C0(ctx, base);
	// 82414D38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414D3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82414D40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414D48: 4E800421  bctrl
	ctx.lr = 0x82414D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82414D54: 481203B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414D58 size=84
    let mut pc: u32 = 0x82414D58;
    'dispatch: loop {
        match pc {
            0x82414D58 => {
    //   block [0x82414D58..0x82414DAC)
	// 82414D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82414D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414D70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82414D74: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414D78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414D7C: 41820014  beq 0x82414d90
	if ctx.cr[0].eq {
	pc = 0x82414D90; continue 'dispatch;
	}
	// 82414D80: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414D84: 4BFFFF05  bl 0x82414c88
	ctx.lr = 0x82414D88;
	sub_82414C88(ctx, base);
	// 82414D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414D8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82414D90: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414D94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414DA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82414DA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414DB0 size=76
    let mut pc: u32 = 0x82414DB0;
    'dispatch: loop {
        match pc {
            0x82414DB0 => {
    //   block [0x82414DB0..0x82414DFC)
	// 82414DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82414DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82414DBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414DC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414DC4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82414DC8: 48001941  bl 0x82416708
	ctx.lr = 0x82414DCC;
	sub_82416708(ctx, base);
	// 82414DCC: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82414DD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414DD4: 41820014  beq 0x82414de8
	if ctx.cr[0].eq {
	pc = 0x82414DE8; continue 'dispatch;
	}
	// 82414DD8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82414DDC: 4BFFFEAD  bl 0x82414c88
	ctx.lr = 0x82414DE0;
	sub_82414C88(ctx, base);
	// 82414DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82414DE4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82414DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82414DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82414DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82414DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82414DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414E00 size=336
    let mut pc: u32 = 0x82414E00;
    'dispatch: loop {
        match pc {
            0x82414E00 => {
    //   block [0x82414E00..0x82414F50)
	// 82414E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414E04: 481202B9  bl 0x825350bc
	ctx.lr = 0x82414E08;
	sub_82535080(ctx, base);
	// 82414E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414E0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82414E10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82414E14: 38801000  li r4, 0x1000
	ctx.r[4].s64 = 4096;
	// 82414E18: 387D0020  addi r3, r29, 0x20
	ctx.r[3].s64 = ctx.r[29].s64 + 32;
	// 82414E1C: 480016F5  bl 0x82416510
	ctx.lr = 0x82414E20;
	sub_82416510(ctx, base);
	// 82414E20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E24: 40820008  bne 0x82414e2c
	if !ctx.cr[0].eq {
	pc = 0x82414E2C; continue 'dispatch;
	}
	// 82414E28: 48000000  b 0x82414e28
	pc = 0x82414E28; continue 'dispatch;
	// 82414E2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414E30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E34: 40820008  bne 0x82414e3c
	if !ctx.cr[0].eq {
	pc = 0x82414E3C; continue 'dispatch;
	}
	// 82414E38: 48000000  b 0x82414e38
	pc = 0x82414E38; continue 'dispatch;
	// 82414E3C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82414E40: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414E44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E48: 40820008  bne 0x82414e50
	if !ctx.cr[0].eq {
	pc = 0x82414E50; continue 'dispatch;
	}
	// 82414E4C: 48000000  b 0x82414e4c
	pc = 0x82414E4C; continue 'dispatch;
	// 82414E50: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82414E54: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414E58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414E5C: 40820008  bne 0x82414e64
	if !ctx.cr[0].eq {
	pc = 0x82414E64; continue 'dispatch;
	}
	// 82414E60: 48000000  b 0x82414e60
	pc = 0x82414E60; continue 'dispatch;
	// 82414E64: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82414E68: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82414E6C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82414E70: 419A003C  beq cr6, 0x82414eac
	if ctx.cr[6].eq {
	pc = 0x82414EAC; continue 'dispatch;
	}
	// 82414E74: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82414E78: 419A002C  beq cr6, 0x82414ea4
	if ctx.cr[6].eq {
	pc = 0x82414EA4; continue 'dispatch;
	}
	// 82414E7C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82414E80: 419A0008  beq cr6, 0x82414e88
	if ctx.cr[6].eq {
	pc = 0x82414E88; continue 'dispatch;
	}
	// 82414E84: 48000000  b 0x82414e84
	pc = 0x82414E84; continue 'dispatch;
	// 82414E88: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82414E8C: 39430016  addi r10, r3, 0x16
	ctx.r[10].s64 = ctx.r[3].s64 + 22;
	// 82414E90: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82414E94: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82414E98: 915D0018  stw r10, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82414E9C: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82414EA0: 48000014  b 0x82414eb4
	pc = 0x82414EB4; continue 'dispatch;
	// 82414EA4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82414EA8: 48000008  b 0x82414eb0
	pc = 0x82414EB0; continue 'dispatch;
	// 82414EAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82414EB0: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82414EB4: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 82414EB8: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414EBC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82414EC0: 616B5555  ori r11, r11, 0x5555
	ctx.r[11].u64 = ctx.r[11].u64 | 21845;
	// 82414EC4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82414EC8: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 * 12;
	// 82414ECC: 40990008  ble cr6, 0x82414ed4
	if !ctx.cr[6].gt {
	pc = 0x82414ED4; continue 'dispatch;
	}
	// 82414ED0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82414ED4: 3940FFFB  li r10, -5
	ctx.r[10].s64 = -5;
	// 82414ED8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82414EDC: 41990008  bgt cr6, 0x82414ee4
	if ctx.cr[6].gt {
	pc = 0x82414EE4; continue 'dispatch;
	}
	// 82414EE0: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82414EE4: 4BFFB8DD  bl 0x824107c0
	ctx.lr = 0x82414EE8;
	sub_824107C0(ctx, base);
	// 82414EE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82414EEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414EF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82414EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82414EF8: 4E800421  bctrl
	ctx.lr = 0x82414EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82414EFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414F00: 41820038  beq 0x82414f38
	if ctx.cr[0].eq {
	pc = 0x82414F38; continue 'dispatch;
	}
	// 82414F04: 39030004  addi r8, r3, 4
	ctx.r[8].s64 = ctx.r[3].s64 + 4;
	// 82414F08: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82414F0C: 355FFFFF  addic. r10, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414F10: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82414F14: 41800028  blt 0x82414f3c
	if ctx.cr[0].lt {
	pc = 0x82414F3C; continue 'dispatch;
	}
	// 82414F18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82414F1C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82414F20: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82414F24: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82414F28: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82414F2C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82414F30: 4080FFEC  bge 0x82414f1c
	if !ctx.cr[0].lt {
	pc = 0x82414F1C; continue 'dispatch;
	}
	// 82414F34: 48000008  b 0x82414f3c
	pc = 0x82414F3C; continue 'dispatch;
	// 82414F38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82414F3C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82414F40: 387D001C  addi r3, r29, 0x1c
	ctx.r[3].s64 = ctx.r[29].s64 + 28;
	// 82414F44: 4BFFFE15  bl 0x82414d58
	ctx.lr = 0x82414F48;
	sub_82414D58(ctx, base);
	// 82414F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82414F4C: 481201C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82414F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82414F50 size=196
    let mut pc: u32 = 0x82414F50;
    'dispatch: loop {
        match pc {
            0x82414F50 => {
    //   block [0x82414F50..0x82415014)
	// 82414F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82414F54: 48120169  bl 0x825350bc
	ctx.lr = 0x82414F58;
	sub_82535080(ctx, base);
	// 82414F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82414F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82414F60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82414F64: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82414F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414F6C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82414F70: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82414F74: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82414F78: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82414F7C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82414F80: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82414F84: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82414F88: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82414F8C: 48001605  bl 0x82416590
	ctx.lr = 0x82414F90;
	sub_82416590(ctx, base);
	// 82414F90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82414F94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82414F98: 388BEC48  addi r4, r11, -0x13b8
	ctx.r[4].s64 = ctx.r[11].s64 + -5048;
	// 82414F9C: 48001435  bl 0x824163d0
	ctx.lr = 0x82414FA0;
	sub_824163D0(ctx, base);
	// 82414FA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82414FA4: 40820008  bne 0x82414fac
	if !ctx.cr[0].eq {
	pc = 0x82414FAC; continue 'dispatch;
	}
	// 82414FA8: 48000000  b 0x82414fa8
	pc = 0x82414FA8; continue 'dispatch;
	// 82414FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FB0: 4BFFFE51  bl 0x82414e00
	ctx.lr = 0x82414FB4;
	sub_82414E00(ctx, base);
	// 82414FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FB8: 4BFFF591  bl 0x82414548
	ctx.lr = 0x82414FBC;
	sub_82414548(ctx, base);
	// 82414FBC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414FC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82414FC4: 40990044  ble cr6, 0x82415008
	if !ctx.cr[6].gt {
	pc = 0x82415008; continue 'dispatch;
	}
	// 82414FC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FD0: 4BFFFBD1  bl 0x82414ba0
	ctx.lr = 0x82414FD4;
	sub_82414BA0(ctx, base);
	// 82414FD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FDC: 4BFFF705  bl 0x824146e0
	ctx.lr = 0x82414FE0;
	sub_824146E0(ctx, base);
	// 82414FE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FE8: 4BFFF769  bl 0x82414750
	ctx.lr = 0x82414FEC;
	sub_82414750(ctx, base);
	// 82414FEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82414FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82414FF4: 4BFFF8DD  bl 0x824148d0
	ctx.lr = 0x82414FF8;
	sub_824148D0(ctx, base);
	// 82414FF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82414FFC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82415000: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415004: 4198FFC4  blt cr6, 0x82414fc8
	if ctx.cr[6].lt {
	pc = 0x82414FC8; continue 'dispatch;
	}
	// 82415008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241500C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415010: 481200FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415018 size=40
    let mut pc: u32 = 0x82415018;
    'dispatch: loop {
        match pc {
            0x82415018 => {
    //   block [0x82415018..0x82415040)
	// 82415018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241501C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415024: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 82415028: 480013F9  bl 0x82416420
	ctx.lr = 0x8241502C;
	sub_82416420(ctx, base);
	// 8241502C: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82415030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82415034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82415038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415040 size=8
    let mut pc: u32 = 0x82415040;
    'dispatch: loop {
        match pc {
            0x82415040 => {
    //   block [0x82415040..0x82415048)
	// 82415040: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 82415044: 48001414  b 0x82416458
	sub_82416458(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415048 size=8
    let mut pc: u32 = 0x82415048;
    'dispatch: loop {
        match pc {
            0x82415048 => {
    //   block [0x82415048..0x82415050)
	// 82415048: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 8241504C: 4800142C  b 0x82416478
	sub_82416478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415050 size=8
    let mut pc: u32 = 0x82415050;
    'dispatch: loop {
        match pc {
            0x82415050 => {
    //   block [0x82415050..0x82415058)
	// 82415050: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415058 size=8
    let mut pc: u32 = 0x82415058;
    'dispatch: loop {
        match pc {
            0x82415058 => {
    //   block [0x82415058..0x82415060)
	// 82415058: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241505C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415060 size=20
    let mut pc: u32 = 0x82415060;
    'dispatch: loop {
        match pc {
            0x82415060 => {
    //   block [0x82415060..0x82415074)
	// 82415060: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82415064: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415068: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 8241506C: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82415070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415078 size=20
    let mut pc: u32 = 0x82415078;
    'dispatch: loop {
        match pc {
            0x82415078 => {
    //   block [0x82415078..0x8241508C)
	// 82415078: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241507C: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82415080: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415084: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82415088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415090 size=8
    let mut pc: u32 = 0x82415090;
    'dispatch: loop {
        match pc {
            0x82415090 => {
    //   block [0x82415090..0x82415098)
	// 82415090: 80630084  lwz r3, 0x84(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82415094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415098 size=76
    let mut pc: u32 = 0x82415098;
    'dispatch: loop {
        match pc {
            0x82415098 => {
    //   block [0x82415098..0x824150E4)
	// 82415098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241509C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824150A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824150A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824150A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824150AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824150B0: 38803000  li r4, 0x3000
	ctx.r[4].s64 = 12288;
	// 824150B4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 824150B8: 48001459  bl 0x82416510
	ctx.lr = 0x824150BC;
	sub_82416510(ctx, base);
	// 824150BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824150C0: 40820008  bne 0x824150c8
	if !ctx.cr[0].eq {
	pc = 0x824150C8; continue 'dispatch;
	}
	// 824150C4: 48000000  b 0x824150c4
	pc = 0x824150C4; continue 'dispatch;
	// 824150C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824150CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824150D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824150D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824150D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824150DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824150E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824150E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824150E8 size=20
    let mut pc: u32 = 0x824150E8;
    'dispatch: loop {
        match pc {
            0x824150E8 => {
    //   block [0x824150E8..0x824150FC)
	// 824150E8: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824150EC: 2B0B8004  cmplwi cr6, r11, 0x8004
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32772 as u32, &mut ctx.xer);
	// 824150F0: 409A000C  bne cr6, 0x824150fc
	if !ctx.cr[6].eq {
		sub_824150FC(ctx, base);
		return;
	}
	// 824150F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824150F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824150FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824150FC size=24
    let mut pc: u32 = 0x824150FC;
    'dispatch: loop {
        match pc {
            0x824150FC => {
    //   block [0x824150FC..0x82415114)
	// 824150FC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82415100: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82415104: 7D6B2810  subfc r11, r11, r5
	ctx.xer.ca = ctx.r[5].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82415108: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8241510C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82415110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415118 size=92
    let mut pc: u32 = 0x82415118;
    'dispatch: loop {
        match pc {
            0x82415118 => {
    //   block [0x82415118..0x82415174)
	// 82415118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241511C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82415124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82415128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241512C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415130: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415134: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415138: 41820024  beq 0x8241515c
	if ctx.cr[0].eq {
	pc = 0x8241515C; continue 'dispatch;
	}
	// 8241513C: 4BFFB685  bl 0x824107c0
	ctx.lr = 0x82415140;
	sub_824107C0(ctx, base);
	// 82415140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415144: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82415148: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241514C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82415150: 4E800421  bctrl
	ctx.lr = 0x82415154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82415154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415158: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241515C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82415164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82415168: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241516C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82415170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415178 size=64
    let mut pc: u32 = 0x82415178;
    'dispatch: loop {
        match pc {
            0x82415178 => {
    //   block [0x82415178..0x824151B8)
	// 82415178: 80E3000C  lwz r7, 0xc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241517C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415180: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415184: 4182002C  beq 0x824151b0
	if ctx.cr[0].eq {
	pc = 0x824151B0; continue 'dispatch;
	}
	// 82415188: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241518C: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82415190: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82415194: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415198: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8241519C: 419A001C  beq cr6, 0x824151b8
	if ctx.cr[6].eq {
		sub_824151B8(ctx, base);
		return;
	}
	// 824151A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824151A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824151A8: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824151AC: 4198FFE8  blt cr6, 0x82415194
	if ctx.cr[6].lt {
	pc = 0x82415194; continue 'dispatch;
	}
	// 824151B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824151B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824151B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824151B8 size=16
    let mut pc: u32 = 0x824151B8;
    'dispatch: loop {
        match pc {
            0x824151B8 => {
    //   block [0x824151B8..0x824151C8)
	// 824151B8: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824151BC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824151C0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824151C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824151C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824151C8 size=64
    let mut pc: u32 = 0x824151C8;
    'dispatch: loop {
        match pc {
            0x824151C8 => {
    //   block [0x824151C8..0x82415208)
	// 824151C8: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824151CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824151D0: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824151D4: 4182002C  beq 0x82415200
	if ctx.cr[0].eq {
	pc = 0x82415200; continue 'dispatch;
	}
	// 824151D8: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824151DC: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 824151E0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 824151E4: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824151E8: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824151EC: 419A001C  beq cr6, 0x82415208
	if ctx.cr[6].eq {
		sub_82415208(ctx, base);
		return;
	}
	// 824151F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824151F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824151F8: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824151FC: 4198FFE8  blt cr6, 0x824151e4
	if ctx.cr[6].lt {
	pc = 0x824151E4; continue 'dispatch;
	}
	// 82415200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415208 size=16
    let mut pc: u32 = 0x82415208;
    'dispatch: loop {
        match pc {
            0x82415208 => {
    //   block [0x82415208..0x82415218)
	// 82415208: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241520C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82415210: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415218 size=60
    let mut pc: u32 = 0x82415218;
    'dispatch: loop {
        match pc {
            0x82415218 => {
    //   block [0x82415218..0x82415254)
	// 82415218: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241521C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415220: 419A0034  beq cr6, 0x82415254
	if ctx.cr[6].eq {
		sub_82415254(ctx, base);
		return;
	}
	// 82415224: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415228: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241522C: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82415230: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82415234: A16B0002  lhz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82415238: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8241523C: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82415240: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415244: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415248: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 8241524C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415250: 48000014  b 0x82415264
	sub_82415254(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415254 size=24
    let mut pc: u32 = 0x82415254;
    'dispatch: loop {
        match pc {
            0x82415254 => {
    //   block [0x82415254..0x8241526C)
	// 82415254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415258: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8241525C: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82415260: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415264: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415270 size=24
    let mut pc: u32 = 0x82415270;
    'dispatch: loop {
        match pc {
            0x82415270 => {
    //   block [0x82415270..0x82415288)
	// 82415270: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82415274: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415278: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241527C: 4082000C  bne 0x82415288
	if !ctx.cr[0].eq {
		sub_82415288(ctx, base);
		return;
	}
	// 82415280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415288 size=52
    let mut pc: u32 = 0x82415288;
    'dispatch: loop {
        match pc {
            0x82415288 => {
    //   block [0x82415288..0x824152BC)
	// 82415288: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8241528C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82415290: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415294: 4099FFEC  ble cr6, 0x82415280
	if !ctx.cr[6].gt {
		sub_82415270(ctx, base);
		return;
	}
	// 82415298: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8241529C: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 824152A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824152A4: 912B0064  stw r9, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 824152A8: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824152AC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824152B0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824152B4: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 824152B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824152C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824152C0 size=144
    let mut pc: u32 = 0x824152C0;
    'dispatch: loop {
        match pc {
            0x824152C0 => {
    //   block [0x824152C0..0x82415350)
	// 824152C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824152C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824152C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824152CC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 824152D0: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 824152D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824152D8: 4082000C  bne 0x824152e4
	if !ctx.cr[0].eq {
	pc = 0x824152E4; continue 'dispatch;
	}
	// 824152DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824152E0: 48000060  b 0x82415340
	pc = 0x82415340; continue 'dispatch;
	// 824152E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824152E8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824152EC: 4198FFF0  blt cr6, 0x824152dc
	if ctx.cr[6].lt {
	pc = 0x824152DC; continue 'dispatch;
	}
	// 824152F0: 81680064  lwz r11, 0x64(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(100 as u32) ) } as u64;
	// 824152F4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824152F8: 419A0044  beq cr6, 0x8241533c
	if ctx.cr[6].eq {
	pc = 0x8241533C; continue 'dispatch;
	}
	// 824152FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415300: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415304: 409A0010  bne cr6, 0x82415314
	if !ctx.cr[6].eq {
	pc = 0x82415314; continue 'dispatch;
	}
	// 82415308: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8241530C: 4BFFFF65  bl 0x82415270
	ctx.lr = 0x82415310;
	sub_82415270(ctx, base);
	// 82415310: 48000030  b 0x82415340
	pc = 0x82415340; continue 'dispatch;
	// 82415314: 81680058  lwz r11, 0x58(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(88 as u32) ) } as u64;
	// 82415318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241531C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82415320: 91480064  stw r10, 0x64(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82415324: 91680060  stw r11, 0x60(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82415328: 419A0014  beq cr6, 0x8241533c
	if ctx.cr[6].eq {
	pc = 0x8241533C; continue 'dispatch;
	}
	// 8241532C: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82415330: 4BFFFF41  bl 0x82415270
	ctx.lr = 0x82415334;
	sub_82415270(ctx, base);
	// 82415334: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82415338: 4082FFF4  bne 0x8241532c
	if !ctx.cr[0].eq {
	pc = 0x8241532C; continue 'dispatch;
	}
	// 8241533C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82415344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82415348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415350 size=64
    let mut pc: u32 = 0x82415350;
    'dispatch: loop {
        match pc {
            0x82415350 => {
    //   block [0x82415350..0x82415390)
	// 82415350: 80E30020  lwz r7, 0x20(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415358: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241535C: 4182002C  beq 0x82415388
	if ctx.cr[0].eq {
	pc = 0x82415388; continue 'dispatch;
	}
	// 82415360: 81230024  lwz r9, 0x24(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82415364: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82415368: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8241536C: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415370: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82415374: 419A001C  beq cr6, 0x82415390
	if ctx.cr[6].eq {
		sub_82415390(ctx, base);
		return;
	}
	// 82415378: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241537C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415380: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82415384: 4198FFE8  blt cr6, 0x8241536c
	if ctx.cr[6].lt {
	pc = 0x8241536C; continue 'dispatch;
	}
	// 82415388: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241538C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415390 size=16
    let mut pc: u32 = 0x82415390;
    'dispatch: loop {
        match pc {
            0x82415390 => {
    //   block [0x82415390..0x824153A0)
	// 82415390: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415394: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82415398: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241539C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824153A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824153A0 size=48
    let mut pc: u32 = 0x824153A0;
    'dispatch: loop {
        match pc {
            0x824153A0 => {
    //   block [0x824153A0..0x824153D0)
	// 824153A0: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 824153A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824153A8: 419A0028  beq cr6, 0x824153d0
	if ctx.cr[6].eq {
		sub_824153D0(ctx, base);
		return;
	}
	// 824153AC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824153B0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824153B4: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824153B8: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 824153BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824153C0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824153C4: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 824153C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824153CC: 48000010  b 0x824153dc
	sub_824153D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824153D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824153D0 size=20
    let mut pc: u32 = 0x824153D0;
    'dispatch: loop {
        match pc {
            0x824153D0 => {
    //   block [0x824153D0..0x824153E4)
	// 824153D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824153D4: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824153D8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824153DC: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824153E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824153E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824153E8 size=24
    let mut pc: u32 = 0x824153E8;
    'dispatch: loop {
        match pc {
            0x824153E8 => {
    //   block [0x824153E8..0x82415400)
	// 824153E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824153EC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824153F0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824153F4: 4082000C  bne 0x82415400
	if !ctx.cr[0].eq {
		sub_82415400(ctx, base);
		return;
	}
	// 824153F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824153FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415400 size=52
    let mut pc: u32 = 0x82415400;
    'dispatch: loop {
        match pc {
            0x82415400 => {
    //   block [0x82415400..0x82415434)
	// 82415400: 812B0074  lwz r9, 0x74(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82415404: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82415408: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8241540C: 4099FFEC  ble cr6, 0x824153f8
	if !ctx.cr[6].gt {
		sub_824153E8(ctx, base);
		return;
	}
	// 82415410: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82415414: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82415418: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241541C: 912B0074  stw r9, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82415420: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415424: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82415428: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8241542C: 914B0070  stw r10, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82415430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415438 size=144
    let mut pc: u32 = 0x82415438;
    'dispatch: loop {
        match pc {
            0x82415438 => {
    //   block [0x82415438..0x824154C8)
	// 82415438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415444: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82415448: 8168001C  lwz r11, 0x1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241544C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415450: 4082000C  bne 0x8241545c
	if !ctx.cr[0].eq {
	pc = 0x8241545C; continue 'dispatch;
	}
	// 82415454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415458: 48000060  b 0x824154b8
	pc = 0x824154B8; continue 'dispatch;
	// 8241545C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82415460: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415464: 4198FFF0  blt cr6, 0x82415454
	if ctx.cr[6].lt {
	pc = 0x82415454; continue 'dispatch;
	}
	// 82415468: 81680074  lwz r11, 0x74(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(116 as u32) ) } as u64;
	// 8241546C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415470: 419A0044  beq cr6, 0x824154b4
	if ctx.cr[6].eq {
	pc = 0x824154B4; continue 'dispatch;
	}
	// 82415474: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415478: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241547C: 409A0010  bne cr6, 0x8241548c
	if !ctx.cr[6].eq {
	pc = 0x8241548C; continue 'dispatch;
	}
	// 82415480: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82415484: 4BFFFF65  bl 0x824153e8
	ctx.lr = 0x82415488;
	sub_824153E8(ctx, base);
	// 82415488: 48000030  b 0x824154b8
	pc = 0x824154B8; continue 'dispatch;
	// 8241548C: 81680068  lwz r11, 0x68(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(104 as u32) ) } as u64;
	// 82415490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415494: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82415498: 91480074  stw r10, 0x74(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8241549C: 91680070  stw r11, 0x70(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 824154A0: 419A0014  beq cr6, 0x824154b4
	if ctx.cr[6].eq {
	pc = 0x824154B4; continue 'dispatch;
	}
	// 824154A4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824154A8: 4BFFFF41  bl 0x824153e8
	ctx.lr = 0x824154AC;
	sub_824153E8(ctx, base);
	// 824154AC: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824154B0: 4082FFF4  bne 0x824154a4
	if !ctx.cr[0].eq {
	pc = 0x824154A4; continue 'dispatch;
	}
	// 824154B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824154B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824154BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824154C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824154C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824154C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824154C8 size=80
    let mut pc: u32 = 0x824154C8;
    'dispatch: loop {
        match pc {
            0x824154C8 => {
    //   block [0x824154C8..0x82415518)
	// 824154C8: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 824154CC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824154D0: 2B0A8004  cmplwi cr6, r10, 0x8004
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32772 as u32, &mut ctx.xer);
	// 824154D4: 419A0044  beq cr6, 0x82415518
	if ctx.cr[6].eq {
		sub_82415518(ctx, base);
		return;
	}
	// 824154D8: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824154DC: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 824154E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824154E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824154E8: 40990030  ble cr6, 0x82415518
	if !ctx.cr[6].gt {
		sub_82415518(ctx, base);
		return;
	}
	// 824154EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824154F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824154F4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824154F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824154FC: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82415500: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415504: B1440000  sth r10, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82415508: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8241550C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415510: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82415514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415518 size=8
    let mut pc: u32 = 0x82415518;
    'dispatch: loop {
        match pc {
            0x82415518 => {
    //   block [0x82415518..0x82415520)
	// 82415518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241551C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415520 size=64
    let mut pc: u32 = 0x82415520;
    'dispatch: loop {
        match pc {
            0x82415520 => {
    //   block [0x82415520..0x82415560)
	// 82415520: 80E3002C  lwz r7, 0x2c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82415524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415528: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241552C: 4182002C  beq 0x82415558
	if ctx.cr[0].eq {
	pc = 0x82415558; continue 'dispatch;
	}
	// 82415530: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82415534: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82415538: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8241553C: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415540: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82415544: 419A001C  beq cr6, 0x82415560
	if ctx.cr[6].eq {
		sub_82415560(ctx, base);
		return;
	}
	// 82415548: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241554C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415550: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82415554: 4198FFE8  blt cr6, 0x8241553c
	if ctx.cr[6].lt {
	pc = 0x8241553C; continue 'dispatch;
	}
	// 82415558: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241555C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415560 size=16
    let mut pc: u32 = 0x82415560;
    'dispatch: loop {
        match pc {
            0x82415560 => {
    //   block [0x82415560..0x82415570)
	// 82415560: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415564: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82415568: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415570 size=276
    let mut pc: u32 = 0x82415570;
    'dispatch: loop {
        match pc {
            0x82415570 => {
    //   block [0x82415570..0x82415684)
	// 82415570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415574: 4811FB49  bl 0x825350bc
	ctx.lr = 0x82415578;
	sub_82535080(ctx, base);
	// 82415578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241557C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415580: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82415584: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 82415588: 38803020  li r4, 0x3020
	ctx.r[4].s64 = 12320;
	// 8241558C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82415590: 48000F81  bl 0x82416510
	ctx.lr = 0x82415594;
	sub_82416510(ctx, base);
	// 82415594: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82415598: 418200D4  beq 0x8241566c
	if ctx.cr[0].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 8241559C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824155A0: 38803020  li r4, 0x3020
	ctx.r[4].s64 = 12320;
	// 824155A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824155A8: 48000EF1  bl 0x82416498
	ctx.lr = 0x824155AC;
	sub_82416498(ctx, base);
	// 824155AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824155B0: 418200BC  beq 0x8241566c
	if ctx.cr[0].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 824155B4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824155B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824155BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824155C0: 4099004C  ble cr6, 0x8241560c
	if !ctx.cr[6].gt {
	pc = 0x8241560C; continue 'dispatch;
	}
	// 824155C4: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824155C8: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824155CC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824155D0: 4182003C  beq 0x8241560c
	if ctx.cr[0].eq {
	pc = 0x8241560C; continue 'dispatch;
	}
	// 824155D4: A12A0002  lhz r9, 2(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 824155D8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824155DC: 41820030  beq 0x8241560c
	if ctx.cr[0].eq {
	pc = 0x8241560C; continue 'dispatch;
	}
	// 824155E0: 554907BF  clrlwi. r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824155E4: 40820090  bne 0x82415674
	if !ctx.cr[0].eq {
	pc = 0x82415674; continue 'dispatch;
	}
	// 824155E8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824155EC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824155F0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824155F4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824155F8: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824155FC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82415600: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415604: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415608: 4198FFBC  blt cr6, 0x824155c4
	if ctx.cr[6].lt {
	pc = 0x824155C4; continue 'dispatch;
	}
	// 8241560C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415610: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415614: 41820058  beq 0x8241566c
	if ctx.cr[0].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 82415618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241561C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82415620: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415624: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82415628: 419A0044  beq cr6, 0x8241566c
	if ctx.cr[6].eq {
	pc = 0x8241566C; continue 'dispatch;
	}
	// 8241562C: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82415630: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415634: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415638: 41820048  beq 0x82415680
	if ctx.cr[0].eq {
	pc = 0x82415680; continue 'dispatch;
	}
	// 8241563C: A15E0002  lhz r10, 2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82415640: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415644: 41820038  beq 0x8241567c
	if ctx.cr[0].eq {
	pc = 0x8241567C; continue 'dispatch;
	}
	// 82415648: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241564C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415650: 41820028  beq 0x82415678
	if ctx.cr[0].eq {
	pc = 0x82415678; continue 'dispatch;
	}
	// 82415654: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415658: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241565C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82415660: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415664: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82415668: 4198FFC4  blt cr6, 0x8241562c
	if ctx.cr[6].lt {
	pc = 0x8241562C; continue 'dispatch;
	}
	// 8241566C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415670: 4811FA9C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82415674: 48000000  b 0x82415674
	pc = 0x82415674; continue 'dispatch;
	// 82415678: 48000000  b 0x82415678
	pc = 0x82415678; continue 'dispatch;
	// 8241567C: 48000000  b 0x8241567c
	pc = 0x8241567C; continue 'dispatch;
	// 82415680: 48000000  b 0x82415680
	pc = 0x82415680; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415688 size=248
    let mut pc: u32 = 0x82415688;
    'dispatch: loop {
        match pc {
            0x82415688 => {
    //   block [0x82415688..0x82415780)
	// 82415688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241568C: 4811FA31  bl 0x825350bc
	ctx.lr = 0x82415690;
	sub_82535080(ctx, base);
	// 82415690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415698: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241569C: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 824156A0: 38803030  li r4, 0x3030
	ctx.r[4].s64 = 12336;
	// 824156A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824156A8: 48000E69  bl 0x82416510
	ctx.lr = 0x824156AC;
	sub_82416510(ctx, base);
	// 824156AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824156B0: 418200BC  beq 0x8241576c
	if ctx.cr[0].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 824156B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824156B8: 38803030  li r4, 0x3030
	ctx.r[4].s64 = 12336;
	// 824156BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824156C0: 48000DD9  bl 0x82416498
	ctx.lr = 0x824156C4;
	sub_82416498(ctx, base);
	// 824156C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824156C8: 418200A4  beq 0x8241576c
	if ctx.cr[0].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 824156CC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824156D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824156D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824156D8: 40990040  ble cr6, 0x82415718
	if !ctx.cr[6].gt {
	pc = 0x82415718; continue 'dispatch;
	}
	// 824156DC: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824156E0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824156E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824156E8: 41820030  beq 0x82415718
	if ctx.cr[0].eq {
	pc = 0x82415718; continue 'dispatch;
	}
	// 824156EC: 554907BF  clrlwi. r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824156F0: 40820084  bne 0x82415774
	if !ctx.cr[0].eq {
	pc = 0x82415774; continue 'dispatch;
	}
	// 824156F4: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824156F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824156FC: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82415700: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415704: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415708: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241570C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415710: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415714: 4198FFC8  blt cr6, 0x824156dc
	if ctx.cr[6].lt {
	pc = 0x824156DC; continue 'dispatch;
	}
	// 82415718: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8241571C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415720: 4182004C  beq 0x8241576c
	if ctx.cr[0].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 82415724: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415728: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8241572C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415730: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82415734: 419A0038  beq cr6, 0x8241576c
	if ctx.cr[6].eq {
	pc = 0x8241576C; continue 'dispatch;
	}
	// 82415738: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8241573C: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415740: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415744: 41820038  beq 0x8241577c
	if ctx.cr[0].eq {
	pc = 0x8241577C; continue 'dispatch;
	}
	// 82415748: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241574C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415750: 41820028  beq 0x82415778
	if ctx.cr[0].eq {
	pc = 0x82415778; continue 'dispatch;
	}
	// 82415754: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415758: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241575C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82415760: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415764: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82415768: 4198FFD0  blt cr6, 0x82415738
	if ctx.cr[6].lt {
	pc = 0x82415738; continue 'dispatch;
	}
	// 8241576C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415770: 4811F99C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82415774: 48000000  b 0x82415774
	pc = 0x82415774; continue 'dispatch;
	// 82415778: 48000000  b 0x82415778
	pc = 0x82415778; continue 'dispatch;
	// 8241577C: 48000000  b 0x8241577c
	pc = 0x8241577C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415780 size=280
    let mut pc: u32 = 0x82415780;
    'dispatch: loop {
        match pc {
            0x82415780 => {
    //   block [0x82415780..0x82415898)
	// 82415780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415784: 4811F939  bl 0x825350bc
	ctx.lr = 0x82415788;
	sub_82535080(ctx, base);
	// 82415788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241578C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415790: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82415794: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 82415798: 38803040  li r4, 0x3040
	ctx.r[4].s64 = 12352;
	// 8241579C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824157A0: 48000D71  bl 0x82416510
	ctx.lr = 0x824157A4;
	sub_82416510(ctx, base);
	// 824157A4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824157A8: 40820008  bne 0x824157b0
	if !ctx.cr[0].eq {
	pc = 0x824157B0; continue 'dispatch;
	}
	// 824157AC: 48000000  b 0x824157ac
	pc = 0x824157AC; continue 'dispatch;
	// 824157B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824157B4: 38803040  li r4, 0x3040
	ctx.r[4].s64 = 12352;
	// 824157B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824157BC: 48000CDD  bl 0x82416498
	ctx.lr = 0x824157C0;
	sub_82416498(ctx, base);
	// 824157C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824157C4: 40820008  bne 0x824157cc
	if !ctx.cr[0].eq {
	pc = 0x824157CC; continue 'dispatch;
	}
	// 824157C8: 48000000  b 0x824157c8
	pc = 0x824157C8; continue 'dispatch;
	// 824157CC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824157D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824157D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824157D8: 40990040  ble cr6, 0x82415818
	if !ctx.cr[6].gt {
	pc = 0x82415818; continue 'dispatch;
	}
	// 824157DC: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824157E0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824157E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824157E8: 41820030  beq 0x82415818
	if ctx.cr[0].eq {
	pc = 0x82415818; continue 'dispatch;
	}
	// 824157EC: 554907BF  clrlwi. r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824157F0: 40820038  bne 0x82415828
	if !ctx.cr[0].eq {
	pc = 0x82415828; continue 'dispatch;
	}
	// 824157F4: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824157F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824157FC: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82415800: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415804: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415808: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8241580C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82415810: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415814: 4198FFC8  blt cr6, 0x824157dc
	if ctx.cr[6].lt {
	pc = 0x824157DC; continue 'dispatch;
	}
	// 82415818: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241581C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415820: 4082000C  bne 0x8241582c
	if !ctx.cr[0].eq {
	pc = 0x8241582C; continue 'dispatch;
	}
	// 82415824: 48000000  b 0x82415824
	pc = 0x82415824; continue 'dispatch;
	// 82415828: 48000000  b 0x82415828
	pc = 0x82415828; continue 'dispatch;
	// 8241582C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415830: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 82415834: 419A0008  beq cr6, 0x8241583c
	if ctx.cr[6].eq {
	pc = 0x8241583C; continue 'dispatch;
	}
	// 82415838: 48000000  b 0x82415838
	pc = 0x82415838; continue 'dispatch;
	// 8241583C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415840: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82415844: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415848: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 8241584C: 419A0034  beq cr6, 0x82415880
	if ctx.cr[6].eq {
	pc = 0x82415880; continue 'dispatch;
	}
	// 82415850: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415854: 2B0A8004  cmplwi cr6, r10, 0x8004
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32772 as u32, &mut ctx.xer);
	// 82415858: 419A0024  beq cr6, 0x8241587c
	if ctx.cr[6].eq {
	pc = 0x8241587C; continue 'dispatch;
	}
	// 8241585C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415860: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415864: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82415868: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8241586C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415870: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82415874: 4198FFDC  blt cr6, 0x82415850
	if ctx.cr[6].lt {
	pc = 0x82415850; continue 'dispatch;
	}
	// 82415878: 48000008  b 0x82415880
	pc = 0x82415880; continue 'dispatch;
	// 8241587C: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82415880: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82415884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415888: 409A0008  bne cr6, 0x82415890
	if !ctx.cr[6].eq {
	pc = 0x82415890; continue 'dispatch;
	}
	// 8241588C: 48000000  b 0x8241588c
	pc = 0x8241588C; continue 'dispatch;
	// 82415890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415894: 4811F878  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415898 size=24
    let mut pc: u32 = 0x82415898;
    'dispatch: loop {
        match pc {
            0x82415898 => {
    //   block [0x82415898..0x824158B0)
	// 82415898: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8241589C: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 824158A0: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 824158A4: 4098000C  bge cr6, 0x824158b0
	if !ctx.cr[6].lt {
		sub_824158B0(ctx, base);
		return;
	}
	// 824158A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824158AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824158B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824158B0 size=72
    let mut pc: u32 = 0x824158B0;
    'dispatch: loop {
        match pc {
            0x824158B0 => {
    //   block [0x824158B0..0x824158F8)
	// 824158B0: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158B4: 7F065040  cmplw cr6, r6, r10
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824158B8: 419A0084  beq cr6, 0x8241593c
	if ctx.cr[6].eq {
		sub_824158F8(ctx, base);
		return;
	}
	// 824158BC: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 824158C0: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824158C4: 409A0034  bne cr6, 0x824158f8
	if !ctx.cr[6].eq {
		sub_824158F8(ctx, base);
		return;
	}
	// 824158C8: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158CC: A1290000  lhz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158D0: 2B098004  cmplwi cr6, r9, 0x8004
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32772 as u32, &mut ctx.xer);
	// 824158D4: 419AFFD4  beq cr6, 0x824158a8
	if ctx.cr[6].eq {
		sub_82415898(ctx, base);
		return;
	}
	// 824158D8: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824158DC: 4099FFCC  ble cr6, 0x824158a8
	if !ctx.cr[6].gt {
		sub_82415898(ctx, base);
		return;
	}
	// 824158E0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824158E4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824158E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824158EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824158F0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824158F4: 48000044  b 0x82415938
	sub_824158F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824158F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824158F8 size=76
    let mut pc: u32 = 0x824158F8;
    'dispatch: loop {
        match pc {
            0x824158F8 => {
    //   block [0x824158F8..0x82415944)
	// 824158F8: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 824158FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82415900: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82415904: 419A0030  beq cr6, 0x82415934
	if ctx.cr[6].eq {
	pc = 0x82415934; continue 'dispatch;
	}
	// 82415908: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241590C: 2B098004  cmplwi cr6, r9, 0x8004
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32772 as u32, &mut ctx.xer);
	// 82415910: 419A001C  beq cr6, 0x8241592c
	if ctx.cr[6].eq {
	pc = 0x8241592C; continue 'dispatch;
	}
	// 82415914: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82415918: 40990014  ble cr6, 0x8241592c
	if !ctx.cr[6].gt {
	pc = 0x8241592C; continue 'dispatch;
	}
	// 8241591C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415920: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82415924: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82415928: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8241592C: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82415930: 4082FFD8  bne 0x82415908
	if !ctx.cr[0].eq {
	pc = 0x82415908; continue 'dispatch;
	}
	// 82415934: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82415938: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241593C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415948 size=80
    let mut pc: u32 = 0x82415948;
    'dispatch: loop {
        match pc {
            0x82415948 => {
    //   block [0x82415948..0x82415998)
	// 82415948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241594C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82415950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82415954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241595C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82415960: 48000DA9  bl 0x82416708
	ctx.lr = 0x82415964;
	sub_82416708(ctx, base);
	// 82415964: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82415968: 4BFFF7B1  bl 0x82415118
	ctx.lr = 0x8241596C;
	sub_82415118(ctx, base);
	// 8241596C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82415970: 4BFFF7A9  bl 0x82415118
	ctx.lr = 0x82415974;
	sub_82415118(ctx, base);
	// 82415974: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82415978: 4BFFF7A1  bl 0x82415118
	ctx.lr = 0x8241597C;
	sub_82415118(ctx, base);
	// 8241597C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82415980: 4BFFF799  bl 0x82415118
	ctx.lr = 0x82415984;
	sub_82415118(ctx, base);
	// 82415984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82415988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82415990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82415994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415998 size=16
    let mut pc: u32 = 0x82415998;
    'dispatch: loop {
        match pc {
            0x82415998 => {
    //   block [0x82415998..0x824159A8)
	// 82415998: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8241599C: 38A30084  addi r5, r3, 0x84
	ctx.r[5].s64 = ctx.r[3].s64 + 132;
	// 824159A0: 38830080  addi r4, r3, 0x80
	ctx.r[4].s64 = ctx.r[3].s64 + 128;
	// 824159A4: 4BFFFEF4  b 0x82415898
	sub_82415898(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824159A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824159A8 size=204
    let mut pc: u32 = 0x824159A8;
    'dispatch: loop {
        match pc {
            0x824159A8 => {
    //   block [0x824159A8..0x82415A74)
	// 824159A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824159AC: 4811F711  bl 0x825350bc
	ctx.lr = 0x824159B0;
	sub_82535080(ctx, base);
	// 824159B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824159B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824159B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824159BC: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 824159C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824159C4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824159C8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824159CC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 824159D0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 824159D4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 824159D8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 824159DC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 824159E0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 824159E4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 824159E8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 824159EC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 824159F0: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 824159F4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 824159F8: 48000B99  bl 0x82416590
	ctx.lr = 0x824159FC;
	sub_82416590(ctx, base);
	// 824159FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82415A00: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82415A04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82415A08: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82415A0C: 388BEC54  addi r4, r11, -0x13ac
	ctx.r[4].s64 = ctx.r[11].s64 + -5036;
	// 82415A10: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82415A14: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82415A18: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82415A1C: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82415A20: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82415A24: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82415A28: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82415A2C: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82415A30: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82415A34: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82415A38: 48000999  bl 0x824163d0
	ctx.lr = 0x82415A3C;
	sub_824163D0(ctx, base);
	// 82415A3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415A40: 40820008  bne 0x82415a48
	if !ctx.cr[0].eq {
	pc = 0x82415A48; continue 'dispatch;
	}
	// 82415A44: 48000000  b 0x82415a44
	pc = 0x82415A44; continue 'dispatch;
	// 82415A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A4C: 4BFFF64D  bl 0x82415098
	ctx.lr = 0x82415A50;
	sub_82415098(ctx, base);
	// 82415A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A54: 4BFFFB1D  bl 0x82415570
	ctx.lr = 0x82415A58;
	sub_82415570(ctx, base);
	// 82415A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A5C: 4BFFFC2D  bl 0x82415688
	ctx.lr = 0x82415A60;
	sub_82415688(ctx, base);
	// 82415A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A64: 4BFFFD1D  bl 0x82415780
	ctx.lr = 0x82415A68;
	sub_82415780(ctx, base);
	// 82415A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82415A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82415A70: 4811F69C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415A78 size=16
    let mut pc: u32 = 0x82415A78;
    'dispatch: loop {
        match pc {
            0x82415A78 => {
    //   block [0x82415A78..0x82415A88)
	// 82415A78: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82415A7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415A80: 419A0008  beq cr6, 0x82415a88
	if ctx.cr[6].eq {
		sub_82415A88(ctx, base);
		return;
	}
	// 82415A84: 48000000  b 0x82415a84
	pc = 0x82415A84; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415A88 size=8
    let mut pc: u32 = 0x82415A88;
    'dispatch: loop {
        match pc {
            0x82415A88 => {
    //   block [0x82415A88..0x82415A90)
	// 82415A88: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82415A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415A90 size=16
    let mut pc: u32 = 0x82415A90;
    'dispatch: loop {
        match pc {
            0x82415A90 => {
    //   block [0x82415A90..0x82415AA0)
	// 82415A90: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82415A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415A98: 419A0008  beq cr6, 0x82415aa0
	if ctx.cr[6].eq {
		sub_82415AA0(ctx, base);
		return;
	}
	// 82415A9C: 48000000  b 0x82415a9c
	pc = 0x82415A9C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AA0 size=8
    let mut pc: u32 = 0x82415AA0;
    'dispatch: loop {
        match pc {
            0x82415AA0 => {
    //   block [0x82415AA0..0x82415AA8)
	// 82415AA0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82415AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AA8 size=20
    let mut pc: u32 = 0x82415AA8;
    'dispatch: loop {
        match pc {
            0x82415AA8 => {
    //   block [0x82415AA8..0x82415ABC)
	// 82415AA8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415AB0: 419A000C  beq cr6, 0x82415abc
	if ctx.cr[6].eq {
		sub_82415ABC(ctx, base);
		return;
	}
	// 82415AB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415ABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415ABC size=8
    let mut pc: u32 = 0x82415ABC;
    'dispatch: loop {
        match pc {
            0x82415ABC => {
    //   block [0x82415ABC..0x82415AC4)
	// 82415ABC: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82415AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AC8 size=20
    let mut pc: u32 = 0x82415AC8;
    'dispatch: loop {
        match pc {
            0x82415AC8 => {
    //   block [0x82415AC8..0x82415ADC)
	// 82415AC8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415ACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415AD0: 419A000C  beq cr6, 0x82415adc
	if ctx.cr[6].eq {
		sub_82415ADC(ctx, base);
		return;
	}
	// 82415AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415ADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415ADC size=8
    let mut pc: u32 = 0x82415ADC;
    'dispatch: loop {
        match pc {
            0x82415ADC => {
    //   block [0x82415ADC..0x82415AE4)
	// 82415ADC: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AE8 size=12
    let mut pc: u32 = 0x82415AE8;
    'dispatch: loop {
        match pc {
            0x82415AE8 => {
    //   block [0x82415AE8..0x82415AF4)
	// 82415AE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82415AEC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82415AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415AF8 size=12
    let mut pc: u32 = 0x82415AF8;
    'dispatch: loop {
        match pc {
            0x82415AF8 => {
    //   block [0x82415AF8..0x82415B04)
	// 82415AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415AFC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82415B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415B08 size=16
    let mut pc: u32 = 0x82415B08;
    'dispatch: loop {
        match pc {
            0x82415B08 => {
    //   block [0x82415B08..0x82415B18)
	// 82415B08: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415B0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82415B10: 409A0008  bne cr6, 0x82415b18
	if !ctx.cr[6].eq {
		sub_82415B18(ctx, base);
		return;
	}
	// 82415B14: 48000000  b 0x82415b14
	pc = 0x82415B14; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415B18 size=16
    let mut pc: u32 = 0x82415B18;
    'dispatch: loop {
        match pc {
            0x82415B18 => {
    //   block [0x82415B18..0x82415B28)
	// 82415B18: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415B1C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415B20: 41980008  blt cr6, 0x82415b28
	if ctx.cr[6].lt {
		sub_82415B28(ctx, base);
		return;
	}
	// 82415B24: 48000000  b 0x82415b24
	pc = 0x82415B24; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415B28 size=28
    let mut pc: u32 = 0x82415B28;
    'dispatch: loop {
        match pc {
            0x82415B28 => {
    //   block [0x82415B28..0x82415B44)
	// 82415B28: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415B2C: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415B30: 7CAB512E  stwx r5, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u32) };
	// 82415B34: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415B38: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82415B3C: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82415B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415B48 size=440
    let mut pc: u32 = 0x82415B48;
    'dispatch: loop {
        match pc {
            0x82415B48 => {
    //   block [0x82415B48..0x82415D00)
	// 82415B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415B4C: 4811F565  bl 0x825350b0
	ctx.lr = 0x82415B50;
	sub_82535080(ctx, base);
	// 82415B50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415B58: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82415B5C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415B60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415B64: 409A0188  bne cr6, 0x82415cec
	if !ctx.cr[6].eq {
	pc = 0x82415CEC; continue 'dispatch;
	}
	// 82415B68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415B6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415B70: 40820008  bne 0x82415b78
	if !ctx.cr[0].eq {
	pc = 0x82415B78; continue 'dispatch;
	}
	// 82415B74: 48000000  b 0x82415b74
	pc = 0x82415B74; continue 'dispatch;
	// 82415B78: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82415B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415B80: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82415B84: 419A00B0  beq cr6, 0x82415c34
	if ctx.cr[6].eq {
	pc = 0x82415C34; continue 'dispatch;
	}
	// 82415B88: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82415B8C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82415B90: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415B94: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82415B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415B9C: 409A0080  bne cr6, 0x82415c1c
	if !ctx.cr[6].eq {
	pc = 0x82415C1C; continue 'dispatch;
	}
	// 82415BA0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415BA4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82415BA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415BAC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82415BB0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82415BB4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82415BB8: FBC10058  std r30, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 82415BBC: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82415BC0: 7D0BE82E  lwzx r8, r11, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82415BC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415BC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82415BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82415BD0: 4E800421  bctrl
	ctx.lr = 0x82415BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82415BD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415BD8: 41820044  beq 0x82415c1c
	if ctx.cr[0].eq {
	pc = 0x82415C1C; continue 'dispatch;
	}
	// 82415BDC: 4BFFABE5  bl 0x824107c0
	ctx.lr = 0x82415BE0;
	sub_824107C0(ctx, base);
	// 82415BE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415BE4: 388000A8  li r4, 0xa8
	ctx.r[4].s64 = 168;
	// 82415BE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82415BF0: 4E800421  bctrl
	ctx.lr = 0x82415BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82415BF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415BF8: 41820018  beq 0x82415c10
	if ctx.cr[0].eq {
	pc = 0x82415C10; continue 'dispatch;
	}
	// 82415BFC: E8C10058  ld r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82415C00: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82415C04: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415C08: 4BFFD991  bl 0x82413598
	ctx.lr = 0x82415C0C;
	sub_82413598(ctx, base);
	// 82415C0C: 48000008  b 0x82415c14
	pc = 0x82415C14; continue 'dispatch;
	// 82415C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82415C14: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C18: 7C6BE12E  stwx r3, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 82415C1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C20: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82415C24: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82415C28: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82415C2C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415C30: 4198FF60  blt cr6, 0x82415b90
	if ctx.cr[6].lt {
	pc = 0x82415B90; continue 'dispatch;
	}
	// 82415C34: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C38: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82415C3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82415C40: 40990028  ble cr6, 0x82415c68
	if !ctx.cr[6].gt {
	pc = 0x82415C68; continue 'dispatch;
	}
	// 82415C44: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C48: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415C4C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82415C50: 419A00A8  beq cr6, 0x82415cf8
	if ctx.cr[6].eq {
	pc = 0x82415CF8; continue 'dispatch;
	}
	// 82415C54: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415C5C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82415C60: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415C64: 4198FFE4  blt cr6, 0x82415c48
	if ctx.cr[6].lt {
	pc = 0x82415C48; continue 'dispatch;
	}
	// 82415C68: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C6C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415C70: 4BFFD491  bl 0x82413100
	ctx.lr = 0x82415C74;
	sub_82413100(ctx, base);
	// 82415C74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415C78: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82415C7C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82415C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415C84: 40990034  ble cr6, 0x82415cb8
	if !ctx.cr[6].gt {
	pc = 0x82415CB8; continue 'dispatch;
	}
	// 82415C88: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82415C8C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415C90: 7C6BE02E  lwzx r3, r11, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82415C94: 4BFFD475  bl 0x82413108
	ctx.lr = 0x82415C98;
	sub_82413108(ctx, base);
	// 82415C98: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415C9C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82415CA0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82415CA4: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82415CA8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82415CAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415CB0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415CB4: 4198FFD8  blt cr6, 0x82415c8c
	if ctx.cr[6].lt {
	pc = 0x82415C8C; continue 'dispatch;
	}
	// 82415CB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415CBC: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82415CC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415CC4: 40990028  ble cr6, 0x82415cec
	if !ctx.cr[6].gt {
	pc = 0x82415CEC; continue 'dispatch;
	}
	// 82415CC8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415CCC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82415CD0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82415CD4: 4BFFD235  bl 0x82412f08
	ctx.lr = 0x82415CD8;
	sub_82412F08(ctx, base);
	// 82415CD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415CDC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82415CE0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82415CE4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415CE8: 4198FFE0  blt cr6, 0x82415cc8
	if ctx.cr[6].lt {
	pc = 0x82415CC8; continue 'dispatch;
	}
	// 82415CEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415CF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82415CF4: 4811F40C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82415CF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415CFC: 4BFFFFF4  b 0x82415cf0
	pc = 0x82415CF0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415D00 size=132
    let mut pc: u32 = 0x82415D00;
    'dispatch: loop {
        match pc {
            0x82415D00 => {
    //   block [0x82415D00..0x82415D84)
	// 82415D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415D04: 4811F3B5  bl 0x825350b8
	ctx.lr = 0x82415D08;
	sub_82535080(ctx, base);
	// 82415D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415D0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82415D10: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415D18: 409A0064  bne cr6, 0x82415d7c
	if !ctx.cr[6].eq {
	pc = 0x82415D7C; continue 'dispatch;
	}
	// 82415D1C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415D20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415D24: 40820008  bne 0x82415d2c
	if !ctx.cr[0].eq {
	pc = 0x82415D2C; continue 'dispatch;
	}
	// 82415D28: 48000000  b 0x82415d28
	pc = 0x82415D28; continue 'dispatch;
	// 82415D2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82415D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415D34: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82415D38: 419A003C  beq cr6, 0x82415d74
	if ctx.cr[6].eq {
	pc = 0x82415D74; continue 'dispatch;
	}
	// 82415D3C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82415D40: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415D44: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82415D48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415D4C: 4182000C  beq 0x82415d58
	if ctx.cr[0].eq {
	pc = 0x82415D58; continue 'dispatch;
	}
	// 82415D50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82415D54: 4BFFC1ED  bl 0x82411f40
	ctx.lr = 0x82415D58;
	sub_82411F40(ctx, base);
	// 82415D58: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415D5C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82415D60: 7F8BF92E  stwx r28, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 82415D64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82415D68: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415D6C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415D70: 4198FFD0  blt cr6, 0x82415d40
	if ctx.cr[6].lt {
	pc = 0x82415D40; continue 'dispatch;
	}
	// 82415D74: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82415D78: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82415D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82415D80: 4811F388  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415D88 size=20
    let mut pc: u32 = 0x82415D88;
    'dispatch: loop {
        match pc {
            0x82415D88 => {
    //   block [0x82415D88..0x82415D9C)
	// 82415D88: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415D8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415D90: 419A000C  beq cr6, 0x82415d9c
	if ctx.cr[6].eq {
		sub_82415D9C(ctx, base);
		return;
	}
	// 82415D94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415D9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415D9C size=20
    let mut pc: u32 = 0x82415D9C;
    'dispatch: loop {
        match pc {
            0x82415D9C => {
    //   block [0x82415D9C..0x82415DB0)
	// 82415D9C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82415DA0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415DA4: 4098000C  bge cr6, 0x82415db0
	if !ctx.cr[6].lt {
		sub_82415DB0(ctx, base);
		return;
	}
	// 82415DA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82415DB0 size=20
    let mut pc: u32 = 0x82415DB0;
    'dispatch: loop {
        match pc {
            0x82415DB0 => {
    //   block [0x82415DB0..0x82415DC4)
	// 82415DB0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415DB4: 7D645810  subfc r11, r4, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[4].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82415DB8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82415DBC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82415DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82415DC8 size=180
    let mut pc: u32 = 0x82415DC8;
    'dispatch: loop {
        match pc {
            0x82415DC8 => {
    //   block [0x82415DC8..0x82415E7C)
	// 82415DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82415DCC: 4811F2E1  bl 0x825350ac
	ctx.lr = 0x82415DD0;
	sub_82535080(ctx, base);
	// 82415DD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82415DD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82415DD8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82415DDC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82415DE0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82415DE4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415DE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415DEC: 409A001C  bne cr6, 0x82415e08
	if !ctx.cr[6].eq {
	pc = 0x82415E08; continue 'dispatch;
	}
	// 82415DF0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82415DF4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415DF8: 41980010  blt cr6, 0x82415e08
	if ctx.cr[6].lt {
	pc = 0x82415E08; continue 'dispatch;
	}
	// 82415DFC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82415E00: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82415E04: 40980010  bge cr6, 0x82415e14
	if !ctx.cr[6].lt {
	pc = 0x82415E14; continue 'dispatch;
	}
	// 82415E08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415E0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82415E10: 4811F2EC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 82415E14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415E18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82415E1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82415E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415E24: 40990044  ble cr6, 0x82415e68
	if !ctx.cr[6].gt {
	pc = 0x82415E68; continue 'dispatch;
	}
	// 82415E28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82415E2C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415E30: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82415E34: 4BFFD2D5  bl 0x82413108
	ctx.lr = 0x82415E38;
	sub_82413108(ctx, base);
	// 82415E38: 7D63EA14  add r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82415E3C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415E40: 4099002C  ble cr6, 0x82415e6c
	if !ctx.cr[6].gt {
	pc = 0x82415E6C; continue 'dispatch;
	}
	// 82415E44: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415E48: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82415E4C: 4BFFD2BD  bl 0x82413108
	ctx.lr = 0x82415E50;
	sub_82413108(ctx, base);
	// 82415E50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415E54: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82415E58: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82415E5C: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82415E60: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82415E64: 4198FFC8  blt cr6, 0x82415e2c
	if ctx.cr[6].lt {
	pc = 0x82415E2C; continue 'dispatch;
	}
	// 82415E68: 48000000  b 0x82415e68
	pc = 0x82415E68; continue 'dispatch;
	// 82415E6C: 93B90000  stw r29, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82415E70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415E74: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82415E78: 4BFFFF94  b 0x82415e0c
	pc = 0x82415E0C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415E80 size=120
    let mut pc: u32 = 0x82415E80;
    'dispatch: loop {
        match pc {
            0x82415E80 => {
    //   block [0x82415E80..0x82415EF8)
	// 82415E80: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82415E84: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415E88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82415E8C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415E90: 41820050  beq 0x82415ee0
	if ctx.cr[0].eq {
	pc = 0x82415EE0; continue 'dispatch;
	}
	// 82415E94: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415E98: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 82415E9C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415EA0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82415EA4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415EA8: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415EAC: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82415EB0: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82415EB4: 41820014  beq 0x82415ec8
	if ctx.cr[0].eq {
	pc = 0x82415EC8; continue 'dispatch;
	}
	// 82415EB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415EBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82415EC0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82415EC4: 419AFFE0  beq cr6, 0x82415ea4
	if ctx.cr[6].eq {
	pc = 0x82415EA4; continue 'dispatch;
	}
	// 82415EC8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82415ECC: 41820020  beq 0x82415eec
	if ctx.cr[0].eq {
	pc = 0x82415EEC; continue 'dispatch;
	}
	// 82415ED0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82415ED4: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 82415ED8: 7F06F840  cmplw cr6, r6, r31
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82415EDC: 4198FFC0  blt cr6, 0x82415e9c
	if ctx.cr[6].lt {
	pc = 0x82415E9C; continue 'dispatch;
	}
	// 82415EE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415EE4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82415EE8: 4E800020  blr
	return;
	// 82415EEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82415EF0: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82415EF4: 4BFFFFF0  b 0x82415ee4
	pc = 0x82415EE4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415EF8 size=20
    let mut pc: u32 = 0x82415EF8;
    'dispatch: loop {
        match pc {
            0x82415EF8 => {
    //   block [0x82415EF8..0x82415F0C)
	// 82415EF8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415F00: 419A000C  beq cr6, 0x82415f0c
	if ctx.cr[6].eq {
		sub_82415F0C(ctx, base);
		return;
	}
	// 82415F04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F0C size=52
    let mut pc: u32 = 0x82415F0C;
    'dispatch: loop {
        match pc {
            0x82415F0C => {
    //   block [0x82415F0C..0x82415F40)
	// 82415F0C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415F14: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415F18: 4182FFEC  beq 0x82415f04
	if ctx.cr[0].eq {
		sub_82415EF8(ctx, base);
		return;
	}
	// 82415F1C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415F20: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415F24: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415F28: 419A0018  beq cr6, 0x82415f40
	if ctx.cr[6].eq {
		sub_82415F40(ctx, base);
		return;
	}
	// 82415F2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415F30: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82415F34: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415F38: 4198FFE8  blt cr6, 0x82415f20
	if ctx.cr[6].lt {
	pc = 0x82415F20; continue 'dispatch;
	}
	// 82415F3C: 4BFFFFC8  b 0x82415f04
	sub_82415EF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F40 size=20
    let mut pc: u32 = 0x82415F40;
    'dispatch: loop {
        match pc {
            0x82415F40 => {
    //   block [0x82415F40..0x82415F54)
	// 82415F40: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415F44: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415F48: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82415F4C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82415F50: 4BFFCF98  b 0x82412ee8
	sub_82412EE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F58 size=20
    let mut pc: u32 = 0x82415F58;
    'dispatch: loop {
        match pc {
            0x82415F58 => {
    //   block [0x82415F58..0x82415F6C)
	// 82415F58: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415F60: 419A000C  beq cr6, 0x82415f6c
	if ctx.cr[6].eq {
		sub_82415F6C(ctx, base);
		return;
	}
	// 82415F64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415F6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415F6C size=52
    let mut pc: u32 = 0x82415F6C;
    'dispatch: loop {
        match pc {
            0x82415F6C => {
    //   block [0x82415F6C..0x82415FA0)
	// 82415F6C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415F74: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415F78: 4182FFEC  beq 0x82415f64
	if ctx.cr[0].eq {
		sub_82415F58(ctx, base);
		return;
	}
	// 82415F7C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415F80: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415F84: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415F88: 419A0018  beq cr6, 0x82415fa0
	if ctx.cr[6].eq {
		sub_82415FA0(ctx, base);
		return;
	}
	// 82415F8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415F90: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82415F94: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415F98: 4198FFE8  blt cr6, 0x82415f80
	if ctx.cr[6].lt {
	pc = 0x82415F80; continue 'dispatch;
	}
	// 82415F9C: 4BFFFFC8  b 0x82415f64
	sub_82415F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415FA0 size=20
    let mut pc: u32 = 0x82415FA0;
    'dispatch: loop {
        match pc {
            0x82415FA0 => {
    //   block [0x82415FA0..0x82415FB4)
	// 82415FA0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82415FA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82415FA8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82415FAC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82415FB0: 4BFFCF40  b 0x82412ef0
	sub_82412EF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415FB8 size=20
    let mut pc: u32 = 0x82415FB8;
    'dispatch: loop {
        match pc {
            0x82415FB8 => {
    //   block [0x82415FB8..0x82415FCC)
	// 82415FB8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82415FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82415FC0: 419A000C  beq cr6, 0x82415fcc
	if ctx.cr[6].eq {
		sub_82415FCC(ctx, base);
		return;
	}
	// 82415FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82415FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82415FCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82415FCC size=52
    let mut pc: u32 = 0x82415FCC;
    'dispatch: loop {
        match pc {
            0x82415FCC => {
    //   block [0x82415FCC..0x82416000)
	// 82415FCC: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82415FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82415FD4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82415FD8: 4182FFEC  beq 0x82415fc4
	if ctx.cr[0].eq {
		sub_82415FB8(ctx, base);
		return;
	}
	// 82415FDC: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82415FE0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82415FE4: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82415FE8: 419A0018  beq cr6, 0x82416000
	if ctx.cr[6].eq {
		sub_82416000(ctx, base);
		return;
	}
	// 82415FEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82415FF0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82415FF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82415FF8: 4198FFE8  blt cr6, 0x82415fe0
	if ctx.cr[6].lt {
	pc = 0x82415FE0; continue 'dispatch;
	}
	// 82415FFC: 4BFFFFC8  b 0x82415fc4
	sub_82415FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416000 size=20
    let mut pc: u32 = 0x82416000;
    'dispatch: loop {
        match pc {
            0x82416000 => {
    //   block [0x82416000..0x82416014)
	// 82416000: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416004: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416008: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8241600C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82416010: 4BFFCEE8  b 0x82412ef8
	sub_82412EF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416018 size=20
    let mut pc: u32 = 0x82416018;
    'dispatch: loop {
        match pc {
            0x82416018 => {
    //   block [0x82416018..0x8241602C)
	// 82416018: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8241601C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416020: 419A000C  beq cr6, 0x8241602c
	if ctx.cr[6].eq {
		sub_8241602C(ctx, base);
		return;
	}
	// 82416024: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241602C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241602C size=52
    let mut pc: u32 = 0x8241602C;
    'dispatch: loop {
        match pc {
            0x8241602C => {
    //   block [0x8241602C..0x82416060)
	// 8241602C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416034: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416038: 4182FFEC  beq 0x82416024
	if ctx.cr[0].eq {
		sub_82416018(ctx, base);
		return;
	}
	// 8241603C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416040: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416044: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82416048: 419A0018  beq cr6, 0x82416060
	if ctx.cr[6].eq {
		sub_82416060(ctx, base);
		return;
	}
	// 8241604C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82416050: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82416054: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82416058: 4198FFE8  blt cr6, 0x82416040
	if ctx.cr[6].lt {
	pc = 0x82416040; continue 'dispatch;
	}
	// 8241605C: 4BFFFFC8  b 0x82416024
	sub_82416018(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416060 size=20
    let mut pc: u32 = 0x82416060;
    'dispatch: loop {
        match pc {
            0x82416060 => {
    //   block [0x82416060..0x82416074)
	// 82416060: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416064: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416068: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8241606C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82416070: 4BFFCE90  b 0x82412f00
	sub_82412F00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416078 size=44
    let mut pc: u32 = 0x82416078;
    'dispatch: loop {
        match pc {
            0x82416078 => {
    //   block [0x82416078..0x824160A4)
	// 82416078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241607C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82416080: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82416084: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82416088: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8241608C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82416090: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82416094: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82416098: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8241609C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824160A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824160A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824160A8 size=140
    let mut pc: u32 = 0x824160A8;
    'dispatch: loop {
        match pc {
            0x824160A8 => {
    //   block [0x824160A8..0x82416134)
	// 824160A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824160AC: 4811F00D  bl 0x825350b8
	ctx.lr = 0x824160B0;
	sub_82535080(ctx, base);
	// 824160B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824160B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824160B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824160BC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 824160C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824160C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824160C8: 40990048  ble cr6, 0x82416110
	if !ctx.cr[6].gt {
	pc = 0x82416110; continue 'dispatch;
	}
	// 824160CC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 824160D0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824160D4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824160D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824160DC: 419A0020  beq cr6, 0x824160fc
	if ctx.cr[6].eq {
	pc = 0x824160FC; continue 'dispatch;
	}
	// 824160E0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824160E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824160E8: 4182000C  beq 0x824160f4
	if ctx.cr[0].eq {
	pc = 0x824160F4; continue 'dispatch;
	}
	// 824160EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824160F0: 4BFFBE51  bl 0x82411f40
	ctx.lr = 0x824160F4;
	sub_82411F40(ctx, base);
	// 824160F4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824160F8: 7F8BF92E  stwx r28, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u32) };
	// 824160FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416100: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82416104: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82416108: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241610C: 4198FFC4  blt cr6, 0x824160d0
	if ctx.cr[6].lt {
	pc = 0x824160D0; continue 'dispatch;
	}
	// 82416110: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82416114: 4BFFF005  bl 0x82415118
	ctx.lr = 0x82416118;
	sub_82415118(ctx, base);
	// 82416118: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241611C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416120: 4182000C  beq 0x8241612c
	if ctx.cr[0].eq {
	pc = 0x8241612C; continue 'dispatch;
	}
	// 82416124: 4811CA95  bl 0x82532bb8
	ctx.lr = 0x82416128;
	sub_82532BB8(ctx, base);
	// 82416128: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8241612C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82416130: 4811EFD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416138 size=256
    let mut pc: u32 = 0x82416138;
    'dispatch: loop {
        match pc {
            0x82416138 => {
    //   block [0x82416138..0x82416238)
	// 82416138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241613C: 4811EF79  bl 0x825350b4
	ctx.lr = 0x82416140;
	sub_82535080(ctx, base);
	// 82416140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416144: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82416148: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241614C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416150: 419A0008  beq cr6, 0x82416158
	if ctx.cr[6].eq {
	pc = 0x82416158; continue 'dispatch;
	}
	// 82416154: 48000000  b 0x82416154
	pc = 0x82416154; continue 'dispatch;
	// 82416158: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 8241615C: 909C0004  stw r4, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82416160: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82416164: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82416168: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8241616C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82416170: 40990008  ble cr6, 0x82416178
	if !ctx.cr[6].gt {
	pc = 0x82416178; continue 'dispatch;
	}
	// 82416174: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82416178: 4BFEEC51  bl 0x82404dc8
	ctx.lr = 0x8241617C;
	sub_82404DC8(ctx, base);
	// 8241617C: 3BFC0010  addi r31, r28, 0x10
	ctx.r[31].s64 = ctx.r[28].s64 + 16;
	// 82416180: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82416184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416188: 4BFFDE91  bl 0x82414018
	ctx.lr = 0x8241618C;
	sub_82414018(ctx, base);
	// 8241618C: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416190: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82416194: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82416198: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8241619C: 40990024  ble cr6, 0x824161c0
	if !ctx.cr[6].gt {
	pc = 0x824161C0; continue 'dispatch;
	}
	// 824161A0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 824161A4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824161A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824161AC: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 824161B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824161B4: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824161B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824161BC: 4198FFE8  blt cr6, 0x824161a4
	if ctx.cr[6].lt {
	pc = 0x824161A4; continue 'dispatch;
	}
	// 824161C0: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 824161C4: 83BC0004  lwz r29, 4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824161C8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 824161CC: 57BF1838  slwi r31, r29, 3
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824161D0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824161D4: 40990008  ble cr6, 0x824161dc
	if !ctx.cr[6].gt {
	pc = 0x824161DC; continue 'dispatch;
	}
	// 824161D8: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 824161DC: 4BFFA5E5  bl 0x824107c0
	ctx.lr = 0x824161E0;
	sub_824107C0(ctx, base);
	// 824161E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824161E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824161E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824161EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824161F0: 4E800421  bctrl
	ctx.lr = 0x824161F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824161F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824161F8: 41820028  beq 0x82416220
	if ctx.cr[0].eq {
	pc = 0x82416220; continue 'dispatch;
	}
	// 824161FC: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82416200: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82416204: 41800018  blt 0x8241621c
	if ctx.cr[0].lt {
	pc = 0x8241621C; continue 'dispatch;
	}
	// 82416208: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8241620C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82416210: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82416214: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82416218: 4080FFF0  bge 0x82416208
	if !ctx.cr[0].lt {
	pc = 0x82416208; continue 'dispatch;
	}
	// 8241621C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82416220: 3BFC0014  addi r31, r28, 0x14
	ctx.r[31].s64 = ctx.r[28].s64 + 20;
	// 82416224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416228: 4BFFEEF1  bl 0x82415118
	ctx.lr = 0x8241622C;
	sub_82415118(ctx, base);
	// 8241622C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82416230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82416234: 4811EED0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82416238 size=188
    let mut pc: u32 = 0x82416238;
    'dispatch: loop {
        match pc {
            0x82416238 => {
    //   block [0x82416238..0x824162F4)
	// 82416238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82416244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416248: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8241624C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82416250: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416258: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241625C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82416260: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82416264: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82416268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8241626C: 409A0068  bne cr6, 0x824162d4
	if !ctx.cr[6].eq {
	pc = 0x824162D4; continue 'dispatch;
	}
	// 82416270: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82416274: FC00FE5E  fctidz f0, f31
	ctx.f[0].s64 = if ctx.f[31].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[31].f64.trunc() as i64 };
	// 82416278: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8241627C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82416280: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82416284: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82416288: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8241628C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82416290: 4BFFFB39  bl 0x82415dc8
	ctx.lr = 0x82416294;
	sub_82415DC8(ctx, base);
	// 82416294: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416298: 4182003C  beq 0x824162d4
	if ctx.cr[0].eq {
	pc = 0x824162D4; continue 'dispatch;
	}
	// 8241629C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824162A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824162A4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824162A8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824162AC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824162B0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824162B4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824162B8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 824162BC: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 824162C0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824162C4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824162C8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824162CC: EC3F0028  fsubs f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 824162D0: 4BFFCE41  bl 0x82413110
	ctx.lr = 0x824162D4;
	sub_82413110(ctx, base);
	// 824162D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824162D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824162DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824162E0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 824162E4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824162E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824162EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824162F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824162F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824162F8 size=56
    let mut pc: u32 = 0x824162F8;
    'dispatch: loop {
        match pc {
            0x824162F8 => {
    //   block [0x824162F8..0x82416330)
	// 824162F8: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824162FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416300: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416304: 41820024  beq 0x82416328
	if ctx.cr[0].eq {
	pc = 0x82416328; continue 'dispatch;
	}
	// 82416308: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241630C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416310: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82416314: 419A001C  beq cr6, 0x82416330
	if ctx.cr[6].eq {
		sub_82416330(ctx, base);
		return;
	}
	// 82416318: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241631C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82416320: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82416324: 4198FFE8  blt cr6, 0x8241630c
	if ctx.cr[6].lt {
	pc = 0x8241630C; continue 'dispatch;
	}
	// 82416328: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241632C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416330 size=8
    let mut pc: u32 = 0x82416330;
    'dispatch: loop {
        match pc {
            0x82416330 => {
    //   block [0x82416330..0x82416338)
	// 82416330: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82416334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416338 size=36
    let mut pc: u32 = 0x82416338;
    'dispatch: loop {
        match pc {
            0x82416338 => {
    //   block [0x82416338..0x8241635C)
	// 82416338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416344: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82416348: 4BFFFB39  bl 0x82415e80
	ctx.lr = 0x8241634C;
	sub_82415E80(ctx, base);
	// 8241634C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82416360 size=108
    let mut pc: u32 = 0x82416360;
    'dispatch: loop {
        match pc {
            0x82416360 => {
    //   block [0x82416360..0x824163CC)
	// 82416360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241636C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416370: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82416374: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82416378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241637C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82416380: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82416384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416388: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8241638C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82416390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82416394: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82416398: 4BFFFA31  bl 0x82415dc8
	ctx.lr = 0x8241639C;
	sub_82415DC8(ctx, base);
	// 8241639C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824163A0: 41820018  beq 0x824163b8
	if ctx.cr[0].eq {
	pc = 0x824163B8; continue 'dispatch;
	}
	// 824163A4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824163A8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824163AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824163B0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824163B4: 4BFFBCBD  bl 0x82412070
	ctx.lr = 0x824163B8;
	sub_82412070(ctx, base);
	// 824163B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824163BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824163C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824163C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824163C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824163D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824163D0 size=16
    let mut pc: u32 = 0x824163D0;
    'dispatch: loop {
        match pc {
            0x824163D0 => {
    //   block [0x824163D0..0x824163E0)
	// 824163D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824163D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824163D8: 409A0008  bne cr6, 0x824163e0
	if !ctx.cr[6].eq {
		sub_824163E0(ctx, base);
		return;
	}
	// 824163DC: 48000000  b 0x824163dc
	pc = 0x824163DC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824163E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824163E0 size=56
    let mut pc: u32 = 0x824163E0;
    'dispatch: loop {
        match pc {
            0x824163E0 => {
    //   block [0x824163E0..0x82416418)
	// 824163E0: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824163E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824163E8: 7D4958AE  lbzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824163EC: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824163F0: 41820020  beq 0x82416410
	if ctx.cr[0].eq {
	pc = 0x82416410; continue 'dispatch;
	}
	// 824163F4: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824163F8: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 824163FC: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82416400: 409A0018  bne cr6, 0x82416418
	if !ctx.cr[6].eq {
		sub_82416418(ctx, base);
		return;
	}
	// 82416404: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82416408: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8241640C: 4198FFDC  blt cr6, 0x824163e8
	if ctx.cr[6].lt {
	pc = 0x824163E8; continue 'dispatch;
	}
	// 82416410: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82416414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416418 size=8
    let mut pc: u32 = 0x82416418;
    'dispatch: loop {
        match pc {
            0x82416418 => {
    //   block [0x82416418..0x82416420)
	// 82416418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416420 size=16
    let mut pc: u32 = 0x82416420;
    'dispatch: loop {
        match pc {
            0x82416420 => {
    //   block [0x82416420..0x82416430)
	// 82416420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416428: 409A0008  bne cr6, 0x82416430
	if !ctx.cr[6].eq {
		sub_82416430(ctx, base);
		return;
	}
	// 8241642C: 48000000  b 0x8241642c
	pc = 0x8241642C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416430 size=32
    let mut pc: u32 = 0x82416430;
    'dispatch: loop {
        match pc {
            0x82416430 => {
    //   block [0x82416430..0x82416450)
	// 82416430: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416434: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416438: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8241643C: 419A0014  beq cr6, 0x82416450
	if ctx.cr[6].eq {
		sub_82416450(ctx, base);
		return;
	}
	// 82416440: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82416444: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82416448: 5563E7BC  rlwinm r3, r11, 0x1c, 0x1e, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8241644C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416450 size=8
    let mut pc: u32 = 0x82416450;
    'dispatch: loop {
        match pc {
            0x82416450 => {
    //   block [0x82416450..0x82416458)
	// 82416450: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82416454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416458 size=16
    let mut pc: u32 = 0x82416458;
    'dispatch: loop {
        match pc {
            0x82416458 => {
    //   block [0x82416458..0x82416468)
	// 82416458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241645C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416460: 409A0008  bne cr6, 0x82416468
	if !ctx.cr[6].eq {
		sub_82416468(ctx, base);
		return;
	}
	// 82416464: 48000000  b 0x82416464
	pc = 0x82416464; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416468 size=12
    let mut pc: u32 = 0x82416468;
    'dispatch: loop {
        match pc {
            0x82416468 => {
    //   block [0x82416468..0x82416474)
	// 82416468: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241646C: 886B000A  lbz r3, 0xa(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82416470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416478 size=16
    let mut pc: u32 = 0x82416478;
    'dispatch: loop {
        match pc {
            0x82416478 => {
    //   block [0x82416478..0x82416488)
	// 82416478: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241647C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416480: 409A0008  bne cr6, 0x82416488
	if !ctx.cr[6].eq {
		sub_82416488(ctx, base);
		return;
	}
	// 82416484: 48000000  b 0x82416484
	pc = 0x82416484; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416488 size=12
    let mut pc: u32 = 0x82416488;
    'dispatch: loop {
        match pc {
            0x82416488 => {
    //   block [0x82416488..0x82416494)
	// 82416488: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8241648C: 886B000B  lbz r3, 0xb(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82416490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416498 size=16
    let mut pc: u32 = 0x82416498;
    'dispatch: loop {
        match pc {
            0x82416498 => {
    //   block [0x82416498..0x824164A8)
	// 82416498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241649C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824164A0: 409A0008  bne cr6, 0x824164a8
	if !ctx.cr[6].eq {
		sub_824164A8(ctx, base);
		return;
	}
	// 824164A4: 48000000  b 0x824164a4
	pc = 0x824164A4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824164A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824164A8 size=12
    let mut pc: u32 = 0x824164A8;
    'dispatch: loop {
        match pc {
            0x824164A8 => {
    //   block [0x824164A8..0x824164B4)
	// 824164A8: 5486043F  clrlwi. r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824164AC: 40820008  bne 0x824164b4
	if !ctx.cr[0].eq {
		sub_824164B4(ctx, base);
		return;
	}
	// 824164B0: 48000000  b 0x824164b0
	pc = 0x824164B0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824164B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824164B4 size=80
    let mut pc: u32 = 0x824164B4;
    'dispatch: loop {
        match pc {
            0x824164B4 => {
    //   block [0x824164B4..0x82416504)
	// 824164B4: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824164B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824164BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824164C0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824164C4: 41820038  beq 0x824164fc
	if ctx.cr[0].eq {
	pc = 0x824164FC; continue 'dispatch;
	}
	// 824164C8: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824164CC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 824164D0: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824164D4: A0840000  lhz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824164D8: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 824164DC: 409A0010  bne cr6, 0x824164ec
	if !ctx.cr[6].eq {
	pc = 0x824164EC; continue 'dispatch;
	}
	// 824164E0: 7F054040  cmplw cr6, r5, r8
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824164E4: 419A0020  beq cr6, 0x82416504
	if ctx.cr[6].eq {
		sub_82416504(ctx, base);
		return;
	}
	// 824164E8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 824164EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824164F0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824164F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824164F8: 4198FFD8  blt cr6, 0x824164d0
	if ctx.cr[6].lt {
	pc = 0x824164D0; continue 'dispatch;
	}
	// 824164FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416504 size=12
    let mut pc: u32 = 0x82416504;
    'dispatch: loop {
        match pc {
            0x82416504 => {
    //   block [0x82416504..0x82416510)
	// 82416504: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416508: 7C6B382E  lwzx r3, r11, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8241650C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416510 size=16
    let mut pc: u32 = 0x82416510;
    'dispatch: loop {
        match pc {
            0x82416510 => {
    //   block [0x82416510..0x82416520)
	// 82416510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416518: 409A0008  bne cr6, 0x82416520
	if !ctx.cr[6].eq {
		sub_82416520(ctx, base);
		return;
	}
	// 8241651C: 48000000  b 0x8241651c
	pc = 0x8241651C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416520 size=12
    let mut pc: u32 = 0x82416520;
    'dispatch: loop {
        match pc {
            0x82416520 => {
    //   block [0x82416520..0x8241652C)
	// 82416520: 5486043F  clrlwi. r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82416524: 40820008  bne 0x8241652c
	if !ctx.cr[0].eq {
		sub_8241652C(ctx, base);
		return;
	}
	// 82416528: 48000000  b 0x82416528
	pc = 0x82416528; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241652C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241652C size=80
    let mut pc: u32 = 0x8241652C;
    'dispatch: loop {
        match pc {
            0x8241652C => {
    //   block [0x8241652C..0x8241657C)
	// 8241652C: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82416534: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416538: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241653C: 41820038  beq 0x82416574
	if ctx.cr[0].eq {
	pc = 0x82416574; continue 'dispatch;
	}
	// 82416540: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416544: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82416548: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241654C: A0840000  lhz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416550: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82416554: 409A0010  bne cr6, 0x82416564
	if !ctx.cr[6].eq {
	pc = 0x82416564; continue 'dispatch;
	}
	// 82416558: 7F054040  cmplw cr6, r5, r8
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8241655C: 419A0020  beq cr6, 0x8241657c
	if ctx.cr[6].eq {
		sub_8241657C(ctx, base);
		return;
	}
	// 82416560: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82416564: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82416568: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8241656C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82416570: 4198FFD8  blt cr6, 0x82416548
	if ctx.cr[6].lt {
	pc = 0x82416548; continue 'dispatch;
	}
	// 82416574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241657C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241657C size=16
    let mut pc: u32 = 0x8241657C;
    'dispatch: loop {
        match pc {
            0x8241657C => {
    //   block [0x8241657C..0x8241658C)
	// 8241657C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416580: 7D6B382E  lwzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82416584: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82416588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416590 size=376
    let mut pc: u32 = 0x82416590;
    'dispatch: loop {
        match pc {
            0x82416590 => {
    //   block [0x82416590..0x82416708)
	// 82416590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416594: 4811EB29  bl 0x825350bc
	ctx.lr = 0x82416598;
	sub_82535080(ctx, base);
	// 82416598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241659C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824165A0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824165A4: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 824165A8: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824165AC: F8BF0008  std r5, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 824165B0: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 824165B4: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 824165B8: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824165BC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824165C0: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824165C4: 40820008  bne 0x824165cc
	if !ctx.cr[0].eq {
	pc = 0x824165CC; continue 'dispatch;
	}
	// 824165C8: 48000000  b 0x824165c8
	pc = 0x824165C8; continue 'dispatch;
	// 824165CC: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 824165D0: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 824165D4: 409A0008  bne cr6, 0x824165dc
	if !ctx.cr[6].eq {
	pc = 0x824165DC; continue 'dispatch;
	}
	// 824165D8: 48000000  b 0x824165d8
	pc = 0x824165D8; continue 'dispatch;
	// 824165DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824165E0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824165E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824165E8: 409A0008  bne cr6, 0x824165f0
	if !ctx.cr[6].eq {
	pc = 0x824165F0; continue 'dispatch;
	}
	// 824165EC: 48000000  b 0x824165ec
	pc = 0x824165EC; continue 'dispatch;
	// 824165F0: 892B0008  lbz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824165F4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824165F8: 40820008  bne 0x82416600
	if !ctx.cr[0].eq {
	pc = 0x82416600; continue 'dispatch;
	}
	// 824165FC: 48000000  b 0x824165fc
	pc = 0x824165FC; continue 'dispatch;
	// 82416600: 892B000A  lbz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82416604: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416608: 40820008  bne 0x82416610
	if !ctx.cr[0].eq {
	pc = 0x82416610; continue 'dispatch;
	}
	// 8241660C: 48000000  b 0x8241660c
	pc = 0x8241660C; continue 'dispatch;
	// 82416610: 896B000B  lbz r11, 0xb(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82416614: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416618: 40820008  bne 0x82416620
	if !ctx.cr[0].eq {
	pc = 0x82416620; continue 'dispatch;
	}
	// 8241661C: 48000000  b 0x8241661c
	pc = 0x8241661C; continue 'dispatch;
	// 82416620: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82416624: 2F2A0010  cmpdi cr6, r10, 0x10
	ctx.cr[6].compare_i64(ctx.r[10].s64, 16, &mut ctx.xer);
	// 82416628: 40990050  ble cr6, 0x82416678
	if !ctx.cr[6].gt {
	pc = 0x82416678; continue 'dispatch;
	}
	// 8241662C: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82416630: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416634: 2B09FFFF  cmplwi cr6, r9, 0xffff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 65535 as u32, &mut ctx.xer);
	// 82416638: 419A0040  beq cr6, 0x82416678
	if ctx.cr[6].eq {
	pc = 0x82416678; continue 'dispatch;
	}
	// 8241663C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82416640: 419A00C4  beq cr6, 0x82416704
	if ctx.cr[6].eq {
	pc = 0x82416704; continue 'dispatch;
	}
	// 82416644: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416648: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8241664C: 419A00B4  beq cr6, 0x82416700
	if ctx.cr[6].eq {
	pc = 0x82416700; continue 'dispatch;
	}
	// 82416650: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416654: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82416658: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8241665C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416660: E93F0008  ld r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82416664: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82416668: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8241666C: 796A0020  clrldi r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82416670: 7F2A4800  cmpd cr6, r10, r9
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[9].s64, &mut ctx.xer);
	// 82416674: 4198FFB8  blt cr6, 0x8241662c
	if ctx.cr[6].lt {
	pc = 0x8241662C; continue 'dispatch;
	}
	// 82416678: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241667C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416680: 41820074  beq 0x824166f4
	if ctx.cr[0].eq {
	pc = 0x824166F4; continue 'dispatch;
	}
	// 82416684: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82416688: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8241668C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82416690: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82416694: 40990008  ble cr6, 0x8241669c
	if !ctx.cr[6].gt {
	pc = 0x8241669C; continue 'dispatch;
	}
	// 82416698: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8241669C: 4BFEE72D  bl 0x82404dc8
	ctx.lr = 0x824166A0;
	sub_82404DC8(ctx, base);
	// 824166A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824166A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824166A8: 4BFFD971  bl 0x82414018
	ctx.lr = 0x824166AC;
	sub_82414018(ctx, base);
	// 824166AC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824166B0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824166B4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 824166B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824166BC: 40990038  ble cr6, 0x824166f4
	if !ctx.cr[6].gt {
	pc = 0x824166F4; continue 'dispatch;
	}
	// 824166C0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 824166C4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824166C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824166CC: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824166D0: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824166D4: 7D07492E  stwx r8, r7, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 824166D8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824166DC: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824166E0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824166E4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824166E8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824166EC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824166F0: 4198FFD4  blt cr6, 0x824166c4
	if ctx.cr[6].lt {
	pc = 0x824166C4; continue 'dispatch;
	}
	// 824166F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824166F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824166FC: 4811EA10  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82416700: 48000000  b 0x82416700
	pc = 0x82416700; continue 'dispatch;
	// 82416704: 48000000  b 0x82416704
	pc = 0x82416704; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416708 size=64
    let mut pc: u32 = 0x82416708;
    'dispatch: loop {
        match pc {
            0x82416708 => {
    //   block [0x82416708..0x82416748)
	// 82416708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241671C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416720: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416724: 41820010  beq 0x82416734
	if ctx.cr[0].eq {
	pc = 0x82416734; continue 'dispatch;
	}
	// 82416728: 4811C491  bl 0x82532bb8
	ctx.lr = 0x8241672C;
	sub_82532BB8(ctx, base);
	// 8241672C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416730: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82416734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416748 size=8
    let mut pc: u32 = 0x82416748;
    'dispatch: loop {
        match pc {
            0x82416748 => {
    //   block [0x82416748..0x82416750)
	// 82416748: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8241674C: 480064BC  b 0x8241cc08
	sub_8241CC08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416750 size=228
    let mut pc: u32 = 0x82416750;
    'dispatch: loop {
        match pc {
            0x82416750 => {
    //   block [0x82416750..0x82416834)
	// 82416750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241675C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416764: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82416768: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241676C: 38EBF210  addi r7, r11, -0xdf0
	ctx.r[7].s64 = ctx.r[11].s64 + -3568;
	// 82416770: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82416774: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82416778: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8241677C: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82416780: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82416784: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82416788: 4082FFEC  bne 0x82416774
	if !ctx.cr[0].eq {
	pc = 0x82416774; continue 'dispatch;
	}
	// 8241678C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82416790: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82416794: 419A0014  beq cr6, 0x824167a8
	if ctx.cr[6].eq {
	pc = 0x824167A8; continue 'dispatch;
	}
	// 82416798: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241679C: 386BEC98  addi r3, r11, -0x1368
	ctx.r[3].s64 = ctx.r[11].s64 + -4968;
	// 824167A0: 48006469  bl 0x8241cc08
	ctx.lr = 0x824167A4;
	sub_8241CC08(ctx, base);
	// 824167A4: 48000078  b 0x8241681c
	pc = 0x8241681C; continue 'dispatch;
	// 824167A8: 48008C79  bl 0x8241f420
	ctx.lr = 0x824167AC;
	sub_8241F420(ctx, base);
	// 824167AC: 3D608241  lis r11, -0x7dbf
	ctx.r[11].s64 = -2109669376;
	// 824167B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824167B4: 386B6748  addi r3, r11, 0x6748
	ctx.r[3].s64 = ctx.r[11].s64 + 26440;
	// 824167B8: 480094E1  bl 0x8241fc98
	ctx.lr = 0x824167BC;
	sub_8241FC98(ctx, base);
	// 824167BC: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 824167C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824167C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824167C8: 386AEC90  addi r3, r10, -0x1370
	ctx.r[3].s64 = ctx.r[10].s64 + -4976;
	// 824167CC: 388B05E8  addi r4, r11, 0x5e8
	ctx.r[4].s64 = ctx.r[11].s64 + 1512;
	// 824167D0: 48009D49  bl 0x82420518
	ctx.lr = 0x824167D4;
	sub_82420518(ctx, base);
	// 824167D4: 48007595  bl 0x8241dd68
	ctx.lr = 0x824167D8;
	sub_8241DD68(ctx, base);
	// 824167D8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824167DC: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 824167E0: 3BEAE0F4  addi r31, r10, -0x1f0c
	ctx.r[31].s64 = ctx.r[10].s64 + -7948;
	// 824167E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824167E8: 388BCC80  addi r4, r11, -0x3380
	ctx.r[4].s64 = ctx.r[11].s64 + -13184;
	// 824167EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824167F0: 48009D29  bl 0x82420518
	ctx.lr = 0x824167F4;
	sub_82420518(ctx, base);
	// 824167F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824167F8: 480097F9  bl 0x8241fff0
	ctx.lr = 0x824167FC;
	sub_8241FFF0(ctx, base);
	// 824167FC: 48006325  bl 0x8241cb20
	ctx.lr = 0x82416800;
	sub_8241CB20(ctx, base);
	// 82416800: 48008B19  bl 0x8241f318
	ctx.lr = 0x82416804;
	sub_8241F318(ctx, base);
	// 82416804: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82416808: 419A000C  beq cr6, 0x82416814
	if ctx.cr[6].eq {
	pc = 0x82416814; continue 'dispatch;
	}
	// 8241680C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416810: 48000008  b 0x82416818
	pc = 0x82416818; continue 'dispatch;
	// 82416814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416818: 48007461  bl 0x8241dc78
	ctx.lr = 0x8241681C;
	sub_8241DC78(ctx, base);
	// 8241681C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82416820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416828: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241682C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416838 size=132
    let mut pc: u32 = 0x82416838;
    'dispatch: loop {
        match pc {
            0x82416838 => {
    //   block [0x82416838..0x824168BC)
	// 82416838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241683C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416844: 48005715  bl 0x8241bf58
	ctx.lr = 0x82416848;
	sub_8241BF58(ctx, base);
	// 82416848: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8241684C: 41820014  beq 0x82416860
	if ctx.cr[0].eq {
	pc = 0x82416860; continue 'dispatch;
	}
	// 82416850: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416854: 386BED48  addi r3, r11, -0x12b8
	ctx.r[3].s64 = ctx.r[11].s64 + -4792;
	// 82416858: 480063B1  bl 0x8241cc08
	ctx.lr = 0x8241685C;
	sub_8241CC08(ctx, base);
	// 8241685C: 48000050  b 0x824168ac
	pc = 0x824168AC; continue 'dispatch;
	// 82416860: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 82416864: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82416868: 38EBF210  addi r7, r11, -0xdf0
	ctx.r[7].s64 = ctx.r[11].s64 + -3568;
	// 8241686C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82416870: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82416874: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82416878: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8241687C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82416880: 4082FFEC  bne 0x8241686c
	if !ctx.cr[0].eq {
	pc = 0x8241686C; continue 'dispatch;
	}
	// 82416884: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82416888: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241688C: 409A0010  bne cr6, 0x8241689c
	if !ctx.cr[6].eq {
	pc = 0x8241689C; continue 'dispatch;
	}
	// 82416890: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416894: 386BECE8  addi r3, r11, -0x1318
	ctx.r[3].s64 = ctx.r[11].s64 + -4888;
	// 82416898: 4BFFFFC0  b 0x82416858
	pc = 0x82416858; continue 'dispatch;
	// 8241689C: 48008AD5  bl 0x8241f370
	ctx.lr = 0x824168A0;
	sub_8241F370(ctx, base);
	// 824168A0: 480062B9  bl 0x8241cb58
	ctx.lr = 0x824168A4;
	sub_8241CB58(ctx, base);
	// 824168A4: 48008BED  bl 0x8241f490
	ctx.lr = 0x824168A8;
	sub_8241F490(ctx, base);
	// 824168A8: 48008671  bl 0x8241ef18
	ctx.lr = 0x824168AC;
	sub_8241EF18(ctx, base);
	// 824168AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824168B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824168B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824168B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824168C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824168C0 size=4
    let mut pc: u32 = 0x824168C0;
    'dispatch: loop {
        match pc {
            0x824168C0 => {
    //   block [0x824168C0..0x824168C4)
	// 824168C0: 480068E0  b 0x8241d1a0
	sub_8241D1A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824168C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824168C8 size=56
    let mut pc: u32 = 0x824168C8;
    'dispatch: loop {
        match pc {
            0x824168C8 => {
    //   block [0x824168C8..0x82416900)
	// 824168C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824168CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824168D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824168D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824168D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824168DC: 4800A755  bl 0x82421030
	ctx.lr = 0x824168E0;
	sub_82421030(ctx, base);
	// 824168E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824168E4: 4800856D  bl 0x8241ee50
	ctx.lr = 0x824168E8;
	sub_8241EE50(ctx, base);
	// 824168E8: 4800A789  bl 0x82421070
	ctx.lr = 0x824168EC;
	sub_82421070(ctx, base);
	// 824168EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824168F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824168F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824168F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824168FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82416900 size=132
    let mut pc: u32 = 0x82416900;
    'dispatch: loop {
        match pc {
            0x82416900 => {
    //   block [0x82416900..0x82416984)
	// 82416900: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82416904: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82416908: 3D208313  lis r9, -0x7ced
	ctx.r[9].s64 = -2095906816;
	// 8241690C: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82416910: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82416914: 390AC420  addi r8, r10, -0x3be0
	ctx.r[8].s64 = ctx.r[10].s64 + -15328;
	// 82416918: 8169C520  lwz r11, -0x3ae0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15072 as u32) ) } as u64;
	// 8241691C: 7D7F2670  srawi r31, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82416920: 7D5F0194  addze r10, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82416924: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82416928: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8241692C: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82416930: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82416934: 3D008313  lis r8, -0x7ced
	ctx.r[8].s64 = -2095906816;
	// 82416938: 3908C540  addi r8, r8, -0x3ac0
	ctx.r[8].s64 = ctx.r[8].s64 + -15040;
	// 8241693C: 409A0014  bne cr6, 0x82416950
	if !ctx.cr[6].eq {
	pc = 0x82416950; continue 'dispatch;
	}
	// 82416940: 547F083C  slwi r31, r3, 1
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82416944: 7FDF422E  lhzx r30, r31, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82416948: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241694C: 7FDF432E  sthx r30, r31, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u16) };
	// 82416950: 547F083C  slwi r31, r3, 1
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82416954: 986B0000  stb r3, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 82416958: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241695C: 988B0001  stb r4, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 82416960: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82416964: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82416968: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8241696C: 7D1F422E  lhzx r8, r31, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82416970: 9149C520  stw r10, -0x3ae0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15072 as u32), ctx.r[10].u32 ) };
	// 82416974: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82416978: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241697C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82416980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416988 size=80
    let mut pc: u32 = 0x82416988;
    'dispatch: loop {
        match pc {
            0x82416988 => {
    //   block [0x82416988..0x824169D8)
	// 82416988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241698C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416994: 2B0300FF  cmplwi cr6, r3, 0xff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 255 as u32, &mut ctx.xer);
	// 82416998: 41990020  bgt cr6, 0x824169b8
	if ctx.cr[6].gt {
	pc = 0x824169B8; continue 'dispatch;
	}
	// 8241699C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824169A0: 409A0010  bne cr6, 0x824169b0
	if !ctx.cr[6].eq {
	pc = 0x824169B0; continue 'dispatch;
	}
	// 824169A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824169A8: 386BEDE4  addi r3, r11, -0x121c
	ctx.r[3].s64 = ctx.r[11].s64 + -4636;
	// 824169AC: 48000014  b 0x824169c0
	pc = 0x824169C0; continue 'dispatch;
	// 824169B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824169B4: 48000014  b 0x824169c8
	pc = 0x824169C8; continue 'dispatch;
	// 824169B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824169BC: 386BEDAC  addi r3, r11, -0x1254
	ctx.r[3].s64 = ctx.r[11].s64 + -4692;
	// 824169C0: 4800A8D9  bl 0x82421298
	ctx.lr = 0x824169C4;
	sub_82421298(ctx, base);
	// 824169C4: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 824169C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824169CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824169D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824169D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824169D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824169D8 size=56
    let mut pc: u32 = 0x824169D8;
    'dispatch: loop {
        match pc {
            0x824169D8 => {
    //   block [0x824169D8..0x82416A10)
	// 824169D8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824169DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824169E0: 396BC980  addi r11, r11, -0x3680
	ctx.r[11].s64 = ctx.r[11].s64 + -13952;
	// 824169E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824169E8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824169EC: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824169F0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824169F4: 419A001C  beq cr6, 0x82416a10
	if ctx.cr[6].eq {
		sub_82416A10(ctx, base);
		return;
	}
	// 824169F8: 394A0034  addi r10, r10, 0x34
	ctx.r[10].s64 = ctx.r[10].s64 + 52;
	// 824169FC: 390B0340  addi r8, r11, 0x340
	ctx.r[8].s64 = ctx.r[11].s64 + 832;
	// 82416A00: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82416A04: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82416A08: 4198FFE4  blt cr6, 0x824169ec
	if ctx.cr[6].lt {
	pc = 0x824169EC; continue 'dispatch;
	}
	// 82416A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82416A10 size=12
    let mut pc: u32 = 0x82416A10;
    'dispatch: loop {
        match pc {
            0x82416A10 => {
    //   block [0x82416A10..0x82416A1C)
	// 82416A10: 1D490034  mulli r10, r9, 0x34
	ctx.r[10].s64 = ctx.r[9].s64 * 52;
	// 82416A14: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82416A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416A20 size=156
    let mut pc: u32 = 0x82416A20;
    'dispatch: loop {
        match pc {
            0x82416A20 => {
    //   block [0x82416A20..0x82416ABC)
	// 82416A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416A2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416A30: 4BFFFFA9  bl 0x824169d8
	ctx.lr = 0x82416A34;
	sub_824169D8(ctx, base);
	// 82416A34: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82416A38: 40820018  bne 0x82416a50
	if !ctx.cr[0].eq {
	pc = 0x82416A50; continue 'dispatch;
	}
	// 82416A3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416A40: 386BEE48  addi r3, r11, -0x11b8
	ctx.r[3].s64 = ctx.r[11].s64 + -4536;
	// 82416A44: 4800A855  bl 0x82421298
	ctx.lr = 0x82416A48;
	sub_82421298(ctx, base);
	// 82416A48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416A4C: 4800005C  b 0x82416aa8
	pc = 0x82416AA8; continue 'dispatch;
	// 82416A50: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82416A54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416A58: 4800B759  bl 0x824221b0
	ctx.lr = 0x82416A5C;
	sub_824221B0(ctx, base);
	// 82416A5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416A60: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82416A64: 40820010  bne 0x82416a74
	if !ctx.cr[0].eq {
	pc = 0x82416A74; continue 'dispatch;
	}
	// 82416A68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416A6C: 386BEE10  addi r3, r11, -0x11f0
	ctx.r[3].s64 = ctx.r[11].s64 + -4592;
	// 82416A70: 4BFFFFD4  b 0x82416a44
	pc = 0x82416A44; continue 'dispatch;
	// 82416A74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416A78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82416A7C: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 82416A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416A84: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82416A88: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82416A8C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82416A90: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82416A94: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82416A98: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82416A9C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82416AA0: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82416AA4: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82416AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416AC0 size=108
    let mut pc: u32 = 0x82416AC0;
    'dispatch: loop {
        match pc {
            0x82416AC0 => {
    //   block [0x82416AC0..0x82416B2C)
	// 82416AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416ACC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82416AD0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82416AD4: 409A0018  bne cr6, 0x82416aec
	if !ctx.cr[6].eq {
	pc = 0x82416AEC; continue 'dispatch;
	}
	// 82416AD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416ADC: 386BEE7C  addi r3, r11, -0x1184
	ctx.r[3].s64 = ctx.r[11].s64 + -4484;
	// 82416AE0: 4800A7B9  bl 0x82421298
	ctx.lr = 0x82416AE4;
	sub_82421298(ctx, base);
	// 82416AE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416AE8: 48000034  b 0x82416b1c
	pc = 0x82416B1C; continue 'dispatch;
	// 82416AEC: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82416AF0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82416AF8: 90AB0030  stw r5, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82416AFC: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82416B00: 38E0F800  li r7, -0x800
	ctx.r[7].s64 = -2048;
	// 82416B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82416B08: 78E70580  clrldi r7, r7, 0x16
	ctx.r[7].u64 = ctx.r[7].u64 & 0x000003FFFFFFFFFFu64;
	// 82416B0C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82416B10: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82416B14: 4800B6ED  bl 0x82422200
	ctx.lr = 0x82416B18;
	sub_82422200(ctx, base);
	// 82416B18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416B30 size=120
    let mut pc: u32 = 0x82416B30;
    'dispatch: loop {
        match pc {
            0x82416B30 => {
    //   block [0x82416B30..0x82416BA8)
	// 82416B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416B44: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416B4C: 419A0048  beq cr6, 0x82416b94
	if ctx.cr[6].eq {
	pc = 0x82416B94; continue 'dispatch;
	}
	// 82416B50: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82416B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416B58: 409A003C  bne cr6, 0x82416b94
	if !ctx.cr[6].eq {
	pc = 0x82416B94; continue 'dispatch;
	}
	// 82416B5C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82416B60: 816BC524  lwz r11, -0x3adc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15068 as u32) ) } as u64;
	// 82416B64: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82416B68: 409A0010  bne cr6, 0x82416b78
	if !ctx.cr[6].eq {
	pc = 0x82416B78; continue 'dispatch;
	}
	// 82416B6C: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82416B70: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82416B74: 480183DD  bl 0x8242ef50
	ctx.lr = 0x82416B78;
	sub_8242EF50(ctx, base);
	// 82416B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82416B7C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416B80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82416B84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416B88: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416B8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82416B90: 4E800421  bctrl
	ctx.lr = 0x82416B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82416B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416BA8 size=228
    let mut pc: u32 = 0x82416BA8;
    'dispatch: loop {
        match pc {
            0x82416BA8 => {
    //   block [0x82416BA8..0x82416C8C)
	// 82416BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416BAC: 4811E511  bl 0x825350bc
	ctx.lr = 0x82416BB0;
	sub_82535080(ctx, base);
	// 82416BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416BB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82416BBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82416BC0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416BC4: 4800ACBD  bl 0x82421880
	ctx.lr = 0x82416BC8;
	sub_82421880(ctx, base);
	// 82416BC8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82416BCC: 419A000C  beq cr6, 0x82416bd8
	if ctx.cr[6].eq {
	pc = 0x82416BD8; continue 'dispatch;
	}
	// 82416BD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416BD4: 4800B96D  bl 0x82422540
	ctx.lr = 0x82416BD8;
	sub_82422540(ctx, base);
	// 82416BD8: 48018379  bl 0x8242ef50
	ctx.lr = 0x82416BDC;
	sub_8242EF50(ctx, base);
	// 82416BDC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416BE0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416BE4: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82416BE8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82416BEC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82416BF0: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82416BF4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82416BF8: 40980008  bge cr6, 0x82416c00
	if !ctx.cr[6].lt {
	pc = 0x82416C00; continue 'dispatch;
	}
	// 82416BFC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82416C00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82416C04: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82416C08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82416C0C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82416C10: 409A0010  bne cr6, 0x82416c20
	if !ctx.cr[6].eq {
	pc = 0x82416C20; continue 'dispatch;
	}
	// 82416C14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82416C18: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416C1C: 4800005C  b 0x82416c78
	pc = 0x82416C78; continue 'dispatch;
	// 82416C20: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82416C24: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C28: 4800AD71  bl 0x82421998
	ctx.lr = 0x82416C2C;
	sub_82421998(ctx, base);
	// 82416C2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82416C30: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C34: 4800B82D  bl 0x82422460
	ctx.lr = 0x82416C38;
	sub_82422460(ctx, base);
	// 82416C38: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82416C3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C40: 4800B479  bl 0x824220b8
	ctx.lr = 0x82416C44;
	sub_824220B8(ctx, base);
	// 82416C44: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82416C48: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82416C50: 9BDF0003  stb r30, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 82416C54: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416C58: 4800B519  bl 0x82422170
	ctx.lr = 0x82416C5C;
	sub_82422170(ctx, base);
	// 82416C5C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416C60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C64: 4800AC5D  bl 0x824218c0
	ctx.lr = 0x82416C68;
	sub_824218C0(ctx, base);
	// 82416C68: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416C6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416C70: 4800B679  bl 0x824222e8
	ctx.lr = 0x82416C74;
	sub_824222E8(ctx, base);
	// 82416C74: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82416C78: 480182D9  bl 0x8242ef50
	ctx.lr = 0x82416C7C;
	sub_8242EF50(ctx, base);
	// 82416C7C: 4800BA55  bl 0x824226d0
	ctx.lr = 0x82416C80;
	sub_824226D0(ctx, base);
	// 82416C80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82416C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82416C88: 4811E484  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416C90 size=460
    let mut pc: u32 = 0x82416C90;
    'dispatch: loop {
        match pc {
            0x82416C90 => {
    //   block [0x82416C90..0x82416E5C)
	// 82416C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416C94: 4811E421  bl 0x825350b4
	ctx.lr = 0x82416C98;
	sub_82535080(ctx, base);
	// 82416C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416CA0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82416CA4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82416CA8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82416CAC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82416CB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82416CB8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82416CBC: 4BFFFC45  bl 0x82416900
	ctx.lr = 0x82416CC0;
	sub_82416900(ctx, base);
	// 82416CC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82416CC4: 409A0018  bne cr6, 0x82416cdc
	if !ctx.cr[6].eq {
	pc = 0x82416CDC; continue 'dispatch;
	}
	// 82416CC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416CCC: 386BEFA4  addi r3, r11, -0x105c
	ctx.r[3].s64 = ctx.r[11].s64 + -4188;
	// 82416CD0: 4800A5C9  bl 0x82421298
	ctx.lr = 0x82416CD4;
	sub_82421298(ctx, base);
	// 82416CD4: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82416CD8: 4800017C  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
	// 82416CDC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82416CE0: 40980010  bge cr6, 0x82416cf0
	if !ctx.cr[6].lt {
	pc = 0x82416CF0; continue 'dispatch;
	}
	// 82416CE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416CE8: 386BEF78  addi r3, r11, -0x1088
	ctx.r[3].s64 = ctx.r[11].s64 + -4232;
	// 82416CEC: 4BFFFFE4  b 0x82416cd0
	pc = 0x82416CD0; continue 'dispatch;
	// 82416CF0: 3D600010  lis r11, 0x10
	ctx.r[11].s64 = 1048576;
	// 82416CF4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82416CF8: 41980010  blt cr6, 0x82416d08
	if ctx.cr[6].lt {
	pc = 0x82416D08; continue 'dispatch;
	}
	// 82416CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416D00: 386BEF48  addi r3, r11, -0x10b8
	ctx.r[3].s64 = ctx.r[11].s64 + -4280;
	// 82416D04: 4BFFFFCC  b 0x82416cd0
	pc = 0x82416CD0; continue 'dispatch;
	// 82416D08: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82416D0C: 409A0010  bne cr6, 0x82416d1c
	if !ctx.cr[6].eq {
	pc = 0x82416D1C; continue 'dispatch;
	}
	// 82416D10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416D14: 386BEF20  addi r3, r11, -0x10e0
	ctx.r[3].s64 = ctx.r[11].s64 + -4320;
	// 82416D18: 4BFFFFB8  b 0x82416cd0
	pc = 0x82416CD0; continue 'dispatch;
	// 82416D1C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82416D20: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82416D24: 409A000C  bne cr6, 0x82416d30
	if !ctx.cr[6].eq {
	pc = 0x82416D30; continue 'dispatch;
	}
	// 82416D28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82416D2C: 48000128  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
	// 82416D30: 48018221  bl 0x8242ef50
	ctx.lr = 0x82416D34;
	sub_8242EF50(ctx, base);
	// 82416D34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416D38: 4800A8D9  bl 0x82421610
	ctx.lr = 0x82416D3C;
	sub_82421610(ctx, base);
	// 82416D3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82416D40: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416D44: 4800A88D  bl 0x824215d0
	ctx.lr = 0x82416D48;
	sub_824215D0(ctx, base);
	// 82416D48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82416D4C: 48018205  bl 0x8242ef50
	ctx.lr = 0x82416D50;
	sub_8242EF50(ctx, base);
	// 82416D50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82416D54: 409A001C  bne cr6, 0x82416d70
	if !ctx.cr[6].eq {
	pc = 0x82416D70; continue 'dispatch;
	}
	// 82416D58: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82416D5C: 409A0014  bne cr6, 0x82416d70
	if !ctx.cr[6].eq {
	pc = 0x82416D70; continue 'dispatch;
	}
	// 82416D60: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82416D64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416D68: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416D6C: 480000E8  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
	// 82416D70: 480181E1  bl 0x8242ef50
	ctx.lr = 0x82416D74;
	sub_8242EF50(ctx, base);
	// 82416D74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82416D7C: 419A001C  beq cr6, 0x82416d98
	if ctx.cr[6].eq {
	pc = 0x82416D98; continue 'dispatch;
	}
	// 82416D80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416D84: 386BEEF4  addi r3, r11, -0x110c
	ctx.r[3].s64 = ctx.r[11].s64 + -4364;
	// 82416D88: 4800A511  bl 0x82421298
	ctx.lr = 0x82416D8C;
	sub_82421298(ctx, base);
	// 82416D8C: 480181C5  bl 0x8242ef50
	ctx.lr = 0x82416D90;
	sub_8242EF50(ctx, base);
	// 82416D90: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416D94: 480000C0  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
	// 82416D98: 480181B9  bl 0x8242ef50
	ctx.lr = 0x82416D9C;
	sub_8242EF50(ctx, base);
	// 82416D9C: 577E5828  slwi r30, r27, 0xb
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(11);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82416DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82416DA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82416DA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82416DAC: 4800C69D  bl 0x82423448
	ctx.lr = 0x82416DB0;
	sub_82423448(ctx, base);
	// 82416DB0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82416DB4: 4082000C  bne 0x82416dc0
	if !ctx.cr[0].eq {
	pc = 0x82416DC0; continue 'dispatch;
	}
	// 82416DB8: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82416DBC: 48000098  b 0x82416e54
	pc = 0x82416E54; continue 'dispatch;
	// 82416DC0: 48018191  bl 0x8242ef50
	ctx.lr = 0x82416DC4;
	sub_8242EF50(ctx, base);
	// 82416DC4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82416DC8: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82416DCC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82416DD0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82416DD4: 816BC524  lwz r11, -0x3adc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15068 as u32) ) } as u64;
	// 82416DD8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82416DDC: 409A0010  bne cr6, 0x82416dec
	if !ctx.cr[6].eq {
	pc = 0x82416DEC; continue 'dispatch;
	}
	// 82416DE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82416DE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82416DE8: 48018169  bl 0x8242ef50
	ctx.lr = 0x82416DEC;
	sub_8242EF50(ctx, base);
	// 82416DEC: 48018165  bl 0x8242ef50
	ctx.lr = 0x82416DF0;
	sub_8242EF50(ctx, base);
	// 82416DF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82416DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416DF8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416DFC: 4BFFFDAD  bl 0x82416ba8
	ctx.lr = 0x82416E00;
	sub_82416BA8(ctx, base);
	// 82416E00: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82416E04: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82416E08: 4181002C  bgt 0x82416e34
	if ctx.cr[0].gt {
	pc = 0x82416E34; continue 'dispatch;
	}
	// 82416E0C: 48018145  bl 0x8242ef50
	ctx.lr = 0x82416E10;
	sub_8242EF50(ctx, base);
	// 82416E10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82416E14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416E18: 41820018  beq 0x82416e30
	if ctx.cr[0].eq {
	pc = 0x82416E30; continue 'dispatch;
	}
	// 82416E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82416E20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416E24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82416E28: 4E800421  bctrl
	ctx.lr = 0x82416E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82416E2C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82416E30: 48018121  bl 0x8242ef50
	ctx.lr = 0x82416E34;
	sub_8242EF50(ctx, base);
	// 82416E34: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82416E38: 9BBF0002  stb r29, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82416E3C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82416E40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416E44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82416E48: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82416E4C: 4BFFFAB5  bl 0x82416900
	ctx.lr = 0x82416E50;
	sub_82416900(ctx, base);
	// 82416E50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82416E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82416E58: 4811E2AC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416E60 size=240
    let mut pc: u32 = 0x82416E60;
    'dispatch: loop {
        match pc {
            0x82416E60 => {
    //   block [0x82416E60..0x82416F50)
	// 82416E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416E74: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82416E78: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82416E7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416E80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82416E84: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82416E88: 4BFFFA79  bl 0x82416900
	ctx.lr = 0x82416E8C;
	sub_82416900(ctx, base);
	// 82416E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82416E90: 409A0018  bne cr6, 0x82416ea8
	if !ctx.cr[6].eq {
	pc = 0x82416EA8; continue 'dispatch;
	}
	// 82416E94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416E98: 386BF02C  addi r3, r11, -0xfd4
	ctx.r[3].s64 = ctx.r[11].s64 + -4052;
	// 82416E9C: 4800A3FD  bl 0x82421298
	ctx.lr = 0x82416EA0;
	sub_82421298(ctx, base);
	// 82416EA0: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82416EA4: 48000098  b 0x82416f3c
	pc = 0x82416F3C; continue 'dispatch;
	// 82416EA8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82416EAC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82416EB0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82416EB4: 419A0084  beq cr6, 0x82416f38
	if ctx.cr[6].eq {
	pc = 0x82416F38; continue 'dispatch;
	}
	// 82416EB8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82416EBC: 409A0010  bne cr6, 0x82416ecc
	if !ctx.cr[6].eq {
	pc = 0x82416ECC; continue 'dispatch;
	}
	// 82416EC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82416EC4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416EC8: 48000070  b 0x82416f38
	pc = 0x82416F38; continue 'dispatch;
	// 82416ECC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416ED0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82416ED4: 40820018  bne 0x82416eec
	if !ctx.cr[0].eq {
	pc = 0x82416EEC; continue 'dispatch;
	}
	// 82416ED8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82416EDC: 386BF000  addi r3, r11, -0x1000
	ctx.r[3].s64 = ctx.r[11].s64 + -4096;
	// 82416EE0: 4800A3B9  bl 0x82421298
	ctx.lr = 0x82416EE4;
	sub_82421298(ctx, base);
	// 82416EE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82416EE8: 48000054  b 0x82416f3c
	pc = 0x82416F3C; continue 'dispatch;
	// 82416EEC: 4800B655  bl 0x82422540
	ctx.lr = 0x82416EF0;
	sub_82422540(ctx, base);
	// 82416EF0: 48018061  bl 0x8242ef50
	ctx.lr = 0x82416EF4;
	sub_8242EF50(ctx, base);
	// 82416EF4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416EF8: 4800AA21  bl 0x82421918
	ctx.lr = 0x82416EFC;
	sub_82421918(ctx, base);
	// 82416EFC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416F00: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82416F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416F08: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82416F0C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82416F10: 4BFFFC21  bl 0x82416b30
	ctx.lr = 0x82416F14;
	sub_82416B30(ctx, base);
	// 82416F14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82416F18: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416F1C: 48018035  bl 0x8242ef50
	ctx.lr = 0x82416F20;
	sub_8242EF50(ctx, base);
	// 82416F20: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82416F24: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82416F28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82416F2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82416F30: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82416F34: 4BFFF9CD  bl 0x82416900
	ctx.lr = 0x82416F38;
	sub_82416900(ctx, base);
	// 82416F38: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82416F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82416F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82416F48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82416F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82416F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82416F50 size=320
    let mut pc: u32 = 0x82416F50;
    'dispatch: loop {
        match pc {
            0x82416F50 => {
    //   block [0x82416F50..0x82417090)
	// 82416F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82416F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82416F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82416F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82416F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82416F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82416F68: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416F6C: 4800A915  bl 0x82421880
	ctx.lr = 0x82416F70;
	sub_82421880(ctx, base);
	// 82416F70: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82416F74: 409A0014  bne cr6, 0x82416f88
	if !ctx.cr[6].eq {
	pc = 0x82416F88; continue 'dispatch;
	}
	// 82416F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416F7C: 4BFFFBB5  bl 0x82416b30
	ctx.lr = 0x82416F80;
	sub_82416B30(ctx, base);
	// 82416F80: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82416F84: 480000F0  b 0x82417074
	pc = 0x82417074; continue 'dispatch;
	// 82416F88: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82416F8C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82416F90: 409A009C  bne cr6, 0x8241702c
	if !ctx.cr[6].eq {
	pc = 0x8241702C; continue 'dispatch;
	}
	// 82416F94: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82416F98: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82416F9C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82416FA0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82416FA4: 409A003C  bne cr6, 0x82416fe0
	if !ctx.cr[6].eq {
	pc = 0x82416FE0; continue 'dispatch;
	}
	// 82416FA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FAC: 4800A625  bl 0x824215d0
	ctx.lr = 0x82416FB0;
	sub_824215D0(ctx, base);
	// 82416FB0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82416FB4: 4182002C  beq 0x82416fe0
	if ctx.cr[0].eq {
	pc = 0x82416FE0; continue 'dispatch;
	}
	// 82416FB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FBC: 4800B17D  bl 0x82422138
	ctx.lr = 0x82416FC0;
	sub_82422138(ctx, base);
	// 82416FC0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82416FC4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82416FC8: 41990018  bgt cr6, 0x82416fe0
	if ctx.cr[6].gt {
	pc = 0x82416FE0; continue 'dispatch;
	}
	// 82416FCC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82416FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82416FD4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416FD8: 4BFFFB59  bl 0x82416b30
	ctx.lr = 0x82416FDC;
	sub_82416B30(ctx, base);
	// 82416FDC: 4800009C  b 0x82417078
	pc = 0x82417078; continue 'dispatch;
	// 82416FE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FE4: 4800A89D  bl 0x82421880
	ctx.lr = 0x82416FE8;
	sub_82421880(ctx, base);
	// 82416FE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82416FEC: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82416FF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82416FF4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82416FF8: 4800A921  bl 0x82421918
	ctx.lr = 0x82416FFC;
	sub_82421918(ctx, base);
	// 82416FFC: 895F0001  lbz r10, 1(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82417000: 7D7E1850  subf r11, r30, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 82417004: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82417008: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8241700C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82417010: 419A000C  beq cr6, 0x8241701c
	if ctx.cr[6].eq {
	pc = 0x8241701C; continue 'dispatch;
	}
	// 82417014: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82417018: 409A0014  bne cr6, 0x8241702c
	if !ctx.cr[6].eq {
	pc = 0x8241702C; continue 'dispatch;
	}
	// 8241701C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82417020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417024: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82417028: 4BFFFB09  bl 0x82416b30
	ctx.lr = 0x8241702C;
	sub_82416B30(ctx, base);
	// 8241702C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82417030: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82417034: 409A0044  bne cr6, 0x82417078
	if !ctx.cr[6].eq {
	pc = 0x82417078; continue 'dispatch;
	}
	// 82417038: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241703C: 4800A845  bl 0x82421880
	ctx.lr = 0x82417040;
	sub_82421880(ctx, base);
	// 82417040: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82417044: 409A0034  bne cr6, 0x82417078
	if !ctx.cr[6].eq {
	pc = 0x82417078; continue 'dispatch;
	}
	// 82417048: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241704C: 4800A8CD  bl 0x82421918
	ctx.lr = 0x82417050;
	sub_82421918(ctx, base);
	// 82417050: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82417054: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82417058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241705C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82417060: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82417064: 4BFFFACD  bl 0x82416b30
	ctx.lr = 0x82417068;
	sub_82416B30(ctx, base);
	// 82417068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241706C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82417070: 995F0003  stb r10, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82417074: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82417078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241707C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417084: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241708C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417090 size=120
    let mut pc: u32 = 0x82417090;
    'dispatch: loop {
        match pc {
            0x82417090 => {
    //   block [0x82417090..0x82417108)
	// 82417090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241709C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824170A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824170A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824170A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824170AC: 3BCBEECC  addi r30, r11, -0x1134
	ctx.r[30].s64 = ctx.r[11].s64 + -4404;
	// 824170B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824170B4: 409A0010  bne cr6, 0x824170c4
	if !ctx.cr[6].eq {
	pc = 0x824170C4; continue 'dispatch;
	}
	// 824170B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824170BC: 4800A1DD  bl 0x82421298
	ctx.lr = 0x824170C0;
	sub_82421298(ctx, base);
	// 824170C0: 48000014  b 0x824170d4
	pc = 0x824170D4; continue 'dispatch;
	// 824170C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824170C8: 4800A509  bl 0x824215d0
	ctx.lr = 0x824170CC;
	sub_824215D0(ctx, base);
	// 824170CC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824170D0: 419A0020  beq cr6, 0x824170f0
	if ctx.cr[6].eq {
	pc = 0x824170F0; continue 'dispatch;
	}
	// 824170D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824170D8: 4800A539  bl 0x82421610
	ctx.lr = 0x824170DC;
	sub_82421610(ctx, base);
	// 824170DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824170E0: 41820010  beq 0x824170f0
	if ctx.cr[0].eq {
	pc = 0x824170F0; continue 'dispatch;
	}
	// 824170E4: 4800B5ED  bl 0x824226d0
	ctx.lr = 0x824170E8;
	sub_824226D0(ctx, base);
	// 824170E8: 48003BF1  bl 0x8241acd8
	ctx.lr = 0x824170EC;
	sub_8241ACD8(ctx, base);
	// 824170EC: 4BFFFFC4  b 0x824170b0
	pc = 0x824170B0; continue 'dispatch;
	// 824170F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824170F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824170F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824170FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417108 size=116
    let mut pc: u32 = 0x82417108;
    'dispatch: loop {
        match pc {
            0x82417108 => {
    //   block [0x82417108..0x8241717C)
	// 82417108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241710C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417114: 2B0300FF  cmplwi cr6, r3, 0xff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 255 as u32, &mut ctx.xer);
	// 82417118: 41990044  bgt cr6, 0x8241715c
	if ctx.cr[6].gt {
	pc = 0x8241715C; continue 'dispatch;
	}
	// 8241711C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417120: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417124: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 82417128: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8241712C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82417130: 4182002C  beq 0x8241715c
	if ctx.cr[0].eq {
	pc = 0x8241715C; continue 'dispatch;
	}
	// 82417134: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82417138: 41980018  blt cr6, 0x82417150
	if ctx.cr[6].lt {
	pc = 0x82417150; continue 'dispatch;
	}
	// 8241713C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82417140: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82417144: 4098000C  bge cr6, 0x82417150
	if !ctx.cr[6].lt {
	pc = 0x82417150; continue 'dispatch;
	}
	// 82417148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241714C: 48000020  b 0x8241716c
	pc = 0x8241716C; continue 'dispatch;
	// 82417150: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417154: 386BF0F0  addi r3, r11, -0xf10
	ctx.r[3].s64 = ctx.r[11].s64 + -3856;
	// 82417158: 4800000C  b 0x82417164
	pc = 0x82417164; continue 'dispatch;
	// 8241715C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417160: 386BF0CC  addi r3, r11, -0xf34
	ctx.r[3].s64 = ctx.r[11].s64 + -3892;
	// 82417164: 4800A135  bl 0x82421298
	ctx.lr = 0x82417168;
	sub_82421298(ctx, base);
	// 82417168: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 8241716C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82417170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417180 size=144
    let mut pc: u32 = 0x82417180;
    'dispatch: loop {
        match pc {
            0x82417180 => {
    //   block [0x82417180..0x82417210)
	// 82417180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417184: 4811DF39  bl 0x825350bc
	ctx.lr = 0x82417188;
	sub_82535080(ctx, base);
	// 82417188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241718C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417190: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82417194: 48009E9D  bl 0x82421030
	ctx.lr = 0x82417198;
	sub_82421030(ctx, base);
	// 82417198: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241719C: 409A0018  bne cr6, 0x824171b4
	if !ctx.cr[6].eq {
	pc = 0x824171B4; continue 'dispatch;
	}
	// 824171A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824171A4: 386BEEA4  addi r3, r11, -0x115c
	ctx.r[3].s64 = ctx.r[11].s64 + -4444;
	// 824171A8: 4800A0F1  bl 0x82421298
	ctx.lr = 0x824171AC;
	sub_82421298(ctx, base);
	// 824171AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824171B0: 48000050  b 0x82417200
	pc = 0x82417200; continue 'dispatch;
	// 824171B4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824171B8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824171BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824171C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824171C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824171C8: 4BFFF739  bl 0x82416900
	ctx.lr = 0x824171CC;
	sub_82416900(ctx, base);
	// 824171CC: 4BFFF855  bl 0x82416a20
	ctx.lr = 0x824171D0;
	sub_82416A20(ctx, base);
	// 824171D0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824171D4: 41820014  beq 0x824171e8
	if ctx.cr[0].eq {
	pc = 0x824171E8; continue 'dispatch;
	}
	// 824171D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824171DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824171E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824171E4: 4BFFF8DD  bl 0x82416ac0
	ctx.lr = 0x824171E8;
	sub_82416AC0(ctx, base);
	// 824171E8: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824171EC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824171F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824171F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824171F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824171FC: 4BFFF705  bl 0x82416900
	ctx.lr = 0x82417200;
	sub_82416900(ctx, base);
	// 82417200: 48009E71  bl 0x82421070
	ctx.lr = 0x82417204;
	sub_82421070(ctx, base);
	// 82417204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82417208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241720C: 4811DF00  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417210 size=88
    let mut pc: u32 = 0x82417210;
    'dispatch: loop {
        match pc {
            0x82417210 => {
    //   block [0x82417210..0x82417268)
	// 82417210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241721C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417224: 48009E0D  bl 0x82421030
	ctx.lr = 0x82417228;
	sub_82421030(ctx, base);
	// 82417228: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241722C: 409A0014  bne cr6, 0x82417240
	if !ctx.cr[6].eq {
	pc = 0x82417240; continue 'dispatch;
	}
	// 82417230: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417234: 386BEECC  addi r3, r11, -0x1134
	ctx.r[3].s64 = ctx.r[11].s64 + -4404;
	// 82417238: 4800A061  bl 0x82421298
	ctx.lr = 0x8241723C;
	sub_82421298(ctx, base);
	// 8241723C: 48000010  b 0x8241724c
	pc = 0x8241724C; continue 'dispatch;
	// 82417240: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417244: 4800A38D  bl 0x824215d0
	ctx.lr = 0x82417248;
	sub_824215D0(ctx, base);
	// 82417248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241724C: 48009E25  bl 0x82421070
	ctx.lr = 0x82417250;
	sub_82421070(ctx, base);
	// 82417250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82417258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241725C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417268 size=188
    let mut pc: u32 = 0x82417268;
    'dispatch: loop {
        match pc {
            0x82417268 => {
    //   block [0x82417268..0x82417324)
	// 82417268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82417274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241727C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417280: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82417284: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82417288: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8241728C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82417290: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82417294: 4BFFF66D  bl 0x82416900
	ctx.lr = 0x82417298;
	sub_82416900(ctx, base);
	// 82417298: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8241729C: 419A0070  beq cr6, 0x8241730c
	if ctx.cr[6].eq {
	pc = 0x8241730C; continue 'dispatch;
	}
	// 824172A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824172A4: 4BFFFDED  bl 0x82417090
	ctx.lr = 0x824172A8;
	sub_82417090(ctx, base);
	// 824172A8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824172AC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824172B0: 409A000C  bne cr6, 0x824172bc
	if !ctx.cr[6].eq {
	pc = 0x824172BC; continue 'dispatch;
	}
	// 824172B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824172B8: 4BFFFBA9  bl 0x82416e60
	ctx.lr = 0x824172BC;
	sub_82416E60(ctx, base);
	// 824172BC: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824172C0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824172C4: 41820020  beq 0x824172e4
	if ctx.cr[0].eq {
	pc = 0x824172E4; continue 'dispatch;
	}
	// 824172C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824172CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824172D0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824172D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824172D8: 4800B329  bl 0x82422600
	ctx.lr = 0x824172DC;
	sub_82422600(ctx, base);
	// 824172DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824172E0: 4800B391  bl 0x82422670
	ctx.lr = 0x824172E4;
	sub_82422670(ctx, base);
	// 824172E4: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 824172E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824172EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824172F0: 4811DEE1  bl 0x825351d0
	ctx.lr = 0x824172F4;
	sub_825351D0(ctx, base);
	// 824172F4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824172F8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 824172FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82417300: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82417304: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82417308: 4BFFF5F9  bl 0x82416900
	ctx.lr = 0x8241730C;
	sub_82416900(ctx, base);
	// 8241730C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82417310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241731C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417328 size=92
    let mut pc: u32 = 0x82417328;
    'dispatch: loop {
        match pc {
            0x82417328 => {
    //   block [0x82417328..0x82417384)
	// 82417328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241732C: 4811DD91  bl 0x825350bc
	ctx.lr = 0x82417330;
	sub_82535080(ctx, base);
	// 82417330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417334: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82417338: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241733C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82417340: 48009CF1  bl 0x82421030
	ctx.lr = 0x82417344;
	sub_82421030(ctx, base);
	// 82417344: 57EB07BF  clrlwi. r11, r31, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82417348: 41820018  beq 0x82417360
	if ctx.cr[0].eq {
	pc = 0x82417360; continue 'dispatch;
	}
	// 8241734C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417350: 386BEFCC  addi r3, r11, -0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + -4148;
	// 82417354: 48009F45  bl 0x82421298
	ctx.lr = 0x82417358;
	sub_82421298(ctx, base);
	// 82417358: 3BE0FFFD  li r31, -3
	ctx.r[31].s64 = -3;
	// 8241735C: 48000018  b 0x82417374
	pc = 0x82417374; continue 'dispatch;
	// 82417360: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82417364: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82417368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241736C: 4BFFF925  bl 0x82416c90
	ctx.lr = 0x82417370;
	sub_82416C90(ctx, base);
	// 82417370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417374: 48009CFD  bl 0x82421070
	ctx.lr = 0x82417378;
	sub_82421070(ctx, base);
	// 82417378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241737C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82417380: 4811DD8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417388 size=220
    let mut pc: u32 = 0x82417388;
    'dispatch: loop {
        match pc {
            0x82417388 => {
    //   block [0x82417388..0x82417464)
	// 82417388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82417394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241739C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824173A0: 48009C91  bl 0x82421030
	ctx.lr = 0x824173A4;
	sub_82421030(ctx, base);
	// 824173A4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824173A8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 824173AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824173B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824173B4: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 824173B8: 4BFFF549  bl 0x82416900
	ctx.lr = 0x824173BC;
	sub_82416900(ctx, base);
	// 824173BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824173C0: 409A0018  bne cr6, 0x824173d8
	if !ctx.cr[6].eq {
	pc = 0x824173D8; continue 'dispatch;
	}
	// 824173C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824173C8: 386BF07C  addi r3, r11, -0xf84
	ctx.r[3].s64 = ctx.r[11].s64 + -3972;
	// 824173CC: 48009ECD  bl 0x82421298
	ctx.lr = 0x824173D0;
	sub_82421298(ctx, base);
	// 824173D0: 3BC0FFFD  li r30, -3
	ctx.r[30].s64 = -3;
	// 824173D4: 48000070  b 0x82417444
	pc = 0x82417444; continue 'dispatch;
	// 824173D8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824173DC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824173E0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824173E4: 419A005C  beq cr6, 0x82417440
	if ctx.cr[6].eq {
	pc = 0x82417440; continue 'dispatch;
	}
	// 824173E8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824173EC: 409A0010  bne cr6, 0x824173fc
	if !ctx.cr[6].eq {
	pc = 0x824173FC; continue 'dispatch;
	}
	// 824173F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824173F4: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824173F8: 48000048  b 0x82417440
	pc = 0x82417440; continue 'dispatch;
	// 824173FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417400: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82417404: 40820018  bne 0x8241741c
	if !ctx.cr[0].eq {
	pc = 0x8241741C; continue 'dispatch;
	}
	// 82417408: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241740C: 386BF050  addi r3, r11, -0xfb0
	ctx.r[3].s64 = ctx.r[11].s64 + -4016;
	// 82417410: 48009E89  bl 0x82421298
	ctx.lr = 0x82417414;
	sub_82421298(ctx, base);
	// 82417414: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82417418: 4800002C  b 0x82417444
	pc = 0x82417444; continue 'dispatch;
	// 8241741C: 4800AF55  bl 0x82422370
	ctx.lr = 0x82417420;
	sub_82422370(ctx, base);
	// 82417420: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82417424: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82417428: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8241742C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82417430: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82417434: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82417438: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8241743C: 4BFFF4C5  bl 0x82416900
	ctx.lr = 0x82417440;
	sub_82416900(ctx, base);
	// 82417440: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82417444: 48009C2D  bl 0x82421070
	ctx.lr = 0x82417448;
	sub_82421070(ctx, base);
	// 82417448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241744C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82417450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417458: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241745C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417468 size=108
    let mut pc: u32 = 0x82417468;
    'dispatch: loop {
        match pc {
            0x82417468 => {
    //   block [0x82417468..0x824174D4)
	// 82417468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82417474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241747C: 48009BB5  bl 0x82421030
	ctx.lr = 0x82417480;
	sub_82421030(ctx, base);
	// 82417480: 48017AD1  bl 0x8242ef50
	ctx.lr = 0x82417484;
	sub_8242EF50(ctx, base);
	// 82417484: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417488: 3BCBC980  addi r30, r11, -0x3680
	ctx.r[30].s64 = ctx.r[11].s64 + -13952;
	// 8241748C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82417490: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82417494: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82417498: 409A000C  bne cr6, 0x824174a4
	if !ctx.cr[6].eq {
	pc = 0x824174A4; continue 'dispatch;
	}
	// 8241749C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824174A0: 4BFFFAB1  bl 0x82416f50
	ctx.lr = 0x824174A4;
	sub_82416F50(ctx, base);
	// 824174A4: 3BFF0034  addi r31, r31, 0x34
	ctx.r[31].s64 = ctx.r[31].s64 + 52;
	// 824174A8: 397E0340  addi r11, r30, 0x340
	ctx.r[11].s64 = ctx.r[30].s64 + 832;
	// 824174AC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824174B0: 4198FFE0  blt cr6, 0x82417490
	if ctx.cr[6].lt {
	pc = 0x82417490; continue 'dispatch;
	}
	// 824174B4: 48017A9D  bl 0x8242ef50
	ctx.lr = 0x824174B8;
	sub_8242EF50(ctx, base);
	// 824174B8: 48009BB9  bl 0x82421070
	ctx.lr = 0x824174BC;
	sub_82421070(ctx, base);
	// 824174BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824174C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824174C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824174C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824174CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824174D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824174D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824174D8 size=152
    let mut pc: u32 = 0x824174D8;
    'dispatch: loop {
        match pc {
            0x824174D8 => {
    //   block [0x824174D8..0x82417570)
	// 824174D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824174DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824174E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824174E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824174E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824174EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824174F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824174F4: 409A0018  bne cr6, 0x8241750c
	if !ctx.cr[6].eq {
	pc = 0x8241750C; continue 'dispatch;
	}
	// 824174F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824174FC: 386BF118  addi r3, r11, -0xee8
	ctx.r[3].s64 = ctx.r[11].s64 + -3816;
	// 82417500: 48009D99  bl 0x82421298
	ctx.lr = 0x82417504;
	sub_82421298(ctx, base);
	// 82417504: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82417508: 48000050  b 0x82417558
	pc = 0x82417558; continue 'dispatch;
	// 8241750C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82417510: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82417514: 617EFFFF  ori r30, r11, 0xffff
	ctx.r[30].u64 = ctx.r[11].u64 | 65535;
	// 82417518: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8241751C: 409A003C  bne cr6, 0x82417558
	if !ctx.cr[6].eq {
	pc = 0x82417558; continue 'dispatch;
	}
	// 82417520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417524: 4BFFFB6D  bl 0x82417090
	ctx.lr = 0x82417528;
	sub_82417090(ctx, base);
	// 82417528: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241752C: 4800ABD5  bl 0x82422100
	ctx.lr = 0x82417530;
	sub_82422100(ctx, base);
	// 82417530: 396307FF  addi r11, r3, 0x7ff
	ctx.r[11].s64 = ctx.r[3].s64 + 2047;
	// 82417534: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 82417538: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 8241753C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82417540: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 82417544: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 82417548: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8241754C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82417550: 41980008  blt cr6, 0x82417558
	if ctx.cr[6].lt {
	pc = 0x82417558; continue 'dispatch;
	}
	// 82417554: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82417558: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241755C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417564: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241756C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417570 size=92
    let mut pc: u32 = 0x82417570;
    'dispatch: loop {
        match pc {
            0x82417570 => {
    //   block [0x82417570..0x824175CC)
	// 82417570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241757C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417584: 4800B14D  bl 0x824226d0
	ctx.lr = 0x82417588;
	sub_824226D0(ctx, base);
	// 82417588: 48009AA9  bl 0x82421030
	ctx.lr = 0x8241758C;
	sub_82421030(ctx, base);
	// 8241758C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82417590: 409A0018  bne cr6, 0x824175a8
	if !ctx.cr[6].eq {
	pc = 0x824175A8; continue 'dispatch;
	}
	// 82417594: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417598: 386BF0A4  addi r3, r11, -0xf5c
	ctx.r[3].s64 = ctx.r[11].s64 + -3932;
	// 8241759C: 48009CFD  bl 0x82421298
	ctx.lr = 0x824175A0;
	sub_82421298(ctx, base);
	// 824175A0: 3BE0FFFD  li r31, -3
	ctx.r[31].s64 = -3;
	// 824175A4: 4800000C  b 0x824175b0
	pc = 0x824175B0; continue 'dispatch;
	// 824175A8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824175AC: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 824175B0: 48009AC1  bl 0x82421070
	ctx.lr = 0x824175B4;
	sub_82421070(ctx, base);
	// 824175B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824175B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824175BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824175C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824175C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824175C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824175D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824175D0 size=420
    let mut pc: u32 = 0x824175D0;
    'dispatch: loop {
        match pc {
            0x824175D0 => {
    //   block [0x824175D0..0x82417774)
	// 824175D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824175D4: 4811DAD1  bl 0x825350a4
	ctx.lr = 0x824175D8;
	sub_82535080(ctx, base);
	// 824175D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824175DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824175E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824175E4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 824175E8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 824175EC: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 824175F0: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 824175F4: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 824175F8: 4BFFFB11  bl 0x82417108
	ctx.lr = 0x824175FC;
	sub_82417108(ctx, base);
	// 824175FC: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82417600: 4080002C  bge 0x8241762c
	if !ctx.cr[0].lt {
	pc = 0x8241762C; continue 'dispatch;
	}
	// 82417604: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82417608: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8241760C: 419A0008  beq cr6, 0x82417614
	if ctx.cr[6].eq {
	pc = 0x82417614; continue 'dispatch;
	}
	// 82417610: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82417614: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82417618: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241761C: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82417620: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82417624: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82417628: 48000140  b 0x82417768
	pc = 0x82417768; continue 'dispatch;
	// 8241762C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417630: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417634: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 82417638: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8241763C: 897D000F  lbz r11, 0xf(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(15 as u32) ) } as u64;
	// 82417640: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82417644: 409A00AC  bne cr6, 0x824176f0
	if !ctx.cr[6].eq {
	pc = 0x824176F0; continue 'dispatch;
	}
	// 82417648: 395D0118  addi r10, r29, 0x118
	ctx.r[10].s64 = ctx.r[29].s64 + 280;
	// 8241764C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82417650: 7D695E70  srawi r9, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82417654: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82417658: 7D685E70  srawi r8, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8241765C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82417660: 55085828  slwi r8, r8, 0xb
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(11);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82417664: 7D685851  subf. r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82417668: 40810008  ble 0x82417670
	if !ctx.cr[0].gt {
	pc = 0x82417670; continue 'dispatch;
	}
	// 8241766C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82417670: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82417674: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 82417678: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8241767C: 40990040  ble cr6, 0x824176bc
	if !ctx.cr[6].gt {
	pc = 0x824176BC; continue 'dispatch;
	}
	// 82417680: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82417684: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82417688: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241768C: 7D275E70  srawi r7, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 82417690: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 82417694: 7D265E70  srawi r6, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 82417698: 7CC60194  addze r6, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[6].s64 = tmp.s64;
	// 8241769C: 54C65828  slwi r6, r6, 0xb
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(11);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824176A0: 7D264851  subf. r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824176A4: 40810008  ble 0x824176ac
	if !ctx.cr[0].gt {
	pc = 0x824176AC; continue 'dispatch;
	}
	// 824176A8: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 824176AC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824176B0: 7FC7F214  add r30, r7, r30
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 824176B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824176B8: 4082FFD0  bne 0x82417688
	if !ctx.cr[0].eq {
	pc = 0x82417688; continue 'dispatch;
	}
	// 824176BC: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824176C0: 7D6A402E  lwzx r11, r10, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824176C4: 7D695E70  srawi r9, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 824176C8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824176CC: 7D675E70  srawi r7, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 824176D0: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 824176D4: 54E75828  slwi r7, r7, 0xb
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(11);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824176D8: 7D675851  subf. r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824176DC: 40810008  ble 0x824176e4
	if !ctx.cr[0].gt {
	pc = 0x824176E4; continue 'dispatch;
	}
	// 824176E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824176E4: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824176E8: 7D6A402E  lwzx r11, r10, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824176EC: 48000048  b 0x82417734
	pc = 0x82417734; continue 'dispatch;
	// 824176F0: 397D0118  addi r11, r29, 0x118
	ctx.r[11].s64 = ctx.r[29].s64 + 280;
	// 824176F4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824176F8: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 824176FC: A3CB0000  lhz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82417700: 40990020  ble cr6, 0x82417720
	if !ctx.cr[6].gt {
	pc = 0x82417720; continue 'dispatch;
	}
	// 82417704: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82417708: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8241770C: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82417710: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82417714: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82417718: 7FC8F214  add r30, r8, r30
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 8241771C: 4082FFF0  bne 0x8241770c
	if !ctx.cr[0].eq {
	pc = 0x8241770C; continue 'dispatch;
	}
	// 82417720: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82417724: 7D4B4A2E  lhzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82417728: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241772C: 7D6B4A2E  lhzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82417730: 556B583E  rotlwi r11, r11, 0xb
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(11)) as u64;
	// 82417734: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82417738: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8241773C: 419A0018  beq cr6, 0x82417754
	if ctx.cr[6].eq {
	pc = 0x82417754; continue 'dispatch;
	}
	// 82417740: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 82417744: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 82417748: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 8241774C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82417750: 4800BE09  bl 0x82423558
	ctx.lr = 0x82417754;
	sub_82423558(ctx, base);
	// 82417754: 817D0110  lwz r11, 0x110(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(272 as u32) ) } as u64;
	// 82417758: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241775C: 817D0114  lwz r11, 0x114(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(276 as u32) ) } as u64;
	// 82417760: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82417764: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82417768: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8241776C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82417770: 4811D984  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417778 size=68
    let mut pc: u32 = 0x82417778;
    'dispatch: loop {
        match pc {
            0x82417778 => {
    //   block [0x82417778..0x824177BC)
	// 82417778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241778C: 480098A5  bl 0x82421030
	ctx.lr = 0x82417790;
	sub_82421030(ctx, base);
	// 82417790: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417794: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417798: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 8241779C: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824177A0: 480098D1  bl 0x82421070
	ctx.lr = 0x824177A4;
	sub_82421070(ctx, base);
	// 824177A4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 824177A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824177AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824177B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824177B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824177B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824177C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824177C0 size=72
    let mut pc: u32 = 0x824177C0;
    'dispatch: loop {
        match pc {
            0x824177C0 => {
    //   block [0x824177C0..0x82417808)
	// 824177C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824177C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824177C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824177CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824177D0: 3FE08313  lis r31, -0x7ced
	ctx.r[31].s64 = -2095906816;
	// 824177D4: 807FC560  lwz r3, -0x3aa0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 824177D8: 4BFFFA91  bl 0x82417268
	ctx.lr = 0x824177DC;
	sub_82417268(ctx, base);
	// 824177DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824177E0: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824177E4: 917FC560  stw r11, -0x3aa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-15008 as u32), ctx.r[11].u32 ) };
	// 824177E8: 916AC564  stw r11, -0x3a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15004 as u32), ctx.r[11].u32 ) };
	// 824177EC: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 824177F0: 916AF218  stw r11, -0xde8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3560 as u32), ctx.r[11].u32 ) };
	// 824177F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824177F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824177FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82417808 size=984
    let mut pc: u32 = 0x82417808;
    'dispatch: loop {
        match pc {
            0x82417808 => {
    //   block [0x82417808..0x82417BE0)
	// 82417808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241780C: 4811D895  bl 0x825350a0
	ctx.lr = 0x82417810;
	sub_82535080(ctx, base);
	// 82417810: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417814: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417818: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8241781C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82417820: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82417824: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82417828: 816BC400  lwz r11, -0x3c00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15360 as u32) ) } as u64;
	// 8241782C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82417830: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82417834: 419A0018  beq cr6, 0x8241784c
	if ctx.cr[6].eq {
	pc = 0x8241784C; continue 'dispatch;
	}
	// 82417838: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241783C: 386BF224  addi r3, r11, -0xddc
	ctx.r[3].s64 = ctx.r[11].s64 + -3548;
	// 82417840: 48009A59  bl 0x82421298
	ctx.lr = 0x82417844;
	sub_82421298(ctx, base);
	// 82417844: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82417848: 48000390  b 0x82417bd8
	pc = 0x82417BD8; continue 'dispatch;
	// 8241784C: 3F008313  lis r24, -0x7ced
	ctx.r[24].s64 = -2095906816;
	// 82417850: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82417854: 3ACAF0A4  addi r22, r10, -0xf5c
	ctx.r[22].s64 = ctx.r[10].s64 + -3932;
	// 82417858: 8078C560  lwz r3, -0x3aa0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 8241785C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82417860: 409A005C  bne cr6, 0x824178bc
	if !ctx.cr[6].eq {
	pc = 0x824178BC; continue 'dispatch;
	}
	// 82417864: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82417868: 419A0010  beq cr6, 0x82417878
	if ctx.cr[6].eq {
	pc = 0x82417878; continue 'dispatch;
	}
	// 8241786C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417870: 806BC404  lwz r3, -0x3bfc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15356 as u32) ) } as u64;
	// 82417874: 48000364  b 0x82417bd8
	pc = 0x82417BD8; continue 'dispatch;
	// 82417878: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8241787C: 48009A1D  bl 0x82421298
	ctx.lr = 0x82417880;
	sub_82421298(ctx, base);
	// 82417880: 8078C560  lwz r3, -0x3aa0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 82417884: 3BE0FFFD  li r31, -3
	ctx.r[31].s64 = -3;
	// 82417888: 4BFFF989  bl 0x82417210
	ctx.lr = 0x8241788C;
	sub_82417210(ctx, base);
	// 8241788C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82417890: 4182FFDC  beq 0x8241786c
	if ctx.cr[0].eq {
	pc = 0x8241786C; continue 'dispatch;
	}
	// 82417894: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 82417898: 409A006C  bne cr6, 0x82417904
	if !ctx.cr[6].eq {
	pc = 0x82417904; continue 'dispatch;
	}
	// 8241789C: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824178A0: 80ABF214  lwz r5, -0xdec(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3564 as u32) ) } as u64;
	// 824178A4: 54AB07BF  clrlwi. r11, r5, 0x1e
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824178A8: 41820038  beq 0x824178e0
	if ctx.cr[0].eq {
	pc = 0x824178E0; continue 'dispatch;
	}
	// 824178AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824178B0: 386BEFCC  addi r3, r11, -0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + -4148;
	// 824178B4: 480099E5  bl 0x82421298
	ctx.lr = 0x824178B8;
	sub_82421298(ctx, base);
	// 824178B8: 48000040  b 0x824178f8
	pc = 0x824178F8; continue 'dispatch;
	// 824178BC: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 824178C0: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 824178C4: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 824178C8: 409AFFC0  bne cr6, 0x82417888
	if !ctx.cr[6].eq {
	pc = 0x82417888; continue 'dispatch;
	}
	// 824178CC: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824178D0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 824178D4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 824178D8: 916AC404  stw r11, -0x3bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 824178DC: 480002FC  b 0x82417bd8
	pc = 0x82417BD8; continue 'dispatch;
	// 824178E0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824178E4: 8078C560  lwz r3, -0x3aa0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 824178E8: 808BF218  lwz r4, -0xde8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3560 as u32) ) } as u64;
	// 824178EC: 4BFFF3A5  bl 0x82416c90
	ctx.lr = 0x824178F0;
	sub_82416C90(ctx, base);
	// 824178F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824178F4: 4080FF78  bge 0x8241786c
	if !ctx.cr[0].lt {
	pc = 0x8241786C; continue 'dispatch;
	}
	// 824178F8: 8078C560  lwz r3, -0x3aa0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 824178FC: 4BFFF96D  bl 0x82417268
	ctx.lr = 0x82417900;
	sub_82417268(ctx, base);
	// 82417900: 4BFFFFCC  b 0x824178cc
	pc = 0x824178CC; continue 'dispatch;
	// 82417904: 3F408313  lis r26, -0x7ced
	ctx.r[26].s64 = -2095906816;
	// 82417908: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 8241790C: 93FAC404  stw r31, -0x3bfc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-15356 as u32), ctx.r[31].u32 ) };
	// 82417910: 419A000C  beq cr6, 0x8241791c
	if ctx.cr[6].eq {
	pc = 0x8241791C; continue 'dispatch;
	}
	// 82417914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417918: 480002C0  b 0x82417bd8
	pc = 0x82417BD8; continue 'dispatch;
	// 8241791C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417920: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417924: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 82417928: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8241792C: 8965000F  lbz r11, 0xf(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(15 as u32) ) } as u64;
	// 82417930: 7D660774  extsb r6, r11
	ctx.r[6].s64 = ctx.r[11].s8 as i64;
	// 82417934: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 82417938: 409A0010  bne cr6, 0x82417948
	if !ctx.cr[6].eq {
	pc = 0x82417948; continue 'dispatch;
	}
	// 8241793C: 3B650118  addi r27, r5, 0x118
	ctx.r[27].s64 = ctx.r[5].s64 + 280;
	// 82417940: 3B3B0004  addi r25, r27, 4
	ctx.r[25].s64 = ctx.r[27].s64 + 4;
	// 82417944: 4800000C  b 0x82417950
	pc = 0x82417950; continue 'dispatch;
	// 82417948: 3B850118  addi r28, r5, 0x118
	ctx.r[28].s64 = ctx.r[5].s64 + 280;
	// 8241794C: 3AFC0002  addi r23, r28, 2
	ctx.r[23].s64 = ctx.r[28].s64 + 2;
	// 82417950: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82417954: 3FA08289  lis r29, -0x7d77
	ctx.r[29].s64 = -2104950784;
	// 82417958: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241795C: 409A0120  bne cr6, 0x82417a7c
	if !ctx.cr[6].eq {
	pc = 0x82417A7C; continue 'dispatch;
	}
	// 82417960: 813DF214  lwz r9, -0xdec(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-3564 as u32) ) } as u64;
	// 82417964: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82417968: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8241796C: 394AF114  addi r10, r10, -0xeec
	ctx.r[10].s64 = ctx.r[10].s64 + -3820;
	// 82417970: 390B0003  addi r8, r11, 3
	ctx.r[8].s64 = ctx.r[11].s64 + 3;
	// 82417974: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82417978: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241797C: 7CE43851  subf. r7, r4, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82417980: 40820014  bne 0x82417994
	if !ctx.cr[0].eq {
	pc = 0x82417994; continue 'dispatch;
	}
	// 82417984: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82417988: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8241798C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82417990: 409AFFE4  bne cr6, 0x82417974
	if !ctx.cr[6].eq {
	pc = 0x82417974; continue 'dispatch;
	}
	// 82417994: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82417998: 41820010  beq 0x824179a8
	if ctx.cr[0].eq {
	pc = 0x824179A8; continue 'dispatch;
	}
	// 8241799C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824179A0: 386BF1F0  addi r3, r11, -0xe10
	ctx.r[3].s64 = ctx.r[11].s64 + -3600;
	// 824179A4: 480001E0  b 0x82417b84
	pc = 0x82417B84; continue 'dispatch;
	// 824179A8: 89490007  lbz r10, 7(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 824179AC: 39690004  addi r11, r9, 4
	ctx.r[11].s64 = ctx.r[9].s64 + 4;
	// 824179B0: 89090006  lbz r8, 6(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 824179B4: 3CE00001  lis r7, 1
	ctx.r[7].s64 = 65536;
	// 824179B8: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 824179BC: 88890005  lbz r4, 5(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 824179C0: 89290004  lbz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824179C4: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 824179C8: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824179CC: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 824179D0: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824179D4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 824179D8: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 824179DC: 40990010  ble cr6, 0x824179ec
	if !ctx.cr[6].gt {
	pc = 0x824179EC; continue 'dispatch;
	}
	// 824179E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824179E4: 386BF1BC  addi r3, r11, -0xe44
	ctx.r[3].s64 = ctx.r[11].s64 + -3652;
	// 824179E8: 4800019C  b 0x82417b84
	pc = 0x82417B84; continue 'dispatch;
	// 824179EC: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824179F0: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 824179F4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824179F8: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 824179FC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82417A00: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82417A04: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82417A08: B165000C  sth r11, 0xc(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82417A0C: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82417A10: 409A0010  bne cr6, 0x82417a20
	if !ctx.cr[6].eq {
	pc = 0x82417A20; continue 'dispatch;
	}
	// 82417A14: 396A0048  addi r11, r10, 0x48
	ctx.r[11].s64 = ctx.r[10].s64 + 72;
	// 82417A18: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82417A1C: 4800000C  b 0x82417a28
	pc = 0x82417A28; continue 'dispatch;
	// 82417A20: 396A008E  addi r11, r10, 0x8e
	ctx.r[11].s64 = ctx.r[10].s64 + 142;
	// 82417A24: 556B083A  rlwinm r11, r11, 1, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82417A28: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82417A2C: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 82417A30: 817DF214  lwz r11, -0xdec(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-3564 as u32) ) } as u64;
	// 82417A34: 894B000B  lbz r10, 0xb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82417A38: 892B000A  lbz r9, 0xa(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82417A3C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82417A40: 890B0009  lbz r8, 9(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(9 as u32) ) } as u64;
	// 82417A44: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82417A48: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82417A4C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417A50: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82417A54: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417A58: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82417A5C: 409A000C  bne cr6, 0x82417a68
	if !ctx.cr[6].eq {
	pc = 0x82417A68; continue 'dispatch;
	}
	// 82417A60: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82417A64: 48000010  b 0x82417a74
	pc = 0x82417A74; continue 'dispatch;
	// 82417A68: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82417A6C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82417A70: B17C0000  sth r11, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82417A74: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 82417A78: 48000008  b 0x82417a80
	pc = 0x82417A80; continue 'dispatch;
	// 82417A7C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82417A80: 3F808289  lis r28, -0x7d77
	ctx.r[28].s64 = -2104950784;
	// 82417A84: 809CF218  lwz r4, -0xde8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-3560 as u32) ) } as u64;
	// 82417A88: 548B5828  slwi r11, r4, 0xb
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82417A8C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82417A90: 7FCB0194  addze r30, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[30].s64 = tmp.s64;
	// 82417A94: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82417A98: 409800D8  bge cr6, 0x82417b70
	if !ctx.cr[6].lt {
	pc = 0x82417B70; continue 'dispatch;
	}
	// 82417A9C: 3CE08313  lis r7, -0x7ced
	ctx.r[7].s64 = -2095906816;
	// 82417AA0: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82417AA4: 8147C564  lwz r10, -0x3a9c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-15004 as u32) ) } as u64;
	// 82417AA8: 8965000F  lbz r11, 0xf(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(15 as u32) ) } as u64;
	// 82417AAC: 811DF214  lwz r8, -0xdec(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-3564 as u32) ) } as u64;
	// 82417AB0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82417AB4: 7D694214  add r11, r9, r8
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82417AB8: 7D0940AE  lbzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82417ABC: 88CB0003  lbz r6, 3(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82417AC0: 888B0002  lbz r4, 2(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82417AC4: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82417AC8: 896B0001  lbz r11, 1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82417ACC: 7CC62378  or r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[4].u64;
	// 82417AD0: 54C6402E  slwi r6, r6, 8
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82417AD4: 7CCB5B78  or r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 | ctx.r[11].u64;
	// 82417AD8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82417ADC: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82417AE0: 409A0010  bne cr6, 0x82417af0
	if !ctx.cr[6].eq {
	pc = 0x82417AF0; continue 'dispatch;
	}
	// 82417AE4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82417AE8: 7D6AC92E  stwx r11, r10, r25
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32), ctx.r[11].u32) };
	// 82417AEC: 48000034  b 0x82417b20
	pc = 0x82417B20; continue 'dispatch;
	// 82417AF0: 7D685E70  srawi r8, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82417AF4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82417AF8: 7D665E70  srawi r6, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82417AFC: 7CC60194  addze r6, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[6].s64 = tmp.s64;
	// 82417B00: 54C65828  slwi r6, r6, 0xb
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(11);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82417B04: 7D665851  subf. r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82417B08: 40810008  ble 0x82417b10
	if !ctx.cr[0].gt {
	pc = 0x82417B10; continue 'dispatch;
	}
	// 82417B0C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82417B10: 550B001F  rlwinm. r11, r8, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82417B14: 40820038  bne 0x82417b4c
	if !ctx.cr[0].eq {
	pc = 0x82417B4C; continue 'dispatch;
	}
	// 82417B18: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82417B1C: 7D0BBB2E  sthx r8, r11, r23
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32), ctx.r[8].u16) };
	// 82417B20: 8167C564  lwz r11, -0x3a9c(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-15004 as u32) ) } as u64;
	// 82417B24: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82417B28: 9147C564  stw r10, -0x3a9c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-15004 as u32), ctx.r[10].u32 ) };
	// 82417B2C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82417B30: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82417B34: 40980024  bge cr6, 0x82417b58
	if !ctx.cr[6].lt {
	pc = 0x82417B58; continue 'dispatch;
	}
	// 82417B38: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82417B3C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82417B40: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82417B44: 4198FF64  blt cr6, 0x82417aa8
	if ctx.cr[6].lt {
	pc = 0x82417AA8; continue 'dispatch;
	}
	// 82417B48: 4800001C  b 0x82417b64
	pc = 0x82417B64; continue 'dispatch;
	// 82417B4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417B50: 386BF178  addi r3, r11, -0xe88
	ctx.r[3].s64 = ctx.r[11].s64 + -3720;
	// 82417B54: 48000030  b 0x82417b84
	pc = 0x82417B84; continue 'dispatch;
	// 82417B58: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82417B5C: 917AC404  stw r11, -0x3bfc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82417B60: 4BFFFC61  bl 0x824177c0
	ctx.lr = 0x82417B64;
	sub_824177C0(ctx, base);
	// 82417B64: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82417B68: 4198006C  blt cr6, 0x82417bd4
	if ctx.cr[6].lt {
	pc = 0x82417BD4; continue 'dispatch;
	}
	// 82417B6C: 809CF218  lwz r4, -0xde8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-3560 as u32) ) } as u64;
	// 82417B70: 80BDF214  lwz r5, -0xdec(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-3564 as u32) ) } as u64;
	// 82417B74: 54AB07BF  clrlwi. r11, r5, 0x1e
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82417B78: 41820014  beq 0x82417b8c
	if ctx.cr[0].eq {
	pc = 0x82417B8C; continue 'dispatch;
	}
	// 82417B7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417B80: 386BEFCC  addi r3, r11, -0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + -4148;
	// 82417B84: 48009715  bl 0x82421298
	ctx.lr = 0x82417B88;
	sub_82421298(ctx, base);
	// 82417B88: 48000014  b 0x82417b9c
	pc = 0x82417B9C; continue 'dispatch;
	// 82417B8C: 8078C560  lwz r3, -0x3aa0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 82417B90: 4BFFF101  bl 0x82416c90
	ctx.lr = 0x82417B94;
	sub_82416C90(ctx, base);
	// 82417B94: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82417B98: 40800014  bge 0x82417bac
	if !ctx.cr[0].lt {
	pc = 0x82417BAC; continue 'dispatch;
	}
	// 82417B9C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82417BA0: 917AC404  stw r11, -0x3bfc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82417BA4: 4BFFFC1D  bl 0x824177c0
	ctx.lr = 0x82417BA8;
	sub_824177C0(ctx, base);
	// 82417BA8: 4800002C  b 0x82417bd4
	pc = 0x82417BD4; continue 'dispatch;
	// 82417BAC: 8178C560  lwz r11, -0x3aa0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 82417BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82417BB4: 409A0014  bne cr6, 0x82417bc8
	if !ctx.cr[6].eq {
	pc = 0x82417BC8; continue 'dispatch;
	}
	// 82417BB8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82417BBC: 480096DD  bl 0x82421298
	ctx.lr = 0x82417BC0;
	sub_82421298(ctx, base);
	// 82417BC0: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 82417BC4: 4800000C  b 0x82417bd0
	pc = 0x82417BD0; continue 'dispatch;
	// 82417BC8: 896B0001  lbz r11, 1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82417BCC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82417BD0: 917AC404  stw r11, -0x3bfc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82417BD4: 807AC404  lwz r3, -0x3bfc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-15356 as u32) ) } as u64;
	// 82417BD8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82417BDC: 4811D514  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417BE0 size=200
    let mut pc: u32 = 0x82417BE0;
    'dispatch: loop {
        match pc {
            0x82417BE0 => {
    //   block [0x82417BE0..0x82417CA8)
	// 82417BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417BE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82417BEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417BF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82417BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417BFC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82417C00: 3921005C  addi r9, r1, 0x5c
	ctx.r[9].s64 = ctx.r[1].s64 + 92;
	// 82417C04: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82417C08: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82417C0C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82417C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82417C14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82417C18: 4BFFF9B9  bl 0x824175d0
	ctx.lr = 0x82417C1C;
	sub_824175D0(ctx, base);
	// 82417C1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82417C20: 4080000C  bge 0x82417c2c
	if !ctx.cr[0].lt {
	pc = 0x82417C2C; continue 'dispatch;
	}
	// 82417C24: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82417C28: 48000068  b 0x82417c90
	pc = 0x82417C90; continue 'dispatch;
	// 82417C2C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417C30: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82417C34: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82417C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82417C3C: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 82417C40: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82417C44: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82417C48: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417C4C: E8E1005E  lwa r7, 0x5c(r1)
	ctx.r[7].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as i32) as i64;
	// 82417C50: 90DF002C  stw r6, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82417C54: 90BF0030  stw r5, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82417C58: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82417C5C: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82417C60: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82417C64: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82417C68: 4800A871  bl 0x824224d8
	ctx.lr = 0x82417C6C;
	sub_824224D8(ctx, base);
	// 82417C6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417C70: 48009C11  bl 0x82421880
	ctx.lr = 0x82417C74;
	sub_82421880(ctx, base);
	// 82417C74: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82417C78: 409A0014  bne cr6, 0x82417c8c
	if !ctx.cr[6].eq {
	pc = 0x82417C8C; continue 'dispatch;
	}
	// 82417C7C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417C80: 4800A981  bl 0x82422600
	ctx.lr = 0x82417C84;
	sub_82422600(ctx, base);
	// 82417C84: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82417C88: 48000008  b 0x82417c90
	pc = 0x82417C90; continue 'dispatch;
	// 82417C8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82417C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82417C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417C9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417CA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417CA8 size=168
    let mut pc: u32 = 0x82417CA8;
    'dispatch: loop {
        match pc {
            0x82417CA8 => {
    //   block [0x82417CA8..0x82417D50)
	// 82417CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417CB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82417CB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417CBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82417CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417CC4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82417CC8: 3921005C  addi r9, r1, 0x5c
	ctx.r[9].s64 = ctx.r[1].s64 + 92;
	// 82417CCC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82417CD0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82417CD4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82417CD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82417CDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82417CE0: 4BFFF8F1  bl 0x824175d0
	ctx.lr = 0x82417CE4;
	sub_824175D0(ctx, base);
	// 82417CE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82417CE8: 4080000C  bge 0x82417cf4
	if !ctx.cr[0].lt {
	pc = 0x82417CF4; continue 'dispatch;
	}
	// 82417CEC: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82417CF0: 48000048  b 0x82417d38
	pc = 0x82417D38; continue 'dispatch;
	// 82417CF4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417CF8: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82417CFC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82417D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82417D04: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 82417D08: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82417D0C: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82417D10: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417D14: E8E1005E  lwa r7, 0x5c(r1)
	ctx.r[7].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as i32) as i64;
	// 82417D18: 90DF002C  stw r6, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82417D1C: 90BF0030  stw r5, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82417D20: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82417D24: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82417D28: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82417D2C: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82417D30: 4800A4D1  bl 0x82422200
	ctx.lr = 0x82417D34;
	sub_82422200(ctx, base);
	// 82417D34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82417D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82417D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417D44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417D48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417D50 size=56
    let mut pc: u32 = 0x82417D50;
    'dispatch: loop {
        match pc {
            0x82417D50 => {
    //   block [0x82417D50..0x82417D88)
	// 82417D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417D64: 480092CD  bl 0x82421030
	ctx.lr = 0x82417D68;
	sub_82421030(ctx, base);
	// 82417D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417D6C: 4BFFF4FD  bl 0x82417268
	ctx.lr = 0x82417D70;
	sub_82417268(ctx, base);
	// 82417D70: 48009301  bl 0x82421070
	ctx.lr = 0x82417D74;
	sub_82421070(ctx, base);
	// 82417D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82417D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417D88 size=100
    let mut pc: u32 = 0x82417D88;
    'dispatch: loop {
        match pc {
            0x82417D88 => {
    //   block [0x82417D88..0x82417DEC)
	// 82417D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82417D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417D9C: 48009295  bl 0x82421030
	ctx.lr = 0x82417DA0;
	sub_82421030(ctx, base);
	// 82417DA0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82417DA4: 3BCBC980  addi r30, r11, -0x3680
	ctx.r[30].s64 = ctx.r[11].s64 + -13952;
	// 82417DA8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82417DAC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82417DB0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82417DB4: 409A000C  bne cr6, 0x82417dc0
	if !ctx.cr[6].eq {
	pc = 0x82417DC0; continue 'dispatch;
	}
	// 82417DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417DBC: 4BFFF4AD  bl 0x82417268
	ctx.lr = 0x82417DC0;
	sub_82417268(ctx, base);
	// 82417DC0: 3BFF0034  addi r31, r31, 0x34
	ctx.r[31].s64 = ctx.r[31].s64 + 52;
	// 82417DC4: 397E0340  addi r11, r30, 0x340
	ctx.r[11].s64 = ctx.r[30].s64 + 832;
	// 82417DC8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82417DCC: 4198FFE0  blt cr6, 0x82417dac
	if ctx.cr[6].lt {
	pc = 0x82417DAC; continue 'dispatch;
	}
	// 82417DD0: 480092A1  bl 0x82421070
	ctx.lr = 0x82417DD4;
	sub_82421070(ctx, base);
	// 82417DD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82417DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417DE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82417DE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417DF0 size=64
    let mut pc: u32 = 0x82417DF0;
    'dispatch: loop {
        match pc {
            0x82417DF0 => {
    //   block [0x82417DF0..0x82417E30)
	// 82417DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417DF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417DFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417E00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417E04: 4800922D  bl 0x82421030
	ctx.lr = 0x82417E08;
	sub_82421030(ctx, base);
	// 82417E08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417E0C: 4BFFF6CD  bl 0x824174d8
	ctx.lr = 0x82417E10;
	sub_824174D8(ctx, base);
	// 82417E10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417E14: 4800925D  bl 0x82421070
	ctx.lr = 0x82417E18;
	sub_82421070(ctx, base);
	// 82417E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82417E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417E30 size=120
    let mut pc: u32 = 0x82417E30;
    'dispatch: loop {
        match pc {
            0x82417E30 => {
    //   block [0x82417E30..0x82417EA8)
	// 82417E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82417E38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82417E3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417E40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417E44: 480091ED  bl 0x82421030
	ctx.lr = 0x82417E48;
	sub_82421030(ctx, base);
	// 82417E48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82417E4C: 409A0018  bne cr6, 0x82417e64
	if !ctx.cr[6].eq {
	pc = 0x82417E64; continue 'dispatch;
	}
	// 82417E50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417E54: 386BF144  addi r3, r11, -0xebc
	ctx.r[3].s64 = ctx.r[11].s64 + -3772;
	// 82417E58: 48009441  bl 0x82421298
	ctx.lr = 0x82417E5C;
	sub_82421298(ctx, base);
	// 82417E5C: 3BE0FFFD  li r31, -3
	ctx.r[31].s64 = -3;
	// 82417E60: 4800002C  b 0x82417e8c
	pc = 0x82417E8C; continue 'dispatch;
	// 82417E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417E68: 4BFFF229  bl 0x82417090
	ctx.lr = 0x82417E6C;
	sub_82417090(ctx, base);
	// 82417E6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82417E70: 4800A291  bl 0x82422100
	ctx.lr = 0x82417E74;
	sub_82422100(ctx, base);
	// 82417E74: 3960F800  li r11, -0x800
	ctx.r[11].s64 = -2048;
	// 82417E78: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82417E7C: 796B0580  clrldi r11, r11, 0x16
	ctx.r[11].u64 = ctx.r[11].u64 & 0x000003FFFFFFFFFFu64;
	// 82417E80: 7F235800  cmpd cr6, r3, r11
	ctx.cr[6].compare_i64(ctx.r[3].s64, ctx.r[11].s64, &mut ctx.xer);
	// 82417E84: 40980008  bge cr6, 0x82417e8c
	if !ctx.cr[6].lt {
	pc = 0x82417E8C; continue 'dispatch;
	}
	// 82417E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417E8C: 480091E5  bl 0x82421070
	ctx.lr = 0x82417E90;
	sub_82421070(ctx, base);
	// 82417E90: 7FE307B4  extsw r3, r31
	ctx.r[3].s64 = ctx.r[31].s32 as i64;
	// 82417E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82417E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82417E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82417EA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82417EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82417EA8 size=92
    let mut pc: u32 = 0x82417EA8;
    'dispatch: loop {
        match pc {
            0x82417EA8 => {
    //   block [0x82417EA8..0x82417F04)
	// 82417EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417EAC: 4811D205  bl 0x825350b0
	ctx.lr = 0x82417EB0;
	sub_82535080(ctx, base);
	// 82417EB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417EB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82417EBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82417EC0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82417EC4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82417EC8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82417ECC: 48009165  bl 0x82421030
	ctx.lr = 0x82417ED0;
	sub_82421030(ctx, base);
	// 82417ED0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82417ED4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82417ED8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82417EDC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82417EE0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82417EE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82417EE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417EEC: 4BFFF6E5  bl 0x824175d0
	ctx.lr = 0x82417EF0;
	sub_824175D0(ctx, base);
	// 82417EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82417EF4: 4800917D  bl 0x82421070
	ctx.lr = 0x82417EF8;
	sub_82421070(ctx, base);
	// 82417EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82417EFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82417F00: 4811D200  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82417F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82417F08 size=696
    let mut pc: u32 = 0x82417F08;
    'dispatch: loop {
        match pc {
            0x82417F08 => {
    //   block [0x82417F08..0x824181C0)
	// 82417F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82417F0C: 4811D181  bl 0x8253508c
	ctx.lr = 0x82417F10;
	sub_82535080(ctx, base);
	// 82417F10: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82417F14: 3EA08313  lis r21, -0x7ced
	ctx.r[21].s64 = -2095906816;
	// 82417F18: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82417F1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82417F20: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82417F24: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82417F28: 8175C404  lwz r11, -0x3bfc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-15356 as u32) ) } as u64;
	// 82417F2C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82417F30: 93810114  stw r28, 0x114(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), ctx.r[28].u32 ) };
	// 82417F34: 7D164378  mr r22, r8
	ctx.r[22].u64 = ctx.r[8].u64;
	// 82417F38: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82417F3C: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82417F40: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82417F44: 409A0018  bne cr6, 0x82417f5c
	if !ctx.cr[6].eq {
	pc = 0x82417F5C; continue 'dispatch;
	}
	// 82417F48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417F4C: 386BF354  addi r3, r11, -0xcac
	ctx.r[3].s64 = ctx.r[11].s64 + -3244;
	// 82417F50: 48009349  bl 0x82421298
	ctx.lr = 0x82417F54;
	sub_82421298(ctx, base);
	// 82417F54: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82417F58: 48000260  b 0x824181b8
	pc = 0x824181B8; continue 'dispatch;
	// 82417F5C: 3E808313  lis r20, -0x7ced
	ctx.r[20].s64 = -2095906816;
	// 82417F60: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82417F64: 8074C560  lwz r3, -0x3aa0(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-15008 as u32) ) } as u64;
	// 82417F68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82417F6C: 419A0010  beq cr6, 0x82417f7c
	if ctx.cr[6].eq {
	pc = 0x82417F7C; continue 'dispatch;
	}
	// 82417F70: 4BFFF2F9  bl 0x82417268
	ctx.lr = 0x82417F74;
	sub_82417268(ctx, base);
	// 82417F74: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82417F78: 9174C560  stw r11, -0x3aa0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(-15008 as u32), ctx.r[11].u32 ) };
	// 82417F7C: 3A60FFFF  li r19, -1
	ctx.r[19].s64 = -1;
	// 82417F80: 82410134  lwz r18, 0x134(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82417F84: 3FC08313  lis r30, -0x7ced
	ctx.r[30].s64 = -2095906816;
	// 82417F88: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82417F8C: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 82417F90: 917EC400  stw r11, -0x3c00(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-15360 as u32), ctx.r[11].u32 ) };
	// 82417F94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82417F98: 9175C404  stw r11, -0x3bfc(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82417F9C: 409A0020  bne cr6, 0x82417fbc
	if !ctx.cr[6].eq {
	pc = 0x82417FBC; continue 'dispatch;
	}
	// 82417FA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417FA4: 386BF328  addi r3, r11, -0xcd8
	ctx.r[3].s64 = ctx.r[11].s64 + -3288;
	// 82417FA8: 480092F1  bl 0x82421298
	ctx.lr = 0x82417FAC;
	sub_82421298(ctx, base);
	// 82417FAC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82417FB0: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82417FB4: 9175C404  stw r11, -0x3bfc(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82417FB8: 48000200  b 0x824181b8
	pc = 0x824181B8; continue 'dispatch;
	// 82417FBC: 8221013C  lwz r17, 0x13c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(316 as u32) ) } as u64;
	// 82417FC0: 2F110000  cmpwi cr6, r17, 0
	ctx.cr[6].compare_i32(ctx.r[17].s32, 0, &mut ctx.xer);
	// 82417FC4: 41990020  bgt cr6, 0x82417fe4
	if ctx.cr[6].gt {
	pc = 0x82417FE4; continue 'dispatch;
	}
	// 82417FC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82417FCC: 386BF300  addi r3, r11, -0xd00
	ctx.r[3].s64 = ctx.r[11].s64 + -3328;
	// 82417FD0: 480092C9  bl 0x82421298
	ctx.lr = 0x82417FD4;
	sub_82421298(ctx, base);
	// 82417FD4: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 82417FD8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82417FDC: 9175C404  stw r11, -0x3bfc(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82417FE0: 480001D8  b 0x824181b8
	pc = 0x824181B8; continue 'dispatch;
	// 82417FE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82417FE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82417FEC: 4BFFE99D  bl 0x82416988
	ctx.lr = 0x82417FF0;
	sub_82416988(ctx, base);
	// 82417FF0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82417FF4: 4180FFE4  blt 0x82417fd8
	if ctx.cr[0].lt {
	pc = 0x82417FD8; continue 'dispatch;
	}
	// 82417FF8: 817EC400  lwz r11, -0x3c00(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-15360 as u32) ) } as u64;
	// 82417FFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82418000: 41980018  blt cr6, 0x82418018
	if ctx.cr[6].lt {
	pc = 0x82418018; continue 'dispatch;
	}
	// 82418004: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418008: 386BF2D0  addi r3, r11, -0xd30
	ctx.r[3].s64 = ctx.r[11].s64 + -3376;
	// 8241800C: 4800928D  bl 0x82421298
	ctx.lr = 0x82418010;
	sub_82421298(ctx, base);
	// 82418010: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82418014: 4BFFFFC4  b 0x82417fd8
	pc = 0x82417FD8; continue 'dispatch;
	// 82418018: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8241801C: 93BEC400  stw r29, -0x3c00(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-15360 as u32), ctx.r[29].u32 ) };
	// 82418020: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82418024: 38A0011C  li r5, 0x11c
	ctx.r[5].s64 = 284;
	// 82418028: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241802C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418030: 9175C404  stw r11, -0x3bfc(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(-15356 as u32), ctx.r[11].u32 ) };
	// 82418034: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82418038: 916AC564  stw r11, -0x3a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15004 as u32), ctx.r[11].u32 ) };
	// 8241803C: 4811D195  bl 0x825351d0
	ctx.lr = 0x82418040;
	sub_825351D0(ctx, base);
	// 82418040: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82418044: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82418048: 396BC580  addi r11, r11, -0x3a80
	ctx.r[11].s64 = ctx.r[11].s64 + -14976;
	// 8241804C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82418050: 7FEA592E  stwx r31, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u32) };
	// 82418054: 931F0000  stw r24, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82418058: 409A00A0  bne cr6, 0x824180f8
	if !ctx.cr[6].eq {
	pc = 0x824180F8; continue 'dispatch;
	}
	// 8241805C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82418060: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82418064: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82418068: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241806C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82418070: 4BFFE891  bl 0x82416900
	ctx.lr = 0x82418074;
	sub_82416900(ctx, base);
	// 82418074: 4BFFE9AD  bl 0x82416a20
	ctx.lr = 0x82418078;
	sub_82416A20(ctx, base);
	// 82418078: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241807C: 41820014  beq 0x82418090
	if ctx.cr[0].eq {
	pc = 0x82418090; continue 'dispatch;
	}
	// 82418080: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82418084: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82418088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241808C: 4BFFFC1D  bl 0x82417ca8
	ctx.lr = 0x82418090;
	sub_82417CA8(ctx, base);
	// 82418090: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82418094: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82418098: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8241809C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824180A0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824180A4: 4BFFE85D  bl 0x82416900
	ctx.lr = 0x824180A8;
	sub_82416900(ctx, base);
	// 824180A8: 93D4C560  stw r30, -0x3aa0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(-15008 as u32), ctx.r[30].u32 ) };
	// 824180AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824180B0: 409A0010  bne cr6, 0x824180c0
	if !ctx.cr[6].eq {
	pc = 0x824180C0; continue 'dispatch;
	}
	// 824180B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824180B8: 386BF2A4  addi r3, r11, -0xd5c
	ctx.r[3].s64 = ctx.r[11].s64 + -3420;
	// 824180BC: 4BFFFF50  b 0x8241800c
	pc = 0x8241800C; continue 'dispatch;
	// 824180C0: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 824180C4: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 824180C8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824180CC: 38C10114  addi r6, r1, 0x114
	ctx.r[6].s64 = ctx.r[1].s64 + 276;
	// 824180D0: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 824180D4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824180D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824180DC: 4BFFF4F5  bl 0x824175d0
	ctx.lr = 0x824180E0;
	sub_824175D0(ctx, base);
	// 824180E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824180E4: 4180FEF4  blt 0x82417fd8
	if ctx.cr[0].lt {
	pc = 0x82417FD8; continue 'dispatch;
	}
	// 824180E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824180EC: 83810114  lwz r28, 0x114(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 824180F0: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 824180F4: 48000094  b 0x82418188
	pc = 0x82418188; continue 'dispatch;
	// 824180F8: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 824180FC: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 82418100: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82418104: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82418108: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8241810C: 4800B44D  bl 0x82423558
	ctx.lr = 0x82418110;
	sub_82423558(ctx, base);
	// 82418110: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82418114: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82418118: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241811C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82418120: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82418124: 4BFFE7DD  bl 0x82416900
	ctx.lr = 0x82418128;
	sub_82416900(ctx, base);
	// 82418128: 4BFFE8F9  bl 0x82416a20
	ctx.lr = 0x8241812C;
	sub_82416A20(ctx, base);
	// 8241812C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82418130: 41820030  beq 0x82418160
	if ctx.cr[0].eq {
	pc = 0x82418160; continue 'dispatch;
	}
	// 82418134: 7EEB07B4  extsw r11, r23
	ctx.r[11].s64 = ctx.r[23].s32 as i64;
	// 82418138: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241813C: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82418140: 92DE002C  stw r22, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[22].u32 ) };
	// 82418144: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 82418148: 939E0030  stw r28, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[28].u32 ) };
	// 8241814C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82418150: 931E0010  stw r24, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[24].u32 ) };
	// 82418154: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82418158: 92FE000C  stw r23, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[23].u32 ) };
	// 8241815C: 4800A0A5  bl 0x82422200
	ctx.lr = 0x82418160;
	sub_82422200(ctx, base);
	// 82418160: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82418164: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82418168: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8241816C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418170: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82418174: 4BFFE78D  bl 0x82416900
	ctx.lr = 0x82418178;
	sub_82416900(ctx, base);
	// 82418178: 93D4C560  stw r30, -0x3aa0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(-15008 as u32), ctx.r[30].u32 ) };
	// 8241817C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82418180: 419AFF34  beq cr6, 0x824180b4
	if ctx.cr[6].eq {
	pc = 0x824180B4; continue 'dispatch;
	}
	// 82418184: 92DF0114  stw r22, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[22].u32 ) };
	// 82418188: 81610144  lwz r11, 0x144(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 8241818C: 7E295E70  srawi r9, r17, 0xb
	ctx.xer.ca = (ctx.r[17].s32 < 0) && ((ctx.r[17].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[17].s32 >> 11) as i64;
	// 82418190: 939F0110  stw r28, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[28].u32 ) };
	// 82418194: 3D408289  lis r10, -0x7d77
	ctx.r[10].s64 = -2104950784;
	// 82418198: 9B1F000E  stb r24, 0xe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[24].u8 ) };
	// 8241819C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824181A0: 997F000F  stb r11, 0xf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 824181A4: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824181A8: 924BF214  stw r18, -0xdec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3564 as u32), ctx.r[18].u32 ) };
	// 824181AC: 7D690194  addze r11, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824181B0: 916AF218  stw r11, -0xde8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3560 as u32), ctx.r[11].u32 ) };
	// 824181B4: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 824181B8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824181BC: 4811CF20  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824181C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824181C0 size=64
    let mut pc: u32 = 0x824181C0;
    'dispatch: loop {
        match pc {
            0x824181C0 => {
    //   block [0x824181C0..0x82418200)
	// 824181C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824181C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824181C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824181CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824181D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824181D4: 48008E5D  bl 0x82421030
	ctx.lr = 0x824181D8;
	sub_82421030(ctx, base);
	// 824181D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824181DC: 4BFFF62D  bl 0x82417808
	ctx.lr = 0x824181E0;
	sub_82417808(ctx, base);
	// 824181E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824181E4: 48008E8D  bl 0x82421070
	ctx.lr = 0x824181E8;
	sub_82421070(ctx, base);
	// 824181E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824181EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824181F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824181F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824181F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824181FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418200 size=120
    let mut pc: u32 = 0x82418200;
    'dispatch: loop {
        match pc {
            0x82418200 => {
    //   block [0x82418200..0x82418278)
	// 82418200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241820C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418210: 4BFFEF71  bl 0x82417180
	ctx.lr = 0x82418214;
	sub_82417180(ctx, base);
	// 82418214: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82418218: 40820020  bne 0x82418238
	if !ctx.cr[0].eq {
	pc = 0x82418238; continue 'dispatch;
	}
	// 8241821C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82418220: 48000044  b 0x82418264
	pc = 0x82418264; continue 'dispatch;
	// 82418224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418228: 4BFFEFE9  bl 0x82417210
	ctx.lr = 0x8241822C;
	sub_82417210(ctx, base);
	// 8241822C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82418230: 40820030  bne 0x82418260
	if !ctx.cr[0].eq {
	pc = 0x82418260; continue 'dispatch;
	}
	// 82418234: 4800442D  bl 0x8241c660
	ctx.lr = 0x82418238;
	sub_8241C660(ctx, base);
	// 82418238: 48003EF9  bl 0x8241c130
	ctx.lr = 0x8241823C;
	sub_8241C130(ctx, base);
	// 8241823C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418240: 4BFFF331  bl 0x82417570
	ctx.lr = 0x82418244;
	sub_82417570(ctx, base);
	// 82418244: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82418248: 409AFFDC  bne cr6, 0x82418224
	if !ctx.cr[6].eq {
	pc = 0x82418224; continue 'dispatch;
	}
	// 8241824C: 48008DE5  bl 0x82421030
	ctx.lr = 0x82418250;
	sub_82421030(ctx, base);
	// 82418250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418254: 4BFFF015  bl 0x82417268
	ctx.lr = 0x82418258;
	sub_82417268(ctx, base);
	// 82418258: 48008E19  bl 0x82421070
	ctx.lr = 0x8241825C;
	sub_82421070(ctx, base);
	// 8241825C: 4BFFFFC0  b 0x8241821c
	pc = 0x8241821C; continue 'dispatch;
	// 82418260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82418268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241826C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418278 size=136
    let mut pc: u32 = 0x82418278;
    'dispatch: loop {
        match pc {
            0x82418278 => {
    //   block [0x82418278..0x82418300)
	// 82418278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241827C: 4811CE41  bl 0x825350bc
	ctx.lr = 0x82418280;
	sub_82535080(ctx, base);
	// 82418280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418284: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82418288: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8241828C: 48008DA5  bl 0x82421030
	ctx.lr = 0x82418290;
	sub_82421030(ctx, base);
	// 82418290: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82418294: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82418298: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241829C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824182A0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824182A4: 4BFFE65D  bl 0x82416900
	ctx.lr = 0x824182A8;
	sub_82416900(ctx, base);
	// 824182A8: 4BFFE779  bl 0x82416a20
	ctx.lr = 0x824182AC;
	sub_82416A20(ctx, base);
	// 824182AC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824182B0: 41820028  beq 0x824182d8
	if ctx.cr[0].eq {
	pc = 0x824182D8; continue 'dispatch;
	}
	// 824182B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824182B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824182BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824182C0: 4BFFF921  bl 0x82417be0
	ctx.lr = 0x824182C4;
	sub_82417BE0(ctx, base);
	// 824182C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824182C8: 40800010  bge 0x824182d8
	if !ctx.cr[0].lt {
	pc = 0x824182D8; continue 'dispatch;
	}
	// 824182CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824182D0: 4BFFEF99  bl 0x82417268
	ctx.lr = 0x824182D4;
	sub_82417268(ctx, base);
	// 824182D4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824182D8: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824182DC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824182E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824182E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824182E8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824182EC: 4BFFE615  bl 0x82416900
	ctx.lr = 0x824182F0;
	sub_82416900(ctx, base);
	// 824182F0: 48008D81  bl 0x82421070
	ctx.lr = 0x824182F4;
	sub_82421070(ctx, base);
	// 824182F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824182F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824182FC: 4811CE10  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418300 size=276
    let mut pc: u32 = 0x82418300;
    'dispatch: loop {
        match pc {
            0x82418300 => {
    //   block [0x82418300..0x82418414)
	// 82418300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418304: 4811CDB9  bl 0x825350bc
	ctx.lr = 0x82418308;
	sub_82535080(ctx, base);
	// 82418308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241830C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418310: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82418314: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82418318: 48008D19  bl 0x82421030
	ctx.lr = 0x8241831C;
	sub_82421030(ctx, base);
	// 8241831C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418320: 409A0010  bne cr6, 0x82418330
	if !ctx.cr[6].eq {
	pc = 0x82418330; continue 'dispatch;
	}
	// 82418324: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418328: 386BF280  addi r3, r11, -0xd80
	ctx.r[3].s64 = ctx.r[11].s64 + -3456;
	// 8241832C: 480000D0  b 0x824183fc
	pc = 0x824183FC; continue 'dispatch;
	// 82418330: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82418334: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82418338: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8241833C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82418340: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82418344: 4BFFE5BD  bl 0x82416900
	ctx.lr = 0x82418348;
	sub_82416900(ctx, base);
	// 82418348: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8241834C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418350: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82418354: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82418358: 409A0010  bne cr6, 0x82418368
	if !ctx.cr[6].eq {
	pc = 0x82418368; continue 'dispatch;
	}
	// 8241835C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418360: 4BFFF179  bl 0x824174d8
	ctx.lr = 0x82418364;
	sub_824174D8(ctx, base);
	// 82418364: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82418368: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241836C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82418370: 409A000C  bne cr6, 0x8241837c
	if !ctx.cr[6].eq {
	pc = 0x8241837C; continue 'dispatch;
	}
	// 82418374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418378: 4BFFEAE9  bl 0x82416e60
	ctx.lr = 0x8241837C;
	sub_82416E60(ctx, base);
	// 8241837C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82418380: 409A000C  bne cr6, 0x8241838c
	if !ctx.cr[6].eq {
	pc = 0x8241838C; continue 'dispatch;
	}
	// 82418384: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82418388: 48000028  b 0x824183b0
	pc = 0x824183B0; continue 'dispatch;
	// 8241838C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82418390: 409A000C  bne cr6, 0x8241839c
	if !ctx.cr[6].eq {
	pc = 0x8241839C; continue 'dispatch;
	}
	// 82418394: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82418398: 48000010  b 0x824183a8
	pc = 0x824183A8; continue 'dispatch;
	// 8241839C: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 824183A0: 409A0054  bne cr6, 0x824183f4
	if !ctx.cr[6].eq {
	pc = 0x824183F4; continue 'dispatch;
	}
	// 824183A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824183A8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824183AC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824183B0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824183B4: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824183B8: 4080000C  bge 0x824183c4
	if !ctx.cr[0].lt {
	pc = 0x824183C4; continue 'dispatch;
	}
	// 824183BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824183C0: 48000010  b 0x824183d0
	pc = 0x824183D0; continue 'dispatch;
	// 824183C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824183C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824183CC: 40990008  ble cr6, 0x824183d4
	if !ctx.cr[6].gt {
	pc = 0x824183D4; continue 'dispatch;
	}
	// 824183D0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824183D4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 824183D8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824183DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824183E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824183E4: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 824183E8: 4BFFE519  bl 0x82416900
	ctx.lr = 0x824183EC;
	sub_82416900(ctx, base);
	// 824183EC: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824183F0: 48000014  b 0x82418404
	pc = 0x82418404; continue 'dispatch;
	// 824183F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824183F8: 386BF258  addi r3, r11, -0xda8
	ctx.r[3].s64 = ctx.r[11].s64 + -3496;
	// 824183FC: 48008E9D  bl 0x82421298
	ctx.lr = 0x82418400;
	sub_82421298(ctx, base);
	// 82418400: 3BE0FFFD  li r31, -3
	ctx.r[31].s64 = -3;
	// 82418404: 48008C6D  bl 0x82421070
	ctx.lr = 0x82418408;
	sub_82421070(ctx, base);
	// 82418408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241840C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418410: 4811CCFC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418418 size=116
    let mut pc: u32 = 0x82418418;
    'dispatch: loop {
        match pc {
            0x82418418 => {
    //   block [0x82418418..0x8241848C)
	// 82418418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241841C: 4811CC95  bl 0x825350b0
	ctx.lr = 0x82418420;
	sub_82535080(ctx, base);
	// 82418420: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8241842C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82418430: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82418434: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82418438: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8241843C: 48008BF5  bl 0x82421030
	ctx.lr = 0x82418440;
	sub_82421030(ctx, base);
	// 82418440: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82418444: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82418448: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8241844C: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82418450: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82418454: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82418458: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8241845C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82418460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82418464: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82418468: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8241846C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82418470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418474: 4BFFFA95  bl 0x82417f08
	ctx.lr = 0x82418478;
	sub_82417F08(ctx, base);
	// 82418478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241847C: 48008BF5  bl 0x82421070
	ctx.lr = 0x82418480;
	sub_82421070(ctx, base);
	// 82418480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418484: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82418488: 4811CC78  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418490 size=116
    let mut pc: u32 = 0x82418490;
    'dispatch: loop {
        match pc {
            0x82418490 => {
    //   block [0x82418490..0x82418504)
	// 82418490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418494: 4811CC1D  bl 0x825350b0
	ctx.lr = 0x82418498;
	sub_82535080(ctx, base);
	// 82418498: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241849C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824184A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824184A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824184A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824184AC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 824184B0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 824184B4: 48008B7D  bl 0x82421030
	ctx.lr = 0x824184B8;
	sub_82421030(ctx, base);
	// 824184B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824184BC: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 824184C0: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 824184C4: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824184C8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 824184CC: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 824184D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824184D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824184D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824184DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824184E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824184E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824184E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824184EC: 4BFFFA1D  bl 0x82417f08
	ctx.lr = 0x824184F0;
	sub_82417F08(ctx, base);
	// 824184F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824184F4: 48008B7D  bl 0x82421070
	ctx.lr = 0x824184F8;
	sub_82421070(ctx, base);
	// 824184F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824184FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82418500: 4811CC00  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418508 size=40
    let mut pc: u32 = 0x82418508;
    'dispatch: loop {
        match pc {
            0x82418508 => {
    //   block [0x82418508..0x82418530)
	// 82418508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241850C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418514: 4800B125  bl 0x82423638
	ctx.lr = 0x82418518;
	sub_82423638(ctx, base);
	// 82418518: 48016A31  bl 0x8242ef48
	ctx.lr = 0x8241851C;
	sub_8242EF48(ctx, base);
	// 8241851C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82418520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82418524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82418530 size=172
    let mut pc: u32 = 0x82418530;
    'dispatch: loop {
        match pc {
            0x82418530 => {
    //   block [0x82418530..0x824185DC)
	// 82418530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418534: 4811CB81  bl 0x825350b4
	ctx.lr = 0x82418538;
	sub_82535080(ctx, base);
	// 82418538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241853C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82418540: 409A0018  bne cr6, 0x82418558
	if !ctx.cr[6].eq {
	pc = 0x82418558; continue 'dispatch;
	}
	// 82418544: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418548: 386BF560  addi r3, r11, -0xaa0
	ctx.r[3].s64 = ctx.r[11].s64 + -2720;
	// 8241854C: 4800C18D  bl 0x824246d8
	ctx.lr = 0x82418550;
	sub_824246D8(ctx, base);
	// 82418550: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82418554: 48000080  b 0x824185d4
	pc = 0x824185D4; continue 'dispatch;
	// 82418558: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241855C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82418560: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82418564: 4198006C  blt cr6, 0x824185d0
	if ctx.cr[6].lt {
	pc = 0x824185D0; continue 'dispatch;
	}
	// 82418568: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8241856C: 41990064  bgt cr6, 0x824185d0
	if ctx.cr[6].gt {
	pc = 0x824185D0; continue 'dispatch;
	}
	// 82418570: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82418578: 4800B241  bl 0x824237b8
	ctx.lr = 0x8241857C;
	sub_824237B8(ctx, base);
	// 8241857C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82418580: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82418584: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82418588: 40810040  ble 0x824185c8
	if !ctx.cr[0].gt {
	pc = 0x824185C8; continue 'dispatch;
	}
	// 8241858C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82418590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82418594: 4800B235  bl 0x824237c8
	ctx.lr = 0x82418598;
	sub_824237C8(ctx, base);
	// 82418598: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8241859C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824185A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824185A4: 4800B24D  bl 0x824237f0
	ctx.lr = 0x824185A8;
	sub_824237F0(ctx, base);
	// 824185A8: 7D63D9D6  mullw r11, r3, r27
	ctx.r[11].s64 = (ctx.r[3].s32 as i64) * (ctx.r[27].s32 as i64);
	// 824185AC: 1D6B0012  mulli r11, r11, 0x12
	ctx.r[11].s64 = ctx.r[11].s64 * 18;
	// 824185B0: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 824185B4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824185B8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824185BC: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824185C0: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824185C4: 4198FFC8  blt cr6, 0x8241858c
	if ctx.cr[6].lt {
	pc = 0x8241858C; continue 'dispatch;
	}
	// 824185C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824185CC: 48000008  b 0x824185d4
	pc = 0x824185D4; continue 'dispatch;
	// 824185D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824185D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824185D8: 4811CB2C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824185E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824185E0 size=356
    let mut pc: u32 = 0x824185E0;
    'dispatch: loop {
        match pc {
            0x824185E0 => {
    //   block [0x824185E0..0x82418744)
	// 824185E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824185E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824185E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824185EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824185F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824185F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824185F8: 817F01B0  lwz r11, 0x1b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) } as u64;
	// 824185FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82418600: 419A0010  beq cr6, 0x82418610
	if ctx.cr[6].eq {
	pc = 0x82418610; continue 'dispatch;
	}
	// 82418604: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82418608: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8241860C: 409A0120  bne cr6, 0x8241872c
	if !ctx.cr[6].eq {
	pc = 0x8241872C; continue 'dispatch;
	}
	// 82418610: 817F0274  lwz r11, 0x274(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(628 as u32) ) } as u64;
	// 82418614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82418618: 419A0044  beq cr6, 0x8241865c
	if ctx.cr[6].eq {
	pc = 0x8241865C; continue 'dispatch;
	}
	// 8241861C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418620: 4800B1E9  bl 0x82423808
	ctx.lr = 0x82418624;
	sub_82423808(ctx, base);
	// 82418624: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82418628: 809F01C0  lwz r4, 0x1c0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 8241862C: 807F0278  lwz r3, 0x278(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 82418630: 815F0274  lwz r10, 0x274(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(628 as u32) ) } as u64;
	// 82418634: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82418638: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8241863C: 4E800421  bctrl
	ctx.lr = 0x82418640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82418640: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82418644: 907F01C0  stw r3, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[3].u32 ) };
	// 82418648: 40800020  bge 0x82418668
	if !ctx.cr[0].lt {
	pc = 0x82418668; continue 'dispatch;
	}
	// 8241864C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82418650: 997F01B4  stb r11, 0x1b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[11].u8 ) };
	// 82418654: 997F01B5  stb r11, 0x1b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(437 as u32), ctx.r[11].u8 ) };
	// 82418658: 48000010  b 0x82418668
	pc = 0x82418668; continue 'dispatch;
	// 8241865C: 817F01C0  lwz r11, 0x1c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 82418660: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82418664: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 82418668: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 8241866C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418670: 409A001C  bne cr6, 0x8241868c
	if !ctx.cr[6].eq {
	pc = 0x8241868C; continue 'dispatch;
	}
	// 82418674: 817F01C0  lwz r11, 0x1c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 82418678: 815F01CC  lwz r10, 0x1cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 8241867C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82418680: 4198000C  blt cr6, 0x8241868c
	if ctx.cr[6].lt {
	pc = 0x8241868C; continue 'dispatch;
	}
	// 82418684: 817F01C8  lwz r11, 0x1c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 82418688: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 8241868C: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 82418690: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418694: 409A0098  bne cr6, 0x8241872c
	if !ctx.cr[6].eq {
	pc = 0x8241872C; continue 'dispatch;
	}
	// 82418698: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241869C: 4800B16D  bl 0x82423808
	ctx.lr = 0x824186A0;
	sub_82423808(ctx, base);
	// 824186A0: 817F01C0  lwz r11, 0x1c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 824186A4: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 824186A8: 40980084  bge cr6, 0x8241872c
	if !ctx.cr[6].lt {
	pc = 0x8241872C; continue 'dispatch;
	}
	// 824186AC: 480168A5  bl 0x8242ef50
	ctx.lr = 0x824186B0;
	sub_8242EF50(ctx, base);
	// 824186B0: 83DF01C0  lwz r30, 0x1c0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(448 as u32) ) } as u64;
	// 824186B4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824186B8: 4800B191  bl 0x82423848
	ctx.lr = 0x824186BC;
	sub_82423848(ctx, base);
	// 824186BC: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 824186C0: 4198001C  blt cr6, 0x824186dc
	if ctx.cr[6].lt {
	pc = 0x824186DC; continue 'dispatch;
	}
	// 824186C4: 409A0064  bne cr6, 0x82418728
	if !ctx.cr[6].eq {
	pc = 0x82418728; continue 'dispatch;
	}
	// 824186C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824186CC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824186D0: 480091F1  bl 0x824218c0
	ctx.lr = 0x824186D4;
	sub_824218C0(ctx, base);
	// 824186D4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 824186D8: 48000040  b 0x82418718
	pc = 0x82418718; continue 'dispatch;
	// 824186DC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 824186E0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824186E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824186E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824186EC: 4800B125  bl 0x82423810
	ctx.lr = 0x824186F0;
	sub_82423810(ctx, base);
	// 824186F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824186F4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824186F8: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 824186FC: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82418700: 480091C1  bl 0x824218c0
	ctx.lr = 0x82418704;
	sub_824218C0(ctx, base);
	// 82418704: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82418708: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241870C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82418710: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82418714: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82418718: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241871C: 4800927D  bl 0x82421998
	ctx.lr = 0x82418720;
	sub_82421998(ctx, base);
	// 82418720: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418724: 48009B45  bl 0x82422268
	ctx.lr = 0x82418728;
	sub_82422268(ctx, base);
	// 82418728: 48016829  bl 0x8242ef50
	ctx.lr = 0x8241872C;
	sub_8242EF50(ctx, base);
	// 8241872C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418738: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241873C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82418748 size=224
    let mut pc: u32 = 0x82418748;
    'dispatch: loop {
        match pc {
            0x82418748 => {
    //   block [0x82418748..0x82418828)
	// 82418748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241874C: 4811C971  bl 0x825350bc
	ctx.lr = 0x82418750;
	sub_82535080(ctx, base);
	// 82418750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418758: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241875C: 48009E45  bl 0x824225a0
	ctx.lr = 0x82418760;
	sub_824225A0(ctx, base);
	// 82418760: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82418764: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418768: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241876C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82418770: 4E800421  bctrl
	ctx.lr = 0x82418774;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82418774: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82418778: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241877C: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82418780: 40810034  ble 0x824187b4
	if !ctx.cr[0].gt {
	pc = 0x824187B4; continue 'dispatch;
	}
	// 82418784: 3BDF0098  addi r30, r31, 0x98
	ctx.r[30].s64 = ctx.r[31].s64 + 152;
	// 82418788: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241878C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418790: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82418794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82418798: 4E800421  bctrl
	ctx.lr = 0x8241879C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241879C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824187A0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824187A4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824187A8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824187AC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824187B0: 4198FFD8  blt cr6, 0x82418788
	if ctx.cr[6].lt {
	pc = 0x82418788; continue 'dispatch;
	}
	// 824187B4: 817F01E0  lwz r11, 0x1e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(480 as u32) ) } as u64;
	// 824187B8: 80DF01DC  lwz r6, 0x1dc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 824187BC: 79675FE4  rldicr r7, r11, 0xb, 0x3f
	ctx.r[7].u64 = (ctx.r[11].u64).rotate_left(11) & 0xFFFFFFFFFFFFFFFF;
	// 824187C0: 80BF01D8  lwz r5, 0x1d8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(472 as u32) ) } as u64;
	// 824187C4: 809F01D4  lwz r4, 0x1d4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(468 as u32) ) } as u64;
	// 824187C8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824187CC: 48009A35  bl 0x82422200
	ctx.lr = 0x824187D0;
	sub_82422200(ctx, base);
	// 824187D0: 817F011C  lwz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 824187D4: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 824187D8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824187DC: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824187E0: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 824187E4: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 824187E8: 48009899  bl 0x82422080
	ctx.lr = 0x824187EC;
	sub_82422080(ctx, base);
	// 824187EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824187F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824187F4: 480090CD  bl 0x824218c0
	ctx.lr = 0x824187F8;
	sub_824218C0(ctx, base);
	// 824187F8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824187FC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418800: 48009199  bl 0x82421998
	ctx.lr = 0x82418804;
	sub_82421998(ctx, base);
	// 82418804: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418808: 48009A61  bl 0x82422268
	ctx.lr = 0x8241880C;
	sub_82422268(ctx, base);
	// 8241880C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418810: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418814: 4800AF95  bl 0x824237a8
	ctx.lr = 0x82418818;
	sub_824237A8(ctx, base);
	// 82418818: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241881C: 4800AF2D  bl 0x82423748
	ctx.lr = 0x82418820;
	sub_82423748(ctx, base);
	// 82418820: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418824: 4811C8E8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418828 size=260
    let mut pc: u32 = 0x82418828;
    'dispatch: loop {
        match pc {
            0x82418828 => {
    //   block [0x82418828..0x8241892C)
	// 82418828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241882C: 4811C885  bl 0x825350b0
	ctx.lr = 0x82418830;
	sub_82535080(ctx, base);
	// 82418830: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418834: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82418838: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 8241883C: 7B5A0040  clrldi r26, r26, 1
	ctx.r[26].u64 = ctx.r[26].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 82418840: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82418844: 409A0014  bne cr6, 0x82418858
	if !ctx.cr[6].eq {
	pc = 0x82418858; continue 'dispatch;
	}
	// 82418848: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241884C: 386BF528  addi r3, r11, -0xad8
	ctx.r[3].s64 = ctx.r[11].s64 + -2776;
	// 82418850: 4800BE89  bl 0x824246d8
	ctx.lr = 0x82418854;
	sub_824246D8(ctx, base);
	// 82418854: 480000D0  b 0x82418924
	pc = 0x82418924; continue 'dispatch;
	// 82418858: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8241885C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82418860: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82418864: 4198001C  blt cr6, 0x82418880
	if ctx.cr[6].lt {
	pc = 0x82418880; continue 'dispatch;
	}
	// 82418868: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8241886C: 41990014  bgt cr6, 0x82418880
	if ctx.cr[6].gt {
	pc = 0x82418880; continue 'dispatch;
	}
	// 82418870: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418874: 4800AF45  bl 0x824237b8
	ctx.lr = 0x82418878;
	sub_824237B8(ctx, base);
	// 82418878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241887C: 48000008  b 0x82418884
	pc = 0x82418884; continue 'dispatch;
	// 82418880: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82418884: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82418888: 4099009C  ble cr6, 0x82418924
	if !ctx.cr[6].gt {
	pc = 0x82418924; continue 'dispatch;
	}
	// 8241888C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418890: 48002081  bl 0x8241a910
	ctx.lr = 0x82418894;
	sub_8241A910(ctx, base);
	// 82418894: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82418898: 4081008C  ble 0x82418924
	if !ctx.cr[0].gt {
	pc = 0x82418924; continue 'dispatch;
	}
	// 8241889C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 824188A0: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 824188A4: 40990028  ble cr6, 0x824188cc
	if !ctx.cr[6].gt {
	pc = 0x824188CC; continue 'dispatch;
	}
	// 824188A8: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 824188AC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824188B0: 48002061  bl 0x8241a910
	ctx.lr = 0x824188B4;
	sub_8241A910(ctx, base);
	// 824188B4: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 824188B8: 409A006C  bne cr6, 0x82418924
	if !ctx.cr[6].eq {
	pc = 0x82418924; continue 'dispatch;
	}
	// 824188BC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824188C0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824188C4: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824188C8: 4198FFE4  blt cr6, 0x824188ac
	if ctx.cr[6].lt {
	pc = 0x824188AC; continue 'dispatch;
	}
	// 824188CC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824188D0: 40990054  ble cr6, 0x82418924
	if !ctx.cr[6].gt {
	pc = 0x82418924; continue 'dispatch;
	}
	// 824188D4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 824188D8: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 824188DC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824188E0: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824188E4: 4800C43D  bl 0x82424d20
	ctx.lr = 0x824188E8;
	sub_82424D20(ctx, base);
	// 824188E8: 7F3A1800  cmpd cr6, r26, r3
	ctx.cr[6].compare_i64(ctx.r[26].s64, ctx.r[3].s64, &mut ctx.xer);
	// 824188EC: 41980008  blt cr6, 0x824188f4
	if ctx.cr[6].lt {
	pc = 0x824188F4; continue 'dispatch;
	}
	// 824188F0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824188F4: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824188F8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824188FC: 4082FFE0  bne 0x824188dc
	if !ctx.cr[0].eq {
	pc = 0x824188DC; continue 'dispatch;
	}
	// 82418900: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82418904: 40990020  ble cr6, 0x82418924
	if !ctx.cr[6].gt {
	pc = 0x82418924; continue 'dispatch;
	}
	// 82418908: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241890C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82418910: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418914: 4800C465  bl 0x82424d78
	ctx.lr = 0x82418918;
	sub_82424D78(ctx, base);
	// 82418918: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8241891C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82418920: 4082FFE8  bne 0x82418908
	if !ctx.cr[0].eq {
	pc = 0x82418908; continue 'dispatch;
	}
	// 82418924: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82418928: 4811C7D8  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418930 size=200
    let mut pc: u32 = 0x82418930;
    'dispatch: loop {
        match pc {
            0x82418930 => {
    //   block [0x82418930..0x824189F8)
	// 82418930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418934: 4811C785  bl 0x825350b8
	ctx.lr = 0x82418938;
	sub_82535080(ctx, base);
	// 82418938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241893C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418944: 409A0014  bne cr6, 0x82418958
	if !ctx.cr[6].eq {
	pc = 0x82418958; continue 'dispatch;
	}
	// 82418948: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241894C: 386BF630  addi r3, r11, -0x9d0
	ctx.r[3].s64 = ctx.r[11].s64 + -2512;
	// 82418950: 4800BD89  bl 0x824246d8
	ctx.lr = 0x82418954;
	sub_824246D8(ctx, base);
	// 82418954: 4800009C  b 0x824189f0
	pc = 0x824189F0; continue 'dispatch;
	// 82418958: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241895C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82418960: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82418964: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82418968: 40810028  ble 0x82418990
	if !ctx.cr[0].gt {
	pc = 0x82418990; continue 'dispatch;
	}
	// 8241896C: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82418970: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418974: 48001E5D  bl 0x8241a7d0
	ctx.lr = 0x82418978;
	sub_8241A7D0(ctx, base);
	// 82418978: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241897C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82418980: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82418984: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82418988: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241898C: 4198FFE4  blt cr6, 0x82418970
	if ctx.cr[6].lt {
	pc = 0x82418970; continue 'dispatch;
	}
	// 82418990: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418994: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82418998: 41820008  beq 0x824189a0
	if ctx.cr[0].eq {
	pc = 0x824189A0; continue 'dispatch;
	}
	// 8241899C: 480099D5  bl 0x82422370
	ctx.lr = 0x824189A0;
	sub_82422370(ctx, base);
	// 824189A0: 480165B1  bl 0x8242ef50
	ctx.lr = 0x824189A4;
	sub_8242EF50(ctx, base);
	// 824189A4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824189A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824189AC: 41820008  beq 0x824189b4
	if ctx.cr[0].eq {
	pc = 0x824189B4; continue 'dispatch;
	}
	// 824189B0: 4800ADE1  bl 0x82423790
	ctx.lr = 0x824189B4;
	sub_82423790(ctx, base);
	// 824189B4: 817F01E4  lwz r11, 0x1e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(484 as u32) ) } as u64;
	// 824189B8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824189BC: 409A0024  bne cr6, 0x824189e0
	if !ctx.cr[6].eq {
	pc = 0x824189E0; continue 'dispatch;
	}
	// 824189C0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 824189C4: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 824189C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824189CC: 41820014  beq 0x824189e0
	if ctx.cr[0].eq {
	pc = 0x824189E0; continue 'dispatch;
	}
	// 824189D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824189D4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824189D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824189DC: 4E800421  bctrl
	ctx.lr = 0x824189E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824189E0: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 824189E4: 9B9F0001  stb r28, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[28].u8 ) };
	// 824189E8: 939F01B0  stw r28, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[28].u32 ) };
	// 824189EC: 48016565  bl 0x8242ef50
	ctx.lr = 0x824189F0;
	sub_8242EF50(ctx, base);
	// 824189F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824189F4: 4811C714  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824189F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824189F8 size=88
    let mut pc: u32 = 0x824189F8;
    'dispatch: loop {
        match pc {
            0x824189F8 => {
    //   block [0x824189F8..0x82418A50)
	// 824189F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824189FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82418A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418A0C: 4800AC2D  bl 0x82423638
	ctx.lr = 0x82418A10;
	sub_82423638(ctx, base);
	// 82418A10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418A14: 409A0018  bne cr6, 0x82418a2c
	if !ctx.cr[6].eq {
	pc = 0x82418A2C; continue 'dispatch;
	}
	// 82418A18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418A1C: 386BF3C8  addi r3, r11, -0xc38
	ctx.r[3].s64 = ctx.r[11].s64 + -3128;
	// 82418A20: 4800BCB9  bl 0x824246d8
	ctx.lr = 0x82418A24;
	sub_824246D8(ctx, base);
	// 82418A24: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82418A28: 4800000C  b 0x82418a34
	pc = 0x82418A34; continue 'dispatch;
	// 82418A2C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82418A30: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 82418A34: 48016515  bl 0x8242ef48
	ctx.lr = 0x82418A38;
	sub_8242EF48(ctx, base);
	// 82418A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82418A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418A50 size=124
    let mut pc: u32 = 0x82418A50;
    'dispatch: loop {
        match pc {
            0x82418A50 => {
    //   block [0x82418A50..0x82418ACC)
	// 82418A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418A54: 4811C669  bl 0x825350bc
	ctx.lr = 0x82418A58;
	sub_82535080(ctx, base);
	// 82418A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418A60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82418A64: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82418A68: 4800ABD1  bl 0x82423638
	ctx.lr = 0x82418A6C;
	sub_82423638(ctx, base);
	// 82418A6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418A70: 409A0014  bne cr6, 0x82418a84
	if !ctx.cr[6].eq {
	pc = 0x82418A84; continue 'dispatch;
	}
	// 82418A74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418A78: 386BF3F0  addi r3, r11, -0xc10
	ctx.r[3].s64 = ctx.r[11].s64 + -3088;
	// 82418A7C: 4800BC5D  bl 0x824246d8
	ctx.lr = 0x82418A80;
	sub_824246D8(ctx, base);
	// 82418A80: 48000040  b 0x82418ac0
	pc = 0x82418AC0; continue 'dispatch;
	// 82418A84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82418A88: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82418A8C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82418A90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82418A94: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82418A98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82418A9C: 48001DC5  bl 0x8241a860
	ctx.lr = 0x82418AA0;
	sub_8241A860(ctx, base);
	// 82418AA0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82418AA4: 419A000C  beq cr6, 0x82418ab0
	if ctx.cr[6].eq {
	pc = 0x82418AB0; continue 'dispatch;
	}
	// 82418AA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82418AAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82418AB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82418AB4: 419A000C  beq cr6, 0x82418ac0
	if ctx.cr[6].eq {
	pc = 0x82418AC0; continue 'dispatch;
	}
	// 82418AB8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82418ABC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82418AC0: 48016489  bl 0x8242ef48
	ctx.lr = 0x82418AC4;
	sub_8242EF48(ctx, base);
	// 82418AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82418AC8: 4811C644  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418AD0 size=124
    let mut pc: u32 = 0x82418AD0;
    'dispatch: loop {
        match pc {
            0x82418AD0 => {
    //   block [0x82418AD0..0x82418B4C)
	// 82418AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82418ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82418AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418AE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82418AEC: 4800AB4D  bl 0x82423638
	ctx.lr = 0x82418AF0;
	sub_82423638(ctx, base);
	// 82418AF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418AF4: 409A0018  bne cr6, 0x82418b0c
	if !ctx.cr[6].eq {
	pc = 0x82418B0C; continue 'dispatch;
	}
	// 82418AF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418AFC: 386BF44C  addi r3, r11, -0xbb4
	ctx.r[3].s64 = ctx.r[11].s64 + -2996;
	// 82418B00: 4800BBD9  bl 0x824246d8
	ctx.lr = 0x82418B04;
	sub_824246D8(ctx, base);
	// 82418B04: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82418B08: 48000024  b 0x82418b2c
	pc = 0x82418B2C; continue 'dispatch;
	// 82418B0C: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 82418B10: 40990010  ble cr6, 0x82418b20
	if !ctx.cr[6].gt {
	pc = 0x82418B20; continue 'dispatch;
	}
	// 82418B14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418B18: 386BF418  addi r3, r11, -0xbe8
	ctx.r[3].s64 = ctx.r[11].s64 + -3048;
	// 82418B1C: 4BFFFFE4  b 0x82418b00
	pc = 0x82418B00; continue 'dispatch;
	// 82418B20: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82418B24: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82418B28: 7FEBF82E  lwzx r31, r11, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82418B2C: 4801641D  bl 0x8242ef48
	ctx.lr = 0x82418B30;
	sub_8242EF48(ctx, base);
	// 82418B30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418B34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418B40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82418B44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418B50 size=88
    let mut pc: u32 = 0x82418B50;
    'dispatch: loop {
        match pc {
            0x82418B50 => {
    //   block [0x82418B50..0x82418BA8)
	// 82418B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418B58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82418B5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82418B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418B68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82418B6C: 4800AACD  bl 0x82423638
	ctx.lr = 0x82418B70;
	sub_82423638(ctx, base);
	// 82418B70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418B74: 409A0014  bne cr6, 0x82418b88
	if !ctx.cr[6].eq {
	pc = 0x82418B88; continue 'dispatch;
	}
	// 82418B78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418B7C: 386BF474  addi r3, r11, -0xb8c
	ctx.r[3].s64 = ctx.r[11].s64 + -2956;
	// 82418B80: 4800BB59  bl 0x824246d8
	ctx.lr = 0x82418B84;
	sub_824246D8(ctx, base);
	// 82418B84: 48000008  b 0x82418b8c
	pc = 0x82418B8C; continue 'dispatch;
	// 82418B88: 9BDF01B4  stb r30, 0x1b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[30].u8 ) };
	// 82418B8C: 480163BD  bl 0x8242ef48
	ctx.lr = 0x82418B90;
	sub_8242EF48(ctx, base);
	// 82418B90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418B9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82418BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418BA8 size=88
    let mut pc: u32 = 0x82418BA8;
    'dispatch: loop {
        match pc {
            0x82418BA8 => {
    //   block [0x82418BA8..0x82418C00)
	// 82418BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82418BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82418BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418BBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418BC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82418BC4: 4800AA75  bl 0x82423638
	ctx.lr = 0x82418BC8;
	sub_82423638(ctx, base);
	// 82418BC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418BCC: 409A0014  bne cr6, 0x82418be0
	if !ctx.cr[6].eq {
	pc = 0x82418BE0; continue 'dispatch;
	}
	// 82418BD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418BD4: 386BF49C  addi r3, r11, -0xb64
	ctx.r[3].s64 = ctx.r[11].s64 + -2916;
	// 82418BD8: 4800BB01  bl 0x824246d8
	ctx.lr = 0x82418BDC;
	sub_824246D8(ctx, base);
	// 82418BDC: 48000008  b 0x82418be4
	pc = 0x82418BE4; continue 'dispatch;
	// 82418BE0: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 82418BE4: 48016365  bl 0x8242ef48
	ctx.lr = 0x82418BE8;
	sub_8242EF48(ctx, base);
	// 82418BE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418BF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82418BF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418C00 size=84
    let mut pc: u32 = 0x82418C00;
    'dispatch: loop {
        match pc {
            0x82418C00 => {
    //   block [0x82418C00..0x82418C54)
	// 82418C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418C04: 4811C4B9  bl 0x825350bc
	ctx.lr = 0x82418C08;
	sub_82535080(ctx, base);
	// 82418C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418C10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82418C14: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82418C18: 4800AA21  bl 0x82423638
	ctx.lr = 0x82418C1C;
	sub_82423638(ctx, base);
	// 82418C1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418C20: 409A0014  bne cr6, 0x82418c34
	if !ctx.cr[6].eq {
	pc = 0x82418C34; continue 'dispatch;
	}
	// 82418C24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418C28: 386BF4C4  addi r3, r11, -0xb3c
	ctx.r[3].s64 = ctx.r[11].s64 + -2876;
	// 82418C2C: 4800BAAD  bl 0x824246d8
	ctx.lr = 0x82418C30;
	sub_824246D8(ctx, base);
	// 82418C30: 48000018  b 0x82418c48
	pc = 0x82418C48; continue 'dispatch;
	// 82418C34: 4801631D  bl 0x8242ef50
	ctx.lr = 0x82418C38;
	sub_8242EF50(ctx, base);
	// 82418C38: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82418C3C: 93DF01C8  stw r30, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[30].u32 ) };
	// 82418C40: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 82418C44: 4801630D  bl 0x8242ef50
	ctx.lr = 0x82418C48;
	sub_8242EF50(ctx, base);
	// 82418C48: 48016301  bl 0x8242ef48
	ctx.lr = 0x82418C4C;
	sub_8242EF48(ctx, base);
	// 82418C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82418C50: 4811C4BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82418C58 size=80
    let mut pc: u32 = 0x82418C58;
    'dispatch: loop {
        match pc {
            0x82418C58 => {
    //   block [0x82418C58..0x82418CA8)
	// 82418C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82418C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82418C64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418C6C: 4800A9CD  bl 0x82423638
	ctx.lr = 0x82418C70;
	sub_82423638(ctx, base);
	// 82418C70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82418C74: 409A0014  bne cr6, 0x82418c88
	if !ctx.cr[6].eq {
	pc = 0x82418C88; continue 'dispatch;
	}
	// 82418C78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418C7C: 386BF4EC  addi r3, r11, -0xb14
	ctx.r[3].s64 = ctx.r[11].s64 + -2836;
	// 82418C80: 4800BA59  bl 0x824246d8
	ctx.lr = 0x82418C84;
	sub_824246D8(ctx, base);
	// 82418C84: 48000008  b 0x82418c8c
	pc = 0x82418C8C; continue 'dispatch;
	// 82418C88: 83FF01AC  lwz r31, 0x1ac(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 82418C8C: 480162BD  bl 0x8242ef48
	ctx.lr = 0x82418C90;
	sub_8242EF48(ctx, base);
	// 82418C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82418C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82418C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82418CA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82418CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82418CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82418CA8 size=1544
    let mut pc: u32 = 0x82418CA8;
    'dispatch: loop {
        match pc {
            0x82418CA8 => {
    //   block [0x82418CA8..0x824192B0)
	// 82418CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82418CAC: 4811C3FD  bl 0x825350a8
	ctx.lr = 0x82418CB0;
	sub_82535080(ctx, base);
	// 82418CB0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82418CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82418CB8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82418CBC: 3B5F0010  addi r26, r31, 0x10
	ctx.r[26].s64 = ctx.r[31].s64 + 16;
	// 82418CC0: 837F0008  lwz r27, 8(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418CC4: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 82418CC8: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82418CCC: 4BFFFB5D  bl 0x82418828
	ctx.lr = 0x82418CD0;
	sub_82418828(ctx, base);
	// 82418CD0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82418CD4: 3B000004  li r24, 4
	ctx.r[24].s64 = 4;
	// 82418CD8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418CDC: 409A04DC  bne cr6, 0x824191b8
	if !ctx.cr[6].eq {
	pc = 0x824191B8; continue 'dispatch;
	}
	// 82418CE0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82418CE4: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82418CE8: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82418CEC: 40810034  ble 0x82418d20
	if !ctx.cr[0].gt {
	pc = 0x82418D20; continue 'dispatch;
	}
	// 82418CF0: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82418CF4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418CF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82418CFC: 4182000C  beq 0x82418d08
	if ctx.cr[0].eq {
	pc = 0x82418D08; continue 'dispatch;
	}
	// 82418D00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418D04: 480021A5  bl 0x8241aea8
	ctx.lr = 0x82418D08;
	sub_8241AEA8(ctx, base);
	// 82418D08: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82418D0C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82418D10: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82418D14: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82418D18: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82418D1C: 4198FFD8  blt cr6, 0x82418cf4
	if ctx.cr[6].lt {
	pc = 0x82418CF4; continue 'dispatch;
	}
	// 82418D20: 817F01B0  lwz r11, 0x1b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) } as u64;
	// 82418D24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82418D28: 409A035C  bne cr6, 0x82419084
	if !ctx.cr[6].eq {
	pc = 0x82419084; continue 'dispatch;
	}
	// 82418D2C: 817F01E4  lwz r11, 0x1e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(484 as u32) ) } as u64;
	// 82418D30: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82418D34: 418200E0  beq 0x82418e14
	if ctx.cr[0].eq {
	pc = 0x82418E14; continue 'dispatch;
	}
	// 82418D38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82418D3C: 419A00D8  beq cr6, 0x82418e14
	if ctx.cr[6].eq {
	pc = 0x82418E14; continue 'dispatch;
	}
	// 82418D40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418D44: 4800A9F5  bl 0x82423738
	ctx.lr = 0x82418D48;
	sub_82423738(ctx, base);
	// 82418D48: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82418D4C: 40820074  bne 0x82418dc0
	if !ctx.cr[0].eq {
	pc = 0x82418DC0; continue 'dispatch;
	}
	// 82418D50: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82418D54: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82418D58: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82418D5C: 40810034  ble 0x82418d90
	if !ctx.cr[0].gt {
	pc = 0x82418D90; continue 'dispatch;
	}
	// 82418D60: 3BDF0098  addi r30, r31, 0x98
	ctx.r[30].s64 = ctx.r[31].s64 + 152;
	// 82418D64: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418D68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418D6C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82418D70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82418D74: 4E800421  bctrl
	ctx.lr = 0x82418D78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82418D78: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82418D7C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82418D80: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82418D84: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82418D88: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82418D8C: 4198FFD8  blt cr6, 0x82418d64
	if ctx.cr[6].lt {
	pc = 0x82418D64; continue 'dispatch;
	}
	// 82418D90: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 82418D94: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418D98: 419A0010  beq cr6, 0x82418da8
	if ctx.cr[6].eq {
	pc = 0x82418DA8; continue 'dispatch;
	}
	// 82418D9C: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 82418DA0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418DA4: 409A0010  bne cr6, 0x82418db4
	if !ctx.cr[6].eq {
	pc = 0x82418DB4; continue 'dispatch;
	}
	// 82418DA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418DAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418DB0: 4800AA01  bl 0x824237b0
	ctx.lr = 0x82418DB4;
	sub_824237B0(ctx, base);
	// 82418DB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418DB8: 4800A991  bl 0x82423748
	ctx.lr = 0x82418DBC;
	sub_82423748(ctx, base);
	// 82418DBC: 480003FC  b 0x824191b8
	pc = 0x824191B8; continue 'dispatch;
	// 82418DC0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82418DC4: 409A02B4  bne cr6, 0x82419078
	if !ctx.cr[6].eq {
	pc = 0x82419078; continue 'dispatch;
	}
	// 82418DC8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418DCC: 4800A9ED  bl 0x824237b8
	ctx.lr = 0x82418DD0;
	sub_824237B8(ctx, base);
	// 82418DD0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82418DD4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82418DD8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82418DDC: 41990250  bgt cr6, 0x8241902c
	if ctx.cr[6].gt {
	pc = 0x8241902C; continue 'dispatch;
	}
	// 82418DE0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82418DE4: 40990024  ble cr6, 0x82418e08
	if !ctx.cr[6].gt {
	pc = 0x82418E08; continue 'dispatch;
	}
	// 82418DE8: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82418DEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82418DF0: 809E0088  lwz r4, 0x88(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82418DF4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418DF8: 480029B9  bl 0x8241b7b0
	ctx.lr = 0x82418DFC;
	sub_8241B7B0(ctx, base);
	// 82418DFC: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82418E00: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82418E04: 4082FFEC  bne 0x82418df0
	if !ctx.cr[0].eq {
	pc = 0x82418DF0; continue 'dispatch;
	}
	// 82418E08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82418E0C: 917F01B0  stw r11, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 82418E10: 480003A8  b 0x824191b8
	pc = 0x824191B8; continue 'dispatch;
	// 82418E14: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418E18: 48008A69  bl 0x82421880
	ctx.lr = 0x82418E1C;
	sub_82421880(ctx, base);
	// 82418E1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82418E20: 48016131  bl 0x8242ef50
	ctx.lr = 0x82418E24;
	sub_8242EF50(ctx, base);
	// 82418E24: 897F01B6  lbz r11, 0x1b6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(438 as u32) ) } as u64;
	// 82418E28: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418E2C: 409A0020  bne cr6, 0x82418e4c
	if !ctx.cr[6].eq {
	pc = 0x82418E4C; continue 'dispatch;
	}
	// 82418E30: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82418E34: 419A0018  beq cr6, 0x82418e4c
	if ctx.cr[6].eq {
	pc = 0x82418E4C; continue 'dispatch;
	}
	// 82418E38: 9B3F01B6  stb r25, 0x1b6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(438 as u32), ctx.r[25].u8 ) };
	// 82418E3C: 48016115  bl 0x8242ef50
	ctx.lr = 0x82418E40;
	sub_8242EF50(ctx, base);
	// 82418E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82418E44: 4BFFF905  bl 0x82418748
	ctx.lr = 0x82418E48;
	sub_82418748(ctx, base);
	// 82418E48: 48000008  b 0x82418e50
	pc = 0x82418E50; continue 'dispatch;
	// 82418E4C: 48016105  bl 0x8242ef50
	ctx.lr = 0x82418E50;
	sub_8242EF50(ctx, base);
	// 82418E50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418E54: 4800A8E5  bl 0x82423738
	ctx.lr = 0x82418E58;
	sub_82423738(ctx, base);
	// 82418E58: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82418E5C: 409A021C  bne cr6, 0x82419078
	if !ctx.cr[6].eq {
	pc = 0x82419078; continue 'dispatch;
	}
	// 82418E60: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418E64: 4800A955  bl 0x824237b8
	ctx.lr = 0x82418E68;
	sub_824237B8(ctx, base);
	// 82418E68: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418E6C: 4800A9DD  bl 0x82423848
	ctx.lr = 0x82418E70;
	sub_82423848(ctx, base);
	// 82418E70: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82418E74: 4198007C  blt cr6, 0x82418ef0
	if ctx.cr[6].lt {
	pc = 0x82418EF0; continue 'dispatch;
	}
	// 82418E78: 409AFF90  bne cr6, 0x82418e08
	if !ctx.cr[6].eq {
	pc = 0x82418E08; continue 'dispatch;
	}
	// 82418E7C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82418E80: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418E84: 48008B15  bl 0x82421998
	ctx.lr = 0x82418E88;
	sub_82421998(ctx, base);
	// 82418E88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82418E8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418E90: 4800A919  bl 0x824237a8
	ctx.lr = 0x82418E94;
	sub_824237A8(ctx, base);
	// 82418E94: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 82418E98: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418E9C: 419A0010  beq cr6, 0x82418eac
	if ctx.cr[6].eq {
	pc = 0x82418EAC; continue 'dispatch;
	}
	// 82418EA0: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 82418EA4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418EA8: 409A0010  bne cr6, 0x82418eb8
	if !ctx.cr[6].eq {
	pc = 0x82418EB8; continue 'dispatch;
	}
	// 82418EAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418EB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418EB4: 4800A8FD  bl 0x824237b0
	ctx.lr = 0x82418EB8;
	sub_824237B0(ctx, base);
	// 82418EB8: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418EBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82418EC0: 4182001C  beq 0x82418edc
	if ctx.cr[0].eq {
	pc = 0x82418EDC; continue 'dispatch;
	}
	// 82418EC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418EC8: 48002141  bl 0x8241b008
	ctx.lr = 0x82418ECC;
	sub_8241B008(ctx, base);
	// 82418ECC: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82418ED0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82418ED4: 480028DD  bl 0x8241b7b0
	ctx.lr = 0x82418ED8;
	sub_8241B7B0(ctx, base);
	// 82418ED8: 4BFFFF30  b 0x82418e08
	pc = 0x82418E08; continue 'dispatch;
	// 82418EDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82418EE0: 386BF680  addi r3, r11, -0x980
	ctx.r[3].s64 = ctx.r[11].s64 + -2432;
	// 82418EE4: 4800B7F5  bl 0x824246d8
	ctx.lr = 0x82418EE8;
	sub_824246D8(ctx, base);
	// 82418EE8: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 82418EEC: 4BFFFF1C  b 0x82418e08
	pc = 0x82418E08; continue 'dispatch;
	// 82418EF0: 83DF01C4  lwz r30, 0x1c4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 82418EF4: 2C1E0000  cmpwi r30, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82418EF8: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 82418EFC: 41800170  blt 0x8241906c
	if ctx.cr[0].lt {
	pc = 0x8241906C; continue 'dispatch;
	}
	// 82418F00: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418F04: 4800A905  bl 0x82423808
	ctx.lr = 0x82418F08;
	sub_82423808(ctx, base);
	// 82418F08: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82418F0C: 40980160  bge cr6, 0x8241906c
	if !ctx.cr[6].lt {
	pc = 0x8241906C; continue 'dispatch;
	}
	// 82418F10: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82418F14: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82418F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82418F1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418F20: 4800A8F1  bl 0x82423810
	ctx.lr = 0x82418F24;
	sub_82423810(ctx, base);
	// 82418F24: 817F01BC  lwz r11, 0x1bc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(444 as u32) ) } as u64;
	// 82418F28: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82418F2C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82418F30: 40990050  ble cr6, 0x82418f80
	if !ctx.cr[6].gt {
	pc = 0x82418F80; continue 'dispatch;
	}
	// 82418F34: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 82418F38: 817F01CC  lwz r11, 0x1cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 82418F3C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82418F40: 41990030  bgt cr6, 0x82418f70
	if ctx.cr[6].gt {
	pc = 0x82418F70; continue 'dispatch;
	}
	// 82418F44: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82418F48: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82418F4C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82418F50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418F54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82418F58: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82418F5C: 4800A8B5  bl 0x82423810
	ctx.lr = 0x82418F60;
	sub_82423810(ctx, base);
	// 82418F60: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82418F64: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82418F68: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82418F6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82418F70: 817F01BC  lwz r11, 0x1bc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(444 as u32) ) } as u64;
	// 82418F74: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82418F78: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82418F7C: 4198FFBC  blt cr6, 0x82418f38
	if ctx.cr[6].lt {
	pc = 0x82418F38; continue 'dispatch;
	}
	// 82418F80: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82418F84: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418F88: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82418F8C: 93DF01C0  stw r30, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[30].u32 ) };
	// 82418F90: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82418F94: 4800892D  bl 0x824218c0
	ctx.lr = 0x82418F98;
	sub_824218C0(ctx, base);
	// 82418F98: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82418F9C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82418FA0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82418FA4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82418FA8: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82418FAC: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82418FB0: 480089E9  bl 0x82421998
	ctx.lr = 0x82418FB4;
	sub_82421998(ctx, base);
	// 82418FB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82418FB8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418FBC: 4800A7ED  bl 0x824237a8
	ctx.lr = 0x82418FC0;
	sub_824237A8(ctx, base);
	// 82418FC0: 897F01B4  lbz r11, 0x1b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(436 as u32) ) } as u64;
	// 82418FC4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418FC8: 419A0010  beq cr6, 0x82418fd8
	if ctx.cr[6].eq {
	pc = 0x82418FD8; continue 'dispatch;
	}
	// 82418FCC: 897F01B5  lbz r11, 0x1b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(437 as u32) ) } as u64;
	// 82418FD0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82418FD4: 409A0010  bne cr6, 0x82418fe4
	if !ctx.cr[6].eq {
	pc = 0x82418FE4; continue 'dispatch;
	}
	// 82418FD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82418FDC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82418FE0: 4800A7D1  bl 0x824237b0
	ctx.lr = 0x82418FE4;
	sub_824237B0(ctx, base);
	// 82418FE4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82418FE8: 4800A7D1  bl 0x824237b8
	ctx.lr = 0x82418FEC;
	sub_824237B8(ctx, base);
	// 82418FEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82418FF0: 40810024  ble 0x82419014
	if !ctx.cr[0].gt {
	pc = 0x82419014; continue 'dispatch;
	}
	// 82418FF4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82418FF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82418FFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82419000: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419004: 48002005  bl 0x8241b008
	ctx.lr = 0x82419008;
	sub_8241B008(ctx, base);
	// 82419008: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241900C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82419010: 4082FFEC  bne 0x82418ffc
	if !ctx.cr[0].eq {
	pc = 0x82418FFC; continue 'dispatch;
	}
	// 82419014: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82419018: 4800A7A1  bl 0x824237b8
	ctx.lr = 0x8241901C;
	sub_824237B8(ctx, base);
	// 8241901C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419020: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419024: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419028: 40990018  ble cr6, 0x82419040
	if !ctx.cr[6].gt {
	pc = 0x82419040; continue 'dispatch;
	}
	// 8241902C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82419030: 386BF598  addi r3, r11, -0xa68
	ctx.r[3].s64 = ctx.r[11].s64 + -2664;
	// 82419034: 4800B6A5  bl 0x824246d8
	ctx.lr = 0x82419038;
	sub_824246D8(ctx, base);
	// 82419038: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8241903C: 48000034  b 0x82419070
	pc = 0x82419070; continue 'dispatch;
	// 82419040: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82419044: 4099FDC4  ble cr6, 0x82418e08
	if !ctx.cr[6].gt {
	pc = 0x82418E08; continue 'dispatch;
	}
	// 82419048: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8241904C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82419050: 809E0088  lwz r4, 0x88(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82419054: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419058: 48002759  bl 0x8241b7b0
	ctx.lr = 0x8241905C;
	sub_8241B7B0(ctx, base);
	// 8241905C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82419060: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82419064: 4082FFEC  bne 0x82419050
	if !ctx.cr[0].eq {
	pc = 0x82419050; continue 'dispatch;
	}
	// 82419068: 4BFFFDA0  b 0x82418e08
	pc = 0x82418E08; continue 'dispatch;
	// 8241906C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82419070: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82419074: 48000234  b 0x824192a8
	pc = 0x824192A8; continue 'dispatch;
	// 82419078: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8241907C: 409A013C  bne cr6, 0x824191b8
	if !ctx.cr[6].eq {
	pc = 0x824191B8; continue 'dispatch;
	}
	// 82419080: 4BFFFFB8  b 0x82419038
	pc = 0x82419038; continue 'dispatch;
	// 82419084: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82419088: 4800A7C1  bl 0x82423848
	ctx.lr = 0x8241908C;
	sub_82423848(ctx, base);
	// 8241908C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82419090: 41980014  blt cr6, 0x824190a4
	if ctx.cr[6].lt {
	pc = 0x824190A4; continue 'dispatch;
	}
	// 82419094: 409A0124  bne cr6, 0x824191b8
	if !ctx.cr[6].eq {
	pc = 0x824191B8; continue 'dispatch;
	}
	// 82419098: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8241909C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824190A0: 48000118  b 0x824191b8
	pc = 0x824191B8; continue 'dispatch;
	// 824190A4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824190A8: 4800A711  bl 0x824237b8
	ctx.lr = 0x824190AC;
	sub_824237B8(ctx, base);
	// 824190AC: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 824190B0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824190B4: 40810034  ble 0x824190e8
	if !ctx.cr[0].gt {
	pc = 0x824190E8; continue 'dispatch;
	}
	// 824190B8: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 824190BC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824190C0: 48001749  bl 0x8241a808
	ctx.lr = 0x824190C4;
	sub_8241A808(ctx, base);
	// 824190C4: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 824190C8: 409A0008  bne cr6, 0x824190d0
	if !ctx.cr[6].eq {
	pc = 0x824190D0; continue 'dispatch;
	}
	// 824190CC: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 824190D0: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 824190D4: 409A0014  bne cr6, 0x824190e8
	if !ctx.cr[6].eq {
	pc = 0x824190E8; continue 'dispatch;
	}
	// 824190D8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824190DC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824190E0: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824190E4: 4198FFD8  blt cr6, 0x824190bc
	if ctx.cr[6].lt {
	pc = 0x824190BC; continue 'dispatch;
	}
	// 824190E8: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824190EC: 409A00CC  bne cr6, 0x824191b8
	if !ctx.cr[6].eq {
	pc = 0x824191B8; continue 'dispatch;
	}
	// 824190F0: 48015E61  bl 0x8242ef50
	ctx.lr = 0x824190F4;
	sub_8242EF50(ctx, base);
	// 824190F4: 817F01AC  lwz r11, 0x1ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 824190F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824190FC: 409A0044  bne cr6, 0x82419140
	if !ctx.cr[6].eq {
	pc = 0x82419140; continue 'dispatch;
	}
	// 82419100: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419104: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82419108: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241910C: 40810034  ble 0x82419140
	if !ctx.cr[0].gt {
	pc = 0x82419140; continue 'dispatch;
	}
	// 82419110: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82419114: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419118: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241911C: 4182000C  beq 0x82419128
	if ctx.cr[0].eq {
	pc = 0x82419128; continue 'dispatch;
	}
	// 82419120: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419124: 48001D85  bl 0x8241aea8
	ctx.lr = 0x82419128;
	sub_8241AEA8(ctx, base);
	// 82419128: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241912C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82419130: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82419134: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419138: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8241913C: 4198FFD8  blt cr6, 0x82419114
	if ctx.cr[6].lt {
	pc = 0x82419114; continue 'dispatch;
	}
	// 82419140: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82419144: C1BF01E8  lfs f13, 0x1e8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82419148: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8241914C: 933F01B0  stw r25, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[25].u32 ) };
	// 82419150: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82419154: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82419158: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8241915C: 419A0044  beq cr6, 0x824191a0
	if ctx.cr[6].eq {
	pc = 0x824191A0; continue 'dispatch;
	}
	// 82419160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419164: 4BFFF3CD  bl 0x82418530
	ctx.lr = 0x82419168;
	sub_82418530(ctx, base);
	// 82419168: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8241916C: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 82419170: C01F01E8  lfs f0, 0x1e8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82419174: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82419178: 80BF011C  lwz r5, 0x11c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 8241917C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82419180: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82419184: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82419188: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8241918C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82419190: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82419194: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82419198: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8241919C: 48000010  b 0x824191ac
	pc = 0x824191AC; continue 'dispatch;
	// 824191A0: 80BF011C  lwz r5, 0x11c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 824191A4: 7CAB0E70  srawi r11, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 824191A8: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 824191AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824191B0: 48008ED1  bl 0x82422080
	ctx.lr = 0x824191B4;
	sub_82422080(ctx, base);
	// 824191B4: 48015D9D  bl 0x8242ef50
	ctx.lr = 0x824191B8;
	sub_8242EF50(ctx, base);
	// 824191B8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 824191BC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824191C0: 409A00C4  bne cr6, 0x82419284
	if !ctx.cr[6].eq {
	pc = 0x82419284; continue 'dispatch;
	}
	// 824191C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824191C8: 4800A681  bl 0x82423848
	ctx.lr = 0x824191CC;
	sub_82423848(ctx, base);
	// 824191CC: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 824191D0: 41980038  blt cr6, 0x82419208
	if ctx.cr[6].lt {
	pc = 0x82419208; continue 'dispatch;
	}
	// 824191D4: 419A000C  beq cr6, 0x824191e0
	if ctx.cr[6].eq {
	pc = 0x824191E0; continue 'dispatch;
	}
	// 824191D8: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 824191DC: 480000A8  b 0x82419284
	pc = 0x82419284; continue 'dispatch;
	// 824191E0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824191E4: 4800869D  bl 0x82421880
	ctx.lr = 0x824191E8;
	sub_82421880(ctx, base);
	// 824191E8: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 824191EC: 419A0098  beq cr6, 0x82419284
	if ctx.cr[6].eq {
	pc = 0x82419284; continue 'dispatch;
	}
	// 824191F0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824191F4: 48001615  bl 0x8241a808
	ctx.lr = 0x824191F8;
	sub_8241A808(ctx, base);
	// 824191F8: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 824191FC: 419A0080  beq cr6, 0x8241927c
	if ctx.cr[6].eq {
	pc = 0x8241927C; continue 'dispatch;
	}
	// 82419200: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82419204: 48000074  b 0x82419278
	pc = 0x82419278; continue 'dispatch;
	// 82419208: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8241920C: 4800A52D  bl 0x82423738
	ctx.lr = 0x82419210;
	sub_82423738(ctx, base);
	// 82419210: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82419214: 409A0070  bne cr6, 0x82419284
	if !ctx.cr[6].eq {
	pc = 0x82419284; continue 'dispatch;
	}
	// 82419218: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241921C: 4800A51D  bl 0x82423738
	ctx.lr = 0x82419220;
	sub_82423738(ctx, base);
	// 82419220: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82419224: 409A0060  bne cr6, 0x82419284
	if !ctx.cr[6].eq {
	pc = 0x82419284; continue 'dispatch;
	}
	// 82419228: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8241922C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82419230: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82419234: 40810038  ble 0x8241926c
	if !ctx.cr[0].gt {
	pc = 0x8241926C; continue 'dispatch;
	}
	// 82419238: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8241923C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82419240: 480015C9  bl 0x8241a808
	ctx.lr = 0x82419244;
	sub_8241A808(ctx, base);
	// 82419244: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 82419248: 419A000C  beq cr6, 0x82419254
	if ctx.cr[6].eq {
	pc = 0x82419254; continue 'dispatch;
	}
	// 8241924C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82419250: 409A001C  bne cr6, 0x8241926c
	if !ctx.cr[6].eq {
	pc = 0x8241926C; continue 'dispatch;
	}
	// 82419254: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419258: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8241925C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82419260: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419264: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419268: 4198FFD4  blt cr6, 0x8241923c
	if ctx.cr[6].lt {
	pc = 0x8241923C; continue 'dispatch;
	}
	// 8241926C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419270: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419274: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419278: 409A000C  bne cr6, 0x82419284
	if !ctx.cr[6].eq {
	pc = 0x82419284; continue 'dispatch;
	}
	// 8241927C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82419280: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82419284: 48015CCD  bl 0x8242ef50
	ctx.lr = 0x82419288;
	sub_8242EF50(ctx, base);
	// 82419288: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8241928C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419290: 41820014  beq 0x824192a4
	if ctx.cr[0].eq {
	pc = 0x824192A4; continue 'dispatch;
	}
	// 82419294: 480085ED  bl 0x82421880
	ctx.lr = 0x82419298;
	sub_82421880(ctx, base);
	// 82419298: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8241929C: 409A0008  bne cr6, 0x824192a4
	if !ctx.cr[6].eq {
	pc = 0x824192A4; continue 'dispatch;
	}
	// 824192A0: 9B1F0001  stb r24, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 824192A4: 48015CAD  bl 0x8242ef50
	ctx.lr = 0x824192A8;
	sub_8242EF50(ctx, base);
	// 824192A8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824192AC: 4811BE4C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824192B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824192B0 size=108
    let mut pc: u32 = 0x824192B0;
    'dispatch: loop {
        match pc {
            0x824192B0 => {
    //   block [0x824192B0..0x8241931C)
	// 824192B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824192B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824192B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824192BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824192C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824192C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824192C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824192CC: 4800A36D  bl 0x82423638
	ctx.lr = 0x824192D0;
	sub_82423638(ctx, base);
	// 824192D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824192D4: 409A0014  bne cr6, 0x824192e8
	if !ctx.cr[6].eq {
	pc = 0x824192E8; continue 'dispatch;
	}
	// 824192D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824192DC: 386BF608  addi r3, r11, -0x9f8
	ctx.r[3].s64 = ctx.r[11].s64 + -2552;
	// 824192E0: 4800B3F9  bl 0x824246d8
	ctx.lr = 0x824192E4;
	sub_824246D8(ctx, base);
	// 824192E4: 4800001C  b 0x82419300
	pc = 0x82419300; continue 'dispatch;
	// 824192E8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824192EC: 41990010  bgt cr6, 0x824192fc
	if ctx.cr[6].gt {
	pc = 0x824192FC; continue 'dispatch;
	}
	// 824192F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824192F4: 386BF5E0  addi r3, r11, -0xa20
	ctx.r[3].s64 = ctx.r[11].s64 + -2592;
	// 824192F8: 4BFFFFE8  b 0x824192e0
	pc = 0x824192E0; continue 'dispatch;
	// 824192FC: 93DF01BC  stw r30, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[30].u32 ) };
	// 82419300: 48015C49  bl 0x8242ef48
	ctx.lr = 0x82419304;
	sub_8242EF48(ctx, base);
	// 82419304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82419308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241930C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82419310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82419314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82419318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82419320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82419320 size=332
    let mut pc: u32 = 0x82419320;
    'dispatch: loop {
        match pc {
            0x82419320 => {
    //   block [0x82419320..0x8241946C)
	// 82419320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82419324: 4811BD95  bl 0x825350b8
	ctx.lr = 0x82419328;
	sub_82535080(ctx, base);
	// 82419328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241932C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82419330: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82419334: 409A0014  bne cr6, 0x82419348
	if !ctx.cr[6].eq {
	pc = 0x82419348; continue 'dispatch;
	}
	// 82419338: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8241933C: 386BF6EC  addi r3, r11, -0x914
	ctx.r[3].s64 = ctx.r[11].s64 + -2324;
	// 82419340: 4800B399  bl 0x824246d8
	ctx.lr = 0x82419344;
	sub_824246D8(ctx, base);
	// 82419344: 48000120  b 0x82419464
	pc = 0x82419464; continue 'dispatch;
	// 82419348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241934C: 4BFFF5E5  bl 0x82418930
	ctx.lr = 0x82419350;
	sub_82418930(ctx, base);
	// 82419350: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419354: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82419358: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241935C: 41820020  beq 0x8241937c
	if ctx.cr[0].eq {
	pc = 0x8241937C; continue 'dispatch;
	}
	// 82419360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82419364: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82419368: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8241936C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82419370: 480085F9  bl 0x82421968
	ctx.lr = 0x82419374;
	sub_82421968(ctx, base);
	// 82419374: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82419378: 480092F9  bl 0x82422670
	ctx.lr = 0x8241937C;
	sub_82422670(ctx, base);
	// 8241937C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82419380: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82419384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82419388: 419A0040  beq cr6, 0x824193c8
	if ctx.cr[6].eq {
	pc = 0x824193C8; continue 'dispatch;
	}
	// 8241938C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419390: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82419394: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82419398: 40810030  ble 0x824193c8
	if !ctx.cr[0].gt {
	pc = 0x824193C8; continue 'dispatch;
	}
	// 8241939C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824193A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824193A4: 4182000C  beq 0x824193b0
	if ctx.cr[0].eq {
	pc = 0x824193B0; continue 'dispatch;
	}
	// 824193A8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824193AC: 4800205D  bl 0x8241b408
	ctx.lr = 0x824193B0;
	sub_8241B408(ctx, base);
	// 824193B0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824193B4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824193B8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824193BC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824193C0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824193C4: 4198FFD8  blt cr6, 0x8241939c
	if ctx.cr[6].lt {
	pc = 0x8241939C; continue 'dispatch;
	}
	// 824193C8: 48015B89  bl 0x8242ef50
	ctx.lr = 0x824193CC;
	sub_8242EF50(ctx, base);
	// 824193CC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824193D0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 824193D4: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824193D8: 40810040  ble 0x82419418
	if !ctx.cr[0].gt {
	pc = 0x82419418; continue 'dispatch;
	}
	// 824193DC: 3BDF0098  addi r30, r31, 0x98
	ctx.r[30].s64 = ctx.r[31].s64 + 152;
	// 824193E0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824193E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824193E8: 41820018  beq 0x82419400
	if ctx.cr[0].eq {
	pc = 0x82419400; continue 'dispatch;
	}
	// 824193EC: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824193F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824193F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824193F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824193FC: 4E800421  bctrl
	ctx.lr = 0x82419400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82419400: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82419404: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82419408: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8241940C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82419410: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82419414: 4198FFCC  blt cr6, 0x824193e0
	if ctx.cr[6].lt {
	pc = 0x824193E0; continue 'dispatch;
	}
	// 82419418: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8241941C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419420: 41820018  beq 0x82419438
	if ctx.cr[0].eq {
	pc = 0x82419438; continue 'dispatch;
	}
	// 82419424: 939F0090  stw r28, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 82419428: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241942C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82419430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82419434: 4E800421  bctrl
	ctx.lr = 0x82419438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82419438: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241943C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82419440: 4182000C  beq 0x8241944c
	if ctx.cr[0].eq {
	pc = 0x8241944C; continue 'dispatch;
	}
	// 82419444: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82419448: 4800A2A9  bl 0x824236f0
	ctx.lr = 0x8241944C;
	sub_824236F0(ctx, base);
	// 8241944C: 38A002A0  li r5, 0x2a0
	ctx.r[5].s64 = 672;
	// 82419450: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82419454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82419458: 4811BD79  bl 0x825351d0
	ctx.lr = 0x8241945C;
	sub_825351D0(ctx, base);
	// 8241945C: 9B9F0000  stb r28, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82419460: 48015AF1  bl 0x8242ef50
	ctx.lr = 0x82419464;
	sub_8242EF50(ctx, base);
	// 82419464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82419468: 4811BCA0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


