pub fn sub_8240D880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D880 size=100
    let mut pc: u32 = 0x8240D880;
    'dispatch: loop {
        match pc {
            0x8240D880 => {
    //   block [0x8240D880..0x8240D8E4)
	// 8240D880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D88C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D890: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D894: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D898: 41980020  blt cr6, 0x8240d8b8
	if ctx.cr[6].lt {
	pc = 0x8240D8B8; continue 'dispatch;
	}
	// 8240D89C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D8A0: C00BE57C  lfs f0, -0x1a84(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6788 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D8A4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D8A8: 41990010  bgt cr6, 0x8240d8b8
	if ctx.cr[6].gt {
	pc = 0x8240D8B8; continue 'dispatch;
	}
	// 8240D8AC: D0230010  stfs f1, 0x10(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D8B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D8B4: 48000020  b 0x8240d8d4
	pc = 0x8240D8D4; continue 'dispatch;
	// 8240D8B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D8BC: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D8C0: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D8C4: 386BE530  addi r3, r11, -0x1ad0
	ctx.r[3].s64 = ctx.r[11].s64 + -6864;
	// 8240D8C8: 4BEA56B9  bl 0x822b2f80
	ctx.lr = 0x8240D8CC;
	sub_822B2F80(ctx, base);
	// 8240D8CC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D8D0: 6063003F  ori r3, r3, 0x3f
	ctx.r[3].u64 = ctx.r[3].u64 | 63;
	// 8240D8D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D8E8 size=192
    let mut pc: u32 = 0x8240D8E8;
    'dispatch: loop {
        match pc {
            0x8240D8E8 => {
    //   block [0x8240D8E8..0x8240D9A8)
	// 8240D8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240D8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240D8F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D8FC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D900: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D904: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D908: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240D90C: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240D910: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D914: C1650004  lfs f11, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240D918: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8240D91C: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8240D920: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D924: C1440008  lfs f10, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240D928: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D92C: EDAC5828  fsubs f13, f12, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8240D930: C1850008  lfs f12, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D934: ED8A6028  fsubs f12, f10, f12
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240D938: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240D93C: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240D940: 409A0014  bne cr6, 0x8240d954
	if !ctx.cr[6].eq {
	pc = 0x8240D954; continue 'dispatch;
	}
	// 8240D944: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D948: 409A000C  bne cr6, 0x8240d954
	if !ctx.cr[6].eq {
	pc = 0x8240D954; continue 'dispatch;
	}
	// 8240D94C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240D950: 419A0038  beq cr6, 0x8240d988
	if ctx.cr[6].eq {
	pc = 0x8240D988; continue 'dispatch;
	}
	// 8240D954: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240D958: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240D95C: 48002305  bl 0x8240fc60
	ctx.lr = 0x8240D960;
	sub_8240FC60(ctx, base);
	// 8240D960: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240D964: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8240D968: 480022F9  bl 0x8240fc60
	ctx.lr = 0x8240D96C;
	sub_8240FC60(ctx, base);
	// 8240D96C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240D970: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240D974: 480022FD  bl 0x8240fc70
	ctx.lr = 0x8240D978;
	sub_8240FC70(ctx, base);
	// 8240D978: 480022D9  bl 0x8240fc50
	ctx.lr = 0x8240D97C;
	sub_8240FC50(ctx, base);
	// 8240D97C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D980: C00BE580  lfs f0, -0x1a80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D984: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240D988: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D98C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D990: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240D994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D99C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240D9A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240D9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D9A8 size=312
    let mut pc: u32 = 0x8240D9A8;
    'dispatch: loop {
        match pc {
            0x8240D9A8 => {
    //   block [0x8240D9A8..0x8240DAE0)
	// 8240D9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D9AC: 4812770D  bl 0x825350b8
	ctx.lr = 0x8240D9B0;
	sub_82535080(ctx, base);
	// 8240D9B0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8240D9B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D9B8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240D9BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240D9C0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8240D9C4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240D9C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240D9CC: 419A00F4  beq cr6, 0x8240dac0
	if ctx.cr[6].eq {
	pc = 0x8240DAC0; continue 'dispatch;
	}
	// 8240D9D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8240D9D4: 419A00EC  beq cr6, 0x8240dac0
	if ctx.cr[6].eq {
	pc = 0x8240DAC0; continue 'dispatch;
	}
	// 8240D9D8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8240D9DC: 409A001C  bne cr6, 0x8240d9f8
	if !ctx.cr[6].eq {
	pc = 0x8240D9F8; continue 'dispatch;
	}
	// 8240D9E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D9E4: 386BE5D0  addi r3, r11, -0x1a30
	ctx.r[3].s64 = ctx.r[11].s64 + -6704;
	// 8240D9E8: 4BEA5599  bl 0x822b2f80
	ctx.lr = 0x8240D9EC;
	sub_822B2F80(ctx, base);
	// 8240D9EC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D9F0: 60630033  ori r3, r3, 0x33
	ctx.r[3].u64 = ctx.r[3].u64 | 51;
	// 8240D9F4: 480000E0  b 0x8240dad4
	pc = 0x8240DAD4; continue 'dispatch;
	// 8240D9F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240D9FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240DA00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240DA04: 48002255  bl 0x8240fc58
	ctx.lr = 0x8240DA08;
	sub_8240FC58(ctx, base);
	// 8240DA08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240DA0C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8240DA10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8240DA14: 48002245  bl 0x8240fc58
	ctx.lr = 0x8240DA18;
	sub_8240FC58(ctx, base);
	// 8240DA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240DA1C: 4800225D  bl 0x8240fc78
	ctx.lr = 0x8240DA20;
	sub_8240FC78(ctx, base);
	// 8240DA20: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240DA24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240DA28: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240DA2C: 48002245  bl 0x8240fc70
	ctx.lr = 0x8240DA30;
	sub_8240FC70(ctx, base);
	// 8240DA30: EC1F07F2  fmuls f0, f31, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240DA34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DA38: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA3C: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240DA40: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240DA44: 4098001C  bge cr6, 0x8240da60
	if !ctx.cr[6].lt {
	pc = 0x8240DA60; continue 'dispatch;
	}
	// 8240DA48: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA4C: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DA50: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA54: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DA58: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA5C: 48000058  b 0x8240dab4
	pc = 0x8240DAB4; continue 'dispatch;
	// 8240DA60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240DA64: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA68: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240DA6C: 4099001C  ble cr6, 0x8240da88
	if !ctx.cr[6].gt {
	pc = 0x8240DA88; continue 'dispatch;
	}
	// 8240DA70: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA74: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DA78: C01D0004  lfs f0, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA7C: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DA80: C01D0008  lfs f0, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA84: 48000030  b 0x8240dab4
	pc = 0x8240DAB4; continue 'dispatch;
	// 8240DA88: C19F0000  lfs f12, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DA8C: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA90: EDAD603A  fmadds f13, f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 8240DA94: D1BE0000  stfs f13, 0(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DA98: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA9C: C1610054  lfs f11, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240DAA0: EDAB683A  fmadds f13, f11, f0, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240DAA4: D1BE0004  stfs f13, 4(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DAA8: C1410058  lfs f10, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240DAAC: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DAB0: EC0A683A  fmadds f0, f10, f0, f13
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240DAB4: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240DAB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DABC: 48000018  b 0x8240dad4
	pc = 0x8240DAD4; continue 'dispatch;
	// 8240DAC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DAC4: 386BE588  addi r3, r11, -0x1a78
	ctx.r[3].s64 = ctx.r[11].s64 + -6776;
	// 8240DAC8: 4BEA54B9  bl 0x822b2f80
	ctx.lr = 0x8240DACC;
	sub_822B2F80(ctx, base);
	// 8240DACC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DAD0: 60630031  ori r3, r3, 0x31
	ctx.r[3].u64 = ctx.r[3].u64 | 49;
	// 8240DAD4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240DAD8: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8240DADC: 4812762C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240DAE0 size=28
    let mut pc: u32 = 0x8240DAE0;
    'dispatch: loop {
        match pc {
            0x8240DAE0 => {
    //   block [0x8240DAE0..0x8240DAFC)
	// 8240DAE0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DAE4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240DAE8: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DAEC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240DAF0: 4098000C  bge cr6, 0x8240dafc
	if !ctx.cr[6].lt {
		sub_8240DAFC(ctx, base);
		return;
	}
	// 8240DAF4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DAFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240DAFC size=12
    let mut pc: u32 = 0x8240DAFC;
    'dispatch: loop {
        match pc {
            0x8240DAFC => {
    //   block [0x8240DAFC..0x8240DB08)
	// 8240DAFC: D02A0014  stfs f1, 0x14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240DB00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240DB08 size=224
    let mut pc: u32 = 0x8240DB08;
    'dispatch: loop {
        match pc {
            0x8240DB08 => {
    //   block [0x8240DB08..0x8240DBE8)
	// 8240DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240DB0C: 481275A9  bl 0x825350b4
	ctx.lr = 0x8240DB10;
	sub_82535080(ctx, base);
	// 8240DB10: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8240DB14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240DB18: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240DB1C: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 8240DB20: 419900AC  bgt cr6, 0x8240dbcc
	if ctx.cr[6].gt {
	pc = 0x8240DBCC; continue 'dispatch;
	}
	// 8240DB24: 1D640130  mulli r11, r4, 0x130
	ctx.r[11].s64 = ctx.r[4].s64 * 304;
	// 8240DB28: 7FAB1A14  add r29, r11, r3
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240DB2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DB30: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 8240DB34: 3B60000A  li r27, 0xa
	ctx.r[27].s64 = 10;
	// 8240DB38: 3BDD001C  addi r30, r29, 0x1c
	ctx.r[30].s64 = ctx.r[29].s64 + 28;
	// 8240DB3C: C3EB2140  lfs f31, 0x2140(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8512 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240DB40: C01FFFF8  lfs f0, -8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB44: D01EFFFC  stfs f0, -4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240DB48: C03FFFFC  lfs f1, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DB4C: 48000A0D  bl 0x8240e558
	ctx.lr = 0x8240DB50;
	sub_8240E558(ctx, base);
	// 8240DB50: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DB54: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB58: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240DB5C: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DB60: C03F0004  lfs f1, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DB64: 480009F5  bl 0x8240e558
	ctx.lr = 0x8240DB68;
	sub_8240E558(ctx, base);
	// 8240DB68: D03E0008  stfs f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240DB6C: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8240DB70: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB74: D01E000C  stfs f0, 0xc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240DB78: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB7C: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240DB80: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB84: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240DB88: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240DB8C: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 8240DB90: 4082FFB0  bne 0x8240db40
	if !ctx.cr[0].eq {
	pc = 0x8240DB40; continue 'dispatch;
	}
	// 8240DB94: C01C0118  lfs f0, 0x118(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(280 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DB9C: D01D0130  stfs f0, 0x130(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8240DBA0: C01C011C  lfs f0, 0x11c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(284 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBA4: D01D0134  stfs f0, 0x134(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 8240DBA8: C01C0120  lfs f0, 0x120(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBAC: D01D0138  stfs f0, 0x138(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 8240DBB0: C01C0124  lfs f0, 0x124(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBB4: D01D013C  stfs f0, 0x13c(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 8240DBB8: C01C0128  lfs f0, 0x128(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBBC: D01D0140  stfs f0, 0x140(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 8240DBC0: C01C012C  lfs f0, 0x12c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBC4: D01D0144  stfs f0, 0x144(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(324 as u32), tmp.u32 ) };
	// 8240DBC8: 48000014  b 0x8240dbdc
	pc = 0x8240DBDC; continue 'dispatch;
	// 8240DBCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DBD0: 386BE618  addi r3, r11, -0x19e8
	ctx.r[3].s64 = ctx.r[11].s64 + -6632;
	// 8240DBD4: 4BEA53AD  bl 0x822b2f80
	ctx.lr = 0x8240DBD8;
	sub_822B2F80(ctx, base);
	// 8240DBD8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DBDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240DBE0: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8240DBE4: 48127520  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240DBE8 size=440
    let mut pc: u32 = 0x8240DBE8;
    'dispatch: loop {
        match pc {
            0x8240DBE8 => {
    //   block [0x8240DBE8..0x8240DDA0)
	// 8240DBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240DBEC: 481274BD  bl 0x825350a8
	ctx.lr = 0x8240DBF0;
	sub_82535080(ctx, base);
	// 8240DBF0: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 8240DBF4: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8240DBF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240DBFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DC00: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8240DC04: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8240DC08: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8240DC0C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8240DC10: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240DC14: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8240DC18: D3F80000  stfs f31, 0(r24)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DC1C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240DC20: D3F90000  stfs f31, 0(r25)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DC24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240DC28: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240DC2C: 2B1B001F  cmplwi cr6, r27, 0x1f
	ctx.cr[6].compare_u32(ctx.r[27].u32, 31 as u32, &mut ctx.xer);
	// 8240DC30: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240DC34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8240DC38: 41990144  bgt cr6, 0x8240dd7c
	if ctx.cr[6].gt {
	pc = 0x8240DD7C; continue 'dispatch;
	}
	// 8240DC3C: 1D7B0130  mulli r11, r27, 0x130
	ctx.r[11].s64 = ctx.r[27].s64 * 304;
	// 8240DC40: C03F0018  lfs f1, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DC44: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240DC48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8240DC4C: 3B8B0018  addi r28, r11, 0x18
	ctx.r[28].s64 = ctx.r[11].s64 + 24;
	// 8240DC50: 4BFFFA41  bl 0x8240d690
	ctx.lr = 0x8240DC54;
	sub_8240D690(ctx, base);
	// 8240DC54: 4BFF773D  bl 0x82405390
	ctx.lr = 0x8240DC58;
	sub_82405390(ctx, base);
	// 8240DC58: C3C10050  lfs f30, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240DC5C: FF1E0800  fcmpu cr6, f30, f1
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8240DC60: 41990010  bgt cr6, 0x8240dc70
	if ctx.cr[6].gt {
	pc = 0x8240DC70; continue 'dispatch;
	}
	// 8240DC64: D3F80000  stfs f31, 0(r24)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DC68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DC6C: 48000124  b 0x8240dd90
	pc = 0x8240DD90; continue 'dispatch;
	// 8240DC70: C1BD0004  lfs f13, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DC74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240DC78: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DC7C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8240DC80: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240DC84: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DC88: C1BD0008  lfs f13, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DC8C: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240DC90: C17D0000  lfs f11, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240DC94: C19F0000  lfs f12, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DC98: ED8C5828  fsubs f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8240DC9C: C17E000C  lfs f11, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240DCA0: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240DCA4: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240DCA8: EC2C033A  fmadds f1, f12, f12, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240DCAC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DCB0: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240DCB4: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240DCB8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240DCBC: 41980018  blt cr6, 0x8240dcd4
	if ctx.cr[6].lt {
	pc = 0x8240DCD4; continue 'dispatch;
	}
	// 8240DCC0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8240DCC4: 394A001C  addi r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + 28;
	// 8240DCC8: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8240DCCC: 4198FFE0  blt cr6, 0x8240dcac
	if ctx.cr[6].lt {
	pc = 0x8240DCAC; continue 'dispatch;
	}
	// 8240DCD0: 48000008  b 0x8240dcd8
	pc = 0x8240DCD8; continue 'dispatch;
	// 8240DCD4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240DCD8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240DCDC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240DCE0: 4082000C  bne 0x8240dcec
	if !ctx.cr[0].eq {
	pc = 0x8240DCEC; continue 'dispatch;
	}
	// 8240DCE4: C01C0004  lfs f0, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DCE8: 48000010  b 0x8240dcf8
	pc = 0x8240DCF8; continue 'dispatch;
	// 8240DCEC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8240DCF0: 409A0010  bne cr6, 0x8240dd00
	if !ctx.cr[6].eq {
	pc = 0x8240DD00; continue 'dispatch;
	}
	// 8240DCF4: C01C0100  lfs f0, 0x100(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(256 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DCF8: FDA0F890  fmr f13, f31
	ctx.f[13].f64 = ctx.f[31].f64;
	// 8240DCFC: 48000014  b 0x8240dd10
	pc = 0x8240DD10; continue 'dispatch;
	// 8240DD00: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 * 28;
	// 8240DD04: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240DD08: C00BFFE8  lfs f0, -0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DD0C: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DD10: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8240DD14: 4199000C  bgt cr6, 0x8240dd20
	if ctx.cr[6].gt {
	pc = 0x8240DD20; continue 'dispatch;
	}
	// 8240DD18: FF0DF800  fcmpu cr6, f13, f31
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[31].f64);
	// 8240DD1C: 4099FF48  ble cr6, 0x8240dc64
	if !ctx.cr[6].gt {
	pc = 0x8240DC64; continue 'dispatch;
	}
	// 8240DD20: 48002199  bl 0x8240feb8
	ctx.lr = 0x8240DD24;
	sub_8240FEB8(ctx, base);
	// 8240DD24: C01E000C  lfs f0, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DD28: EC210024  fdivs f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240DD2C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8240DD30: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8240DD34: D0390000  stfs f1, 0(r25)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DD38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240DD3C: 80DA0000  lwz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240DD40: 4BFFF861  bl 0x8240d5a0
	ctx.lr = 0x8240DD44;
	sub_8240D5A0(ctx, base);
	// 8240DD44: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DD48: 480007C9  bl 0x8240e510
	ctx.lr = 0x8240DD4C;
	sub_8240E510(ctx, base);
	// 8240DD4C: EC21F02A  fadds f1, f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[30].f64) as f32) as f64;
	// 8240DD50: C01C012C  lfs f0, 0x12c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DD54: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240DD58: 40990010  ble cr6, 0x8240dd68
	if !ctx.cr[6].gt {
	pc = 0x8240DD68; continue 'dispatch;
	}
	// 8240DD5C: ED810028  fsubs f12, f1, f0
	ctx.f[12].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240DD60: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DD64: EC2C037A  fmadds f1, f12, f13, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240DD68: 480007F1  bl 0x8240e558
	ctx.lr = 0x8240DD6C;
	sub_8240E558(ctx, base);
	// 8240DD6C: D0380000  stfs f1, 0(r24)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DD70: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8240DD74: 4098FEF4  bge cr6, 0x8240dc68
	if !ctx.cr[6].lt {
	pc = 0x8240DC68; continue 'dispatch;
	}
	// 8240DD78: 4BFFFEEC  b 0x8240dc64
	pc = 0x8240DC64; continue 'dispatch;
	// 8240DD7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DD80: 386BE660  addi r3, r11, -0x19a0
	ctx.r[3].s64 = ctx.r[11].s64 + -6560;
	// 8240DD84: 4BEA51FD  bl 0x822b2f80
	ctx.lr = 0x8240DD88;
	sub_822B2F80(ctx, base);
	// 8240DD88: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DD8C: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	// 8240DD90: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8240DD94: CBC1FFA8  lfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8240DD98: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8240DD9C: 4812735C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240DDA0 size=1676
    let mut pc: u32 = 0x8240DDA0;
    'dispatch: loop {
        match pc {
            0x8240DDA0 => {
    //   block [0x8240DDA0..0x8240E42C)
	// 8240DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240DDA4: 48127301  bl 0x825350a4
	ctx.lr = 0x8240DDA8;
	sub_82535080(ctx, base);
	// 8240DDA8: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 8240DDAC: 48128231  bl 0x82535fdc
	ctx.lr = 0x8240DDB0;
	sub_82535FB0(ctx, base);
	// 8240DDB0: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240DDB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240DDB8: FF200890  fmr f25, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[25].f64 = ctx.f[1].f64;
	// 8240DDBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DDC0: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8240DDC4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8240DDC8: FF401890  fmr f26, f3
	ctx.f[26].f64 = ctx.f[3].f64;
	// 8240DDCC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8240DDD0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8240DDD4: C01E0018  lfs f0, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DDD8: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 8240DDDC: C38B1FF8  lfs f28, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240DDE0: 3BFE0018  addi r31, r30, 0x18
	ctx.r[31].s64 = ctx.r[30].s64 + 24;
	// 8240DDE4: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DDE8: 409A0034  bne cr6, 0x8240de1c
	if !ctx.cr[6].eq {
	pc = 0x8240DE1C; continue 'dispatch;
	}
	// 8240DDEC: C01E001C  lfs f0, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DDF0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DDF4: 409A0028  bne cr6, 0x8240de1c
	if !ctx.cr[6].eq {
	pc = 0x8240DE1C; continue 'dispatch;
	}
	// 8240DDF8: C01E0020  lfs f0, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DDFC: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE00: 409A001C  bne cr6, 0x8240de1c
	if !ctx.cr[6].eq {
	pc = 0x8240DE1C; continue 'dispatch;
	}
	// 8240DE04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DE08: 386BE850  addi r3, r11, -0x17b0
	ctx.r[3].s64 = ctx.r[11].s64 + -6064;
	// 8240DE0C: 4BEA5175  bl 0x822b2f80
	ctx.lr = 0x8240DE10;
	sub_822B2F80(ctx, base);
	// 8240DE10: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DE14: 6063003B  ori r3, r3, 0x3b
	ctx.r[3].u64 = ctx.r[3].u64 | 59;
	// 8240DE18: 48000604  b 0x8240e41c
	pc = 0x8240E41C; continue 'dispatch;
	// 8240DE1C: C01E0024  lfs f0, 0x24(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DE20: 3B9E0024  addi r28, r30, 0x24
	ctx.r[28].s64 = ctx.r[30].s64 + 36;
	// 8240DE24: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE28: 409A0028  bne cr6, 0x8240de50
	if !ctx.cr[6].eq {
	pc = 0x8240DE50; continue 'dispatch;
	}
	// 8240DE2C: C01E0028  lfs f0, 0x28(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DE30: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE34: 409A001C  bne cr6, 0x8240de50
	if !ctx.cr[6].eq {
	pc = 0x8240DE50; continue 'dispatch;
	}
	// 8240DE38: C01E002C  lfs f0, 0x2c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DE3C: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE40: 409A0010  bne cr6, 0x8240de50
	if !ctx.cr[6].eq {
	pc = 0x8240DE50; continue 'dispatch;
	}
	// 8240DE44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DE48: 386BE808  addi r3, r11, -0x17f8
	ctx.r[3].s64 = ctx.r[11].s64 + -6136;
	// 8240DE4C: 480005C8  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
	// 8240DE50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240DE54: 4800206D  bl 0x8240fec0
	ctx.lr = 0x8240DE58;
	sub_8240FEC0(ctx, base);
	// 8240DE58: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE5C: 419A05B0  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE60: 3AFE000C  addi r23, r30, 0xc
	ctx.r[23].s64 = ctx.r[30].s64 + 12;
	// 8240DE64: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8240DE68: 48002059  bl 0x8240fec0
	ctx.lr = 0x8240DE6C;
	sub_8240FEC0(ctx, base);
	// 8240DE6C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE70: 419A059C  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240DE78: 48002049  bl 0x8240fec0
	ctx.lr = 0x8240DE7C;
	sub_8240FEC0(ctx, base);
	// 8240DE7C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE80: 419A058C  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240DE88: 48002039  bl 0x8240fec0
	ctx.lr = 0x8240DE8C;
	sub_8240FEC0(ctx, base);
	// 8240DE8C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE90: 419A057C  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8240DE98: 48002029  bl 0x8240fec0
	ctx.lr = 0x8240DE9C;
	sub_8240FEC0(ctx, base);
	// 8240DE9C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DEA0: 419A0560  beq cr6, 0x8240e400
	if ctx.cr[6].eq {
	pc = 0x8240E400; continue 'dispatch;
	}
	// 8240DEA4: 3B1A000C  addi r24, r26, 0xc
	ctx.r[24].s64 = ctx.r[26].s64 + 12;
	// 8240DEA8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8240DEAC: 48002015  bl 0x8240fec0
	ctx.lr = 0x8240DEB0;
	sub_8240FEC0(ctx, base);
	// 8240DEB0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DEB4: 419A054C  beq cr6, 0x8240e400
	if ctx.cr[6].eq {
	pc = 0x8240E400; continue 'dispatch;
	}
	// 8240DEB8: 2B1D001F  cmplwi cr6, r29, 0x1f
	ctx.cr[6].compare_u32(ctx.r[29].u32, 31 as u32, &mut ctx.xer);
	// 8240DEBC: 41990528  bgt cr6, 0x8240e3e4
	if ctx.cr[6].gt {
	pc = 0x8240E3E4; continue 'dispatch;
	}
	// 8240DEC0: C1BF0000  lfs f13, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DEC4: 1D7D0130  mulli r11, r29, 0x130
	ctx.r[11].s64 = ctx.r[29].s64 * 304;
	// 8240DEC8: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DECC: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8240DED0: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8240DED4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8240DED8: C19E001C  lfs f12, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DEDC: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DEE0: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240DEE4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240DEE8: C1BE0020  lfs f13, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DEEC: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DEF0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240DEF4: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240DEF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240DEFC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240DF00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240DF04: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8240DF08: 3B8B0018  addi r28, r11, 0x18
	ctx.r[28].s64 = ctx.r[11].s64 + 24;
	// 8240DF0C: 48001D7D  bl 0x8240fc88
	ctx.lr = 0x8240DF10;
	sub_8240FC88(ctx, base);
	// 8240DF10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240DF14: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8240DF18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8240DF1C: 48126C35  bl 0x82534b50
	ctx.lr = 0x8240DF20;
	sub_82534B50(ctx, base);
	// 8240DF20: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 8240DF24: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8240DF28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240DF2C: 48001D55  bl 0x8240fc80
	ctx.lr = 0x8240DF30;
	sub_8240FC80(ctx, base);
	// 8240DF30: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240DF34: 48001FF5  bl 0x8240ff28
	ctx.lr = 0x8240DF38;
	sub_8240FF28(ctx, base);
	// 8240DF38: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DF3C: 409A0010  bne cr6, 0x8240df4c
	if !ctx.cr[6].eq {
	pc = 0x8240DF4C; continue 'dispatch;
	}
	// 8240DF40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DF44: 386BE7C0  addi r3, r11, -0x1840
	ctx.r[3].s64 = ctx.r[11].s64 + -6208;
	// 8240DF48: 480004CC  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
	// 8240DF4C: 83E10224  lwz r31, 0x224(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(548 as u32) ) } as u64;
	// 8240DF50: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8240DF54: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 8240DF58: 38FF00A4  addi r7, r31, 0xa4
	ctx.r[7].s64 = ctx.r[31].s64 + 164;
	// 8240DF5C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8240DF60: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8240DF64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240DF68: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8240DF6C: 4BFFFC7D  bl 0x8240dbe8
	ctx.lr = 0x8240DF70;
	sub_8240DBE8(ctx, base);
	// 8240DF70: C01F00A4  lfs f0, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DF74: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DF78: 41990050  bgt cr6, 0x8240dfc8
	if ctx.cr[6].gt {
	pc = 0x8240DFC8; continue 'dispatch;
	}
	// 8240DF7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8240DF80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240DF84: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 8240DF88: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8240DF8C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240DF90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240DF94: 4200FFF8  bdnz 0x8240df8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240DF8C; continue 'dispatch;
	}
	// 8240DF98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DF9C: D39F0094  stfs f28, 0x94(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240DFA0: D39F0098  stfs f28, 0x98(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8240DFA4: D39F009C  stfs f28, 0x9c(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240DFA8: D39F00A8  stfs f28, 0xa8(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8240DFAC: C00B2268  lfs f0, 0x2268(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240DFB4: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8240DFB8: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFBC: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8240DFC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DFC4: 48000458  b 0x8240e41c
	pc = 0x8240E41C; continue 'dispatch;
	// 8240DFC8: C0010078  lfs f0, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFCC: C1A10070  lfs f13, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DFD0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DFD4: 4099003C  ble cr6, 0x8240e010
	if !ctx.cr[6].gt {
	pc = 0x8240E010; continue 'dispatch;
	}
	// 8240DFD8: FF0DE000  fcmpu cr6, f13, f28
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[28].f64);
	// 8240DFDC: EC2D0024  fdivs f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240DFE0: 40990018  ble cr6, 0x8240dff8
	if !ctx.cr[6].gt {
	pc = 0x8240DFF8; continue 'dispatch;
	}
	// 8240DFE4: 48001CED  bl 0x8240fcd0
	ctx.lr = 0x8240DFE8;
	sub_8240FCD0(ctx, base);
	// 8240DFE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240DFEC: C00B2930  lfs f0, 0x2930(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFF0: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8240DFF4: 48000078  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
	// 8240DFF8: 48001CD9  bl 0x8240fcd0
	ctx.lr = 0x8240DFFC;
	sub_8240FCD0(ctx, base);
	// 8240DFFC: FDA00850  fneg f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240E000: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E004: C00B2930  lfs f0, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E008: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E00C: 48000060  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
	// 8240E010: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E014: 409A0028  bne cr6, 0x8240e03c
	if !ctx.cr[6].eq {
	pc = 0x8240E03C; continue 'dispatch;
	}
	// 8240E018: FF0DE000  fcmpu cr6, f13, f28
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[28].f64);
	// 8240E01C: 40990010  ble cr6, 0x8240e02c
	if !ctx.cr[6].gt {
	pc = 0x8240E02C; continue 'dispatch;
	}
	// 8240E020: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E024: C00BE358  lfs f0, -0x1ca8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E028: 48000044  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
	// 8240E02C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E030: C00BE7B8  lfs f0, -0x1848(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E034: FC0D072E  fsel f0, f13, f28, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[28].f64 } else { ctx.f[0].f64 };
	// 8240E038: 48000034  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
	// 8240E03C: FF0DE000  fcmpu cr6, f13, f28
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[28].f64);
	// 8240E040: 409A0020  bne cr6, 0x8240e060
	if !ctx.cr[6].eq {
	pc = 0x8240E060; continue 'dispatch;
	}
	// 8240E044: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E048: 40990010  ble cr6, 0x8240e058
	if !ctx.cr[6].gt {
	pc = 0x8240E058; continue 'dispatch;
	}
	// 8240E04C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E050: C00B2930  lfs f0, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E054: 48000018  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
	// 8240E058: FC00E090  fmr f0, f28
	ctx.f[0].f64 = ctx.f[28].f64;
	// 8240E05C: 48000010  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
	// 8240E060: EC2D0024  fdivs f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E064: 48001C6D  bl 0x8240fcd0
	ctx.lr = 0x8240E068;
	sub_8240FCD0(ctx, base);
	// 8240E068: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240E06C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E070: C1ABE580  lfs f13, -0x1a80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6784 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E074: EFE00372  fmuls f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240E078: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240E07C: 48001F25  bl 0x8240ffa0
	ctx.lr = 0x8240E080;
	sub_8240FFA0(ctx, base);
	// 8240E080: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240E084: 409A0010  bne cr6, 0x8240e094
	if !ctx.cr[6].eq {
	pc = 0x8240E094; continue 'dispatch;
	}
	// 8240E088: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E08C: 386BE778  addi r3, r11, -0x1888
	ctx.r[3].s64 = ctx.r[11].s64 + -6280;
	// 8240E090: 48000384  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
	// 8240E094: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E098: C01C0118  lfs f0, 0x118(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(280 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E09C: C3C1005C  lfs f30, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240E0A0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E0A4: C36B1850  lfs f27, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240E0A8: 4099000C  ble cr6, 0x8240e0b4
	if !ctx.cr[6].gt {
	pc = 0x8240E0B4; continue 'dispatch;
	}
	// 8240E0AC: EC5E0024  fdivs f2, f30, f0
	ctx.f[2].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E0B0: 48000018  b 0x8240e0c8
	pc = 0x8240E0C8; continue 'dispatch;
	// 8240E0B4: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E0B8: 40980020  bge cr6, 0x8240e0d8
	if !ctx.cr[6].lt {
	pc = 0x8240E0D8; continue 'dispatch;
	}
	// 8240E0BC: EC1E0024  fdivs f0, f30, f0
	ctx.f[0].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E0C0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240E0C4: EC5B0028  fsubs f2, f27, f0
	ctx.f[2].f64 = (((ctx.f[27].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E0C8: FF02D800  fcmpu cr6, f2, f27
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[27].f64);
	// 8240E0CC: 41980010  blt cr6, 0x8240e0dc
	if ctx.cr[6].lt {
	pc = 0x8240E0DC; continue 'dispatch;
	}
	// 8240E0D0: FC40D890  fmr f2, f27
	ctx.f[2].f64 = ctx.f[27].f64;
	// 8240E0D4: 48000008  b 0x8240e0dc
	pc = 0x8240E0DC; continue 'dispatch;
	// 8240E0D8: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8240E0DC: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 8240E0E0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240E0E4: 409A004C  bne cr6, 0x8240e130
	if !ctx.cr[6].eq {
	pc = 0x8240E130; continue 'dispatch;
	}
	// 8240E0E8: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 8240E0EC: FC60D090  fmr f3, f26
	ctx.f[3].f64 = ctx.f[26].f64;
	// 8240E0F0: 4BFF8069  bl 0x82406158
	ctx.lr = 0x8240E0F4;
	sub_82406158(ctx, base);
	// 8240E0F4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8240E0F8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8240E0FC: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 8240E100: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8240E104: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8240E108: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8240E10C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E110: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240E114: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8240E118: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8240E11C: 4200FFF0  bdnz 0x8240e10c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240E10C; continue 'dispatch;
	}
	// 8240E120: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8240E124: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8240E128: 4082FFD4  bne 0x8240e0fc
	if !ctx.cr[0].eq {
	pc = 0x8240E0FC; continue 'dispatch;
	}
	// 8240E12C: 48000030  b 0x8240e15c
	pc = 0x8240E15C; continue 'dispatch;
	// 8240E130: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 8240E134: 409A001C  bne cr6, 0x8240e150
	if !ctx.cr[6].eq {
	pc = 0x8240E150; continue 'dispatch;
	}
	// 8240E138: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8240E13C: FC601090  fmr f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[2].f64;
	// 8240E140: FC80D090  fmr f4, f26
	ctx.f[4].f64 = ctx.f[26].f64;
	// 8240E144: FC40C890  fmr f2, f25
	ctx.f[2].f64 = ctx.f[25].f64;
	// 8240E148: 4BFF8121  bl 0x82406268
	ctx.lr = 0x8240E14C;
	sub_82406268(ctx, base);
	// 8240E14C: 48000010  b 0x8240e15c
	pc = 0x8240E15C; continue 'dispatch;
	// 8240E150: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8240E154: FC60D090  fmr f3, f26
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[26].f64;
	// 8240E158: 4BFF8031  bl 0x82406188
	ctx.lr = 0x8240E15C;
	sub_82406188(ctx, base);
	// 8240E15C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8240E160: D3FF00A8  stfs f31, 0xa8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8240E164: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240E168: 419A0134  beq cr6, 0x8240e29c
	if ctx.cr[6].eq {
	pc = 0x8240E29C; continue 'dispatch;
	}
	// 8240E16C: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8240E170: 419A0128  beq cr6, 0x8240e298
	if ctx.cr[6].eq {
	pc = 0x8240E298; continue 'dispatch;
	}
	// 8240E174: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 * 28;
	// 8240E178: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240E17C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E180: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E184: C18BFFF0  lfs f12, -0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E188: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E18C: 419A0020  beq cr6, 0x8240e1ac
	if ctx.cr[6].eq {
	pc = 0x8240E1AC; continue 'dispatch;
	}
	// 8240E190: C16B000C  lfs f11, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E194: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E198: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E19C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E1A0: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E1A4: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E1A8: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240E1AC: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 8240E1B0: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8240E1B4: D1A90000  stfs f13, 0(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240E1B8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E1BC: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 8240E1C0: 4082FFF4  bne 0x8240e1b4
	if !ctx.cr[0].eq {
	pc = 0x8240E1B4; continue 'dispatch;
	}
	// 8240E1C4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E1C8: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E1CC: C18BFFEC  lfs f12, -0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E1D0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E1D4: 419A0020  beq cr6, 0x8240e1f4
	if ctx.cr[6].eq {
	pc = 0x8240E1F4; continue 'dispatch;
	}
	// 8240E1D8: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E1DC: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E1E0: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E1E4: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E1E8: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E1EC: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E1F0: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240E1F4: D1BF0090  stfs f13, 0x90(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8240E1F8: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E1FC: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E200: C18BFFF4  lfs f12, -0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E204: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E208: 419A0020  beq cr6, 0x8240e228
	if ctx.cr[6].eq {
	pc = 0x8240E228; continue 'dispatch;
	}
	// 8240E20C: C16B0010  lfs f11, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E210: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E214: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E218: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E21C: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E220: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E224: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240E228: D1BF0094  stfs f13, 0x94(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240E22C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E230: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E234: C18BFFF8  lfs f12, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E238: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E23C: 419A0020  beq cr6, 0x8240e25c
	if ctx.cr[6].eq {
	pc = 0x8240E25C; continue 'dispatch;
	}
	// 8240E240: C16B0014  lfs f11, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E244: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E248: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E24C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E250: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E254: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E258: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240E25C: D1BF0098  stfs f13, 0x98(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8240E260: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E264: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E268: C18BFFFC  lfs f12, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E26C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E270: 419A0020  beq cr6, 0x8240e290
	if ctx.cr[6].eq {
	pc = 0x8240E290; continue 'dispatch;
	}
	// 8240E274: C16B0018  lfs f11, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E278: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E27C: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E280: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E284: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E288: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E28C: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240E290: D1BF009C  stfs f13, 0x9c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240E294: 4800004C  b 0x8240e2e0
	pc = 0x8240E2E0; continue 'dispatch;
	// 8240E298: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 8240E29C: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 * 28;
	// 8240E2A0: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240E2A4: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 8240E2A8: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8240E2AC: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2B0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E2B4: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240E2B8: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 8240E2BC: 4082FFF0  bne 0x8240e2ac
	if !ctx.cr[0].eq {
	pc = 0x8240E2AC; continue 'dispatch;
	}
	// 8240E2C0: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2C4: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8240E2C8: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2CC: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240E2D0: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2D4: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8240E2D8: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2DC: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240E2E0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8240E2E4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240E2E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240E2EC: 4800196D  bl 0x8240fc58
	ctx.lr = 0x8240E2F0;
	sub_8240FC58(ctx, base);
	// 8240E2F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8240E2F4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8240E2F8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8240E2FC: 4800195D  bl 0x8240fc58
	ctx.lr = 0x8240E300;
	sub_8240FC58(ctx, base);
	// 8240E300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240E304: 48001975  bl 0x8240fc78
	ctx.lr = 0x8240E308;
	sub_8240FC78(ctx, base);
	// 8240E308: FF01E000  fcmpu cr6, f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[28].f64);
	// 8240E30C: 409900A8  ble cr6, 0x8240e3b4
	if !ctx.cr[6].gt {
	pc = 0x8240E3B4; continue 'dispatch;
	}
	// 8240E310: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8240E314: 48001965  bl 0x8240fc78
	ctx.lr = 0x8240E318;
	sub_8240FC78(ctx, base);
	// 8240E318: FF01E000  fcmpu cr6, f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[28].f64);
	// 8240E31C: 40990098  ble cr6, 0x8240e3b4
	if !ctx.cr[6].gt {
	pc = 0x8240E3B4; continue 'dispatch;
	}
	// 8240E320: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240E324: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8240E328: 48001939  bl 0x8240fc60
	ctx.lr = 0x8240E32C;
	sub_8240FC60(ctx, base);
	// 8240E32C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8240E330: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8240E334: 4800192D  bl 0x8240fc60
	ctx.lr = 0x8240E338;
	sub_8240FC60(ctx, base);
	// 8240E338: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8240E33C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8240E340: 48001931  bl 0x8240fc70
	ctx.lr = 0x8240E344;
	sub_8240FC70(ctx, base);
	// 8240E344: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E348: C00B2074  lfs f0, 0x2074(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E34C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240E350: 419800C8  blt cr6, 0x8240e418
	if ctx.cr[6].lt {
	pc = 0x8240E418; continue 'dispatch;
	}
	// 8240E354: FF01D800  fcmpu cr6, f1, f27
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[27].f64);
	// 8240E358: 419900C0  bgt cr6, 0x8240e418
	if ctx.cr[6].gt {
	pc = 0x8240E418; continue 'dispatch;
	}
	// 8240E35C: 480018F5  bl 0x8240fc50
	ctx.lr = 0x8240E360;
	sub_8240FC50(ctx, base);
	// 8240E360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240E364: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E368: 48001911  bl 0x8240fc78
	ctx.lr = 0x8240E36C;
	sub_8240FC78(ctx, base);
	// 8240E36C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240E370: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240E374: 480018D5  bl 0x8240fc48
	ctx.lr = 0x8240E378;
	sub_8240FC48(ctx, base);
	// 8240E378: C1B9000C  lfs f13, 0xc(r25)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E37C: ED6107B2  fmuls f11, f1, f30
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[30].f64) as f32) as f64);
	// 8240E380: C0190014  lfs f0, 0x14(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E384: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E388: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240E38C: C1B90008  lfs f13, 8(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E390: C1990000  lfs f12, 0(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E394: ED8B0332  fmuls f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240E398: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240E39C: C00B2238  lfs f0, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E3A0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E3A4: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E3A8: EC1B0028  fsubs f0, f27, f0
	ctx.f[0].f64 = (((ctx.f[27].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E3AC: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8240E3B0: 48000008  b 0x8240e3b8
	pc = 0x8240E3B8; continue 'dispatch;
	// 8240E3B4: D37F00A0  stfs f27, 0xa0(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8240E3B8: C0190004  lfs f0, 4(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E3BC: C1BF00A0  lfs f13, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E3C0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E3C4: 40990008  ble cr6, 0x8240e3cc
	if !ctx.cr[6].gt {
	pc = 0x8240E3CC; continue 'dispatch;
	}
	// 8240E3C8: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8240E3CC: C0190004  lfs f0, 4(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E3D0: EC1B0024  fdivs f0, f27, f0
	ctx.f[0].f64 = ((ctx.f[27].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E3D4: C1BF00A0  lfs f13, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E3D8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E3DC: 4098FBE4  bge cr6, 0x8240dfc0
	if !ctx.cr[6].lt {
	pc = 0x8240DFC0; continue 'dispatch;
	}
	// 8240E3E0: 4BFFFBDC  b 0x8240dfbc
	pc = 0x8240DFBC; continue 'dispatch;
	// 8240E3E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E3E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240E3EC: 386BE730  addi r3, r11, -0x18d0
	ctx.r[3].s64 = ctx.r[11].s64 + -6352;
	// 8240E3F0: 4BEA4B91  bl 0x822b2f80
	ctx.lr = 0x8240E3F4;
	sub_822B2F80(ctx, base);
	// 8240E3F4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240E3F8: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	// 8240E3FC: 48000020  b 0x8240e41c
	pc = 0x8240E41C; continue 'dispatch;
	// 8240E400: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E404: 386BE6E8  addi r3, r11, -0x1918
	ctx.r[3].s64 = ctx.r[11].s64 + -6424;
	// 8240E408: 4800000C  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
	// 8240E40C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E410: 386BE6A0  addi r3, r11, -0x1960
	ctx.r[3].s64 = ctx.r[11].s64 + -6496;
	// 8240E414: 4BEA4B6D  bl 0x822b2f80
	ctx.lr = 0x8240E418;
	sub_822B2F80(ctx, base);
	// 8240E418: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240E41C: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 8240E420: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 8240E424: 48127C05  bl 0x82536028
	ctx.lr = 0x8240E428;
	sub_82535FFC(ctx, base);
	// 8240E428: 48126CCC  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E430 size=56
    let mut pc: u32 = 0x8240E430;
    'dispatch: loop {
        match pc {
            0x8240E430 => {
    //   block [0x8240E430..0x8240E468)
	// 8240E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E43C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E440: FC601090  fmr f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[2].f64;
	// 8240E444: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8240E448: FC400890  fmr f2, f1
	ctx.f[2].f64 = ctx.f[1].f64;
	// 8240E44C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8240E450: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E454: 4BFFF94D  bl 0x8240dda0
	ctx.lr = 0x8240E458;
	sub_8240DDA0(ctx, base);
	// 8240E458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E468 size=44
    let mut pc: u32 = 0x8240E468;
    'dispatch: loop {
        match pc {
            0x8240E468 => {
    //   block [0x8240E468..0x8240E494)
	// 8240E468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E474: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8240E478: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8240E47C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8240E480: 4BFFF921  bl 0x8240dda0
	ctx.lr = 0x8240E484;
	sub_8240DDA0(ctx, base);
	// 8240E484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E498 size=56
    let mut pc: u32 = 0x8240E498;
    'dispatch: loop {
        match pc {
            0x8240E498 => {
    //   block [0x8240E498..0x8240E4D0)
	// 8240E498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E4A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E4A8: FC601090  fmr f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[2].f64;
	// 8240E4AC: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8240E4B0: FC400890  fmr f2, f1
	ctx.f[2].f64 = ctx.f[1].f64;
	// 8240E4B4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8240E4B8: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E4BC: 4BFFF8E5  bl 0x8240dda0
	ctx.lr = 0x8240E4C0;
	sub_8240DDA0(ctx, base);
	// 8240E4C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E4D0 size=4
    let mut pc: u32 = 0x8240E4D0;
    'dispatch: loop {
        match pc {
            0x8240E4D0 => {
    //   block [0x8240E4D0..0x8240E4D4)
	// 8240E4D0: 48001AD8  b 0x8240ffa8
	sub_8240FFA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E4D8 size=4
    let mut pc: u32 = 0x8240E4D8;
    'dispatch: loop {
        match pc {
            0x8240E4D8 => {
    //   block [0x8240E4D8..0x8240E4DC)
	// 8240E4D8: 48001AD8  b 0x8240ffb0
	sub_8240FFB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E4E0 size=40
    let mut pc: u32 = 0x8240E4E0;
    'dispatch: loop {
        match pc {
            0x8240E4E0 => {
    //   block [0x8240E4E0..0x8240E508)
	// 8240E4E0: 3D600696  lis r11, 0x696
	ctx.r[11].s64 = 110493696;
	// 8240E4E4: 6169542D  ori r9, r11, 0x542d
	ctx.r[9].u64 = ctx.r[11].u64 | 21549;
	// 8240E4E8: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8240E4EC: 814BF208  lwz r10, -0xdf8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3576 as u32) ) } as u64;
	// 8240E4F0: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8240E4F4: 394A3039  addi r10, r10, 0x3039
	ctx.r[10].s64 = ctx.r[10].s64 + 12345;
	// 8240E4F8: 914BF208  stw r10, -0xdf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3576 as u32), ctx.r[10].u32 ) };
	// 8240E4FC: A16BF208  lhz r11, -0xdf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-3576 as u32) ) } as u64;
	// 8240E500: 5563047E  clrlwi r3, r11, 0x11
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 8240E504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E508 size=8
    let mut pc: u32 = 0x8240E508;
    'dispatch: loop {
        match pc {
            0x8240E508 => {
    //   block [0x8240E508..0x8240E510)
	// 8240E508: 38607FFF  li r3, 0x7fff
	ctx.r[3].s64 = 32767;
	// 8240E50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E510 size=68
    let mut pc: u32 = 0x8240E510;
    'dispatch: loop {
        match pc {
            0x8240E510 => {
    //   block [0x8240E510..0x8240E554)
	// 8240E510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E51C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E520: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E524: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240E528: 40990018  ble cr6, 0x8240e540
	if !ctx.cr[6].gt {
	pc = 0x8240E540; continue 'dispatch;
	}
	// 8240E52C: 48001A95  bl 0x8240ffc0
	ctx.lr = 0x8240E530;
	sub_8240FFC0(ctx, base);
	// 8240E530: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E534: C00B2938  lfs f0, 0x2938(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10552 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E538: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E53C: 48000008  b 0x8240e544
	pc = 0x8240E544; continue 'dispatch;
	// 8240E540: 4BFF6E51  bl 0x82405390
	ctx.lr = 0x8240E544;
	sub_82405390(ctx, base);
	// 8240E544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E558 size=88
    let mut pc: u32 = 0x8240E558;
    'dispatch: loop {
        match pc {
            0x8240E558 => {
    //   block [0x8240E558..0x8240E5B0)
	// 8240E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E560: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240E564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E568: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E56C: 4BFF6E25  bl 0x82405390
	ctx.lr = 0x8240E570;
	sub_82405390(ctx, base);
	// 8240E570: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8240E574: 41990010  bgt cr6, 0x8240e584
	if ctx.cr[6].gt {
	pc = 0x8240E584; continue 'dispatch;
	}
	// 8240E578: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E57C: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E580: 4800001C  b 0x8240e59c
	pc = 0x8240E59C; continue 'dispatch;
	// 8240E584: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E588: C00B2954  lfs f0, 0x2954(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E58C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E590: EC5F0032  fmuls f2, f31, f0
	ctx.f[2].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E594: C02B2934  lfs f1, 0x2934(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10548 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E598: 48001A79  bl 0x82410010
	ctx.lr = 0x8240E59C;
	sub_82410010(ctx, base);
	// 8240E59C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E5A8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E5B0 size=172
    let mut pc: u32 = 0x8240E5B0;
    'dispatch: loop {
        match pc {
            0x8240E5B0 => {
    //   block [0x8240E5B0..0x8240E65C)
	// 8240E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E5B8: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 8240E5BC: 48127A29  bl 0x82535fe4
	ctx.lr = 0x8240E5C0;
	sub_82535FB0(ctx, base);
	// 8240E5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E5C4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E5C8: FF801090  fmr f28, f2
	ctx.f[28].f64 = ctx.f[2].f64;
	// 8240E5CC: 4BFF6DC5  bl 0x82405390
	ctx.lr = 0x8240E5D0;
	sub_82405390(ctx, base);
	// 8240E5D0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8240E5D4: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8240E5D8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8240E5DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E5E0: C3691FF8  lfs f27, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240E5E4: C3AA2934  lfs f29, 0x2934(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10548 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240E5E8: C3CB8E30  lfs f30, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240E5EC: 4199000C  bgt cr6, 0x8240e5f8
	if ctx.cr[6].gt {
	pc = 0x8240E5F8; continue 'dispatch;
	}
	// 8240E5F0: FFE0D890  fmr f31, f27
	ctx.f[31].f64 = ctx.f[27].f64;
	// 8240E5F4: 48000014  b 0x8240e608
	pc = 0x8240E608; continue 'dispatch;
	// 8240E5F8: EC5F07B2  fmuls f2, f31, f30
	ctx.f[2].f64 = (((ctx.f[31].f64 * ctx.f[30].f64) as f32) as f64);
	// 8240E5FC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240E600: 48001A11  bl 0x82410010
	ctx.lr = 0x8240E604;
	sub_82410010(ctx, base);
	// 8240E604: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E608: 4BFF6D89  bl 0x82405390
	ctx.lr = 0x8240E60C;
	sub_82405390(ctx, base);
	// 8240E60C: FF1C0800  fcmpu cr6, f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[1].f64);
	// 8240E610: 4199000C  bgt cr6, 0x8240e61c
	if ctx.cr[6].gt {
	pc = 0x8240E61C; continue 'dispatch;
	}
	// 8240E614: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 8240E618: 48000010  b 0x8240e628
	pc = 0x8240E628; continue 'dispatch;
	// 8240E61C: EC5C07B2  fmuls f2, f28, f30
	ctx.f[2].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 8240E620: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240E624: 480019ED  bl 0x82410010
	ctx.lr = 0x8240E628;
	sub_82410010(ctx, base);
	// 8240E628: EC21F82A  fadds f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240E62C: FF01D800  fcmpu cr6, f1, f27
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[27].f64);
	// 8240E630: 40990010  ble cr6, 0x8240e640
	if !ctx.cr[6].gt {
	pc = 0x8240E640; continue 'dispatch;
	}
	// 8240E634: 4800198D  bl 0x8240ffc0
	ctx.lr = 0x8240E638;
	sub_8240FFC0(ctx, base);
	// 8240E638: EC210772  fmuls f1, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[29].f64) as f32) as f64);
	// 8240E63C: 48000008  b 0x8240e644
	pc = 0x8240E644; continue 'dispatch;
	// 8240E640: 4BFF6D51  bl 0x82405390
	ctx.lr = 0x8240E644;
	sub_82405390(ctx, base);
	// 8240E644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E648: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 8240E64C: 481279E5  bl 0x82536030
	ctx.lr = 0x8240E650;
	sub_82535FFC(ctx, base);
	// 8240E650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E660 size=56
    let mut pc: u32 = 0x8240E660;
    'dispatch: loop {
        match pc {
            0x8240E660 => {
    //   block [0x8240E660..0x8240E698)
	// 8240E660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E66C: 4800197D  bl 0x8240ffe8
	ctx.lr = 0x8240E670;
	sub_8240FFE8(ctx, base);
	// 8240E670: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E674: C00BE8B4  lfs f0, -0x174c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5964 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E678: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E67C: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E680: C00BE8B8  lfs f0, -0x1748(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5960 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E684: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E698 size=144
    let mut pc: u32 = 0x8240E698;
    'dispatch: loop {
        match pc {
            0x8240E698 => {
    //   block [0x8240E698..0x8240E728)
	// 8240E698: 39632204  addi r11, r3, 0x2204
	ctx.r[11].s64 = ctx.r[3].s64 + 8708;
	// 8240E69C: 39230014  addi r9, r3, 0x14
	ctx.r[9].s64 = ctx.r[3].s64 + 20;
	// 8240E6A0: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 8240E6A4: 8149FFEC  lwz r10, -0x14(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-20 as u32) ) } as u64;
	// 8240E6A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8240E6AC: 419A0060  beq cr6, 0x8240e70c
	if ctx.cr[6].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E6B0: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E6B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240E6B8: 41820054  beq 0x8240e70c
	if ctx.cr[0].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E6BC: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8240E6C0: 409A0010  bne cr6, 0x8240e6d0
	if !ctx.cr[6].eq {
	pc = 0x8240E6D0; continue 'dispatch;
	}
	// 8240E6C4: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8240E6C8: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E6CC: 40820040  bne 0x8240e70c
	if !ctx.cr[0].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E6D0: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8240E6D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E6D8: 41980020  blt cr6, 0x8240e6f8
	if ctx.cr[6].lt {
	pc = 0x8240E6F8; continue 'dispatch;
	}
	// 8240E6DC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E6E0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8240E6E4: 409A0008  bne cr6, 0x8240e6ec
	if !ctx.cr[6].eq {
	pc = 0x8240E6EC; continue 'dispatch;
	}
	// 8240E6E8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240E6EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240E6F0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8240E6F4: 409A0018  bne cr6, 0x8240e70c
	if !ctx.cr[6].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E6F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E6FC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240E700: 4081000C  ble 0x8240e70c
	if !ctx.cr[0].gt {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E704: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 8240E708: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240E70C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8240E710: 39290088  addi r9, r9, 0x88
	ctx.r[9].s64 = ctx.r[9].s64 + 136;
	// 8240E714: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8240E718: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8240E71C: 4082FF88  bne 0x8240e6a4
	if !ctx.cr[0].eq {
	pc = 0x8240E6A4; continue 'dispatch;
	}
	// 8240E720: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E728 size=100
    let mut pc: u32 = 0x8240E728;
    'dispatch: loop {
        match pc {
            0x8240E728 => {
    //   block [0x8240E728..0x8240E78C)
	// 8240E728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E734: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240E738: 41990034  bgt cr6, 0x8240e76c
	if ctx.cr[6].gt {
	pc = 0x8240E76C; continue 'dispatch;
	}
	// 8240E73C: 1D640088  mulli r11, r4, 0x88
	ctx.r[11].s64 = ctx.r[4].s64 * 136;
	// 8240E740: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240E744: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 8240E748: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E74C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8240E750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E754: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240E758: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240E75C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240E760: 816A2200  lwz r11, 0x2200(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8704 as u32) ) } as u64;
	// 8240E764: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240E768: 48000014  b 0x8240e77c
	pc = 0x8240E77C; continue 'dispatch;
	// 8240E76C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E770: 386BE8BC  addi r3, r11, -0x1744
	ctx.r[3].s64 = ctx.r[11].s64 + -5956;
	// 8240E774: 4BEA480D  bl 0x822b2f80
	ctx.lr = 0x8240E778;
	sub_822B2F80(ctx, base);
	// 8240E778: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240E77C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E790 size=140
    let mut pc: u32 = 0x8240E790;
    'dispatch: loop {
        match pc {
            0x8240E790 => {
    //   block [0x8240E790..0x8240E81C)
	// 8240E790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E794: 48126925  bl 0x825350b8
	ctx.lr = 0x8240E798;
	sub_82535080(ctx, base);
	// 8240E798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E79C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240E7A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240E7A4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8240E7A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240E7AC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8240E7B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E7B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8240E7B8: 419A0018  beq cr6, 0x8240e7d0
	if ctx.cr[6].eq {
	pc = 0x8240E7D0; continue 'dispatch;
	}
	// 8240E7BC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240E7C0: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E7C4: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240E7C8: 4198FFE8  blt cr6, 0x8240e7b0
	if ctx.cr[6].lt {
	pc = 0x8240E7B0; continue 'dispatch;
	}
	// 8240E7CC: 4800001C  b 0x8240e7e8
	pc = 0x8240E7E8; continue 'dispatch;
	// 8240E7D0: 1D7F0088  mulli r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 * 136;
	// 8240E7D4: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240E7D8: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 8240E7DC: 48126375  bl 0x82534b50
	ctx.lr = 0x8240E7E0;
	sub_82534B50(ctx, base);
	// 8240E7E0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240E7E4: 4098000C  bge cr6, 0x8240e7f0
	if !ctx.cr[6].lt {
	pc = 0x8240E7F0; continue 'dispatch;
	}
	// 8240E7E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240E7EC: 48000028  b 0x8240e814
	pc = 0x8240E814; continue 'dispatch;
	// 8240E7F0: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 * 12;
	// 8240E7F4: 395F02D6  addi r10, r31, 0x2d6
	ctx.r[10].s64 = ctx.r[31].s64 + 726;
	// 8240E7F8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240E7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240E800: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 8240E804: 93AB2204  stw r29, 0x2204(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8708 as u32), ctx.r[29].u32 ) };
	// 8240E808: 7D2AF12E  stwx r9, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u32) };
	// 8240E80C: 938B2200  stw r28, 0x2200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8704 as u32), ctx.r[28].u32 ) };
	// 8240E810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E818: 481268F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240E820 size=44
    let mut pc: u32 = 0x8240E820;
    'dispatch: loop {
        match pc {
            0x8240E820 => {
    //   block [0x8240E820..0x8240E84C)
	// 8240E820: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E824: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240E828: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E82C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E830: 409A0008  bne cr6, 0x8240e838
	if !ctx.cr[6].eq {
	pc = 0x8240E838; continue 'dispatch;
	}
	// 8240E834: D02B0070  stfs f1, 0x70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8240E838: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E83C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E840: 4082FFE8  bne 0x8240e828
	if !ctx.cr[0].eq {
	pc = 0x8240E828; continue 'dispatch;
	}
	// 8240E844: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240E850 size=44
    let mut pc: u32 = 0x8240E850;
    'dispatch: loop {
        match pc {
            0x8240E850 => {
    //   block [0x8240E850..0x8240E87C)
	// 8240E850: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E854: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240E858: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E85C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E860: 409A0008  bne cr6, 0x8240e868
	if !ctx.cr[6].eq {
	pc = 0x8240E868; continue 'dispatch;
	}
	// 8240E864: D02B0074  stfs f1, 0x74(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8240E868: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E86C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E870: 4082FFE8  bne 0x8240e858
	if !ctx.cr[0].eq {
	pc = 0x8240E858; continue 'dispatch;
	}
	// 8240E874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E880 size=44
    let mut pc: u32 = 0x8240E880;
    'dispatch: loop {
        match pc {
            0x8240E880 => {
    //   block [0x8240E880..0x8240E8AC)
	// 8240E880: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E884: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240E888: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E88C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E890: 409A0008  bne cr6, 0x8240e898
	if !ctx.cr[6].eq {
	pc = 0x8240E898; continue 'dispatch;
	}
	// 8240E894: 90AB0078  stw r5, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[5].u32 ) };
	// 8240E898: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E89C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E8A0: 4082FFE8  bne 0x8240e888
	if !ctx.cr[0].eq {
	pc = 0x8240E888; continue 'dispatch;
	}
	// 8240E8A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E8B0 size=80
    let mut pc: u32 = 0x8240E8B0;
    'dispatch: loop {
        match pc {
            0x8240E8B0 => {
    //   block [0x8240E8B0..0x8240E900)
	// 8240E8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E8B4: 48126805  bl 0x825350b8
	ctx.lr = 0x8240E8B8;
	sub_82535080(ctx, base);
	// 8240E8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240E8C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240E8C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240E8C8: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	// 8240E8CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E8D0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8240E8D4: 409A0014  bne cr6, 0x8240e8e8
	if !ctx.cr[6].eq {
	pc = 0x8240E8E8; continue 'dispatch;
	}
	// 8240E8D8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8240E8DC: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8240E8E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8240E8E4: 4812626D  bl 0x82534b50
	ctx.lr = 0x8240E8E8;
	sub_82534B50(ctx, base);
	// 8240E8E8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240E8EC: 3BFF0088  addi r31, r31, 0x88
	ctx.r[31].s64 = ctx.r[31].s64 + 136;
	// 8240E8F0: 4082FFDC  bne 0x8240e8cc
	if !ctx.cr[0].eq {
	pc = 0x8240E8CC; continue 'dispatch;
	}
	// 8240E8F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E8FC: 4812680C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E900 size=116
    let mut pc: u32 = 0x8240E900;
    'dispatch: loop {
        match pc {
            0x8240E900 => {
    //   block [0x8240E900..0x8240E974)
	// 8240E900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E904: 481267B1  bl 0x825350b4
	ctx.lr = 0x8240E908;
	sub_82535080(ctx, base);
	// 8240E908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E90C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8240E910: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240E914: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8240E918: 3BE30054  addi r31, r3, 0x54
	ctx.r[31].s64 = ctx.r[3].s64 + 84;
	// 8240E91C: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	// 8240E920: 817FFFAC  lwz r11, -0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-84 as u32) ) } as u64;
	// 8240E924: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8240E928: 409A0034  bne cr6, 0x8240e95c
	if !ctx.cr[6].eq {
	pc = 0x8240E95C; continue 'dispatch;
	}
	// 8240E92C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8240E930: 419A0014  beq cr6, 0x8240e944
	if ctx.cr[6].eq {
	pc = 0x8240E944; continue 'dispatch;
	}
	// 8240E934: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 8240E938: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240E93C: 387FFFCC  addi r3, r31, -0x34
	ctx.r[3].s64 = ctx.r[31].s64 + -52;
	// 8240E940: 4BFB22C1  bl 0x823c0c00
	ctx.lr = 0x8240E944;
	sub_823C0C00(ctx, base);
	// 8240E944: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8240E948: 419A0014  beq cr6, 0x8240e95c
	if ctx.cr[6].eq {
	pc = 0x8240E95C; continue 'dispatch;
	}
	// 8240E94C: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8240E950: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8240E954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240E958: 4BFB22A9  bl 0x823c0c00
	ctx.lr = 0x8240E95C;
	sub_823C0C00(ctx, base);
	// 8240E95C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240E960: 3BFF0088  addi r31, r31, 0x88
	ctx.r[31].s64 = ctx.r[31].s64 + 136;
	// 8240E964: 4082FFBC  bne 0x8240e920
	if !ctx.cr[0].eq {
	pc = 0x8240E920; continue 'dispatch;
	}
	// 8240E968: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E96C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E970: 48126794  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E978 size=44
    let mut pc: u32 = 0x8240E978;
    'dispatch: loop {
        match pc {
            0x8240E978 => {
    //   block [0x8240E978..0x8240E9A4)
	// 8240E978: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E97C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E980: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240E984: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E988: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E98C: 409A0008  bne cr6, 0x8240e994
	if !ctx.cr[6].eq {
	pc = 0x8240E994; continue 'dispatch;
	}
	// 8240E990: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8240E994: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E998: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E99C: 4082FFE8  bne 0x8240e984
	if !ctx.cr[0].eq {
	pc = 0x8240E984; continue 'dispatch;
	}
	// 8240E9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E9A8 size=180
    let mut pc: u32 = 0x8240E9A8;
    'dispatch: loop {
        match pc {
            0x8240E9A8 => {
    //   block [0x8240E9A8..0x8240EA5C)
	// 8240E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E9B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E9B4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8240E9B8: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240E9BC: 41990080  bgt cr6, 0x8240ea3c
	if ctx.cr[6].gt {
	pc = 0x8240EA3C; continue 'dispatch;
	}
	// 8240E9C0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8240E9C4: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 8240E9C8: C0091850  lfs f0, 0x1850(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E9CC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8240E9D0: 1D640088  mulli r11, r4, 0x88
	ctx.r[11].s64 = ctx.r[4].s64 * 136;
	// 8240E9D4: C1A91FF8  lfs f13, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E9D8: 38C402D6  addi r6, r4, 0x2d6
	ctx.r[6].s64 = ctx.r[4].s64 + 726;
	// 8240E9DC: 7CEA4214  add r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8240E9E0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8240E9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240E9E8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8240E9EC: 1CC6000C  mulli r6, r6, 0xc
	ctx.r[6].s64 = ctx.r[6].s64 * 12;
	// 8240E9F0: D00B0070  stfs f0, 0x70(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8240E9F4: D1AB0074  stfs f13, 0x74(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8240E9F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240E9FC: D1AB007C  stfs f13, 0x7c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8240EA00: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8240EA04: D00B0080  stfs f0, 0x80(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8240EA08: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8240EA0C: D00B0084  stfs f0, 0x84(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8240EA10: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8240EA14: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8240EA18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EA1C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8240EA20: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8240EA24: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8240EA28: 914B0078  stw r10, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8240EA2C: 91272200  stw r9, 0x2200(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8704 as u32), ctx.r[9].u32 ) };
	// 8240EA30: 91472204  stw r10, 0x2204(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8708 as u32), ctx.r[10].u32 ) };
	// 8240EA34: 7D46412E  stwx r10, r6, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 8240EA38: 48000014  b 0x8240ea4c
	pc = 0x8240EA4C; continue 'dispatch;
	// 8240EA3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240EA40: 386BE8F4  addi r3, r11, -0x170c
	ctx.r[3].s64 = ctx.r[11].s64 + -5900;
	// 8240EA44: 4BEA453D  bl 0x822b2f80
	ctx.lr = 0x8240EA48;
	sub_822B2F80(ctx, base);
	// 8240EA48: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240EA4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240EA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EA60 size=176
    let mut pc: u32 = 0x8240EA60;
    'dispatch: loop {
        match pc {
            0x8240EA60 => {
    //   block [0x8240EA60..0x8240EB10)
	// 8240EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EA68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240EA6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EA70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EA74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EA78: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EA7C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8240EA80: 397E2200  addi r11, r30, 0x2200
	ctx.r[11].s64 = ctx.r[30].s64 + 8704;
	// 8240EA84: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EA88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8240EA8C: 419A0028  beq cr6, 0x8240eab4
	if ctx.cr[6].eq {
	pc = 0x8240EAB4; continue 'dispatch;
	}
	// 8240EA90: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240EA94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240EA98: 4199001C  bgt cr6, 0x8240eab4
	if ctx.cr[6].gt {
	pc = 0x8240EAB4; continue 'dispatch;
	}
	// 8240EA9C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EAA0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240EAA4: 41980040  blt cr6, 0x8240eae4
	if ctx.cr[6].lt {
	pc = 0x8240EAE4; continue 'dispatch;
	}
	// 8240EAA8: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240EAAC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8240EAB0: 419A0034  beq cr6, 0x8240eae4
	if ctx.cr[6].eq {
	pc = 0x8240EAE4; continue 'dispatch;
	}
	// 8240EAB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EAB8: 394A0088  addi r10, r10, 0x88
	ctx.r[10].s64 = ctx.r[10].s64 + 136;
	// 8240EABC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8240EAC0: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EAC4: 4198FFC0  blt cr6, 0x8240ea84
	if ctx.cr[6].lt {
	pc = 0x8240EA84; continue 'dispatch;
	}
	// 8240EAC8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240EACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240EAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EAD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240EADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EAE0: 4E800020  blr
	return;
	// 8240EAE4: 1D7F0088  mulli r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 * 136;
	// 8240EAE8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240EAEC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8240EAF0: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 8240EAF4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8240EAF8: 48126059  bl 0x82534b50
	ctx.lr = 0x8240EAFC;
	sub_82534B50(ctx, base);
	// 8240EAFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EB00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240EB04: 4BFFFEA5  bl 0x8240e9a8
	ctx.lr = 0x8240EB08;
	sub_8240E9A8(ctx, base);
	// 8240EB08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EB0C: 4BFFFFC0  b 0x8240eacc
	pc = 0x8240EACC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EB10 size=112
    let mut pc: u32 = 0x8240EB10;
    'dispatch: loop {
        match pc {
            0x8240EB10 => {
    //   block [0x8240EB10..0x8240EB80)
	// 8240EB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EB14: 481265A1  bl 0x825350b4
	ctx.lr = 0x8240EB18;
	sub_82535080(ctx, base);
	// 8240EB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EB1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EB20: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8240EB24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EB28: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8240EB2C: 3B9E2208  addi r28, r30, 0x2208
	ctx.r[28].s64 = ctx.r[30].s64 + 8712;
	// 8240EB30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EB34: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8240EB38: 409A0028  bne cr6, 0x8240eb60
	if !ctx.cr[6].eq {
	pc = 0x8240EB60; continue 'dispatch;
	}
	// 8240EB3C: 817CFFF8  lwz r11, -8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EB40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240EB44: 41980010  blt cr6, 0x8240eb54
	if ctx.cr[6].lt {
	pc = 0x8240EB54; continue 'dispatch;
	}
	// 8240EB48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240EB4C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240EB50: 48000010  b 0x8240eb60
	pc = 0x8240EB60; continue 'dispatch;
	// 8240EB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240EB5C: 4BFFFE4D  bl 0x8240e9a8
	ctx.lr = 0x8240EB60;
	sub_8240E9A8(ctx, base);
	// 8240EB60: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EB64: 3BBD0088  addi r29, r29, 0x88
	ctx.r[29].s64 = ctx.r[29].s64 + 136;
	// 8240EB68: 3B9C000C  addi r28, r28, 0xc
	ctx.r[28].s64 = ctx.r[28].s64 + 12;
	// 8240EB6C: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EB70: 4198FFC0  blt cr6, 0x8240eb30
	if ctx.cr[6].lt {
	pc = 0x8240EB30; continue 'dispatch;
	}
	// 8240EB74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240EB7C: 48126588  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EB80 size=80
    let mut pc: u32 = 0x8240EB80;
    'dispatch: loop {
        match pc {
            0x8240EB80 => {
    //   block [0x8240EB80..0x8240EBD0)
	// 8240EB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EB84: 48126535  bl 0x825350b8
	ctx.lr = 0x8240EB88;
	sub_82535080(ctx, base);
	// 8240EB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EB8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240EB90: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240EB94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EB98: 3BDD0018  addi r30, r29, 0x18
	ctx.r[30].s64 = ctx.r[29].s64 + 24;
	// 8240EB9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EBA0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240EBA4: 409A0010  bne cr6, 0x8240ebb4
	if !ctx.cr[6].eq {
	pc = 0x8240EBB4; continue 'dispatch;
	}
	// 8240EBA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EBAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240EBB0: 4BFFFDF9  bl 0x8240e9a8
	ctx.lr = 0x8240EBB4;
	sub_8240E9A8(ctx, base);
	// 8240EBB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EBB8: 3BDE0088  addi r30, r30, 0x88
	ctx.r[30].s64 = ctx.r[30].s64 + 136;
	// 8240EBBC: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EBC0: 4198FFDC  blt cr6, 0x8240eb9c
	if ctx.cr[6].lt {
	pc = 0x8240EB9C; continue 'dispatch;
	}
	// 8240EBC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240EBCC: 4812653C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EBD0 size=80
    let mut pc: u32 = 0x8240EBD0;
    'dispatch: loop {
        match pc {
            0x8240EBD0 => {
    //   block [0x8240EBD0..0x8240EC20)
	// 8240EBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EBD4: 481264E5  bl 0x825350b8
	ctx.lr = 0x8240EBD8;
	sub_82535080(ctx, base);
	// 8240EBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EBDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240EBE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240EBE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EBE8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8240EBEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EBF0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240EBF4: 409A0010  bne cr6, 0x8240ec04
	if !ctx.cr[6].eq {
	pc = 0x8240EC04; continue 'dispatch;
	}
	// 8240EBF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EBFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240EC00: 4BFFFDA9  bl 0x8240e9a8
	ctx.lr = 0x8240EC04;
	sub_8240E9A8(ctx, base);
	// 8240EC04: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EC08: 3BDE0088  addi r30, r30, 0x88
	ctx.r[30].s64 = ctx.r[30].s64 + 136;
	// 8240EC0C: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EC10: 4198FFDC  blt cr6, 0x8240ebec
	if ctx.cr[6].lt {
	pc = 0x8240EBEC; continue 'dispatch;
	}
	// 8240EC14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240EC1C: 481264EC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EC20 size=80
    let mut pc: u32 = 0x8240EC20;
    'dispatch: loop {
        match pc {
            0x8240EC20 => {
    //   block [0x8240EC20..0x8240EC70)
	// 8240EC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240EC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EC30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EC34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EC38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EC3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EC40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240EC44: 4BFFFD65  bl 0x8240e9a8
	ctx.lr = 0x8240EC48;
	sub_8240E9A8(ctx, base);
	// 8240EC48: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EC4C: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EC50: 4198FFEC  blt cr6, 0x8240ec3c
	if ctx.cr[6].lt {
	pc = 0x8240EC3C; continue 'dispatch;
	}
	// 8240EC54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EC58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EC64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240EC68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240EC70 size=116
    let mut pc: u32 = 0x8240EC70;
    'dispatch: loop {
        match pc {
            0x8240EC70 => {
    //   block [0x8240EC70..0x8240ECE4)
	// 8240EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EC78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EC7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EC80: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8240EC84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8240EC88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240EC8C: 3940003F  li r10, 0x3f
	ctx.r[10].s64 = 63;
	// 8240EC90: 397F0078  addi r11, r31, 0x78
	ctx.r[11].s64 = ctx.r[31].s64 + 120;
	// 8240EC94: C1A81FF8  lfs f13, 0x1ff8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240EC98: C0091850  lfs f0, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240EC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240ECA0: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8240ECA4: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240ECA8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240ECAC: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240ECB0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240ECB4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240ECB8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240ECBC: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240ECC0: 4080FFDC  bge 0x8240ec9c
	if !ctx.cr[0].lt {
	pc = 0x8240EC9C; continue 'dispatch;
	}
	// 8240ECC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ECC8: 4BFFFF59  bl 0x8240ec20
	ctx.lr = 0x8240ECCC;
	sub_8240EC20(ctx, base);
	// 8240ECCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ECD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240ECD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240ECD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240ECDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240ECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ECE8 size=36
    let mut pc: u32 = 0x8240ECE8;
    'dispatch: loop {
        match pc {
            0x8240ECE8 => {
    //   block [0x8240ECE8..0x8240ED0C)
	// 8240ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240ECF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240ECF4: 4BFFFF2D  bl 0x8240ec20
	ctx.lr = 0x8240ECF8;
	sub_8240EC20(ctx, base);
	// 8240ECF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240ECFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240ED00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240ED04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240ED08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ED10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240ED10 size=4
    let mut pc: u32 = 0x8240ED10;
    'dispatch: loop {
        match pc {
            0x8240ED10 => {
    //   block [0x8240ED10..0x8240ED14)
	// 8240ED10: 4BFFFF10  b 0x8240ec20
	sub_8240EC20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ED18 size=200
    let mut pc: u32 = 0x8240ED18;
    'dispatch: loop {
        match pc {
            0x8240ED18 => {
    //   block [0x8240ED18..0x8240EDE0)
	// 8240ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240ED20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240ED24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240ED28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240ED2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240ED30: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8240ED34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED38: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8240ED3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240ED40: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240ED44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240ED48: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8240ED4C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240ED50: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8240ED54: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8240ED58: 4BFB2809  bl 0x823c1560
	ctx.lr = 0x8240ED5C;
	sub_823C1560(ctx, base);
	// 8240ED5C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 8240ED60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED64: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8240ED68: 4BFB27F9  bl 0x823c1560
	ctx.lr = 0x8240ED6C;
	sub_823C1560(ctx, base);
	// 8240ED6C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8240ED70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED74: 387F043C  addi r3, r31, 0x43c
	ctx.r[3].s64 = ctx.r[31].s64 + 1084;
	// 8240ED78: 4BFB27E9  bl 0x823c1560
	ctx.lr = 0x8240ED7C;
	sub_823C1560(ctx, base);
	// 8240ED7C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8240ED80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED84: 387F047C  addi r3, r31, 0x47c
	ctx.r[3].s64 = ctx.r[31].s64 + 1148;
	// 8240ED88: 4BFB27D9  bl 0x823c1560
	ctx.lr = 0x8240ED8C;
	sub_823C1560(ctx, base);
	// 8240ED8C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 8240ED90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED94: 387F067C  addi r3, r31, 0x67c
	ctx.r[3].s64 = ctx.r[31].s64 + 1660;
	// 8240ED98: 4BFB27C9  bl 0x823c1560
	ctx.lr = 0x8240ED9C;
	sub_823C1560(ctx, base);
	// 8240ED9C: 38A00140  li r5, 0x140
	ctx.r[5].s64 = 320;
	// 8240EDA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240EDA4: 387F0A7C  addi r3, r31, 0xa7c
	ctx.r[3].s64 = ctx.r[31].s64 + 2684;
	// 8240EDA8: 4BFB27B9  bl 0x823c1560
	ctx.lr = 0x8240EDAC;
	sub_823C1560(ctx, base);
	// 8240EDAC: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8240EDB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240EDB4: 387F0BBC  addi r3, r31, 0xbbc
	ctx.r[3].s64 = ctx.r[31].s64 + 3004;
	// 8240EDB8: 4BFB27A9  bl 0x823c1560
	ctx.lr = 0x8240EDBC;
	sub_823C1560(ctx, base);
	// 8240EDBC: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8240EDC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240EDC4: 387F0CBC  addi r3, r31, 0xcbc
	ctx.r[3].s64 = ctx.r[31].s64 + 3260;
	// 8240EDC8: 4BFB2799  bl 0x823c1560
	ctx.lr = 0x8240EDCC;
	sub_823C1560(ctx, base);
	// 8240EDCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240EDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EDD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EDE0 size=48
    let mut pc: u32 = 0x8240EDE0;
    'dispatch: loop {
        match pc {
            0x8240EDE0 => {
    //   block [0x8240EDE0..0x8240EE10)
	// 8240EDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EDEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EDF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240EDF4: 4BFFFF25  bl 0x8240ed18
	ctx.lr = 0x8240EDF8;
	sub_8240ED18(ctx, base);
	// 8240EDF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240EDFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240EE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EE08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EE10 size=372
    let mut pc: u32 = 0x8240EE10;
    'dispatch: loop {
        match pc {
            0x8240EE10 => {
    //   block [0x8240EE10..0x8240EF84)
	// 8240EE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EE18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240EE1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EE20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EE24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240EE28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EE2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240EE30: 409A001C  bne cr6, 0x8240ee4c
	if !ctx.cr[6].eq {
	pc = 0x8240EE4C; continue 'dispatch;
	}
	// 8240EE34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240EE38: 386BE970  addi r3, r11, -0x1690
	ctx.r[3].s64 = ctx.r[11].s64 + -5776;
	// 8240EE3C: 4BEA4145  bl 0x822b2f80
	ctx.lr = 0x8240EE40;
	sub_822B2F80(ctx, base);
	// 8240EE40: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240EE44: 60630041  ori r3, r3, 0x41
	ctx.r[3].u64 = ctx.r[3].u64 | 65;
	// 8240EE48: 48000124  b 0x8240ef6c
	pc = 0x8240EF6C; continue 'dispatch;
	// 8240EE4C: 889F0005  lbz r4, 5(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 8240EE50: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 8240EE54: 419A0020  beq cr6, 0x8240ee74
	if ctx.cr[6].eq {
	pc = 0x8240EE74; continue 'dispatch;
	}
	// 8240EE58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240EE5C: 88BF0006  lbz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8240EE60: 386BE928  addi r3, r11, -0x16d8
	ctx.r[3].s64 = ctx.r[11].s64 + -5848;
	// 8240EE64: 4BEA411D  bl 0x822b2f80
	ctx.lr = 0x8240EE68;
	sub_822B2F80(ctx, base);
	// 8240EE68: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240EE6C: 60630040  ori r3, r3, 0x40
	ctx.r[3].u64 = ctx.r[3].u64 | 64;
	// 8240EE70: 480000FC  b 0x8240ef6c
	pc = 0x8240EF6C; continue 'dispatch;
	// 8240EE74: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240EE78: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8240EE7C: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 8240EE80: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240EE84: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8240EE88: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240EE8C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8240EE90: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240EE94: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240EE98: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8240EE9C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240EEA0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240EEA4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8240EEA8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8240EEAC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240EEB0: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8240EEB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240EEB8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EEBC: 4BFB1D45  bl 0x823c0c00
	ctx.lr = 0x8240EEC0;
	sub_823C0C00(ctx, base);
	// 8240EEC0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240EEC4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240EEC8: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 8240EECC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EED0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EED4: 4BFB1D2D  bl 0x823c0c00
	ctx.lr = 0x8240EED8;
	sub_823C0C00(ctx, base);
	// 8240EED8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8240EEDC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8240EEE0: 387E043C  addi r3, r30, 0x43c
	ctx.r[3].s64 = ctx.r[30].s64 + 1084;
	// 8240EEE4: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EEE8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EEEC: 4BFB1D15  bl 0x823c0c00
	ctx.lr = 0x8240EEF0;
	sub_823C0C00(ctx, base);
	// 8240EEF0: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8240EEF4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8240EEF8: 387E047C  addi r3, r30, 0x47c
	ctx.r[3].s64 = ctx.r[30].s64 + 1148;
	// 8240EEFC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF00: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF04: 4BFB1CFD  bl 0x823c0c00
	ctx.lr = 0x8240EF08;
	sub_823C0C00(ctx, base);
	// 8240EF08: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240EF0C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8240EF10: 387E067C  addi r3, r30, 0x67c
	ctx.r[3].s64 = ctx.r[30].s64 + 1660;
	// 8240EF14: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF18: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF1C: 4BFB1CE5  bl 0x823c0c00
	ctx.lr = 0x8240EF20;
	sub_823C0C00(ctx, base);
	// 8240EF20: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240EF24: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8240EF28: 387E0A7C  addi r3, r30, 0xa7c
	ctx.r[3].s64 = ctx.r[30].s64 + 2684;
	// 8240EF2C: 55452834  slwi r5, r10, 5
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF30: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF34: 4BFB1CCD  bl 0x823c0c00
	ctx.lr = 0x8240EF38;
	sub_823C0C00(ctx, base);
	// 8240EF38: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8240EF3C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8240EF40: 387E0BBC  addi r3, r30, 0xbbc
	ctx.r[3].s64 = ctx.r[30].s64 + 3004;
	// 8240EF44: 55452834  slwi r5, r10, 5
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF48: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF4C: 4BFB1CB5  bl 0x823c0c00
	ctx.lr = 0x8240EF50;
	sub_823C0C00(ctx, base);
	// 8240EF50: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240EF54: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240EF58: 387E0CBC  addi r3, r30, 0xcbc
	ctx.r[3].s64 = ctx.r[30].s64 + 3260;
	// 8240EF5C: 55452834  slwi r5, r10, 5
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF60: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF64: 4BFB1C9D  bl 0x823c0c00
	ctx.lr = 0x8240EF68;
	sub_823C0C00(ctx, base);
	// 8240EF68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EF6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240EF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EF78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240EF7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240EF88 size=904
    let mut pc: u32 = 0x8240EF88;
    'dispatch: loop {
        match pc {
            0x8240EF88 => {
    //   block [0x8240EF88..0x8240F310)
	// 8240EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EF8C: 48126101  bl 0x8253508c
	ctx.lr = 0x8240EF90;
	sub_82535080(ctx, base);
	// 8240EF90: 3981FF80  addi r12, r1, -0x80
	ctx.r[12].s64 = ctx.r[1].s64 + -128;
	// 8240EF94: 48127055  bl 0x82535fe8
	ctx.lr = 0x8240EF98;
	sub_82535FB0(ctx, base);
	// 8240EF98: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EF9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240EFA0: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 8240EFA4: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 8240EFA8: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8240EFAC: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 8240EFB0: C3CB1FF8  lfs f30, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240EFB4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8240EFB8: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8240EFBC: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 8240EFC0: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 8240EFC4: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8240EFC8: 409900D4  ble cr6, 0x8240f09c
	if !ctx.cr[6].gt {
	pc = 0x8240F09C; continue 'dispatch;
	}
	// 8240EFCC: 3B340008  addi r25, r20, 8
	ctx.r[25].s64 = ctx.r[20].s64 + 8;
	// 8240EFD0: 3B160014  addi r24, r22, 0x14
	ctx.r[24].s64 = ctx.r[22].s64 + 20;
	// 8240EFD4: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8240EFD8: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 8240EFDC: 817DFFEC  lwz r11, -0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-20 as u32) ) } as u64;
	// 8240EFE0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EFE4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240EFE8: 7C0BBC2E  lfsx f0, r11, r23
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240EFEC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240EFF0: D01FFFFC  stfs f0, -4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240EFF4: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240EFF8: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240EFFC: 419A007C  beq cr6, 0x8240f078
	if ctx.cr[6].eq {
	pc = 0x8240F078; continue 'dispatch;
	}
	// 8240F000: C01FFFFC  lfs f0, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F004: C1BDFFF4  lfs f13, -0xc(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F008: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240F00C: 4198006C  blt cr6, 0x8240f078
	if ctx.cr[6].lt {
	pc = 0x8240F078; continue 'dispatch;
	}
	// 8240F010: 7E7A9B78  mr r26, r19
	ctx.r[26].u64 = ctx.r[19].u64;
	// 8240F014: 925F0000  stw r18, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 8240F018: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240F01C: 40990060  ble cr6, 0x8240f07c
	if !ctx.cr[6].gt {
	pc = 0x8240F07C; continue 'dispatch;
	}
	// 8240F020: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8240F024: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 8240F028: 817DFFF0  lwz r11, -0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F02C: 815EFFF0  lwz r10, -0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F030: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240F034: 409A002C  bne cr6, 0x8240f060
	if !ctx.cr[6].eq {
	pc = 0x8240F060; continue 'dispatch;
	}
	// 8240F038: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F03C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F040: 409A0020  bne cr6, 0x8240f060
	if !ctx.cr[6].eq {
	pc = 0x8240F060; continue 'dispatch;
	}
	// 8240F044: C03D0000  lfs f1, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F048: 4BFFF489  bl 0x8240e4d0
	ctx.lr = 0x8240F04C;
	sub_8240E4D0(ctx, base);
	// 8240F04C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240F050: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F054: 4BFFF47D  bl 0x8240e4d0
	ctx.lr = 0x8240F058;
	sub_8240E4D0(ctx, base);
	// 8240F058: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8240F05C: 4198001C  blt cr6, 0x8240f078
	if ctx.cr[6].lt {
	pc = 0x8240F078; continue 'dispatch;
	}
	// 8240F060: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8240F064: 3B7B001C  addi r27, r27, 0x1c
	ctx.r[27].s64 = ctx.r[27].s64 + 28;
	// 8240F068: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240F06C: 7F1AE000  cmpw cr6, r26, r28
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8240F070: 4198FFB8  blt cr6, 0x8240f028
	if ctx.cr[6].lt {
	pc = 0x8240F028; continue 'dispatch;
	}
	// 8240F074: 48000008  b 0x8240f07c
	pc = 0x8240F07C; continue 'dispatch;
	// 8240F078: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 8240F07C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240F080: 927F000C  stw r19, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 8240F084: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8240F088: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 8240F08C: 7F1CA800  cmpw cr6, r28, r21
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F090: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240F094: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240F098: 4198FF44  blt cr6, 0x8240efdc
	if ctx.cr[6].lt {
	pc = 0x8240EFDC; continue 'dispatch;
	}
	// 8240F09C: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 8240F0A0: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8240F0A4: 409901D8  ble cr6, 0x8240f27c
	if !ctx.cr[6].gt {
	pc = 0x8240F27C; continue 'dispatch;
	}
	// 8240F0A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F0AC: 3BF40010  addi r31, r20, 0x10
	ctx.r[31].s64 = ctx.r[20].s64 + 16;
	// 8240F0B0: 3BD60014  addi r30, r22, 0x14
	ctx.r[30].s64 = ctx.r[22].s64 + 20;
	// 8240F0B4: C3AB204C  lfs f29, 0x204c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240F0B8: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F0BC: C19FFFF0  lfs f12, -0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240F0C0: FFE06090  fmr f31, f12
	ctx.f[31].f64 = ctx.f[12].f64;
	// 8240F0C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F0C8: 409A00B0  bne cr6, 0x8240f178
	if !ctx.cr[6].eq {
	pc = 0x8240F178; continue 'dispatch;
	}
	// 8240F0CC: 925F0004  stw r18, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[18].u32 ) };
	// 8240F0D0: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 8240F0D4: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F0D8: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240F0DC: 419A0170  beq cr6, 0x8240f24c
	if ctx.cr[6].eq {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F0E0: C1BEFFF8  lfs f13, -8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F0E4: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 8240F0E8: 40990010  ble cr6, 0x8240f0f8
	if !ctx.cr[6].gt {
	pc = 0x8240F0F8; continue 'dispatch;
	}
	// 8240F0EC: EDAD0772  fmuls f13, f13, f29
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[29].f64) as f32) as f64);
	// 8240F0F0: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8240F0F4: 48000008  b 0x8240f0fc
	pc = 0x8240F0FC; continue 'dispatch;
	// 8240F0F8: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 8240F0FC: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F100: 40980010  bge cr6, 0x8240f110
	if !ctx.cr[6].lt {
	pc = 0x8240F110; continue 'dispatch;
	}
	// 8240F104: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240F108: 40980014  bge cr6, 0x8240f11c
	if !ctx.cr[6].lt {
	pc = 0x8240F11C; continue 'dispatch;
	}
	// 8240F10C: 4800000C  b 0x8240f118
	pc = 0x8240F118; continue 'dispatch;
	// 8240F110: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240F114: 40990008  ble cr6, 0x8240f11c
	if !ctx.cr[6].gt {
	pc = 0x8240F11C; continue 'dispatch;
	}
	// 8240F118: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240F11C: 7A2B0020  clrldi r11, r17, 0x20
	ctx.r[11].u64 = ctx.r[17].u64 & 0x00000000FFFFFFFFu64;
	// 8240F120: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F124: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F128: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8240F12C: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8240F130: EFEB637A  fmadds f31, f11, f13, f12
	ctx.f[31].f64 = (((ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 8240F134: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240F138: 4099000C  ble cr6, 0x8240f144
	if !ctx.cr[6].gt {
	pc = 0x8240F144; continue 'dispatch;
	}
	// 8240F13C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F140: 41990014  bgt cr6, 0x8240f154
	if ctx.cr[6].gt {
	pc = 0x8240F154; continue 'dispatch;
	}
	// 8240F144: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240F148: 40980104  bge cr6, 0x8240f24c
	if !ctx.cr[6].lt {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F14C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F150: 409800FC  bge cr6, 0x8240f24c
	if !ctx.cr[6].lt {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F154: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F158: 4BFFF379  bl 0x8240e4d0
	ctx.lr = 0x8240F15C;
	sub_8240E4D0(ctx, base);
	// 8240F15C: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8240F160: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F164: 4BFFF36D  bl 0x8240e4d0
	ctx.lr = 0x8240F168;
	sub_8240E4D0(ctx, base);
	// 8240F168: FF1C0800  fcmpu cr6, f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[1].f64);
	// 8240F16C: 419800E0  blt cr6, 0x8240f24c
	if ctx.cr[6].lt {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F170: C3FE0000  lfs f31, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240F174: 480000D8  b 0x8240f24c
	pc = 0x8240F24C; continue 'dispatch;
	// 8240F178: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 8240F17C: 39340008  addi r9, r20, 8
	ctx.r[9].s64 = ctx.r[20].s64 + 8;
	// 8240F180: 39560004  addi r10, r22, 4
	ctx.r[10].s64 = ctx.r[22].s64 + 4;
	// 8240F184: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240F188: 419A0020  beq cr6, 0x8240f1a8
	if ctx.cr[6].eq {
	pc = 0x8240F1A8; continue 'dispatch;
	}
	// 8240F18C: 811EFFF0  lwz r8, -0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F190: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F194: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8240F198: 409A0010  bne cr6, 0x8240f1a8
	if !ctx.cr[6].eq {
	pc = 0x8240F1A8; continue 'dispatch;
	}
	// 8240F19C: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F1A0: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 8240F1A4: 419A001C  beq cr6, 0x8240f1c0
	if ctx.cr[6].eq {
	pc = 0x8240F1C0; continue 'dispatch;
	}
	// 8240F1A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8240F1AC: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 8240F1B0: 3929001C  addi r9, r9, 0x1c
	ctx.r[9].s64 = ctx.r[9].s64 + 28;
	// 8240F1B4: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F1B8: 4198FFCC  blt cr6, 0x8240f184
	if ctx.cr[6].lt {
	pc = 0x8240F184; continue 'dispatch;
	}
	// 8240F1BC: 4800000C  b 0x8240f1c8
	pc = 0x8240F1C8; continue 'dispatch;
	// 8240F1C0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240F1C4: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 8240F1C8: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8240F1CC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F1D0: 419A0010  beq cr6, 0x8240f1e0
	if ctx.cr[6].eq {
	pc = 0x8240F1E0; continue 'dispatch;
	}
	// 8240F1D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F1D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F1DC: 409A0070  bne cr6, 0x8240f24c
	if !ctx.cr[6].eq {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F1E0: 925F0004  stw r18, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[18].u32 ) };
	// 8240F1E4: C01EFFFC  lfs f0, -4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F1E8: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F1EC: 40990018  ble cr6, 0x8240f204
	if !ctx.cr[6].gt {
	pc = 0x8240F204; continue 'dispatch;
	}
	// 8240F1F0: EC000772  fmuls f0, f0, f29
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[29].f64) as f32) as f64);
	// 8240F1F4: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F1F8: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240F1FC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240F200: 48000008  b 0x8240f208
	pc = 0x8240F208; continue 'dispatch;
	// 8240F204: FC00F090  fmr f0, f30
	ctx.f[0].f64 = ctx.f[30].f64;
	// 8240F208: EDA00332  fmuls f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240F20C: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 8240F210: 40990008  ble cr6, 0x8240f218
	if !ctx.cr[6].gt {
	pc = 0x8240F218; continue 'dispatch;
	}
	// 8240F214: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240F218: 7A2B0020  clrldi r11, r17, 0x20
	ctx.r[11].u64 = ctx.r[17].u64 & 0x00000000FFFFFFFFu64;
	// 8240F21C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8240F220: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F224: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240F228: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240F22C: EFED603A  fmadds f31, f13, f0, f12
	ctx.f[31].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 8240F230: EC1F0332  fmuls f0, f31, f12
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240F234: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F238: 41990010  bgt cr6, 0x8240f248
	if ctx.cr[6].gt {
	pc = 0x8240F248; continue 'dispatch;
	}
	// 8240F23C: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240F240: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 8240F244: 48000008  b 0x8240f24c
	pc = 0x8240F24C; continue 'dispatch;
	// 8240F248: 925F0000  stw r18, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 8240F24C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240F250: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F254: 409A0014  bne cr6, 0x8240f268
	if !ctx.cr[6].eq {
	pc = 0x8240F268; continue 'dispatch;
	}
	// 8240F258: D3FFFFF0  stfs f31, -0x10(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8240F25C: 807EFFF0  lwz r3, -0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F260: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F264: 4BFF7DE5  bl 0x82407048
	ctx.lr = 0x8240F268;
	sub_82407048(ctx, base);
	// 8240F268: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240F26C: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240F270: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240F274: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F278: 4198FE40  blt cr6, 0x8240f0b8
	if ctx.cr[6].lt {
	pc = 0x8240F0B8; continue 'dispatch;
	}
	// 8240F27C: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 8240F280: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8240F284: 4099007C  ble cr6, 0x8240f300
	if !ctx.cr[6].gt {
	pc = 0x8240F300; continue 'dispatch;
	}
	// 8240F288: 38F60004  addi r7, r22, 4
	ctx.r[7].s64 = ctx.r[22].s64 + 4;
	// 8240F28C: 7E88A378  mr r8, r20
	ctx.r[8].u64 = ctx.r[20].u64;
	// 8240F290: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8240F294: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240F298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240F29C: 409A0050  bne cr6, 0x8240f2ec
	if !ctx.cr[6].eq {
	pc = 0x8240F2EC; continue 'dispatch;
	}
	// 8240F2A0: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 8240F2A4: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 8240F2A8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8240F2AC: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240F2B0: 419A0028  beq cr6, 0x8240f2d8
	if ctx.cr[6].eq {
	pc = 0x8240F2D8; continue 'dispatch;
	}
	// 8240F2B4: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F2B8: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F2BC: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8240F2C0: 409A0018  bne cr6, 0x8240f2d8
	if !ctx.cr[6].eq {
	pc = 0x8240F2D8; continue 'dispatch;
	}
	// 8240F2C4: 808A0014  lwz r4, 0x14(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240F2C8: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 8240F2CC: 409A000C  bne cr6, 0x8240f2d8
	if !ctx.cr[6].eq {
	pc = 0x8240F2D8; continue 'dispatch;
	}
	// 8240F2D0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F2D4: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240F2D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8240F2DC: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 8240F2E0: 394A001C  addi r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + 28;
	// 8240F2E4: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F2E8: 4198FFC4  blt cr6, 0x8240f2ac
	if ctx.cr[6].lt {
	pc = 0x8240F2AC; continue 'dispatch;
	}
	// 8240F2EC: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8240F2F0: 3908001C  addi r8, r8, 0x1c
	ctx.r[8].s64 = ctx.r[8].s64 + 28;
	// 8240F2F4: 38C60020  addi r6, r6, 0x20
	ctx.r[6].s64 = ctx.r[6].s64 + 32;
	// 8240F2F8: 7F05A800  cmpw cr6, r5, r21
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F2FC: 4198FF98  blt cr6, 0x8240f294
	if ctx.cr[6].lt {
	pc = 0x8240F294; continue 'dispatch;
	}
	// 8240F300: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 8240F304: 3981FF80  addi r12, r1, -0x80
	ctx.r[12].s64 = ctx.r[1].s64 + -128;
	// 8240F308: 48126D2D  bl 0x82536034
	ctx.lr = 0x8240F30C;
	sub_82535FFC(ctx, base);
	// 8240F30C: 48125DD0  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F310 size=96
    let mut pc: u32 = 0x8240F310;
    'dispatch: loop {
        match pc {
            0x8240F310 => {
    //   block [0x8240F310..0x8240F370)
	// 8240F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F314: 48125DA9  bl 0x825350bc
	ctx.lr = 0x8240F318;
	sub_82535080(ctx, base);
	// 8240F318: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8240F31C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F320: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F324: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 8240F328: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 8240F32C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240F330: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240F334: D3FFFFF8  stfs f31, -8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8240F338: 4BFF6059  bl 0x82405390
	ctx.lr = 0x8240F33C;
	sub_82405390(ctx, base);
	// 8240F33C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240F340: D03FFFFC  stfs f1, -4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240F344: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240F348: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240F34C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8240F350: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8240F354: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8240F358: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240F35C: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240F360: 4082FFD4  bne 0x8240f334
	if !ctx.cr[0].eq {
	pc = 0x8240F334; continue 'dispatch;
	}
	// 8240F364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240F368: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8240F36C: 48125DA0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240F370 size=48
    let mut pc: u32 = 0x8240F370;
    'dispatch: loop {
        match pc {
            0x8240F370 => {
    //   block [0x8240F370..0x8240F3A0)
	// 8240F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F37C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240F384: 4BFFFF8D  bl 0x8240f310
	ctx.lr = 0x8240F388;
	sub_8240F310(ctx, base);
	// 8240F388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240F38C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F3A0 size=120
    let mut pc: u32 = 0x8240F3A0;
    'dispatch: loop {
        match pc {
            0x8240F3A0 => {
    //   block [0x8240F3A0..0x8240F418)
	// 8240F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240F3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F3B0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8240F3B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F3B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240F3BC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240F3C0: 4BFFFF51  bl 0x8240f310
	ctx.lr = 0x8240F3C4;
	sub_8240F310(ctx, base);
	// 8240F3C4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240F3C8: 40990034  ble cr6, 0x8240f3fc
	if !ctx.cr[6].gt {
	pc = 0x8240F3FC; continue 'dispatch;
	}
	// 8240F3CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F3D0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8240F3D4: C3EB1FF8  lfs f31, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240F3D8: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F3DC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8240F3E0: 419A0010  beq cr6, 0x8240f3f0
	if ctx.cr[6].eq {
	pc = 0x8240F3F0; continue 'dispatch;
	}
	// 8240F3E4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F3E8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F3EC: 4BFF7C5D  bl 0x82407048
	ctx.lr = 0x8240F3F0;
	sub_82407048(ctx, base);
	// 8240F3F0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240F3F4: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240F3F8: 4082FFE0  bne 0x8240f3d8
	if !ctx.cr[0].eq {
	pc = 0x8240F3D8; continue 'dispatch;
	}
	// 8240F3FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F408: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240F40C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F418 size=16
    let mut pc: u32 = 0x8240F418;
    'dispatch: loop {
        match pc {
            0x8240F418 => {
    //   block [0x8240F418..0x8240F428)
	// 8240F418: EC01102A  fadds f0, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 8240F41C: EC20182A  fadds f1, f0, f3
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64;
	// 8240F420: FF012800  fcmpu cr6, f1, f5
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[5].f64);
	// 8240F424: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F428 size=8
    let mut pc: u32 = 0x8240F428;
    'dispatch: loop {
        match pc {
            0x8240F428 => {
    //   block [0x8240F428..0x8240F430)
	// 8240F428: EC21202A  fadds f1, f1, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[4].f64) as f32) as f64;
	// 8240F42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F430 size=128
    let mut pc: u32 = 0x8240F430;
    'dispatch: loop {
        match pc {
            0x8240F430 => {
    //   block [0x8240F430..0x8240F4B0)
	// 8240F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F438: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 8240F43C: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240F440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F444: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F448: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240F44C: 7D453214  add r10, r5, r6
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 8240F450: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F454: 7D4B07B4  extsw r11, r10
	ctx.r[11].s64 = ctx.r[10].s32 as i64;
	// 8240F458: FF030000  fcmpu cr6, f3, f0
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[0].f64);
	// 8240F45C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F460: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F464: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F468: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F46C: EFE0102A  fadds f31, f0, f2
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[2].f64) as f32) as f64;
	// 8240F470: 40980020  bge cr6, 0x8240f490
	if !ctx.cr[6].lt {
	pc = 0x8240F490; continue 'dispatch;
	}
	// 8240F474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F478: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 8240F47C: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240F480: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240F484: 386BE9A8  addi r3, r11, -0x1658
	ctx.r[3].s64 = ctx.r[11].s64 + -5720;
	// 8240F488: 4BEA3AF9  bl 0x822b2f80
	ctx.lr = 0x8240F48C;
	sub_822B2F80(ctx, base);
	// 8240F48C: 48000008  b 0x8240f494
	pc = 0x8240F494; continue 'dispatch;
	// 8240F490: EFFF00F2  fmuls f31, f31, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[3].f64) as f32) as f64);
	// 8240F494: EC3FF02A  fadds f1, f31, f30
	ctx.f[1].f64 = ((ctx.f[31].f64 + ctx.f[30].f64) as f32) as f64;
	// 8240F498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F4A4: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F4A8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F4B0 size=12
    let mut pc: u32 = 0x8240F4B0;
    'dispatch: loop {
        match pc {
            0x8240F4B0 => {
    //   block [0x8240F4B0..0x8240F4BC)
	// 8240F4B0: EC01102A  fadds f0, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 8240F4B4: EC20182A  fadds f1, f0, f3
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64;
	// 8240F4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F4C0 size=144
    let mut pc: u32 = 0x8240F4C0;
    'dispatch: loop {
        match pc {
            0x8240F4C0 => {
    //   block [0x8240F4C0..0x8240F550)
	// 8240F4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F4CC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240F4D0: 409A0010  bne cr6, 0x8240f4e0
	if !ctx.cr[6].eq {
	pc = 0x8240F4E0; continue 'dispatch;
	}
	// 8240F4D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F4D8: C02B1850  lfs f1, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F4DC: 48000064  b 0x8240f540
	pc = 0x8240F540; continue 'dispatch;
	// 8240F4E0: 40980014  bge cr6, 0x8240f4f4
	if !ctx.cr[6].lt {
	pc = 0x8240F4F4; continue 'dispatch;
	}
	// 8240F4E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F4E8: 386BE9EC  addi r3, r11, -0x1614
	ctx.r[3].s64 = ctx.r[11].s64 + -5652;
	// 8240F4EC: 4BEA3A95  bl 0x822b2f80
	ctx.lr = 0x8240F4F0;
	sub_822B2F80(ctx, base);
	// 8240F4F0: 4BFFFFE4  b 0x8240f4d4
	pc = 0x8240F4D4; continue 'dispatch;
	// 8240F4F4: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8240F4F8: 4198FFDC  blt cr6, 0x8240f4d4
	if ctx.cr[6].lt {
	pc = 0x8240F4D4; continue 'dispatch;
	}
	// 8240F4FC: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 8240F500: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 8240F504: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F50C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8240F510: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F514: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F518: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F51C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240F520: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F524: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F528: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240F52C: ED806024  fdivs f12, f0, f12
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 8240F530: EC2C0372  fmuls f1, f12, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240F534: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F538: 41980008  blt cr6, 0x8240f540
	if ctx.cr[6].lt {
	pc = 0x8240F540; continue 'dispatch;
	}
	// 8240F53C: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8240F540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F550 size=144
    let mut pc: u32 = 0x8240F550;
    'dispatch: loop {
        match pc {
            0x8240F550 => {
    //   block [0x8240F550..0x8240F5E0)
	// 8240F550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F55C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240F560: 409A0010  bne cr6, 0x8240f570
	if !ctx.cr[6].eq {
	pc = 0x8240F570; continue 'dispatch;
	}
	// 8240F564: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F568: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F56C: 48000064  b 0x8240f5d0
	pc = 0x8240F5D0; continue 'dispatch;
	// 8240F570: 40980014  bge cr6, 0x8240f584
	if !ctx.cr[6].lt {
	pc = 0x8240F584; continue 'dispatch;
	}
	// 8240F574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F578: 386BEA2C  addi r3, r11, -0x15d4
	ctx.r[3].s64 = ctx.r[11].s64 + -5588;
	// 8240F57C: 4BEA3A05  bl 0x822b2f80
	ctx.lr = 0x8240F580;
	sub_822B2F80(ctx, base);
	// 8240F580: 4BFFFFE4  b 0x8240f564
	pc = 0x8240F564; continue 'dispatch;
	// 8240F584: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8240F588: 4198FFDC  blt cr6, 0x8240f564
	if ctx.cr[6].lt {
	pc = 0x8240F564; continue 'dispatch;
	}
	// 8240F58C: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 8240F590: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 8240F594: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F598: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F59C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8240F5A0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F5A4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F5A8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F5AC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240F5B0: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F5B4: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F5B8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240F5BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F5C0: ED806024  fdivs f12, f0, f12
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 8240F5C4: EC0C037C  fnmsubs f0, f12, f13, f0
	ctx.f[0].f64 = -(((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240F5C8: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F5CC: FC20682E  fsel f1, f0, f0, f13
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8240F5D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F5E0 size=188
    let mut pc: u32 = 0x8240F5E0;
    'dispatch: loop {
        match pc {
            0x8240F5E0 => {
    //   block [0x8240F5E0..0x8240F69C)
	// 8240F5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F5EC: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240F5F0: 481269F9  bl 0x82535fe8
	ctx.lr = 0x8240F5F4;
	sub_82535FB0(ctx, base);
	// 8240F5F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F5F8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240F5FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F600: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240F604: C38B1FF8  lfs f28, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240F608: FF1EE000  fcmpu cr6, f30, f28
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[28].f64);
	// 8240F60C: 419A0074  beq cr6, 0x8240f680
	if ctx.cr[6].eq {
	pc = 0x8240F680; continue 'dispatch;
	}
	// 8240F610: 4BFFEEC1  bl 0x8240e4d0
	ctx.lr = 0x8240F614;
	sub_8240E4D0(ctx, base);
	// 8240F614: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240F618: 4BFFEEC9  bl 0x8240e4e0
	ctx.lr = 0x8240F61C;
	sub_8240E4E0(ctx, base);
	// 8240F61C: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 8240F620: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F624: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F628: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F62C: FFA00018  frsp f29, f0
	ctx.f[29].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F630: 4BFFEED9  bl 0x8240e508
	ctx.lr = 0x8240F634;
	sub_8240E508(ctx, base);
	// 8240F634: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 8240F638: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 8240F63C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F640: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F644: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F648: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F64C: EC1D0024  fdivs f0, f29, f0
	ctx.f[0].f64 = ((ctx.f[29].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240F650: EFE007F2  fmuls f31, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240F654: 409A0014  bne cr6, 0x8240f668
	if !ctx.cr[6].eq {
	pc = 0x8240F668; continue 'dispatch;
	}
	// 8240F658: 4BFFEE89  bl 0x8240e4e0
	ctx.lr = 0x8240F65C;
	sub_8240E4E0(ctx, base);
	// 8240F65C: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F660: 4182001C  beq 0x8240f67c
	if ctx.cr[0].eq {
	pc = 0x8240F67C; continue 'dispatch;
	}
	// 8240F664: 4800000C  b 0x8240f670
	pc = 0x8240F670; continue 'dispatch;
	// 8240F668: FF1EE000  fcmpu cr6, f30, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[28].f64);
	// 8240F66C: 40980010  bge cr6, 0x8240f67c
	if !ctx.cr[6].lt {
	pc = 0x8240F67C; continue 'dispatch;
	}
	// 8240F670: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F674: C00B2074  lfs f0, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F678: EFFF0032  fmuls f31, f31, f0
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240F67C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F680: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240F684: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240F688: 481269AD  bl 0x82536034
	ctx.lr = 0x8240F68C;
	sub_82535FFC(ctx, base);
	// 8240F68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240F6A0 size=116
    let mut pc: u32 = 0x8240F6A0;
    'dispatch: loop {
        match pc {
            0x8240F6A0 => {
    //   block [0x8240F6A0..0x8240F714)
	// 8240F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F6A4: 48125A19  bl 0x825350bc
	ctx.lr = 0x8240F6A8;
	sub_82535080(ctx, base);
	// 8240F6A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F6AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240F6B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240F6B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240F6B8: 409A000C  bne cr6, 0x8240f6c4
	if !ctx.cr[6].eq {
	pc = 0x8240F6C4; continue 'dispatch;
	}
	// 8240F6BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240F6C0: 4800004C  b 0x8240f70c
	pc = 0x8240F70C; continue 'dispatch;
	// 8240F6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240F6C8: 4BFFEE11  bl 0x8240e4d8
	ctx.lr = 0x8240F6CC;
	sub_8240E4D8(ctx, base);
	// 8240F6CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240F6D0: 4BFFEE11  bl 0x8240e4e0
	ctx.lr = 0x8240F6D4;
	sub_8240E4E0(ctx, base);
	// 8240F6D4: 7D63FB96  divwu r11, r3, r31
	ctx.r[11].u32 = ctx.r[3].u32 / ctx.r[31].u32;
	// 8240F6D8: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8240F6DC: 7D6BF9D6  mullw r11, r11, r31
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[31].s32 as i64);
	// 8240F6E0: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8240F6E4: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 8240F6E8: 409A0014  bne cr6, 0x8240f6fc
	if !ctx.cr[6].eq {
	pc = 0x8240F6FC; continue 'dispatch;
	}
	// 8240F6EC: 4BFFEDF5  bl 0x8240e4e0
	ctx.lr = 0x8240F6F0;
	sub_8240E4E0(ctx, base);
	// 8240F6F0: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F6F4: 41820014  beq 0x8240f708
	if ctx.cr[0].eq {
	pc = 0x8240F708; continue 'dispatch;
	}
	// 8240F6F8: 4800000C  b 0x8240f704
	pc = 0x8240F704; continue 'dispatch;
	// 8240F6FC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240F700: 40980008  bge cr6, 0x8240f708
	if !ctx.cr[6].lt {
	pc = 0x8240F708; continue 'dispatch;
	}
	// 8240F704: 7FFF00D0  neg r31, r31
	ctx.r[31].s64 = -ctx.r[31].s64;
	// 8240F708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240F70C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F710: 481259FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F718 size=76
    let mut pc: u32 = 0x8240F718;
    'dispatch: loop {
        match pc {
            0x8240F718 => {
    //   block [0x8240F718..0x8240F764)
	// 8240F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F728: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F730: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F734: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F738: 41820008  beq 0x8240f740
	if ctx.cr[0].eq {
	pc = 0x8240F740; continue 'dispatch;
	}
	// 8240F73C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240F740: C03F009C  lfs f1, 0x9c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F744: 4BFFFE9D  bl 0x8240f5e0
	ctx.lr = 0x8240F748;
	sub_8240F5E0(ctx, base);
	// 8240F748: C01F0034  lfs f0, 0x34(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F74C: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8240F750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F75C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F768 size=168
    let mut pc: u32 = 0x8240F768;
    'dispatch: loop {
        match pc {
            0x8240F768 => {
    //   block [0x8240F768..0x8240F810)
	// 8240F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F774: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8240F778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F77C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F780: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F784: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F788: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F78C: 41820008  beq 0x8240f794
	if ctx.cr[0].eq {
	pc = 0x8240F794; continue 'dispatch;
	}
	// 8240F790: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240F794: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8240F798: 4BFFFF09  bl 0x8240f6a0
	ctx.lr = 0x8240F79C;
	sub_8240F6A0(ctx, base);
	// 8240F79C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8240F7A0: C03F00A8  lfs f1, 0xa8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F7A4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F7A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F7AC: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F7B0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F7B4: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F7B8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F7BC: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F7C0: 4098001C  bge cr6, 0x8240f7dc
	if !ctx.cr[6].lt {
	pc = 0x8240F7DC; continue 'dispatch;
	}
	// 8240F7C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F7C8: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240F7CC: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240F7D0: 386BEA70  addi r3, r11, -0x1590
	ctx.r[3].s64 = ctx.r[11].s64 + -5520;
	// 8240F7D4: 4BEA37AD  bl 0x822b2f80
	ctx.lr = 0x8240F7D8;
	sub_822B2F80(ctx, base);
	// 8240F7D8: 48000008  b 0x8240f7e0
	pc = 0x8240F7E0; continue 'dispatch;
	// 8240F7DC: EFE107F2  fmuls f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240F7E0: E97F003E  lwa r11, 0x3c(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as i32) as i64;
	// 8240F7E4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F7E8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F7EC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F7F0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F7F4: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240F7F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F804: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F810 size=76
    let mut pc: u32 = 0x8240F810;
    'dispatch: loop {
        match pc {
            0x8240F810 => {
    //   block [0x8240F810..0x8240F85C)
	// 8240F810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F81C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F820: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F828: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F82C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F830: 41820008  beq 0x8240f838
	if ctx.cr[0].eq {
	pc = 0x8240F838; continue 'dispatch;
	}
	// 8240F834: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240F838: C03F00A0  lfs f1, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F83C: 4BFFFDA5  bl 0x8240f5e0
	ctx.lr = 0x8240F840;
	sub_8240F5E0(ctx, base);
	// 8240F840: C01F0040  lfs f0, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F844: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8240F848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F860 size=112
    let mut pc: u32 = 0x8240F860;
    'dispatch: loop {
        match pc {
            0x8240F860 => {
    //   block [0x8240F860..0x8240F8D0)
	// 8240F860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F86C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F870: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F878: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F87C: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F880: 41820008  beq 0x8240f888
	if ctx.cr[0].eq {
	pc = 0x8240F888; continue 'dispatch;
	}
	// 8240F884: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240F888: C03F00A4  lfs f1, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F88C: 4BFFFD55  bl 0x8240f5e0
	ctx.lr = 0x8240F890;
	sub_8240F5E0(ctx, base);
	// 8240F890: C01F0048  lfs f0, 0x48(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F894: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F898: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8240F89C: C00B2074  lfs f0, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F8A0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F8A4: 41980014  blt cr6, 0x8240f8b8
	if ctx.cr[6].lt {
	pc = 0x8240F8B8; continue 'dispatch;
	}
	// 8240F8A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F8AC: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F8B0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F8B4: 40990008  ble cr6, 0x8240f8bc
	if !ctx.cr[6].gt {
	pc = 0x8240F8BC; continue 'dispatch;
	}
	// 8240F8B8: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8240F8BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F8C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F8D0 size=256
    let mut pc: u32 = 0x8240F8D0;
    'dispatch: loop {
        match pc {
            0x8240F8D0 => {
    //   block [0x8240F8D0..0x8240F9D0)
	// 8240F8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F8D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240F8DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F8E0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8240F8E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F8E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240F8EC: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8240F8F0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240F8F4: 48000585  bl 0x8240fe78
	ctx.lr = 0x8240F8F8;
	sub_8240FE78(ctx, base);
	// 8240F8F8: 4BFFEBD9  bl 0x8240e4d0
	ctx.lr = 0x8240F8FC;
	sub_8240E4D0(ctx, base);
	// 8240F8FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240F900: 419A00B0  beq cr6, 0x8240f9b0
	if ctx.cr[6].eq {
	pc = 0x8240F9B0; continue 'dispatch;
	}
	// 8240F904: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240F908: 419A00A8  beq cr6, 0x8240f9b0
	if ctx.cr[6].eq {
	pc = 0x8240F9B0; continue 'dispatch;
	}
	// 8240F90C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F910: C00B212C  lfs f0, 0x212c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8492 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F914: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8240F918: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240F91C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8240F920: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 8240F924: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240F928: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 8240F92C: 40980084  bge cr6, 0x8240f9b0
	if !ctx.cr[6].lt {
	pc = 0x8240F9B0; continue 'dispatch;
	}
	// 8240F930: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 8240F934: 409A000C  bne cr6, 0x8240f940
	if !ctx.cr[6].eq {
	pc = 0x8240F940; continue 'dispatch;
	}
	// 8240F938: C17F0018  lfs f11, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240F93C: 48000064  b 0x8240f9a0
	pc = 0x8240F9A0; continue 'dispatch;
	// 8240F940: 1D6A001E  mulli r11, r10, 0x1e
	ctx.r[11].s64 = ctx.r[10].s64 * 30;
	// 8240F944: 392B001E  addi r9, r11, 0x1e
	ctx.r[9].s64 = ctx.r[11].s64 + 30;
	// 8240F948: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240F94C: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 8240F950: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8240F954: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8240F958: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F95C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F960: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 8240F964: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F968: C16A0004  lfs f11, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240F96C: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F970: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F974: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F978: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8240F97C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8240F980: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8240F984: 419A001C  beq cr6, 0x8240f9a0
	if ctx.cr[6].eq {
	pc = 0x8240F9A0; continue 'dispatch;
	}
	// 8240F988: ED410028  fsubs f10, f1, f0
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240F98C: ED6B6828  fsubs f11, f11, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240F990: EC0C0028  fsubs f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240F994: ED8B02B2  fmuls f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240F998: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240F99C: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8240F9A0: EC0BF82A  fadds f0, f11, f31
	ctx.f[0].f64 = ((ctx.f[11].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240F9A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240F9A8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240F9AC: 48000008  b 0x8240f9b4
	pc = 0x8240F9B4; continue 'dispatch;
	// 8240F9B0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240F9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240F9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F9C0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240F9C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F9C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F9D0 size=16
    let mut pc: u32 = 0x8240F9D0;
    'dispatch: loop {
        match pc {
            0x8240F9D0 => {
    //   block [0x8240F9D0..0x8240F9E0)
	// 8240F9D0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8240F9D4: 409A000C  bne cr6, 0x8240f9e0
	if !ctx.cr[6].eq {
		sub_8240F9E0(ctx, base);
		return;
	}
	// 8240F9D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240F9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F9E0 size=64
    let mut pc: u32 = 0x8240F9E0;
    'dispatch: loop {
        match pc {
            0x8240F9E0 => {
    //   block [0x8240F9E0..0x8240FA20)
	// 8240F9E0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240F9E4: 41980044  blt cr6, 0x8240fa28
	if ctx.cr[6].lt {
		sub_8240FA28(ctx, base);
		return;
	}
	// 8240F9E8: 419A0038  beq cr6, 0x8240fa20
	if ctx.cr[6].eq {
		sub_8240FA20(ctx, base);
		return;
	}
	// 8240F9EC: 2B030004  cmplwi cr6, r3, 4
	ctx.cr[6].compare_u32(ctx.r[3].u32, 4 as u32, &mut ctx.xer);
	// 8240F9F0: 409AFFE8  bne cr6, 0x8240f9d8
	if !ctx.cr[6].eq {
		sub_8240F9D0(ctx, base);
		return;
	}
	// 8240F9F4: 89640001  lbz r11, 1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 8240F9F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240F9FC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240FA00: 40810050  ble 0x8240fa50
	if !ctx.cr[0].gt {
		sub_8240FA28(ctx, base);
		return;
	}
	// 8240FA04: 39440028  addi r10, r4, 0x28
	ctx.r[10].s64 = ctx.r[4].s64 + 40;
	// 8240FA08: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240FA0C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240FA10: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240FA14: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8240FA18: 4082FFF0  bne 0x8240fa08
	if !ctx.cr[0].eq {
	pc = 0x8240FA08; continue 'dispatch;
	}
	// 8240FA1C: 48000034  b 0x8240fa50
	sub_8240FA28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FA20 size=8
    let mut pc: u32 = 0x8240FA20;
    'dispatch: loop {
        match pc {
            0x8240FA20 => {
    //   block [0x8240FA20..0x8240FA28)
	// 8240FA20: A1240002  lhz r9, 2(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 8240FA24: 4800002C  b 0x8240fa50
	sub_8240FA28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FA28 size=48
    let mut pc: u32 = 0x8240FA28;
    'dispatch: loop {
        match pc {
            0x8240FA28 => {
    //   block [0x8240FA28..0x8240FA58)
	// 8240FA28: A1640008  lhz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240FA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240FA30: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240FA34: 4081001C  ble 0x8240fa50
	if !ctx.cr[0].gt {
	pc = 0x8240FA50; continue 'dispatch;
	}
	// 8240FA38: 3944001D  addi r10, r4, 0x1d
	ctx.r[10].s64 = ctx.r[4].s64 + 29;
	// 8240FA3C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240FA40: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240FA44: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8240FA48: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8240FA4C: 4082FFF0  bne 0x8240fa3c
	if !ctx.cr[0].eq {
	pc = 0x8240FA3C; continue 'dispatch;
	}
	// 8240FA50: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8240FA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FA58 size=76
    let mut pc: u32 = 0x8240FA58;
    'dispatch: loop {
        match pc {
            0x8240FA58 => {
    //   block [0x8240FA58..0x8240FAA4)
	// 8240FA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FA60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240FA64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FA68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FA6C: 3FE30002  addis r31, r3, 2
	ctx.r[31].s64 = ctx.r[3].s64 + 131072;
	// 8240FA70: 3BC00FFF  li r30, 0xfff
	ctx.r[30].s64 = 4095;
	// 8240FA74: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8240FA78: 3BFFFFE0  addi r31, r31, -0x20
	ctx.r[31].s64 = ctx.r[31].s64 + -32;
	// 8240FA7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FA80: 4BEA3501  bl 0x822b2f80
	ctx.lr = 0x8240FA84;
	sub_822B2F80(ctx, base);
	// 8240FA84: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240FA88: 4080FFF0  bge 0x8240fa78
	if !ctx.cr[0].lt {
	pc = 0x8240FA78; continue 'dispatch;
	}
	// 8240FA8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FA98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240FA9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FAA8 size=92
    let mut pc: u32 = 0x8240FAA8;
    'dispatch: loop {
        match pc {
            0x8240FAA8 => {
    //   block [0x8240FAA8..0x8240FB04)
	// 8240FAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240FAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FAB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FABC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240FAC0: 3BC30008  addi r30, r3, 8
	ctx.r[30].s64 = ctx.r[3].s64 + 8;
	// 8240FAC4: 3BE01000  li r31, 0x1000
	ctx.r[31].s64 = 4096;
	// 8240FAC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240FACC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240FAD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240FAD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240FAD8: 48000571  bl 0x82410048
	ctx.lr = 0x8240FADC;
	sub_82410048(ctx, base);
	// 8240FADC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240FAE0: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240FAE4: 4082FFEC  bne 0x8240fad0
	if !ctx.cr[0].eq {
	pc = 0x8240FAD0; continue 'dispatch;
	}
	// 8240FAE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240FAEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FAF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240FAFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FB08 size=116
    let mut pc: u32 = 0x8240FB08;
    'dispatch: loop {
        match pc {
            0x8240FB08 => {
    //   block [0x8240FB08..0x8240FB7C)
	// 8240FB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FB0C: 481255AD  bl 0x825350b8
	ctx.lr = 0x8240FB10;
	sub_82535080(ctx, base);
	// 8240FB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FB14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240FB18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240FB1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240FB20: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	// 8240FB24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240FB28: 48000551  bl 0x82410078
	ctx.lr = 0x8240FB2C;
	sub_82410078(ctx, base);
	// 8240FB2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240FB30: 41820020  beq 0x8240fb50
	if ctx.cr[0].eq {
	pc = 0x8240FB50; continue 'dispatch;
	}
	// 8240FB34: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240FB38: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 8240FB3C: 2F1F1000  cmpwi cr6, r31, 0x1000
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4096, &mut ctx.xer);
	// 8240FB40: 4198FFE4  blt cr6, 0x8240fb24
	if ctx.cr[6].lt {
	pc = 0x8240FB24; continue 'dispatch;
	}
	// 8240FB44: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 8240FB48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240FB4C: 481255BC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8240FB50: 57EB2834  slwi r11, r31, 5
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240FB54: 389C0008  addi r4, r28, 8
	ctx.r[4].s64 = ctx.r[28].s64 + 8;
	// 8240FB58: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240FB5C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8240FB60: 48000531  bl 0x82410090
	ctx.lr = 0x8240FB64;
	sub_82410090(ctx, base);
	// 8240FB64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240FB68: 4082000C  bne 0x8240fb74
	if !ctx.cr[0].eq {
	pc = 0x8240FB74; continue 'dispatch;
	}
	// 8240FB6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FB70: 4BFFFFD8  b 0x8240fb48
	pc = 0x8240FB48; continue 'dispatch;
	// 8240FB74: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240FB78: 4BFFFFD0  b 0x8240fb48
	pc = 0x8240FB48; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FB80 size=16
    let mut pc: u32 = 0x8240FB80;
    'dispatch: loop {
        match pc {
            0x8240FB80 => {
    //   block [0x8240FB80..0x8240FB90)
	// 8240FB80: 548B2834  slwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240FB84: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240FB88: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8240FB8C: 48000564  b 0x824100f0
	sub_824100F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FB90 size=16
    let mut pc: u32 = 0x8240FB90;
    'dispatch: loop {
        match pc {
            0x8240FB90 => {
    //   block [0x8240FB90..0x8240FBA0)
	// 8240FB90: 548B2834  slwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240FB94: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240FB98: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8240FB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FBA0 size=20
    let mut pc: u32 = 0x8240FBA0;
    'dispatch: loop {
        match pc {
            0x8240FBA0 => {
    //   block [0x8240FBA0..0x8240FBB4)
	// 8240FBA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240FBA4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240FBA8: 4098000C  bge cr6, 0x8240fbb4
	if !ctx.cr[6].lt {
		sub_8240FBB4(ctx, base);
		return;
	}
	// 8240FBAC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240FBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FBB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FBB4 size=28
    let mut pc: u32 = 0x8240FBB4;
    'dispatch: loop {
        match pc {
            0x8240FBB4 => {
    //   block [0x8240FBB4..0x8240FBD0)
	// 8240FBB4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240FBB8: 3D200200  lis r9, 0x200
	ctx.r[9].s64 = 33554432;
	// 8240FBBC: 7C6A2214  add r3, r10, r4
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8240FBC0: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8240FBC4: 4199FFE8  bgt cr6, 0x8240fbac
	if ctx.cr[6].gt {
		sub_8240FBA0(ctx, base);
		return;
	}
	// 8240FBC8: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8240FBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FBD0 size=28
    let mut pc: u32 = 0x8240FBD0;
    'dispatch: loop {
        match pc {
            0x8240FBD0 => {
    //   block [0x8240FBD0..0x8240FBEC)
	// 8240FBD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240FBD4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240FBD8: 4198001C  blt cr6, 0x8240fbf4
	if ctx.cr[6].lt {
		sub_8240FBEC(ctx, base);
		return;
	}
	// 8240FBDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240FBE0: 7C645051  subf. r3, r4, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8240FBE4: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8240FBE8: 4C800020  bgelr
	if !ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FBEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FBEC size=16
    let mut pc: u32 = 0x8240FBEC;
    'dispatch: loop {
        match pc {
            0x8240FBEC => {
    //   block [0x8240FBEC..0x8240FBFC)
	// 8240FBEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240FBF0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240FBF4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FC00 size=64
    let mut pc: u32 = 0x8240FC00;
    'dispatch: loop {
        match pc {
            0x8240FC00 => {
    //   block [0x8240FC00..0x8240FC40)
	// 8240FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FC04: 481254B9  bl 0x825350bc
	ctx.lr = 0x8240FC08;
	sub_82535080(ctx, base);
	// 8240FC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FC0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240FC10: 3BE00FFF  li r31, 0xfff
	ctx.r[31].s64 = 4095;
	// 8240FC14: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 8240FC18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240FC1C: 4800041D  bl 0x82410038
	ctx.lr = 0x8240FC20;
	sub_82410038(ctx, base);
	// 8240FC20: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240FC24: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240FC28: 4080FFF0  bge 0x8240fc18
	if !ctx.cr[0].lt {
	pc = 0x8240FC18; continue 'dispatch;
	}
	// 8240FC2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240FC30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240FC34: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240FC38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FC3C: 481254D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC40 size=4
    let mut pc: u32 = 0x8240FC40;
    'dispatch: loop {
        match pc {
            0x8240FC40 => {
    //   block [0x8240FC40..0x8240FC44)
	// 8240FC40: 48000598  b 0x824101d8
	sub_824101D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC48 size=4
    let mut pc: u32 = 0x8240FC48;
    'dispatch: loop {
        match pc {
            0x8240FC48 => {
    //   block [0x8240FC48..0x8240FC4C)
	// 8240FC48: 48000608  b 0x82410250
	sub_82410250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC50 size=4
    let mut pc: u32 = 0x8240FC50;
    'dispatch: loop {
        match pc {
            0x8240FC50 => {
    //   block [0x8240FC50..0x8240FC54)
	// 8240FC50: 48000678  b 0x824102c8
	sub_824102C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC58 size=4
    let mut pc: u32 = 0x8240FC58;
    'dispatch: loop {
        match pc {
            0x8240FC58 => {
    //   block [0x8240FC58..0x8240FC5C)
	// 8240FC58: 48000678  b 0x824102d0
	sub_824102D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC60 size=4
    let mut pc: u32 = 0x8240FC60;
    'dispatch: loop {
        match pc {
            0x8240FC60 => {
    //   block [0x8240FC60..0x8240FC64)
	// 8240FC60: 480006A8  b 0x82410308
	sub_82410308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC68 size=4
    let mut pc: u32 = 0x8240FC68;
    'dispatch: loop {
        match pc {
            0x8240FC68 => {
    //   block [0x8240FC68..0x8240FC6C)
	// 8240FC68: 48000710  b 0x82410378
	sub_82410378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC70 size=4
    let mut pc: u32 = 0x8240FC70;
    'dispatch: loop {
        match pc {
            0x8240FC70 => {
    //   block [0x8240FC70..0x8240FC74)
	// 8240FC70: 48000760  b 0x824103d0
	sub_824103D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC78 size=4
    let mut pc: u32 = 0x8240FC78;
    'dispatch: loop {
        match pc {
            0x8240FC78 => {
    //   block [0x8240FC78..0x8240FC7C)
	// 8240FC78: 48000780  b 0x824103f8
	sub_824103F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC80 size=4
    let mut pc: u32 = 0x8240FC80;
    'dispatch: loop {
        match pc {
            0x8240FC80 => {
    //   block [0x8240FC80..0x8240FC84)
	// 8240FC80: 48000798  b 0x82410418
	sub_82410418(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FC88 size=68
    let mut pc: u32 = 0x8240FC88;
    'dispatch: loop {
        match pc {
            0x8240FC88 => {
    //   block [0x8240FC88..0x8240FCCC)
	// 8240FC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FC90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FC94: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240FC9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240FCA0: 48000831  bl 0x824104d0
	ctx.lr = 0x8240FCA4;
	sub_824104D0(ctx, base);
	// 8240FCA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240FCA8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8240FCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FCB0: 48124EA1  bl 0x82534b50
	ctx.lr = 0x8240FCB4;
	sub_82534B50(ctx, base);
	// 8240FCB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FCB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FCC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FCD0 size=336
    let mut pc: u32 = 0x8240FCD0;
    'dispatch: loop {
        match pc {
            0x8240FCD0 => {
    //   block [0x8240FCD0..0x8240FE20)
	// 8240FCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FCD8: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240FCDC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FCE0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240FCE4: 4BFFE7ED  bl 0x8240e4d0
	ctx.lr = 0x8240FCE8;
	sub_8240E4D0(ctx, base);
	// 8240FCE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240FCEC: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FCF0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240FCF4: 41990080  bgt cr6, 0x8240fd74
	if ctx.cr[6].gt {
	pc = 0x8240FD74; continue 'dispatch;
	}
	// 8240FCF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240FCFC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8240FD00: C00B294C  lfs f0, 0x294c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FD04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FD08: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240FD0C: C00B2038  lfs f0, 0x2038(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FD10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240FD14: 396BEAB8  addi r11, r11, -0x1548
	ctx.r[11].s64 = ctx.r[11].s64 + -5448;
	// 8240FD18: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8240FD1C: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8240FD20: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240FD24: 7D4907B4  extsw r9, r10
	ctx.r[9].s64 = ctx.r[10].s32 as i64;
	// 8240FD28: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8240FD2C: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8240FD30: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240FD34: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240FD38: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240FD3C: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240FD40: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240FD44: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240FD48: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240FD4C: C12B000C  lfs f9, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8240FD50: C10B0010  lfs f8, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8240FD54: C0EB0014  lfs f7, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8240FD58: EC0D083C  fnmsubs f0, f13, f0, f1
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8240FD5C: EDAC583A  fmadds f13, f12, f0, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 8240FD60: EDAD503A  fmadds f13, f13, f0, f10
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 8240FD64: EDAD483A  fmadds f13, f13, f0, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 8240FD68: EDAD403A  fmadds f13, f13, f0, f8
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[8].f64) as f32) as f64);
	// 8240FD6C: EC0D383A  fmadds f0, f13, f0, f7
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64);
	// 8240FD70: 48000094  b 0x8240fe04
	pc = 0x8240FE04; continue 'dispatch;
	// 8240FD74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FD78: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 8240FD7C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8240FD80: C1AB2038  lfs f13, 0x2038(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8248 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240FD84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240FD88: 394BEAB8  addi r10, r11, -0x1548
	ctx.r[10].s64 = ctx.r[11].s64 + -5448;
	// 8240FD8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240FD90: C18B294C  lfs f12, 0x294c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10572 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240FD94: ED800332  fmuls f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240FD98: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 8240FD9C: 7D804FAE  stfiwx f12, 0, r9
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8240FDA0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240FDA4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 8240FDA8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240FDAC: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8240FDB0: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240FDB4: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8240FDB8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8240FDBC: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240FDC0: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240FDC4: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8240FDC8: C12B0008  lfs f9, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8240FDCC: C10B000C  lfs f8, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8240FDD0: C0EB0010  lfs f7, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8240FDD4: C0CB0014  lfs f6, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8240FDD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240FDDC: EC0C037C  fnmsubs f0, f12, f13, f0
	ctx.f[0].f64 = -(((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240FDE0: EDA052FA  fmadds f13, f0, f11, f10
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 8240FDE4: EDAD483A  fmadds f13, f13, f0, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 8240FDE8: EDAD403A  fmadds f13, f13, f0, f8
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[8].f64) as f32) as f64);
	// 8240FDEC: EDAD383A  fmadds f13, f13, f0, f7
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64);
	// 8240FDF0: EDAD303A  fmadds f13, f13, f0, f6
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[6].f64) as f32) as f64);
	// 8240FDF4: C00A007C  lfs f0, 0x7c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FDF8: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240FDFC: C00BEB38  lfs f0, -0x14c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE00: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240FE04: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240FE08: FC3F682E  fsel f1, f31, f0, f13
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8240FE0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FE18: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240FE20 size=68
    let mut pc: u32 = 0x8240FE20;
    'dispatch: loop {
        match pc {
            0x8240FE20 => {
    //   block [0x8240FE20..0x8240FE64)
	// 8240FE20: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 8240FE24: FC00081E  fctiwz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 8240FE28: 39400168  li r10, 0x168
	ctx.r[10].s64 = 360;
	// 8240FE2C: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 8240FE30: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240FE34: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8240FE38: 1D6B0168  mulli r11, r11, 0x168
	ctx.r[11].s64 = ctx.r[11].s64 * 360;
	// 8240FE3C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240FE40: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8240FE44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE48: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FE4C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240FE50: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240FE54: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240FE58: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE5C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240FE60: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FE64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240FE64 size=16
    let mut pc: u32 = 0x8240FE64;
    'dispatch: loop {
        match pc {
            0x8240FE64 => {
    //   block [0x8240FE64..0x8240FE74)
	// 8240FE64: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE68: C00B210C  lfs f0, 0x210c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE6C: EC21002A  fadds f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240FE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FE78 size=60
    let mut pc: u32 = 0x8240FE78;
    'dispatch: loop {
        match pc {
            0x8240FE78 => {
    //   block [0x8240FE78..0x8240FEB4)
	// 8240FE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FE84: 4BFFFF9D  bl 0x8240fe20
	ctx.lr = 0x8240FE88;
	sub_8240FE20(ctx, base);
	// 8240FE88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE8C: C00B213C  lfs f0, 0x213c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8508 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE90: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240FE94: 40990010  ble cr6, 0x8240fea4
	if !ctx.cr[6].gt {
	pc = 0x8240FEA4; continue 'dispatch;
	}
	// 8240FE98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE9C: C00B210C  lfs f0, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FEA0: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240FEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FEB8 size=8
    let mut pc: u32 = 0x8240FEB8;
    'dispatch: loop {
        match pc {
            0x8240FEB8 => {
    //   block [0x8240FEB8..0x8240FEC0)
	// 8240FEB8: EC20082C  fsqrts f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64).sqrt() as f32) as f64;
	// 8240FEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FEC0 size=104
    let mut pc: u32 = 0x8240FEC0;
    'dispatch: loop {
        match pc {
            0x8240FEC0 => {
    //   block [0x8240FEC0..0x8240FF28)
	// 8240FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FEC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FECC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240FED4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240FED8: 419A003C  beq cr6, 0x8240ff14
	if ctx.cr[6].eq {
	pc = 0x8240FF14; continue 'dispatch;
	}
	// 8240FEDC: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FEE0: 48000711  bl 0x824105f0
	ctx.lr = 0x8240FEE4;
	sub_824105F0(ctx, base);
	// 8240FEE4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FEE8: 419A0028  beq cr6, 0x8240ff10
	if ctx.cr[6].eq {
	pc = 0x8240FF10; continue 'dispatch;
	}
	// 8240FEEC: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FEF0: 48000701  bl 0x824105f0
	ctx.lr = 0x8240FEF4;
	sub_824105F0(ctx, base);
	// 8240FEF4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FEF8: 419A0018  beq cr6, 0x8240ff10
	if ctx.cr[6].eq {
	pc = 0x8240FF10; continue 'dispatch;
	}
	// 8240FEFC: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF00: 480006F1  bl 0x824105f0
	ctx.lr = 0x8240FF04;
	sub_824105F0(ctx, base);
	// 8240FF04: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240FF0C: 409A0008  bne cr6, 0x8240ff14
	if !ctx.cr[6].eq {
	pc = 0x8240FF14; continue 'dispatch;
	}
	// 8240FF10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240FF14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FF20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FF28 size=120
    let mut pc: u32 = 0x8240FF28;
    'dispatch: loop {
        match pc {
            0x8240FF28 => {
    //   block [0x8240FF28..0x8240FFA0)
	// 8240FF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FF34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FF38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240FF3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240FF40: 419A004C  beq cr6, 0x8240ff8c
	if ctx.cr[6].eq {
	pc = 0x8240FF8C; continue 'dispatch;
	}
	// 8240FF44: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF48: 480006A9  bl 0x824105f0
	ctx.lr = 0x8240FF4C;
	sub_824105F0(ctx, base);
	// 8240FF4C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF50: 419A0038  beq cr6, 0x8240ff88
	if ctx.cr[6].eq {
	pc = 0x8240FF88; continue 'dispatch;
	}
	// 8240FF54: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF58: 48000699  bl 0x824105f0
	ctx.lr = 0x8240FF5C;
	sub_824105F0(ctx, base);
	// 8240FF5C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF60: 419A0028  beq cr6, 0x8240ff88
	if ctx.cr[6].eq {
	pc = 0x8240FF88; continue 'dispatch;
	}
	// 8240FF64: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF68: 48000689  bl 0x824105f0
	ctx.lr = 0x8240FF6C;
	sub_824105F0(ctx, base);
	// 8240FF6C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF70: 419A0018  beq cr6, 0x8240ff88
	if ctx.cr[6].eq {
	pc = 0x8240FF88; continue 'dispatch;
	}
	// 8240FF74: C03F000C  lfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF78: 48000679  bl 0x824105f0
	ctx.lr = 0x8240FF7C;
	sub_824105F0(ctx, base);
	// 8240FF7C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240FF84: 409A0008  bne cr6, 0x8240ff8c
	if !ctx.cr[6].eq {
	pc = 0x8240FF8C; continue 'dispatch;
	}
	// 8240FF88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240FF8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FF98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FFA0 size=4
    let mut pc: u32 = 0x8240FFA0;
    'dispatch: loop {
        match pc {
            0x8240FFA0 => {
    //   block [0x8240FFA0..0x8240FFA4)
	// 8240FFA0: 48000650  b 0x824105f0
	sub_824105F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FFA8 size=8
    let mut pc: u32 = 0x8240FFA8;
    'dispatch: loop {
        match pc {
            0x8240FFA8 => {
    //   block [0x8240FFA8..0x8240FFB0)
	// 8240FFA8: FC200A10  fabs f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 8240FFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FFB0 size=16
    let mut pc: u32 = 0x8240FFB0;
    'dispatch: loop {
        match pc {
            0x8240FFB0 => {
    //   block [0x8240FFB0..0x8240FFC0)
	// 8240FFB0: 7C6BFE70  srawi r11, r3, 0x1f
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 31) as i64;
	// 8240FFB4: 7C6A5A78  xor r10, r3, r11
	ctx.r[10].u64 = ctx.r[3].u64 ^ ctx.r[11].u64;
	// 8240FFB8: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8240FFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FFC0 size=36
    let mut pc: u32 = 0x8240FFC0;
    'dispatch: loop {
        match pc {
            0x8240FFC0 => {
    //   block [0x8240FFC0..0x8240FFE4)
	// 8240FFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FFC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FFCC: 4812318D  bl 0x82533158
	ctx.lr = 0x8240FFD0;
	sub_82533158(ctx, base);
	// 8240FFD0: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8240FFD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FFE8 size=36
    let mut pc: u32 = 0x8240FFE8;
    'dispatch: loop {
        match pc {
            0x8240FFE8 => {
    //   block [0x8240FFE8..0x8241000C)
	// 8240FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FFF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FFF4: 48122FE5  bl 0x82532fd8
	ctx.lr = 0x8240FFF8;
	sub_82532FD8(ctx, base);
	// 8240FFF8: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8240FFFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410010 size=36
    let mut pc: u32 = 0x82410010;
    'dispatch: loop {
        match pc {
            0x82410010 => {
    //   block [0x82410010..0x82410034)
	// 82410010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241001C: 48123755  bl 0x82533770
	ctx.lr = 0x82410020;
	sub_82533770(ctx, base);
	// 82410020: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82410024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241002C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410038 size=12
    let mut pc: u32 = 0x82410038;
    'dispatch: loop {
        match pc {
            0x82410038 => {
    //   block [0x82410038..0x82410044)
	// 82410038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241003C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82410040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410048 size=44
    let mut pc: u32 = 0x82410048;
    'dispatch: loop {
        match pc {
            0x82410048 => {
    //   block [0x82410048..0x82410074)
	// 82410048: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241004C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410050: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82410054: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410058: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8241005C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82410060: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82410064: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82410068: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8241006C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82410070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410078 size=20
    let mut pc: u32 = 0x82410078;
    'dispatch: loop {
        match pc {
            0x82410078 => {
    //   block [0x82410078..0x8241008C)
	// 82410078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241007C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82410080: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82410084: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82410088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410090 size=92
    let mut pc: u32 = 0x82410090;
    'dispatch: loop {
        match pc {
            0x82410090 => {
    //   block [0x82410090..0x824100EC)
	// 82410090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241009C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824100A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824100A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824100A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824100AC: 419A000C  beq cr6, 0x824100b8
	if ctx.cr[6].eq {
	pc = 0x824100B8; continue 'dispatch;
	}
	// 824100B0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824100B4: 48000024  b 0x824100d8
	pc = 0x824100D8; continue 'dispatch;
	// 824100B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824100BC: 419AFFF4  beq cr6, 0x824100b0
	if ctx.cr[6].eq {
	pc = 0x824100B0; continue 'dispatch;
	}
	// 824100C0: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 824100C4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824100C8: 4BFB0B39  bl 0x823c0c00
	ctx.lr = 0x824100CC;
	sub_823C0C00(ctx, base);
	// 824100CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824100D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824100D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824100D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824100DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824100E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824100E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824100E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824100F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824100F0 size=20
    let mut pc: u32 = 0x824100F0;
    'dispatch: loop {
        match pc {
            0x824100F0 => {
    //   block [0x824100F0..0x82410104)
	// 824100F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824100F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824100F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824100FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82410100: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410104 size=12
    let mut pc: u32 = 0x82410104;
    'dispatch: loop {
        match pc {
            0x82410104 => {
    //   block [0x82410104..0x82410110)
	// 82410104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82410108: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241010C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410110 size=200
    let mut pc: u32 = 0x82410110;
    'dispatch: loop {
        match pc {
            0x82410110 => {
    //   block [0x82410110..0x824101D8)
	// 82410110: D0210014  stfs f1, 0x14(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410114: D021FFE8  stfs f1, -0x18(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82410118: D021FFEC  stfs f1, -0x14(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 8241011C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82410120: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82410124: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82410128: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8241012C: C1A1FFF0  lfs f13, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410130: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410134: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82410138: D001FFD4  stfs f0, -0x2c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 8241013C: 396BDE10  addi r11, r11, -0x21f0
	ctx.r[11].s64 = ctx.r[11].s64 + -8688;
	// 82410140: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82410144: EC0D0072  fmuls f0, f13, f1
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824101D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824101D8 size=120
    let mut pc: u32 = 0x824101D8;
    'dispatch: loop {
        match pc {
            0x824101D8 => {
    //   block [0x824101D8..0x82410250)
	// 824101D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824101DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824101E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824101E4: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824101E8: 3D600124  lis r11, 0x124
	ctx.r[11].s64 = 19136512;
	// 824101EC: 616A3F6D  ori r10, r11, 0x3f6d
	ctx.r[10].u64 = ctx.r[11].u64 | 16237;
	// 824101F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824101F4: C00B2204  lfs f0, 0x2204(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824101F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824101FC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82410200: 514BF07E  rlwimi r11, r10, 0x1e, 1, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(30) as u64) & 0x000000007FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFF80000000);
	// 82410204: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82410208: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241020C: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410210: ED8D082A  fadds f12, f13, f1
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64;
	// 82410214: C1AB2200  lfs f13, 0x2200(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410218: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8241021C: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82410220: 7DA04FAE  stfiwx f13, 0, r9
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82410224: E9610052  lwa r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 82410228: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241022C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82410230: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82410234: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82410238: EC2D083C  fnmsubs f1, f13, f0, f1
	ctx.f[1].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8241023C: 4BD0D28D  bl 0x8211d4c8
	ctx.lr = 0x82410240;
	sub_8211D4C8(ctx, base);
	// 82410240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410250 size=120
    let mut pc: u32 = 0x82410250;
    'dispatch: loop {
        match pc {
            0x82410250 => {
    //   block [0x82410250..0x824102C8)
	// 82410250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241025C: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82410260: 3D600124  lis r11, 0x124
	ctx.r[11].s64 = 19136512;
	// 82410264: 616A3F6D  ori r10, r11, 0x3f6d
	ctx.r[10].u64 = ctx.r[11].u64 | 16237;
	// 82410268: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241026C: C00B2204  lfs f0, 0x2204(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410270: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82410274: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82410278: 514BF07E  rlwimi r11, r10, 0x1e, 1, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(30) as u64) & 0x000000007FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFF80000000);
	// 8241027C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82410280: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410284: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410288: ED8D082A  fadds f12, f13, f1
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64;
	// 8241028C: C1AB2200  lfs f13, 0x2200(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410290: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82410294: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82410298: 7DA04FAE  stfiwx f13, 0, r9
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8241029C: E9610052  lwa r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 824102A0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824102A4: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824102A8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824102AC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824102B0: EC2D083C  fnmsubs f1, f13, f0, f1
	ctx.f[1].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 824102B4: 4BD43005  bl 0x821532b8
	ctx.lr = 0x824102B8;
	sub_821532B8(ctx, base);
	// 824102B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824102BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824102C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824102C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824102C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824102C8 size=4
    let mut pc: u32 = 0x824102C8;
    'dispatch: loop {
        match pc {
            0x824102C8 => {
    //   block [0x824102C8..0x824102CC)
	// 824102C8: 4BFFFE48  b 0x82410110
	sub_82410110(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824102D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824102D0 size=52
    let mut pc: u32 = 0x824102D0;
    'dispatch: loop {
        match pc {
            0x824102D0 => {
    //   block [0x824102D0..0x82410304)
	// 824102D0: C1A50000  lfs f13, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824102D4: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824102D8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824102DC: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824102E0: C1A50004  lfs f13, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824102E4: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824102E8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824102EC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824102F0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824102F4: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824102F8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824102FC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82410300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410308 size=112
    let mut pc: u32 = 0x82410308;
    'dispatch: loop {
        match pc {
            0x82410308 => {
    //   block [0x82410308..0x82410378)
	// 82410308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241031C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410324: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410328: 4BFFF951  bl 0x8240fc78
	ctx.lr = 0x8241032C;
	sub_8240FC78(ctx, base);
	// 8241032C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82410330: C1BE0000  lfs f13, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82410338: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8241033C: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82410340: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410344: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410348: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241034C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410350: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82410354: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410358: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8241035C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82410360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241036C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410378 size=88
    let mut pc: u32 = 0x82410378;
    'dispatch: loop {
        match pc {
            0x82410378 => {
    //   block [0x82410378..0x824103D0)
	// 82410378: C1A40008  lfs f13, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241037C: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410380: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82410384: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82410388: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241038C: EC0D0338  fmsubs f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82410390: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410394: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410398: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8241039C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824103A0: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824103A4: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103A8: EC0D0338  fmsubs f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 824103AC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824103B0: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103B4: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824103B8: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824103BC: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824103C0: C1A50004  lfs f13, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103C4: EC0D0338  fmsubs f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 824103C8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824103CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824103D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824103D0 size=40
    let mut pc: u32 = 0x824103D0;
    'dispatch: loop {
        match pc {
            0x824103D0 => {
    //   block [0x824103D0..0x824103F8)
	// 824103D0: C1A40004  lfs f13, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103D4: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824103D8: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824103DC: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824103E0: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103E4: C1630000  lfs f11, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824103E8: C1440000  lfs f10, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824103EC: EC0D033A  fmadds f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 824103F0: EC2B02BA  fmadds f1, f11, f10, f0
	ctx.f[1].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64);
	// 824103F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824103F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824103F8 size=28
    let mut pc: u32 = 0x824103F8;
    'dispatch: loop {
        match pc {
            0x824103F8 => {
    //   block [0x824103F8..0x82410414)
	// 824103F8: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103FC: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82410400: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410404: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82410408: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8241040C: EC2C033A  fmadds f1, f12, f12, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82410410: 4BFFFAA8  b 0x8240feb8
	sub_8240FEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410418 size=184
    let mut pc: u32 = 0x82410418;
    'dispatch: loop {
        match pc {
            0x82410418 => {
    //   block [0x82410418..0x824104D0)
	// 82410418: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241041C: C1A50010  lfs f13, 0x10(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410420: C1450020  lfs f10, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82410424: C1250000  lfs f9, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82410428: C1050030  lfs f8, 0x30(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8241042C: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 82410430: C001FFF4  lfs f0, -0xc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410434: ED6D0032  fmuls f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410438: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241043C: 9161FFF8  stw r11, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82410440: C1A1FFF8  lfs f13, -8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410444: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410448: ED6A5B7A  fmadds f11, f10, f13, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 8241044C: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82410450: C181FFF0  lfs f12, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82410454: ED695B3A  fmadds f11, f9, f12, f11
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 82410458: ED6B402A  fadds f11, f11, f8
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 8241045C: D1630000  stfs f11, 0(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410460: C1650014  lfs f11, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82410464: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410468: C1450024  lfs f10, 0x24(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8241046C: C1250004  lfs f9, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82410470: C1050034  lfs f8, 0x34(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(52 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82410474: ED6A5B7A  fmadds f11, f10, f13, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 82410478: ED695B3A  fmadds f11, f9, f12, f11
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 8241047C: ED6B402A  fadds f11, f11, f8
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 82410480: D1630004  stfs f11, 4(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82410484: C1650018  lfs f11, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82410488: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8241048C: C1450028  lfs f10, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82410490: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82410494: C1050038  lfs f8, 0x38(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(56 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82410498: ED6A5B7A  fmadds f11, f10, f13, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 8241049C: ED695B3A  fmadds f11, f9, f12, f11
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 824104A0: ED6B402A  fadds f11, f11, f8
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 824104A4: D1630008  stfs f11, 8(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824104A8: C165001C  lfs f11, 0x1c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824104AC: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 824104B0: C165002C  lfs f11, 0x2c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824104B4: C145000C  lfs f10, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824104B8: C125003C  lfs f9, 0x3c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(60 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824104BC: EC0B037A  fmadds f0, f11, f13, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 824104C0: EC0A033A  fmadds f0, f10, f12, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 824104C4: EC00482A  fadds f0, f0, f9
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64;
	// 824104C8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824104CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824104D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824104D0 size=284
    let mut pc: u32 = 0x824104D0;
    'dispatch: loop {
        match pc {
            0x824104D0 => {
    //   block [0x824104D0..0x824105EC)
	// 824104D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824104D4: 48124BE9  bl 0x825350bc
	ctx.lr = 0x824104D8;
	sub_82535080(ctx, base);
	// 824104D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824104DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824104E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824104E4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 824104E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824104EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824104F0: 4BFFF771  bl 0x8240fc60
	ctx.lr = 0x824104F4;
	sub_8240FC60(ctx, base);
	// 824104F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824104F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824104FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82410500: 4BFFF759  bl 0x8240fc58
	ctx.lr = 0x82410504;
	sub_8240FC58(ctx, base);
	// 82410504: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82410508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8241050C: 4BFFF755  bl 0x8240fc60
	ctx.lr = 0x82410510;
	sub_8240FC60(ctx, base);
	// 82410510: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82410514: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82410518: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8241051C: 4BFFF74D  bl 0x8240fc68
	ctx.lr = 0x82410520;
	sub_8240FC68(ctx, base);
	// 82410520: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82410524: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82410528: 4BFFF739  bl 0x8240fc60
	ctx.lr = 0x8241052C;
	sub_8240FC60(ctx, base);
	// 8241052C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82410530: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82410534: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82410538: 4BFFF731  bl 0x8240fc68
	ctx.lr = 0x8241053C;
	sub_8240FC68(ctx, base);
	// 8241053C: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410540: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410544: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410548: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241054C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410550: C0010068  lfs f0, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410554: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82410558: D1BF0010  stfs f13, 0x10(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8241055C: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82410560: C1A10070  lfs f13, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410564: C0010074  lfs f0, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410568: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8241056C: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410570: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410574: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410578: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8241057C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82410580: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410584: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410588: C1810058  lfs f12, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8241058C: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82410590: D19F0028  stfs f12, 0x28(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82410594: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82410598: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8241059C: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824105A0: 4BFFF6D1  bl 0x8240fc70
	ctx.lr = 0x824105A4;
	sub_8240FC70(ctx, base);
	// 824105A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824105A8: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 824105AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824105B0: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824105B4: 4BFFF6BD  bl 0x8240fc70
	ctx.lr = 0x824105B8;
	sub_8240FC70(ctx, base);
	// 824105B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824105BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824105C0: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 824105C4: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824105C8: 4BFFF6A9  bl 0x8240fc70
	ctx.lr = 0x824105CC;
	sub_8240FC70(ctx, base);
	// 824105CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824105D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824105D4: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824105D8: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 824105DC: FC000850  fneg f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 824105E0: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 824105E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824105E8: 48124B24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824105F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824105F0 size=44
    let mut pc: u32 = 0x824105F0;
    'dispatch: loop {
        match pc {
            0x824105F0 => {
    //   block [0x824105F0..0x8241061C)
	// 824105F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824105F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824105F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824105FC: 48123C5D  bl 0x82534258
	ctx.lr = 0x82410600;
	sub_82534258(ctx, base);
	// 82410600: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82410604: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82410608: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8241060C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410620 size=120
    let mut pc: u32 = 0x82410620;
    'dispatch: loop {
        match pc {
            0x82410620 => {
    //   block [0x82410620..0x82410698)
	// 82410620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410624: 48124A95  bl 0x825350b8
	ctx.lr = 0x82410628;
	sub_82535080(ctx, base);
	// 82410628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241062C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82410630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410634: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82410638: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8241063C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82410640: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410644: 4800017D  bl 0x824107c0
	ctx.lr = 0x82410648;
	sub_824107C0(ctx, base);
	// 82410648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241064C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82410650: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410654: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410658: 4E800421  bctrl
	ctx.lr = 0x8241065C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241065C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410660: 41820018  beq 0x82410678
	if ctx.cr[0].eq {
	pc = 0x82410678; continue 'dispatch;
	}
	// 82410664: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82410668: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241066C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82410670: 48000679  bl 0x82410ce8
	ctx.lr = 0x82410674;
	sub_82410CE8(ctx, base);
	// 82410674: 48000008  b 0x8241067c
	pc = 0x8241067C; continue 'dispatch;
	// 82410678: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241067C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82410680: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82410684: 409A0008  bne cr6, 0x8241068c
	if !ctx.cr[6].eq {
	pc = 0x8241068C; continue 'dispatch;
	}
	// 82410688: 48000000  b 0x82410688
	pc = 0x82410688; continue 'dispatch;
	// 8241068C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82410690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82410694: 48124A74  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410698 size=92
    let mut pc: u32 = 0x82410698;
    'dispatch: loop {
        match pc {
            0x82410698 => {
    //   block [0x82410698..0x824106F4)
	// 82410698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824106A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824106A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824106A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824106AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824106B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824106B4: 480006BD  bl 0x82410d70
	ctx.lr = 0x824106B8;
	sub_82410D70(ctx, base);
	// 824106B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824106BC: 4182001C  beq 0x824106d8
	if ctx.cr[0].eq {
	pc = 0x824106D8; continue 'dispatch;
	}
	// 824106C0: 48000101  bl 0x824107c0
	ctx.lr = 0x824106C4;
	sub_824107C0(ctx, base);
	// 824106C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824106C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824106CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824106D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824106D4: 4E800421  bctrl
	ctx.lr = 0x824106D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824106D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824106DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824106E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824106E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824106E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824106EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824106F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824106F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824106F8 size=8
    let mut pc: u32 = 0x824106F8;
    'dispatch: loop {
        match pc {
            0x824106F8 => {
    //   block [0x824106F8..0x82410700)
	// 824106F8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824106FC: 480006C4  b 0x82410dc0
	sub_82410DC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410700 size=8
    let mut pc: u32 = 0x82410700;
    'dispatch: loop {
        match pc {
            0x82410700 => {
    //   block [0x82410700..0x82410708)
	// 82410700: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410704: 48000184  b 0x82410888
	sub_82410888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410708 size=8
    let mut pc: u32 = 0x82410708;
    'dispatch: loop {
        match pc {
            0x82410708 => {
    //   block [0x82410708..0x82410710)
	// 82410708: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241070C: 480001D4  b 0x824108e0
	sub_824108E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410710 size=8
    let mut pc: u32 = 0x82410710;
    'dispatch: loop {
        match pc {
            0x82410710 => {
    //   block [0x82410710..0x82410718)
	// 82410710: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410714: 4800028C  b 0x824109a0
	sub_824109A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410718 size=8
    let mut pc: u32 = 0x82410718;
    'dispatch: loop {
        match pc {
            0x82410718 => {
    //   block [0x82410718..0x82410720)
	// 82410718: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241071C: 4800034C  b 0x82410a68
	sub_82410A68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410720 size=8
    let mut pc: u32 = 0x82410720;
    'dispatch: loop {
        match pc {
            0x82410720 => {
    //   block [0x82410720..0x82410728)
	// 82410720: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410724: 480002A4  b 0x824109c8
	sub_824109C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410728 size=8
    let mut pc: u32 = 0x82410728;
    'dispatch: loop {
        match pc {
            0x82410728 => {
    //   block [0x82410728..0x82410730)
	// 82410728: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241072C: 48000394  b 0x82410ac0
	sub_82410AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410730 size=8
    let mut pc: u32 = 0x82410730;
    'dispatch: loop {
        match pc {
            0x82410730 => {
    //   block [0x82410730..0x82410738)
	// 82410730: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410734: 480002AC  b 0x824109e0
	sub_824109E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410738 size=8
    let mut pc: u32 = 0x82410738;
    'dispatch: loop {
        match pc {
            0x82410738 => {
    //   block [0x82410738..0x82410740)
	// 82410738: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241073C: 480002BC  b 0x824109f8
	sub_824109F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410740 size=8
    let mut pc: u32 = 0x82410740;
    'dispatch: loop {
        match pc {
            0x82410740 => {
    //   block [0x82410740..0x82410748)
	// 82410740: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410744: 480002D4  b 0x82410a18
	sub_82410A18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410748 size=8
    let mut pc: u32 = 0x82410748;
    'dispatch: loop {
        match pc {
            0x82410748 => {
    //   block [0x82410748..0x82410750)
	// 82410748: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241074C: 480002E4  b 0x82410a30
	sub_82410A30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410750 size=8
    let mut pc: u32 = 0x82410750;
    'dispatch: loop {
        match pc {
            0x82410750 => {
    //   block [0x82410750..0x82410758)
	// 82410750: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410754: 480002F4  b 0x82410a48
	sub_82410A48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410758 size=68
    let mut pc: u32 = 0x82410758;
    'dispatch: loop {
        match pc {
            0x82410758 => {
    //   block [0x82410758..0x8241079C)
	// 82410758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241076C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410770: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410774: 4182000C  beq 0x82410780
	if ctx.cr[0].eq {
	pc = 0x82410780; continue 'dispatch;
	}
	// 82410778: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241077C: 4BFFFF1D  bl 0x82410698
	ctx.lr = 0x82410780;
	sub_82410698(ctx, base);
	// 82410780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410784: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824107A0 size=20
    let mut pc: u32 = 0x824107A0;
    'dispatch: loop {
        match pc {
            0x824107A0 => {
    //   block [0x824107A0..0x824107B4)
	// 824107A0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824107A4: 814BF20C  lwz r10, -0xdf4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3572 as u32) ) } as u64;
	// 824107A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824107AC: 419A0008  beq cr6, 0x824107b4
	if ctx.cr[6].eq {
		sub_824107B4(ctx, base);
		return;
	}
	// 824107B0: 48000000  b 0x824107b0
	pc = 0x824107B0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824107B4 size=8
    let mut pc: u32 = 0x824107B4;
    'dispatch: loop {
        match pc {
            0x824107B4 => {
    //   block [0x824107B4..0x824107BC)
	// 824107B4: 906BF20C  stw r3, -0xdf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3572 as u32), ctx.r[3].u32 ) };
	// 824107B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824107C0 size=16
    let mut pc: u32 = 0x824107C0;
    'dispatch: loop {
        match pc {
            0x824107C0 => {
    //   block [0x824107C0..0x824107D0)
	// 824107C0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824107C4: 806BF20C  lwz r3, -0xdf4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3572 as u32) ) } as u64;
	// 824107C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824107CC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824107D0 size=8
    let mut pc: u32 = 0x824107D0;
    'dispatch: loop {
        match pc {
            0x824107D0 => {
    //   block [0x824107D0..0x824107D8)
	// 824107D0: 48000000  b 0x824107d0
	pc = 0x824107D0; continue 'dispatch;
	// 824107D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824107D8 size=80
    let mut pc: u32 = 0x824107D8;
    'dispatch: loop {
        match pc {
            0x824107D8 => {
    //   block [0x824107D8..0x82410828)
	// 824107D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824107DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824107E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824107E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824107E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824107EC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824107F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824107F4: 41820020  beq 0x82410814
	if ctx.cr[0].eq {
	pc = 0x82410814; continue 'dispatch;
	}
	// 824107F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824107FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410800: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410808: 4E800421  bctrl
	ctx.lr = 0x8241080C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241080C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410810: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241081C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410828 size=92
    let mut pc: u32 = 0x82410828;
    'dispatch: loop {
        match pc {
            0x82410828 => {
    //   block [0x82410828..0x82410884)
	// 82410828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241082C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241083C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410840: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410844: 48000AF5  bl 0x82411338
	ctx.lr = 0x82410848;
	sub_82411338(ctx, base);
	// 82410848: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241084C: 4182001C  beq 0x82410868
	if ctx.cr[0].eq {
	pc = 0x82410868; continue 'dispatch;
	}
	// 82410850: 4BFFFF71  bl 0x824107c0
	ctx.lr = 0x82410854;
	sub_824107C0(ctx, base);
	// 82410854: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241085C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82410860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410864: 4E800421  bctrl
	ctx.lr = 0x82410868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241086C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241087C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410888 size=84
    let mut pc: u32 = 0x82410888;
    'dispatch: loop {
        match pc {
            0x82410888 => {
    //   block [0x82410888..0x824108DC)
	// 82410888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241089C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824108A0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824108A4: 4082000C  bne 0x824108b0
	if !ctx.cr[0].eq {
	pc = 0x824108B0; continue 'dispatch;
	}
	// 824108A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824108AC: 4800001C  b 0x824108c8
	pc = 0x824108C8; continue 'dispatch;
	// 824108B0: 48000D11  bl 0x824115c0
	ctx.lr = 0x824108B4;
	sub_824115C0(ctx, base);
	// 824108B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824108B8: 4182FFF0  beq 0x824108a8
	if ctx.cr[0].eq {
	pc = 0x824108A8; continue 'dispatch;
	}
	// 824108BC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824108C0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824108C4: 48000A6D  bl 0x82411330
	ctx.lr = 0x824108C8;
	sub_82411330(ctx, base);
	// 824108C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824108CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824108D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824108D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824108D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824108E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824108E0 size=188
    let mut pc: u32 = 0x824108E0;
    'dispatch: loop {
        match pc {
            0x824108E0 => {
    //   block [0x824108E0..0x8241099C)
	// 824108E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824108E4: 481247D9  bl 0x825350bc
	ctx.lr = 0x824108E8;
	sub_82535080(ctx, base);
	// 824108E8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 824108EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824108F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824108F4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824108F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824108FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410900: 409A000C  bne cr6, 0x8241090c
	if !ctx.cr[6].eq {
	pc = 0x8241090C; continue 'dispatch;
	}
	// 82410904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410908: 48000088  b 0x82410990
	pc = 0x82410990; continue 'dispatch;
	// 8241090C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410910: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82410914: 480005B5  bl 0x82410ec8
	ctx.lr = 0x82410918;
	sub_82410EC8(ctx, base);
	// 82410918: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241091C: 4182FFE8  beq 0x82410904
	if ctx.cr[0].eq {
	pc = 0x82410904; continue 'dispatch;
	}
	// 82410920: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410924: 48000755  bl 0x82411078
	ctx.lr = 0x82410928;
	sub_82411078(ctx, base);
	// 82410928: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8241092C: 409A0018  bne cr6, 0x82410944
	if !ctx.cr[6].eq {
	pc = 0x82410944; continue 'dispatch;
	}
	// 82410930: C01F0014  lfs f0, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410934: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82410938: 409A000C  bne cr6, 0x82410944
	if !ctx.cr[6].eq {
	pc = 0x82410944; continue 'dispatch;
	}
	// 8241093C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410940: 48000050  b 0x82410990
	pc = 0x82410990; continue 'dispatch;
	// 82410944: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410948: 48000731  bl 0x82411078
	ctx.lr = 0x8241094C;
	sub_82411078(ctx, base);
	// 8241094C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410950: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82410954: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410958: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8241095C: 480009D5  bl 0x82411330
	ctx.lr = 0x82410960;
	sub_82411330(ctx, base);
	// 82410960: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82410964: 41820028  beq 0x8241098c
	if ctx.cr[0].eq {
	pc = 0x8241098C; continue 'dispatch;
	}
	// 82410968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241096C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410970: D3FF0014  stfs f31, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410974: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82410978: 48000701  bl 0x82411078
	ctx.lr = 0x8241097C;
	sub_82411078(ctx, base);
	// 8241097C: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82410980: 409A000C  bne cr6, 0x8241098c
	if !ctx.cr[6].eq {
	pc = 0x8241098C; continue 'dispatch;
	}
	// 82410984: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410988: 48000721  bl 0x824110a8
	ctx.lr = 0x8241098C;
	sub_824110A8(ctx, base);
	// 8241098C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82410994: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82410998: 48124774  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109A0 size=20
    let mut pc: u32 = 0x824109A0;
    'dispatch: loop {
        match pc {
            0x824109A0 => {
    //   block [0x824109A0..0x824109B4)
	// 824109A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824109A4: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824109AC: 40820008  bne 0x824109b4
	if !ctx.cr[0].eq {
		sub_824109B4(ctx, base);
		return;
	}
	// 824109B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824109B4 size=16
    let mut pc: u32 = 0x824109B4;
    'dispatch: loop {
        match pc {
            0x824109B4 => {
    //   block [0x824109B4..0x824109C4)
	// 824109B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824109B8: D02B0014  stfs f1, 0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824109BC: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 824109C0: 48000D80  b 0x82411740
	sub_82411740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109C8 size=16
    let mut pc: u32 = 0x824109C8;
    'dispatch: loop {
        match pc {
            0x824109C8 => {
    //   block [0x824109C8..0x824109D8)
	// 824109C8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824109D0: 40820008  bne 0x824109d8
	if !ctx.cr[0].eq {
		sub_824109D8(ctx, base);
		return;
	}
	// 824109D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109D8 size=4
    let mut pc: u32 = 0x824109D8;
    'dispatch: loop {
        match pc {
            0x824109D8 => {
    //   block [0x824109D8..0x824109DC)
	// 824109D8: 48000BD0  b 0x824115a8
	sub_824115A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109E0 size=16
    let mut pc: u32 = 0x824109E0;
    'dispatch: loop {
        match pc {
            0x824109E0 => {
    //   block [0x824109E0..0x824109F0)
	// 824109E0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824109E8: 40820008  bne 0x824109f0
	if !ctx.cr[0].eq {
		sub_824109F0(ctx, base);
		return;
	}
	// 824109EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109F0 size=4
    let mut pc: u32 = 0x824109F0;
    'dispatch: loop {
        match pc {
            0x824109F0 => {
    //   block [0x824109F0..0x824109F4)
	// 824109F0: 48000B28  b 0x82411518
	sub_82411518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109F8 size=16
    let mut pc: u32 = 0x824109F8;
    'dispatch: loop {
        match pc {
            0x824109F8 => {
    //   block [0x824109F8..0x82410A08)
	// 824109F8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109FC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A00: 40820008  bne 0x82410a08
	if !ctx.cr[0].eq {
		sub_82410A08(ctx, base);
		return;
	}
	// 82410A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A08 size=16
    let mut pc: u32 = 0x82410A08;
    'dispatch: loop {
        match pc {
            0x82410A08 => {
    //   block [0x82410A08..0x82410A18)
	// 82410A08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410A0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410A14: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A18 size=16
    let mut pc: u32 = 0x82410A18;
    'dispatch: loop {
        match pc {
            0x82410A18 => {
    //   block [0x82410A18..0x82410A28)
	// 82410A18: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A20: 40820008  bne 0x82410a28
	if !ctx.cr[0].eq {
		sub_82410A28(ctx, base);
		return;
	}
	// 82410A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A28 size=4
    let mut pc: u32 = 0x82410A28;
    'dispatch: loop {
        match pc {
            0x82410A28 => {
    //   block [0x82410A28..0x82410A2C)
	// 82410A28: 48000E70  b 0x82411898
	sub_82411898(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A30 size=16
    let mut pc: u32 = 0x82410A30;
    'dispatch: loop {
        match pc {
            0x82410A30 => {
    //   block [0x82410A30..0x82410A40)
	// 82410A30: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A34: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A38: 40820008  bne 0x82410a40
	if !ctx.cr[0].eq {
		sub_82410A40(ctx, base);
		return;
	}
	// 82410A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A40 size=4
    let mut pc: u32 = 0x82410A40;
    'dispatch: loop {
        match pc {
            0x82410A40 => {
    //   block [0x82410A40..0x82410A44)
	// 82410A40: 48000E70  b 0x824118b0
	sub_824118B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A48 size=16
    let mut pc: u32 = 0x82410A48;
    'dispatch: loop {
        match pc {
            0x82410A48 => {
    //   block [0x82410A48..0x82410A58)
	// 82410A48: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A4C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A50: 40820008  bne 0x82410a58
	if !ctx.cr[0].eq {
		sub_82410A58(ctx, base);
		return;
	}
	// 82410A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A58 size=16
    let mut pc: u32 = 0x82410A58;
    'dispatch: loop {
        match pc {
            0x82410A58 => {
    //   block [0x82410A58..0x82410A68)
	// 82410A58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410A5C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82410A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410A64: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410A68 size=88
    let mut pc: u32 = 0x82410A68;
    'dispatch: loop {
        match pc {
            0x82410A68 => {
    //   block [0x82410A68..0x82410AC0)
	// 82410A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410A78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410A7C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410A84: 409A000C  bne cr6, 0x82410a90
	if !ctx.cr[6].eq {
	pc = 0x82410A90; continue 'dispatch;
	}
	// 82410A88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410A8C: 48000020  b 0x82410aac
	pc = 0x82410AAC; continue 'dispatch;
	// 82410A90: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410A94: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410A98: 48000899  bl 0x82411330
	ctx.lr = 0x82410A9C;
	sub_82411330(ctx, base);
	// 82410A9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410AA0: 4182000C  beq 0x82410aac
	if ctx.cr[0].eq {
	pc = 0x82410AAC; continue 'dispatch;
	}
	// 82410AA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82410AA8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82410AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410AC0 size=356
    //   switch @ 0x82410B24: r3 with 13 label(s)
    //       case  0 → 0x82410B34
    //       case  1 → 0x82410B40
    //       case  2 → 0x82410B54
    //       case  3 → 0x82410B84
    //       case  4 → 0x82410B60
    //       case  5 → 0x82410B78
    //       case  6 → 0x82410B70
    //       case  7 → 0x82410B78
    //       case  8 → 0x82410B8C
    //       case  9 → 0x82410B60
    //       case 10 → 0x82410B78
    //       case 11 → 0x82410B60
    //       case 12 → 0x82410B78
    let mut pc: u32 = 0x82410AC0;
    'dispatch: loop {
        match pc {
            0x82410AC0 => {
    //   block [0x82410AC0..0x82410B34)
	// 82410AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410AC4: 481245F9  bl 0x825350bc
	ctx.lr = 0x82410AC8;
	sub_82535080(ctx, base);
	// 82410AC8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82410ACC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410AD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410AD4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410AD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410ADC: 409A0014  bne cr6, 0x82410af0
	if !ctx.cr[6].eq {
	pc = 0x82410AF0; continue 'dispatch;
	}
	// 82410AE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82410AE8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82410AEC: 48124620  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82410AF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410AF4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82410AF8: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82410AFC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410B00: 48000579  bl 0x82411078
	ctx.lr = 0x82410B04;
	sub_82411078(ctx, base);
	// 82410B04: 2B03000C  cmplwi cr6, r3, 0xc
	ctx.cr[6].compare_u32(ctx.r[3].u32, 12 as u32, &mut ctx.xer);
	// 82410B08: 41990118  bgt cr6, 0x82410c20
	if ctx.cr[6].gt {
	pc = 0x82410C20; continue 'dispatch;
	}
	// 82410B0C: 3D808201  lis r12, -0x7dff
	ctx.r[12].s64 = -2113863680;
	// 82410B10: 398CEB40  addi r12, r12, -0x14c0
	ctx.r[12].s64 = ctx.r[12].s64 + -5312;
	// 82410B14: 7C0C18AE  lbzx r0, r12, r3
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82410B18: 3D808241  lis r12, -0x7dbf
	ctx.r[12].s64 = -2109669376;
	// 82410B1C: 398C0B34  addi r12, r12, 0xb34
	ctx.r[12].s64 = ctx.r[12].s64 + 2868;
	// 82410B20: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82410B24: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82410B28: 60000000  nop
	// 82410B2C: 60000000  nop
	// 82410B30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82410B34 => {
    //   block [0x82410B34..0x82410B40)
	// 82410B34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82410B38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410B3C: 48000060  b 0x82410b9c
	pc = 0x82410B9C; continue 'dispatch;
            }
            0x82410B40 => {
    //   block [0x82410B40..0x82410B54)
	// 82410B40: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B44: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82410B48: 48000E59  bl 0x824119a0
	ctx.lr = 0x82410B4C;
	sub_824119A0(ctx, base);
	// 82410B4C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410B50: 48000048  b 0x82410b98
	pc = 0x82410B98; continue 'dispatch;
            }
            0x82410B54 => {
    //   block [0x82410B54..0x82410B60)
	// 82410B54: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B58: 48000AC9  bl 0x82411620
	ctx.lr = 0x82410B5C;
	sub_82411620(ctx, base);
	// 82410B5C: 4BFFFFF0  b 0x82410b4c
	pc = 0x82410B4C; continue 'dispatch;
            }
            0x82410B60 => {
    //   block [0x82410B60..0x82410B70)
	// 82410B60: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82410B64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B68: 48000BD9  bl 0x82411740
	ctx.lr = 0x82410B6C;
	sub_82411740(ctx, base);
	// 82410B6C: 4800002C  b 0x82410b98
	pc = 0x82410B98; continue 'dispatch;
            }
            0x82410B70 => {
    //   block [0x82410B70..0x82410B78)
	// 82410B70: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82410B74: 4BFFFFF0  b 0x82410b64
	pc = 0x82410B64; continue 'dispatch;
            }
            0x82410B78 => {
    //   block [0x82410B78..0x82410B84)
	// 82410B78: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B7C: 48000C95  bl 0x82411810
	ctx.lr = 0x82410B80;
	sub_82411810(ctx, base);
	// 82410B80: 4BFFFFCC  b 0x82410b4c
	pc = 0x82410B4C; continue 'dispatch;
            }
            0x82410B84 => {
    //   block [0x82410B84..0x82410B8C)
	// 82410B84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410B88: 48000008  b 0x82410b90
	pc = 0x82410B90; continue 'dispatch;
            }
            0x82410B8C => {
    //   block [0x82410B8C..0x82410C24)
	// 82410B8C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82410B90: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B94: 4800098D  bl 0x82411520
	ctx.lr = 0x82410B98;
	sub_82411520(ctx, base);
	// 82410B98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82410B9C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410BA0: 480004D9  bl 0x82411078
	ctx.lr = 0x82410BA4;
	sub_82411078(ctx, base);
	// 82410BA4: 2B030005  cmplwi cr6, r3, 5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 5 as u32, &mut ctx.xer);
	// 82410BA8: 419A0014  beq cr6, 0x82410bbc
	if ctx.cr[6].eq {
	pc = 0x82410BBC; continue 'dispatch;
	}
	// 82410BAC: 2B03000A  cmplwi cr6, r3, 0xa
	ctx.cr[6].compare_u32(ctx.r[3].u32, 10 as u32, &mut ctx.xer);
	// 82410BB0: 419A000C  beq cr6, 0x82410bbc
	if ctx.cr[6].eq {
	pc = 0x82410BBC; continue 'dispatch;
	}
	// 82410BB4: 2B03000C  cmplwi cr6, r3, 0xc
	ctx.cr[6].compare_u32(ctx.r[3].u32, 12 as u32, &mut ctx.xer);
	// 82410BB8: 409A0038  bne cr6, 0x82410bf0
	if !ctx.cr[6].eq {
	pc = 0x82410BF0; continue 'dispatch;
	}
	// 82410BBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82410BC0: 419A0030  beq cr6, 0x82410bf0
	if ctx.cr[6].eq {
	pc = 0x82410BF0; continue 'dispatch;
	}
	// 82410BC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410BCC: 409A000C  bne cr6, 0x82410bd8
	if !ctx.cr[6].eq {
	pc = 0x82410BD8; continue 'dispatch;
	}
	// 82410BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410BD4: 48000010  b 0x82410be4
	pc = 0x82410BE4; continue 'dispatch;
	// 82410BD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410BDC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410BE0: 48000751  bl 0x82411330
	ctx.lr = 0x82410BE4;
	sub_82411330(ctx, base);
	// 82410BE4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82410BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82410BEC: 419A0030  beq cr6, 0x82410c1c
	if ctx.cr[6].eq {
	pc = 0x82410C1C; continue 'dispatch;
	}
	// 82410BF0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410BF4: 480004B5  bl 0x824110a8
	ctx.lr = 0x82410BF8;
	sub_824110A8(ctx, base);
	// 82410BF8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82410BFC: 409AFF00  bne cr6, 0x82410afc
	if !ctx.cr[6].eq {
	pc = 0x82410AFC; continue 'dispatch;
	}
	// 82410C00: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410C04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410C08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82410C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410C10: 4E800421  bctrl
	ctx.lr = 0x82410C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410C14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410C18: 4BFFFECC  b 0x82410ae4
	pc = 0x82410AE4; continue 'dispatch;
	// 82410C1C: 48000000  b 0x82410c1c
	pc = 0x82410C1C; continue 'dispatch;
	// 82410C20: 48000000  b 0x82410c20
	pc = 0x82410C20; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410C28 size=104
    let mut pc: u32 = 0x82410C28;
    'dispatch: loop {
        match pc {
            0x82410C28 => {
    //   block [0x82410C28..0x82410C90)
	// 82410C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410C40: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82410C44: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410C4C: 409A000C  bne cr6, 0x82410c58
	if !ctx.cr[6].eq {
	pc = 0x82410C58; continue 'dispatch;
	}
	// 82410C50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410C54: 48000024  b 0x82410c78
	pc = 0x82410C78; continue 'dispatch;
	// 82410C58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410C5C: 4BFFFB7D  bl 0x824107d8
	ctx.lr = 0x82410C60;
	sub_824107D8(ctx, base);
	// 82410C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410C64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410C68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410C6C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410C70: 480006D1  bl 0x82411340
	ctx.lr = 0x82410C74;
	sub_82411340(ctx, base);
	// 82410C74: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410C90 size=84
    let mut pc: u32 = 0x82410C90;
    'dispatch: loop {
        match pc {
            0x82410C90 => {
    //   block [0x82410C90..0x82410CE4)
	// 82410C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410CA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410CA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410CAC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410CB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410CB4: 41820014  beq 0x82410cc8
	if ctx.cr[0].eq {
	pc = 0x82410CC8; continue 'dispatch;
	}
	// 82410CB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410CBC: 4BFFFB6D  bl 0x82410828
	ctx.lr = 0x82410CC0;
	sub_82410828(ctx, base);
	// 82410CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410CC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410CC8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82410CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410CE8 size=132
    let mut pc: u32 = 0x82410CE8;
    'dispatch: loop {
        match pc {
            0x82410CE8 => {
    //   block [0x82410CE8..0x82410D6C)
	// 82410CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410CEC: 481243D1  bl 0x825350bc
	ctx.lr = 0x82410CF0;
	sub_82535080(ctx, base);
	// 82410CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410CF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410CFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82410D00: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 82410D04: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410D08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82410D0C: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82410D10: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82410D14: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82410D18: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82410D1C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82410D20: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410D24: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82410D28: 4BFFFA99  bl 0x824107c0
	ctx.lr = 0x82410D2C;
	sub_824107C0(ctx, base);
	// 82410D2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410D30: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82410D34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410D38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410D3C: 4E800421  bctrl
	ctx.lr = 0x82410D40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410D40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410D44: 41820010  beq 0x82410d54
	if ctx.cr[0].eq {
	pc = 0x82410D54; continue 'dispatch;
	}
	// 82410D48: 480006D9  bl 0x82411420
	ctx.lr = 0x82410D4C;
	sub_82411420(ctx, base);
	// 82410D4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82410D50: 48000008  b 0x82410d58
	pc = 0x82410D58; continue 'dispatch;
	// 82410D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410D58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82410D5C: 4BFFFF35  bl 0x82410c90
	ctx.lr = 0x82410D60;
	sub_82410C90(ctx, base);
	// 82410D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82410D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410D68: 481243A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410D70 size=80
    let mut pc: u32 = 0x82410D70;
    'dispatch: loop {
        match pc {
            0x82410D70 => {
    //   block [0x82410D70..0x82410DC0)
	// 82410D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410D7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410D84: 4BFFFEA5  bl 0x82410c28
	ctx.lr = 0x82410D88;
	sub_82410C28(ctx, base);
	// 82410D88: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410D90: 41820014  beq 0x82410da4
	if ctx.cr[0].eq {
	pc = 0x82410DA4; continue 'dispatch;
	}
	// 82410D94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410D98: 4BFFFA91  bl 0x82410828
	ctx.lr = 0x82410D9C;
	sub_82410828(ctx, base);
	// 82410D9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410DA0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82410DA4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82410DA8: 4BFFFA31  bl 0x824107d8
	ctx.lr = 0x82410DAC;
	sub_824107D8(ctx, base);
	// 82410DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410DC0 size=136
    let mut pc: u32 = 0x82410DC0;
    'dispatch: loop {
        match pc {
            0x82410DC0 => {
    //   block [0x82410DC0..0x82410E48)
	// 82410DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410DC4: 481242F9  bl 0x825350bc
	ctx.lr = 0x82410DC8;
	sub_82535080(ctx, base);
	// 82410DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410DCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410DD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410DD4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82410DD8: 4BFFFE51  bl 0x82410c28
	ctx.lr = 0x82410DDC;
	sub_82410C28(ctx, base);
	// 82410DDC: 4BFFF9E5  bl 0x824107c0
	ctx.lr = 0x82410DE0;
	sub_824107C0(ctx, base);
	// 82410DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410DE4: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 82410DE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410DF0: 4E800421  bctrl
	ctx.lr = 0x82410DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410DF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410DF8: 41820024  beq 0x82410e1c
	if ctx.cr[0].eq {
	pc = 0x82410E1C; continue 'dispatch;
	}
	// 82410DFC: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82410E00: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82410E04: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82410E08: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410E0C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410E10: 48000FB9  bl 0x82411dc8
	ctx.lr = 0x82410E14;
	sub_82411DC8(ctx, base);
	// 82410E14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82410E18: 48000008  b 0x82410e20
	pc = 0x82410E20; continue 'dispatch;
	// 82410E1C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410E20: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82410E24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410E28: 4BFFF9B1  bl 0x824107d8
	ctx.lr = 0x82410E2C;
	sub_824107D8(ctx, base);
	// 82410E2C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82410E30: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410E34: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82410E38: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82410E3C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82410E40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410E44: 481242C8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410E48 size=128
    let mut pc: u32 = 0x82410E48;
    'dispatch: loop {
        match pc {
            0x82410E48 => {
    //   block [0x82410E48..0x82410EC8)
	// 82410E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410E58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410E60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82410E64: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82410E68: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82410E6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410E70: 48123CE1  bl 0x82534b50
	ctx.lr = 0x82410E74;
	sub_82534B50(ctx, base);
	// 82410E74: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410E7C: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410E80: 41800030  blt 0x82410eb0
	if ctx.cr[0].lt {
	pc = 0x82410EB0; continue 'dispatch;
	}
	// 82410E84: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82410E88: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82410E8C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82410E90: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82410E94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82410E98: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82410E9C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82410EA0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82410EA4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410EA8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82410EAC: 4099FFDC  ble cr6, 0x82410e88
	if !ctx.cr[6].gt {
	pc = 0x82410E88; continue 'dispatch;
	}
	// 82410EB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82410EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410EBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410EC8 size=24
    let mut pc: u32 = 0x82410EC8;
    'dispatch: loop {
        match pc {
            0x82410EC8 => {
    //   block [0x82410EC8..0x82410EE0)
	// 82410EC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82410ECC: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410ED0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410ED4: 409A000C  bne cr6, 0x82410ee0
	if !ctx.cr[6].eq {
		sub_82410EE0(ctx, base);
		return;
	}
	// 82410ED8: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82410EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410EE0 size=24
    let mut pc: u32 = 0x82410EE0;
    'dispatch: loop {
        match pc {
            0x82410EE0 => {
    //   block [0x82410EE0..0x82410EF8)
	// 82410EE0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410EE4: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410EE8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410EEC: 409A000C  bne cr6, 0x82410ef8
	if !ctx.cr[6].eq {
		sub_82410EF8(ctx, base);
		return;
	}
	// 82410EF0: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82410EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410EF8 size=16
    let mut pc: u32 = 0x82410EF8;
    'dispatch: loop {
        match pc {
            0x82410EF8 => {
    //   block [0x82410EF8..0x82410F08)
	// 82410EF8: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410EFC: 4099000C  ble cr6, 0x82410f08
	if !ctx.cr[6].gt {
		sub_82410F08(ctx, base);
		return;
	}
	// 82410F00: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82410F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410F08 size=12
    let mut pc: u32 = 0x82410F08;
    'dispatch: loop {
        match pc {
            0x82410F08 => {
    //   block [0x82410F08..0x82410F14)
	// 82410F08: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410F0C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82410F10: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410F14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410F14 size=8
    let mut pc: u32 = 0x82410F14;
    'dispatch: loop {
        match pc {
            0x82410F14 => {
    //   block [0x82410F14..0x82410F1C)
	// 82410F14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410F20 size=340
    let mut pc: u32 = 0x82410F20;
    'dispatch: loop {
        match pc {
            0x82410F20 => {
    //   block [0x82410F20..0x82411074)
	// 82410F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410F24: 4812418D  bl 0x825350b0
	ctx.lr = 0x82410F28;
	sub_82535080(ctx, base);
	// 82410F28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410F2C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F30: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82410F34: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82410F38: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82410F3C: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82410F40: 409A003C  bne cr6, 0x82410f7c
	if !ctx.cr[6].eq {
	pc = 0x82410F7C; continue 'dispatch;
	}
	// 82410F44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F48: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82410F4C: 40980008  bge cr6, 0x82410f54
	if !ctx.cr[6].lt {
	pc = 0x82410F54; continue 'dispatch;
	}
	// 82410F50: 48000000  b 0x82410f50
	pc = 0x82410F50; continue 'dispatch;
	// 82410F54: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82410F58: 41980008  blt cr6, 0x82410f60
	if ctx.cr[6].lt {
	pc = 0x82410F60; continue 'dispatch;
	}
	// 82410F5C: 48000000  b 0x82410f5c
	pc = 0x82410F5C; continue 'dispatch;
	// 82410F60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82410F64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410F68: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82410F6C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82410F70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410F74: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82410F78: 480000F4  b 0x8241106c
	pc = 0x8241106C; continue 'dispatch;
	// 82410F7C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82410F84: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82410F88: 41980028  blt cr6, 0x82410fb0
	if ctx.cr[6].lt {
	pc = 0x82410FB0; continue 'dispatch;
	}
	// 82410F8C: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82410F90: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F94: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82410F98: 419A00D0  beq cr6, 0x82411068
	if ctx.cr[6].eq {
	pc = 0x82411068; continue 'dispatch;
	}
	// 82410F9C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410FA0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82410FA4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82410FA8: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82410FAC: 4099FFE4  ble cr6, 0x82410f90
	if !ctx.cr[6].gt {
	pc = 0x82410F90; continue 'dispatch;
	}
	// 82410FB0: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82410FB4: 40980008  bge cr6, 0x82410fbc
	if !ctx.cr[6].lt {
	pc = 0x82410FBC; continue 'dispatch;
	}
	// 82410FB8: 48000000  b 0x82410fb8
	pc = 0x82410FB8; continue 'dispatch;
	// 82410FBC: 2F09000C  cmpwi cr6, r9, 0xc
	ctx.cr[6].compare_i32(ctx.r[9].s32, 12, &mut ctx.xer);
	// 82410FC0: 41980008  blt cr6, 0x82410fc8
	if ctx.cr[6].lt {
	pc = 0x82410FC8; continue 'dispatch;
	}
	// 82410FC4: 48000000  b 0x82410fc4
	pc = 0x82410FC4; continue 'dispatch;
	// 82410FC8: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82410FCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410FD0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82410FD4: 3BC50004  addi r30, r5, 4
	ctx.r[30].s64 = ctx.r[5].s64 + 4;
	// 82410FD8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82410FDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410FE0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82410FE4: 3B8B3920  addi r28, r11, 0x3920
	ctx.r[28].s64 = ctx.r[11].s64 + 14624;
	// 82410FE8: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82410FEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410FF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410FF4: 41820034  beq 0x82411028
	if ctx.cr[0].eq {
	pc = 0x82411028; continue 'dispatch;
	}
	// 82410FF8: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 82410FFC: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82411000: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82411004: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82411008: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8241100C: 4BFFFF15  bl 0x82410f20
	ctx.lr = 0x82411010;
	sub_82410F20(ctx, base);
	// 82411010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411014: 40820024  bne 0x82411038
	if !ctx.cr[0].eq {
	pc = 0x82411038; continue 'dispatch;
	}
	// 82411018: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8241101C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82411020: 2B1D000D  cmplwi cr6, r29, 0xd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 13 as u32, &mut ctx.xer);
	// 82411024: 4198FFC8  blt cr6, 0x82410fec
	if ctx.cr[6].lt {
	pc = 0x82410FEC; continue 'dispatch;
	}
	// 82411028: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241102C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82411030: 40800010  bge 0x82411040
	if !ctx.cr[0].lt {
	pc = 0x82411040; continue 'dispatch;
	}
	// 82411034: 48000000  b 0x82411034
	pc = 0x82411034; continue 'dispatch;
	// 82411038: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241103C: 48000030  b 0x8241106c
	pc = 0x8241106C; continue 'dispatch;
	// 82411040: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 82411044: 41980008  blt cr6, 0x8241104c
	if ctx.cr[6].lt {
	pc = 0x8241104C; continue 'dispatch;
	}
	// 82411048: 48000000  b 0x82411048
	pc = 0x82411048; continue 'dispatch;
	// 8241104C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82411054: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82411058: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8241105C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411060: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82411064: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241106C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82411070: 48124090  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411078 size=20
    let mut pc: u32 = 0x82411078;
    'dispatch: loop {
        match pc {
            0x82411078 => {
    //   block [0x82411078..0x8241108C)
	// 82411078: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241107C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411080: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82411084: 40800008  bge 0x8241108c
	if !ctx.cr[0].lt {
		sub_8241108C(ctx, base);
		return;
	}
	// 82411088: 48000000  b 0x82411088
	pc = 0x82411088; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8241108C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8241108C size=12
    let mut pc: u32 = 0x8241108C;
    'dispatch: loop {
        match pc {
            0x8241108C => {
    //   block [0x8241108C..0x82411098)
	// 8241108C: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82411090: 40990008  ble cr6, 0x82411098
	if !ctx.cr[6].gt {
		sub_82411098(ctx, base);
		return;
	}
	// 82411094: 48000000  b 0x82411094
	pc = 0x82411094; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411098 size=16
    let mut pc: u32 = 0x82411098;
    'dispatch: loop {
        match pc {
            0x82411098 => {
    //   block [0x82411098..0x824110A8)
	// 82411098: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8241109C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824110A0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824110A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824110A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824110A8 size=32
    let mut pc: u32 = 0x824110A8;
    'dispatch: loop {
        match pc {
            0x824110A8 => {
    //   block [0x824110A8..0x824110C8)
	// 824110A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824110AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824110B0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824110B4: 41980044  blt cr6, 0x824110f8
	if ctx.cr[6].lt {
		sub_824110F8(ctx, base);
		return;
	}
	// 824110B8: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824110BC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824110C0: 40800008  bge 0x824110c8
	if !ctx.cr[0].lt {
		sub_824110C8(ctx, base);
		return;
	}
	// 824110C4: 48000000  b 0x824110c4
	pc = 0x824110C4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824110C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824110C8 size=12
    let mut pc: u32 = 0x824110C8;
    'dispatch: loop {
        match pc {
            0x824110C8 => {
    //   block [0x824110C8..0x824110D4)
	// 824110C8: 2F0A000D  cmpwi cr6, r10, 0xd
	ctx.cr[6].compare_i32(ctx.r[10].s32, 13, &mut ctx.xer);
	// 824110CC: 41980008  blt cr6, 0x824110d4
	if ctx.cr[6].lt {
		sub_824110D4(ctx, base);
		return;
	}
	// 824110D0: 48000000  b 0x824110d0
	pc = 0x824110D0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824110D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824110D4 size=36
    let mut pc: u32 = 0x824110D4;
    'dispatch: loop {
        match pc {
            0x824110D4 => {
    //   block [0x824110D4..0x824110F8)
	// 824110D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824110D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824110DC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824110E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824110E4: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 824110E8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824110EC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824110F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824110F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824110F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824110F8 size=8
    let mut pc: u32 = 0x824110F8;
    'dispatch: loop {
        match pc {
            0x824110F8 => {
    //   block [0x824110F8..0x82411100)
	// 824110F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824110FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411100 size=556
    let mut pc: u32 = 0x82411100;
    'dispatch: loop {
        match pc {
            0x82411100 => {
    //   block [0x82411100..0x8241132C)
	// 82411100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411104: 48123FA1  bl 0x825350a4
	ctx.lr = 0x82411108;
	sub_82535080(ctx, base);
	// 82411108: 9421FC80  stwu r1, -0x380(r1)
	ea = ctx.r[1].u32.wrapping_add(-896 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241110C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82411110: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82411114: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82411118: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8241111C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82411120: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82411124: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82411128: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8241112C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82411130: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82411134: 4200FFF8  bdnz 0x8241112c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8241112C; continue 'dispatch;
	}
	// 82411138: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8241113C: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82411140: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82411144: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82411148: 4080FFD4  bge 0x8241111c
	if !ctx.cr[0].lt {
	pc = 0x8241111C; continue 'dispatch;
	}
	// 8241114C: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411150: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82411154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82411158: 3B8B3920  addi r28, r11, 0x3920
	ctx.r[28].s64 = ctx.r[11].s64 + 14624;
	// 8241115C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411160: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82411164: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411168: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8241116C: 41980040  blt cr6, 0x824111ac
	if ctx.cr[6].lt {
	pc = 0x824111AC; continue 'dispatch;
	}
	// 82411170: 2F07000C  cmpwi cr6, r7, 0xc
	ctx.cr[6].compare_i32(ctx.r[7].s32, 12, &mut ctx.xer);
	// 82411174: 41990034  bgt cr6, 0x824111a8
	if ctx.cr[6].gt {
	pc = 0x824111A8; continue 'dispatch;
	}
	// 82411178: 38C70001  addi r6, r7, 1
	ctx.r[6].s64 = ctx.r[7].s64 + 1;
	// 8241117C: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411180: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82411184: 7CC6482E  lwzx r6, r6, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82411188: 7F062840  cmplw cr6, r6, r5
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8241118C: 419A0024  beq cr6, 0x824111b0
	if ctx.cr[6].eq {
	pc = 0x824111B0; continue 'dispatch;
	}
	// 82411190: 39080038  addi r8, r8, 0x38
	ctx.r[8].s64 = ctx.r[8].s64 + 56;
	// 82411194: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411198: 394A0038  addi r10, r10, 0x38
	ctx.r[10].s64 = ctx.r[10].s64 + 56;
	// 8241119C: 2B0802D8  cmplwi cr6, r8, 0x2d8
	ctx.cr[6].compare_u32(ctx.r[8].u32, 728 as u32, &mut ctx.xer);
	// 824111A0: 4198FFC8  blt cr6, 0x82411168
	if ctx.cr[6].lt {
	pc = 0x82411168; continue 'dispatch;
	}
	// 824111A4: 48000000  b 0x824111a4
	pc = 0x824111A4; continue 'dispatch;
	// 824111A8: 48000000  b 0x824111a8
	pc = 0x824111A8; continue 'dispatch;
	// 824111AC: 48000000  b 0x824111ac
	pc = 0x824111AC; continue 'dispatch;
	// 824111B0: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 824111B4: 7FABE214  add r29, r11, r28
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824111B8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824111BC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824111C0: 3B7D0004  addi r27, r29, 4
	ctx.r[27].s64 = ctx.r[29].s64 + 4;
	// 824111C4: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 824111C8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 824111CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824111D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824111D4: 419A00A0  beq cr6, 0x82411274
	if ctx.cr[6].eq {
	pc = 0x82411274; continue 'dispatch;
	}
	// 824111D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824111DC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 824111E0: 419800B4  blt cr6, 0x82411294
	if ctx.cr[6].lt {
	pc = 0x82411294; continue 'dispatch;
	}
	// 824111E4: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 824111E8: 409800A8  bge cr6, 0x82411290
	if !ctx.cr[6].lt {
	pc = 0x82411290; continue 'dispatch;
	}
	// 824111EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824111F0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824111F4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824111F8: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 824111FC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82411200: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82411204: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411208: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8241120C: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82411210: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411214: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 82411218: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8241121C: 4BFFFD05  bl 0x82410f20
	ctx.lr = 0x82411220;
	sub_82410F20(ctx, base);
	// 82411220: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411224: 4182000C  beq 0x82411230
	if ctx.cr[0].eq {
	pc = 0x82411230; continue 'dispatch;
	}
	// 82411228: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8241122C: 48000034  b 0x82411260
	pc = 0x82411260; continue 'dispatch;
	// 82411230: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411234: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82411238: 41800054  blt 0x8241128c
	if ctx.cr[0].lt {
	pc = 0x8241128C; continue 'dispatch;
	}
	// 8241123C: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 82411240: 40980048  bge cr6, 0x82411288
	if !ctx.cr[6].lt {
	pc = 0x82411288; continue 'dispatch;
	}
	// 82411244: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8241124C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82411250: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82411254: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411258: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8241125C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411260: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82411264: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82411268: 3BFF0038  addi r31, r31, 0x38
	ctx.r[31].s64 = ctx.r[31].s64 + 56;
	// 8241126C: 2B1A000D  cmplwi cr6, r26, 0xd
	ctx.cr[6].compare_u32(ctx.r[26].u32, 13 as u32, &mut ctx.xer);
	// 82411270: 4198FF5C  blt cr6, 0x824111cc
	if ctx.cr[6].lt {
	pc = 0x824111CC; continue 'dispatch;
	}
	// 82411274: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82411278: 409A0020  bne cr6, 0x82411298
	if !ctx.cr[6].eq {
	pc = 0x82411298; continue 'dispatch;
	}
	// 8241127C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82411280: 38210380  addi r1, r1, 0x380
	ctx.r[1].s64 = ctx.r[1].s64 + 896;
	// 82411284: 48123E70  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 82411288: 48000000  b 0x82411288
	pc = 0x82411288; continue 'dispatch;
	// 8241128C: 48000000  b 0x8241128c
	pc = 0x8241128C; continue 'dispatch;
	// 82411290: 48000000  b 0x82411290
	pc = 0x82411290; continue 'dispatch;
	// 82411294: 48000000  b 0x82411294
	pc = 0x82411294; continue 'dispatch;
	// 82411298: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8241129C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 824112A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824112A4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824112A8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824112AC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824112B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824112B4: 419A0034  beq cr6, 0x824112e8
	if ctx.cr[6].eq {
	pc = 0x824112E8; continue 'dispatch;
	}
	// 824112B8: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824112BC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824112C0: 41800014  blt 0x824112d4
	if ctx.cr[0].lt {
	pc = 0x824112D4; continue 'dispatch;
	}
	// 824112C4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 824112C8: 4098000C  bge cr6, 0x824112d4
	if !ctx.cr[6].lt {
	pc = 0x824112D4; continue 'dispatch;
	}
	// 824112CC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824112D0: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 824112D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824112D8: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 824112DC: 39290038  addi r9, r9, 0x38
	ctx.r[9].s64 = ctx.r[9].s64 + 56;
	// 824112E0: 2B0B000D  cmplwi cr6, r11, 0xd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 13 as u32, &mut ctx.xer);
	// 824112E4: 4198FFC8  blt cr6, 0x824112ac
	if ctx.cr[6].lt {
	pc = 0x824112AC; continue 'dispatch;
	}
	// 824112E8: 2F08000D  cmpwi cr6, r8, 0xd
	ctx.cr[6].compare_i32(ctx.r[8].s32, 13, &mut ctx.xer);
	// 824112EC: 409A0008  bne cr6, 0x824112f4
	if !ctx.cr[6].eq {
	pc = 0x824112F4; continue 'dispatch;
	}
	// 824112F0: 48000000  b 0x824112f0
	pc = 0x824112F0; continue 'dispatch;
	// 824112F4: 2B07000D  cmplwi cr6, r7, 0xd
	ctx.cr[6].compare_u32(ctx.r[7].u32, 13 as u32, &mut ctx.xer);
	// 824112F8: 409A0008  bne cr6, 0x82411300
	if !ctx.cr[6].eq {
	pc = 0x82411300; continue 'dispatch;
	}
	// 824112FC: 48000000  b 0x824112fc
	pc = 0x824112FC; continue 'dispatch;
	// 82411300: 1D670038  mulli r11, r7, 0x38
	ctx.r[11].s64 = ctx.r[7].s64 * 56;
	// 82411304: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82411308: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8241130C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411310: 4BFFFB39  bl 0x82410e48
	ctx.lr = 0x82411314;
	sub_82410E48(ctx, base);
	// 82411314: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82411318: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241131C: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411320: 48123831  bl 0x82534b50
	ctx.lr = 0x82411324;
	sub_82534B50(ctx, base);
	// 82411324: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411328: 4BFFFF58  b 0x82411280
	pc = 0x82411280; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411330 size=4
    let mut pc: u32 = 0x82411330;
    'dispatch: loop {
        match pc {
            0x82411330 => {
    //   block [0x82411330..0x82411334)
	// 82411330: 4BFFFDD0  b 0x82411100
	sub_82411100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411338 size=4
    let mut pc: u32 = 0x82411338;
    'dispatch: loop {
        match pc {
            0x82411338 => {
    //   block [0x82411338..0x8241133C)
	// 82411338: 48003DE0  b 0x82415118
	sub_82415118(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411340 size=220
    let mut pc: u32 = 0x82411340;
    'dispatch: loop {
        match pc {
            0x82411340 => {
    //   block [0x82411340..0x8241141C)
	// 82411340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411344: 48123D75  bl 0x825350b8
	ctx.lr = 0x82411348;
	sub_82535080(ctx, base);
	// 82411348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241134C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411350: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82411354: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82411358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241135C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411360: 409A005C  bne cr6, 0x824113bc
	if !ctx.cr[6].eq {
	pc = 0x824113BC; continue 'dispatch;
	}
	// 82411364: 4BFFF45D  bl 0x824107c0
	ctx.lr = 0x82411368;
	sub_824107C0(ctx, base);
	// 82411368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241136C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82411370: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82411374: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411378: 4E800421  bctrl
	ctx.lr = 0x8241137C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241137C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411380: 4182002C  beq 0x824113ac
	if ctx.cr[0].eq {
	pc = 0x824113AC; continue 'dispatch;
	}
	// 82411384: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82411388: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8241138C: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 82411390: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82411394: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82411398: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8241139C: 4200FFF8  bdnz 0x82411394
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82411394; continue 'dispatch;
	}
	// 824113A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824113A4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824113A8: 48000008  b 0x824113b0
	pc = 0x824113B0; continue 'dispatch;
	// 824113AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824113B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824113B4: 48003D65  bl 0x82415118
	ctx.lr = 0x824113B8;
	sub_82415118(ctx, base);
	// 824113B8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824113BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824113C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824113C4: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 824113C8: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 824113CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824113D0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824113D4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824113D8: 4200FFF8  bdnz 0x824113d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824113D0; continue 'dispatch;
	}
	// 824113DC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824113E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824113E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824113E8: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 824113EC: 40980008  bge cr6, 0x824113f4
	if !ctx.cr[6].lt {
	pc = 0x824113F4; continue 'dispatch;
	}
	// 824113F0: 48000000  b 0x824113f0
	pc = 0x824113F0; continue 'dispatch;
	// 824113F4: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 824113F8: 41980008  blt cr6, 0x82411400
	if ctx.cr[6].lt {
	pc = 0x82411400; continue 'dispatch;
	}
	// 824113FC: 48000000  b 0x824113fc
	pc = 0x824113FC; continue 'dispatch;
	// 82411400: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82411404: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82411408: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8241140C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82411410: 7F89592E  stwx r28, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82411414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82411418: 48123CF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411420 size=60
    let mut pc: u32 = 0x82411420;
    'dispatch: loop {
        match pc {
            0x82411420 => {
    //   block [0x82411420..0x8241145C)
	// 82411420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241142C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411438: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241143C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411440: 4BFFFF01  bl 0x82411340
	ctx.lr = 0x82411444;
	sub_82411340(ctx, base);
	// 82411444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411460 size=68
    let mut pc: u32 = 0x82411460;
    'dispatch: loop {
        match pc {
            0x82411460 => {
    //   block [0x82411460..0x824114A4)
	// 82411460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241146C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411470: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411478: 396BEB50  addi r11, r11, -0x14b0
	ctx.r[11].s64 = ctx.r[11].s64 + -5296;
	// 8241147C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82411480: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411484: 41820008  beq 0x8241148c
	if ctx.cr[0].eq {
	pc = 0x8241148C; continue 'dispatch;
	}
	// 82411488: 48121731  bl 0x82532bb8
	ctx.lr = 0x8241148C;
	sub_82532BB8(ctx, base);
	// 8241148C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241149C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824114A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824114A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824114A8 size=68
    let mut pc: u32 = 0x824114A8;
    'dispatch: loop {
        match pc {
            0x824114A8 => {
    //   block [0x824114A8..0x824114EC)
	// 824114A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824114AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824114B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824114B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824114B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824114BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824114C0: 396BEB58  addi r11, r11, -0x14a8
	ctx.r[11].s64 = ctx.r[11].s64 + -5288;
	// 824114C4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824114C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824114CC: 41820008  beq 0x824114d4
	if ctx.cr[0].eq {
	pc = 0x824114D4; continue 'dispatch;
	}
	// 824114D0: 481216E9  bl 0x82532bb8
	ctx.lr = 0x824114D4;
	sub_82532BB8(ctx, base);
	// 824114D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824114D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824114DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824114E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824114E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824114E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824114F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824114F0 size=40
    let mut pc: u32 = 0x824114F0;
    'dispatch: loop {
        match pc {
            0x824114F0 => {
    //   block [0x824114F0..0x82411518)
	// 824114F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824114F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824114F8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824114FC: 396BEB6C  addi r11, r11, -0x1494
	ctx.r[11].s64 = ctx.r[11].s64 + -5268;
	// 82411500: 394AEB58  addi r10, r10, -0x14a8
	ctx.r[10].s64 = ctx.r[10].s64 + -5288;
	// 82411504: 3929EB50  addi r9, r9, -0x14b0
	ctx.r[9].s64 = ctx.r[9].s64 + -5296;
	// 82411508: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8241150C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411510: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82411514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411518 size=8
    let mut pc: u32 = 0x82411518;
    'dispatch: loop {
        match pc {
            0x82411518 => {
    //   block [0x82411518..0x82411520)
	// 82411518: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8241151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411520 size=136
    let mut pc: u32 = 0x82411520;
    'dispatch: loop {
        match pc {
            0x82411520 => {
    //   block [0x82411520..0x824115A8)
	// 82411520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411524: 48123B99  bl 0x825350bc
	ctx.lr = 0x82411528;
	sub_82535080(ctx, base);
	// 82411528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241152C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411534: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82411538: 409A0008  bne cr6, 0x82411540
	if !ctx.cr[6].eq {
	pc = 0x82411540; continue 'dispatch;
	}
	// 8241153C: 48000000  b 0x8241153c
	pc = 0x8241153C; continue 'dispatch;
	// 82411540: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82411544: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82411548: 419A0054  beq cr6, 0x8241159c
	if ctx.cr[6].eq {
	pc = 0x8241159C; continue 'dispatch;
	}
	// 8241154C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411550: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 82411554: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82411558: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8241155C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411560: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82411564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411568: 4E800421  bctrl
	ctx.lr = 0x8241156C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241156C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411570: 4182002C  beq 0x8241159c
	if ctx.cr[0].eq {
	pc = 0x8241159C; continue 'dispatch;
	}
	// 82411574: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411578: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241157C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82411580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411584: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241158C: 4E800421  bctrl
	ctx.lr = 0x82411590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411590: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411594: 4182000C  beq 0x824115a0
	if ctx.cr[0].eq {
	pc = 0x824115A0; continue 'dispatch;
	}
	// 82411598: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8241159C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824115A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824115A4: 48123B68  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824115A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824115A8 size=16
    let mut pc: u32 = 0x824115A8;
    'dispatch: loop {
        match pc {
            0x824115A8 => {
    //   block [0x824115A8..0x824115B8)
	// 824115A8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 824115AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824115B0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824115B4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824115B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824115B8 size=8
    let mut pc: u32 = 0x824115B8;
    'dispatch: loop {
        match pc {
            0x824115B8 => {
    //   block [0x824115B8..0x824115C0)
	// 824115B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824115BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824115C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824115C0 size=96
    let mut pc: u32 = 0x824115C0;
    'dispatch: loop {
        match pc {
            0x824115C0 => {
    //   block [0x824115C0..0x82411620)
	// 824115C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824115C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824115C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824115CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824115D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824115D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824115D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824115DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824115E0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824115E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824115E8: 4E800421  bctrl
	ctx.lr = 0x824115EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824115EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824115F0: 41820018  beq 0x82411608
	if ctx.cr[0].eq {
	pc = 0x82411608; continue 'dispatch;
	}
	// 824115F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824115F8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 824115FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411600: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82411604: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82411608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411614: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241161C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411620 size=288
    let mut pc: u32 = 0x82411620;
    'dispatch: loop {
        match pc {
            0x82411620 => {
    //   block [0x82411620..0x82411740)
	// 82411620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241162C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411638: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8241163C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411640: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411644: 41820028  beq 0x8241166c
	if ctx.cr[0].eq {
	pc = 0x8241166C; continue 'dispatch;
	}
	// 82411648: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8241164C: 419A0020  beq cr6, 0x8241166c
	if ctx.cr[6].eq {
	pc = 0x8241166C; continue 'dispatch;
	}
	// 82411650: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82411654: 419A0048  beq cr6, 0x8241169c
	if ctx.cr[6].eq {
	pc = 0x8241169C; continue 'dispatch;
	}
	// 82411658: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8241165C: 419A0080  beq cr6, 0x824116dc
	if ctx.cr[6].eq {
	pc = 0x824116DC; continue 'dispatch;
	}
	// 82411660: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82411664: 419A00BC  beq cr6, 0x82411720
	if ctx.cr[6].eq {
	pc = 0x82411720; continue 'dispatch;
	}
	// 82411668: 48000000  b 0x82411668
	pc = 0x82411668; continue 'dispatch;
	// 8241166C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82411670: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82411674: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82411678: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8241167C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411680: 409A001C  bne cr6, 0x8241169c
	if !ctx.cr[6].eq {
	pc = 0x8241169C; continue 'dispatch;
	}
	// 82411684: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241168C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82411690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411694: 4E800421  bctrl
	ctx.lr = 0x82411698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411698: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8241169C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824116A0: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824116A4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 824116A8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824116AC: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824116B0: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 824116B4: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824116B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824116BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824116C0: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824116C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824116C8: 4E800421  bctrl
	ctx.lr = 0x824116CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824116CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824116D0: 41820054  beq 0x82411724
	if ctx.cr[0].eq {
	pc = 0x82411724; continue 'dispatch;
	}
	// 824116D4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 824116D8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824116DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824116E0: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824116E4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 824116E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824116EC: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824116F0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824116F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824116F8: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824116FC: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411700: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82411704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411708: 4E800421  bctrl
	ctx.lr = 0x8241170C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241170C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411710: 41820014  beq 0x82411724
	if ctx.cr[0].eq {
	pc = 0x82411724; continue 'dispatch;
	}
	// 82411714: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82411718: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8241171C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411720: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82411724: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82411728: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8241172C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411734: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241173C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411740 size=204
    let mut pc: u32 = 0x82411740;
    'dispatch: loop {
        match pc {
            0x82411740 => {
    //   block [0x82411740..0x8241180C)
	// 82411740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8241174C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411750: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82411754: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241175C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82411760: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82411764: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411768: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 8241176C: 419A0020  beq cr6, 0x8241178c
	if ctx.cr[6].eq {
	pc = 0x8241178C; continue 'dispatch;
	}
	// 82411770: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82411774: 419A0044  beq cr6, 0x824117b8
	if ctx.cr[6].eq {
	pc = 0x824117B8; continue 'dispatch;
	}
	// 82411778: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8241177C: 419A006C  beq cr6, 0x824117e8
	if ctx.cr[6].eq {
	pc = 0x824117E8; continue 'dispatch;
	}
	// 82411780: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82411784: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82411788: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8241178C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411790: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82411794: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82411798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241179C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824117A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824117A4: 4E800421  bctrl
	ctx.lr = 0x824117A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824117A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824117AC: 41820040  beq 0x824117ec
	if ctx.cr[0].eq {
	pc = 0x824117EC; continue 'dispatch;
	}
	// 824117B0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 824117B4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824117B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824117BC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824117C0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824117C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824117C8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824117CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824117D0: 4E800421  bctrl
	ctx.lr = 0x824117D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824117D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824117D8: 41820014  beq 0x824117ec
	if ctx.cr[0].eq {
	pc = 0x824117EC; continue 'dispatch;
	}
	// 824117DC: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 824117E0: D3FF002C  stfs f31, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824117E4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824117E8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824117EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824117F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824117F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824117F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824117FC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82411800: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411810 size=132
    let mut pc: u32 = 0x82411810;
    'dispatch: loop {
        match pc {
            0x82411810 => {
    //   block [0x82411810..0x82411894)
	// 82411810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241181C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411824: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411828: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8241182C: 419A0010  beq cr6, 0x8241183c
	if ctx.cr[6].eq {
	pc = 0x8241183C; continue 'dispatch;
	}
	// 82411830: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82411834: 419A0018  beq cr6, 0x8241184c
	if ctx.cr[6].eq {
	pc = 0x8241184C; continue 'dispatch;
	}
	// 82411838: 48000000  b 0x82411838
	pc = 0x82411838; continue 'dispatch;
	// 8241183C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411840: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82411844: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82411848: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8241184C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411850: C05F002C  lfs f2, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82411854: C03F0024  lfs f1, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82411858: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8241185C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411860: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82411864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411868: 4E800421  bctrl
	ctx.lr = 0x8241186C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241186C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411874: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82411878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8241187C: 4E800421  bctrl
	ctx.lr = 0x82411880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241188C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82411898 size=24
    let mut pc: u32 = 0x82411898;
    'dispatch: loop {
        match pc {
            0x82411898 => {
    //   block [0x82411898..0x824118B0)
	// 82411898: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 8241189C: C0030024  lfs f0, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824118A0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 824118A4: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 824118A8: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 824118AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824118B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824118B0 size=16
    let mut pc: u32 = 0x824118B0;
    'dispatch: loop {
        match pc {
            0x824118B0 => {
    //   block [0x824118B0..0x824118C0)
	// 824118B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824118B4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 824118B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824118BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824118C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824118C0 size=8
    let mut pc: u32 = 0x824118C0;
    'dispatch: loop {
        match pc {
            0x824118C0 => {
    //   block [0x824118C0..0x824118C8)
	// 824118C0: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 824118C4: 4800006C  b 0x82411930
	sub_82411930(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824118C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824118C8 size=100
    let mut pc: u32 = 0x824118C8;
    'dispatch: loop {
        match pc {
            0x824118C8 => {
    //   block [0x824118C8..0x8241192C)
	// 824118C8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824118CC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824118D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824118D4: 396BEB58  addi r11, r11, -0x14a8
	ctx.r[11].s64 = ctx.r[11].s64 + -5288;
	// 824118D8: C00A1850  lfs f0, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824118DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824118E0: C1A91FF8  lfs f13, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824118E4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824118E8: 394AEB80  addi r10, r10, -0x1480
	ctx.r[10].s64 = ctx.r[10].s64 + -5248;
	// 824118EC: 3929EB6C  addi r9, r9, -0x1494
	ctx.r[9].s64 = ctx.r[9].s64 + -5268;
	// 824118F0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824118F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824118F8: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 824118FC: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82411900: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82411904: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82411908: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 8241190C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82411910: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82411914: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82411918: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8241191C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411920: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82411924: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82411928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411930 size=112
    let mut pc: u32 = 0x82411930;
    'dispatch: loop {
        match pc {
            0x82411930 => {
    //   block [0x82411930..0x824119A0)
	// 82411930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241193C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411940: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411944: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82411948: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8241194C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411950: 396BEB6C  addi r11, r11, -0x1494
	ctx.r[11].s64 = ctx.r[11].s64 + -5268;
	// 82411954: 394AEB58  addi r10, r10, -0x14a8
	ctx.r[10].s64 = ctx.r[10].s64 + -5288;
	// 82411958: 3929EB50  addi r9, r9, -0x14b0
	ctx.r[9].s64 = ctx.r[9].s64 + -5296;
	// 8241195C: 548807FF  clrlwi. r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82411960: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82411964: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411968: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8241196C: 4182001C  beq 0x82411988
	if ctx.cr[0].eq {
	pc = 0x82411988; continue 'dispatch;
	}
	// 82411970: 4BFFEE51  bl 0x824107c0
	ctx.lr = 0x82411974;
	sub_824107C0(ctx, base);
	// 82411974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411978: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241197C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411984: 4E800421  bctrl
	ctx.lr = 0x82411988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241198C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241199C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824119A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824119A0 size=232
    let mut pc: u32 = 0x824119A0;
    'dispatch: loop {
        match pc {
            0x824119A0 => {
    //   block [0x824119A0..0x82411A88)
	// 824119A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824119A4: 48123719  bl 0x825350bc
	ctx.lr = 0x824119A8;
	sub_82535080(ctx, base);
	// 824119A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824119AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824119B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824119B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824119B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824119BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824119C0: 41820038  beq 0x824119f8
	if ctx.cr[0].eq {
	pc = 0x824119F8; continue 'dispatch;
	}
	// 824119C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824119C8: 419A003C  beq cr6, 0x82411a04
	if ctx.cr[6].eq {
	pc = 0x82411A04; continue 'dispatch;
	}
	// 824119CC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824119D0: 419A005C  beq cr6, 0x82411a2c
	if ctx.cr[6].eq {
	pc = 0x82411A2C; continue 'dispatch;
	}
	// 824119D4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824119D8: 419A00A0  beq cr6, 0x82411a78
	if ctx.cr[6].eq {
	pc = 0x82411A78; continue 'dispatch;
	}
	// 824119DC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 824119E0: 419A0018  beq cr6, 0x824119f8
	if ctx.cr[6].eq {
	pc = 0x824119F8; continue 'dispatch;
	}
	// 824119E4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 824119E8: 4099000C  ble cr6, 0x824119f4
	if !ctx.cr[6].gt {
	pc = 0x824119F4; continue 'dispatch;
	}
	// 824119EC: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 824119F0: 40990008  ble cr6, 0x824119f8
	if !ctx.cr[6].gt {
	pc = 0x824119F8; continue 'dispatch;
	}
	// 824119F4: 48000000  b 0x824119f4
	pc = 0x824119F4; continue 'dispatch;
	// 824119F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824119FC: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82411A00: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411A04: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82411A08: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82411A0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411A10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82411A14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411A18: 4E800421  bctrl
	ctx.lr = 0x82411A1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411A1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411A20: 4182005C  beq 0x82411a7c
	if ctx.cr[0].eq {
	pc = 0x82411A7C; continue 'dispatch;
	}
	// 82411A24: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82411A28: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82411A2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411A30: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82411A34: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82411A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411A3C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82411A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411A44: 4E800421  bctrl
	ctx.lr = 0x82411A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411A48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411A4C: 41820030  beq 0x82411a7c
	if ctx.cr[0].eq {
	pc = 0x82411A7C; continue 'dispatch;
	}
	// 82411A50: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82411A54: 419A0010  beq cr6, 0x82411a64
	if ctx.cr[6].eq {
	pc = 0x82411A64; continue 'dispatch;
	}
	// 82411A58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82411A5C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82411A60: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82411A64: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82411A68: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82411A6C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82411A70: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82411A74: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82411A78: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82411A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82411A80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411A84: 48123688  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411A88 size=92
    let mut pc: u32 = 0x82411A88;
    'dispatch: loop {
        match pc {
            0x82411A88 => {
    //   block [0x82411A88..0x82411AE4)
	// 82411A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411AA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411AA4: 48000CDD  bl 0x82412780
	ctx.lr = 0x82411AA8;
	sub_82412780(ctx, base);
	// 82411AA8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411AAC: 4182001C  beq 0x82411ac8
	if ctx.cr[0].eq {
	pc = 0x82411AC8; continue 'dispatch;
	}
	// 82411AB0: 4BFFED11  bl 0x824107c0
	ctx.lr = 0x82411AB4;
	sub_824107C0(ctx, base);
	// 82411AB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411AB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411ABC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411AC4: 4E800421  bctrl
	ctx.lr = 0x82411AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411AE8 size=8
    let mut pc: u32 = 0x82411AE8;
    'dispatch: loop {
        match pc {
            0x82411AE8 => {
    //   block [0x82411AE8..0x82411AF0)
	// 82411AE8: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411AEC: 48000584  b 0x82412070
	sub_82412070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411AF0 size=8
    let mut pc: u32 = 0x82411AF0;
    'dispatch: loop {
        match pc {
            0x82411AF0 => {
    //   block [0x82411AF0..0x82411AF8)
	// 82411AF0: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411AF4: 48000AAC  b 0x824125a0
	sub_824125A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411AF8 size=8
    let mut pc: u32 = 0x82411AF8;
    'dispatch: loop {
        match pc {
            0x82411AF8 => {
    //   block [0x82411AF8..0x82411B00)
	// 82411AF8: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411AFC: 48000AF4  b 0x824125f0
	sub_824125F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B00 size=8
    let mut pc: u32 = 0x82411B00;
    'dispatch: loop {
        match pc {
            0x82411B00 => {
    //   block [0x82411B00..0x82411B08)
	// 82411B00: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411B04: 48000B3C  b 0x82412640
	sub_82412640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B08 size=8
    let mut pc: u32 = 0x82411B08;
    'dispatch: loop {
        match pc {
            0x82411B08 => {
    //   block [0x82411B08..0x82411B10)
	// 82411B08: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82411B0C: 48000B84  b 0x82412690
	sub_82412690(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B10 size=8
    let mut pc: u32 = 0x82411B10;
    'dispatch: loop {
        match pc {
            0x82411B10 => {
    //   block [0x82411B10..0x82411B18)
	// 82411B10: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B14: 480006FC  b 0x82412210
	sub_82412210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B18 size=8
    let mut pc: u32 = 0x82411B18;
    'dispatch: loop {
        match pc {
            0x82411B18 => {
    //   block [0x82411B18..0x82411B20)
	// 82411B18: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B1C: 48001004  b 0x82412b20
	sub_82412B20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411B20 size=8
    let mut pc: u32 = 0x82411B20;
    'dispatch: loop {
        match pc {
            0x82411B20 => {
    //   block [0x82411B20..0x82411B28)
	// 82411B20: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B24: 48001024  b 0x82412b48
	sub_82412B48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82411B28 size=16
    let mut pc: u32 = 0x82411B28;
    'dispatch: loop {
        match pc {
            0x82411B28 => {
    //   block [0x82411B28..0x82411B38)
	// 82411B28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82411B2C: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B30: C02B0024  lfs f1, 0x24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82411B34: 4800103C  b 0x82412b70
	sub_82412B70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411B38 size=44
    let mut pc: u32 = 0x82411B38;
    'dispatch: loop {
        match pc {
            0x82411B38 => {
    //   block [0x82411B38..0x82411B64)
	// 82411B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411B44: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82411B48: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B4C: 48000CCD  bl 0x82412818
	ctx.lr = 0x82411B50;
	sub_82412818(ctx, base);
	// 82411B50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411B54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82411B68 size=96
    let mut pc: u32 = 0x82411B68;
    'dispatch: loop {
        match pc {
            0x82411B68 => {
    //   block [0x82411B68..0x82411BC8)
	// 82411B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411B80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411B84: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411B88: 48000F99  bl 0x82412b20
	ctx.lr = 0x82411B8C;
	sub_82412B20(ctx, base);
	// 82411B8C: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82411B90: 4098000C  bge cr6, 0x82411b9c
	if !ctx.cr[6].lt {
	pc = 0x82411B9C; continue 'dispatch;
	}
	// 82411B94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82411B98: 48000018  b 0x82411bb0
	pc = 0x82411BB0; continue 'dispatch;
	// 82411B9C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411BA0: 48000FA9  bl 0x82412b48
	ctx.lr = 0x82411BA4;
	sub_82412B48(ctx, base);
	// 82411BA4: 7D7E1810  subfc r11, r30, r3
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[30].u32;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 82411BA8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82411BAC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82411BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411BBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411BC8 size=48
    let mut pc: u32 = 0x82411BC8;
    'dispatch: loop {
        match pc {
            0x82411BC8 => {
    //   block [0x82411BC8..0x82411BF8)
	// 82411BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411BD4: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411BD8: 48000641  bl 0x82412218
	ctx.lr = 0x82411BDC;
	sub_82412218(ctx, base);
	// 82411BDC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82411BE0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82411BE4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82411BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411BF8 size=48
    let mut pc: u32 = 0x82411BF8;
    'dispatch: loop {
        match pc {
            0x82411BF8 => {
    //   block [0x82411BF8..0x82411C28)
	// 82411BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411C04: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411C08: 48000689  bl 0x82412290
	ctx.lr = 0x82411C0C;
	sub_82412290(ctx, base);
	// 82411C0C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82411C10: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82411C14: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82411C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411C28 size=48
    let mut pc: u32 = 0x82411C28;
    'dispatch: loop {
        match pc {
            0x82411C28 => {
    //   block [0x82411C28..0x82411C58)
	// 82411C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411C34: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411C38: 48000EE1  bl 0x82412b18
	ctx.lr = 0x82411C3C;
	sub_82412B18(ctx, base);
	// 82411C3C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82411C40: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82411C44: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82411C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411C58 size=164
    let mut pc: u32 = 0x82411C58;
    'dispatch: loop {
        match pc {
            0x82411C58 => {
    //   block [0x82411C58..0x82411CFC)
	// 82411C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411C5C: 48123461  bl 0x825350bc
	ctx.lr = 0x82411C60;
	sub_82535080(ctx, base);
	// 82411C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411C64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411C6C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82411C70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411C74: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82411C78: 41980014  blt cr6, 0x82411c8c
	if ctx.cr[6].lt {
	pc = 0x82411C8C; continue 'dispatch;
	}
	// 82411C7C: 419A0034  beq cr6, 0x82411cb0
	if ctx.cr[6].eq {
	pc = 0x82411CB0; continue 'dispatch;
	}
	// 82411C80: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82411C84: 41980048  blt cr6, 0x82411ccc
	if ctx.cr[6].lt {
	pc = 0x82411CCC; continue 'dispatch;
	}
	// 82411C88: 48000068  b 0x82411cf0
	pc = 0x82411CF0; continue 'dispatch;
	// 82411C8C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82411C90: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411C94: 48000BF5  bl 0x82412888
	ctx.lr = 0x82411C98;
	sub_82412888(ctx, base);
	// 82411C98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411C9C: 40820008  bne 0x82411ca4
	if !ctx.cr[0].eq {
	pc = 0x82411CA4; continue 'dispatch;
	}
	// 82411CA0: 48000000  b 0x82411ca0
	pc = 0x82411CA0; continue 'dispatch;
	// 82411CA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411CA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411CAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411CB0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411CB4: 48000C4D  bl 0x82412900
	ctx.lr = 0x82411CB8;
	sub_82412900(ctx, base);
	// 82411CB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411CBC: 41820038  beq 0x82411cf4
	if ctx.cr[0].eq {
	pc = 0x82411CF4; continue 'dispatch;
	}
	// 82411CC0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411CC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411CC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411CCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82411CD0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411CD4: 48000C55  bl 0x82412928
	ctx.lr = 0x82411CD8;
	sub_82412928(ctx, base);
	// 82411CD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411CDC: 40820008  bne 0x82411ce4
	if !ctx.cr[0].eq {
	pc = 0x82411CE4; continue 'dispatch;
	}
	// 82411CE0: 48000000  b 0x82411ce0
	pc = 0x82411CE0; continue 'dispatch;
	// 82411CE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411CE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411CEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411CF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411CF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411CF8: 48123414  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411D00 size=40
    let mut pc: u32 = 0x82411D00;
    'dispatch: loop {
        match pc {
            0x82411D00 => {
    //   block [0x82411D00..0x82411D28)
	// 82411D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411D0C: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D10: 480004B9  bl 0x824121c8
	ctx.lr = 0x82411D14;
	sub_824121C8(ctx, base);
	// 82411D14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82411D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D28 size=8
    let mut pc: u32 = 0x82411D28;
    'dispatch: loop {
        match pc {
            0x82411D28 => {
    //   block [0x82411D28..0x82411D30)
	// 82411D28: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D2C: 48000C34  b 0x82412960
	sub_82412960(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D30 size=8
    let mut pc: u32 = 0x82411D30;
    'dispatch: loop {
        match pc {
            0x82411D30 => {
    //   block [0x82411D30..0x82411D38)
	// 82411D30: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D34: 48000DBC  b 0x82412af0
	sub_82412AF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D38 size=8
    let mut pc: u32 = 0x82411D38;
    'dispatch: loop {
        match pc {
            0x82411D38 => {
    //   block [0x82411D38..0x82411D40)
	// 82411D38: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D3C: 480005CC  b 0x82412308
	sub_82412308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D40 size=8
    let mut pc: u32 = 0x82411D40;
    'dispatch: loop {
        match pc {
            0x82411D40 => {
    //   block [0x82411D40..0x82411D48)
	// 82411D40: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D44: 48000644  b 0x82412388
	sub_82412388(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82411D48 size=28
    let mut pc: u32 = 0x82411D48;
    'dispatch: loop {
        match pc {
            0x82411D48 => {
    //   block [0x82411D48..0x82411D64)
	// 82411D48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82411D4C: 38AB002C  addi r5, r11, 0x2c
	ctx.r[5].s64 = ctx.r[11].s64 + 44;
	// 82411D50: 388B0024  addi r4, r11, 0x24
	ctx.r[4].s64 = ctx.r[11].s64 + 36;
	// 82411D54: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411D5C: C02BEBCC  lfs f1, -0x1434(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5172 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82411D60: 48000C08  b 0x82412968
	sub_82412968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411D68 size=8
    let mut pc: u32 = 0x82411D68;
    'dispatch: loop {
        match pc {
            0x82411D68 => {
    //   block [0x82411D68..0x82411D70)
	// 82411D68: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411D6C: 4800030C  b 0x82412078
	sub_82412078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411D70 size=84
    let mut pc: u32 = 0x82411D70;
    'dispatch: loop {
        match pc {
            0x82411D70 => {
    //   block [0x82411D70..0x82411DC4)
	// 82411D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411D78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411D7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411D84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411D88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411D8C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411D90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411D94: 41820014  beq 0x82411da8
	if ctx.cr[0].eq {
	pc = 0x82411DA8; continue 'dispatch;
	}
	// 82411D98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82411D9C: 4BFFFCED  bl 0x82411a88
	ctx.lr = 0x82411DA0;
	sub_82411A88(ctx, base);
	// 82411DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411DA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411DA8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82411DAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411DB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411DC8 size=168
    let mut pc: u32 = 0x82411DC8;
    'dispatch: loop {
        match pc {
            0x82411DC8 => {
    //   block [0x82411DC8..0x82411E70)
	// 82411DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411DCC: 481232ED  bl 0x825350b8
	ctx.lr = 0x82411DD0;
	sub_82535080(ctx, base);
	// 82411DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411DD4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82411DD8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82411DDC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82411DE0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82411DE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82411DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411DEC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82411DF0: 4BFFFAD9  bl 0x824118c8
	ctx.lr = 0x82411DF4;
	sub_824118C8(ctx, base);
	// 82411DF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411DF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82411DFC: 396BEBE8  addi r11, r11, -0x1418
	ctx.r[11].s64 = ctx.r[11].s64 + -5144;
	// 82411E00: 394AEBD0  addi r10, r10, -0x1430
	ctx.r[10].s64 = ctx.r[10].s64 + -5168;
	// 82411E04: 3B9F0030  addi r28, r31, 0x30
	ctx.r[28].s64 = ctx.r[31].s64 + 48;
	// 82411E08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411E0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411E10: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411E14: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82411E18: 4BFFE9A9  bl 0x824107c0
	ctx.lr = 0x82411E1C;
	sub_824107C0(ctx, base);
	// 82411E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411E20: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 82411E24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82411E28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411E2C: 4E800421  bctrl
	ctx.lr = 0x82411E30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411E30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411E34: 4182001C  beq 0x82411e50
	if ctx.cr[0].eq {
	pc = 0x82411E50; continue 'dispatch;
	}
	// 82411E38: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82411E3C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82411E40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411E44: 4800089D  bl 0x824126e0
	ctx.lr = 0x82411E48;
	sub_824126E0(ctx, base);
	// 82411E48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82411E4C: 48000008  b 0x82411e54
	pc = 0x82411E54; continue 'dispatch;
	// 82411E50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82411E54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82411E58: 4BFFFF19  bl 0x82411d70
	ctx.lr = 0x82411E5C;
	sub_82411D70(ctx, base);
	// 82411E5C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411E60: 48001029  bl 0x82412e88
	ctx.lr = 0x82411E64;
	sub_82412E88(ctx, base);
	// 82411E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82411E6C: 4812329C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411E70 size=8
    let mut pc: u32 = 0x82411E70;
    'dispatch: loop {
        match pc {
            0x82411E70 => {
    //   block [0x82411E70..0x82411E78)
	// 82411E70: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82411E74: 4800006C  b 0x82411ee0
	sub_82411EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411E78 size=100
    let mut pc: u32 = 0x82411E78;
    'dispatch: loop {
        match pc {
            0x82411E78 => {
    //   block [0x82411E78..0x82411EDC)
	// 82411E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411E88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82411E8C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82411E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411E94: 396BEBE8  addi r11, r11, -0x1418
	ctx.r[11].s64 = ctx.r[11].s64 + -5144;
	// 82411E98: 394AEBD0  addi r10, r10, -0x1430
	ctx.r[10].s64 = ctx.r[10].s64 + -5168;
	// 82411E9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82411EA0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82411EA4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82411EA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411EAC: 41820014  beq 0x82411ec0
	if ctx.cr[0].eq {
	pc = 0x82411EC0; continue 'dispatch;
	}
	// 82411EB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82411EB4: 4BFFFBD5  bl 0x82411a88
	ctx.lr = 0x82411EB8;
	sub_82411A88(ctx, base);
	// 82411EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82411EBC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82411EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411EC4: 4BFFF62D  bl 0x824114f0
	ctx.lr = 0x82411EC8;
	sub_824114F0(ctx, base);
	// 82411EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82411ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411EE0 size=92
    let mut pc: u32 = 0x82411EE0;
    'dispatch: loop {
        match pc {
            0x82411EE0 => {
    //   block [0x82411EE0..0x82411F3C)
	// 82411EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411EF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411EF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411EFC: 4BFFFF7D  bl 0x82411e78
	ctx.lr = 0x82411F00;
	sub_82411E78(ctx, base);
	// 82411F00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411F04: 4182001C  beq 0x82411f20
	if ctx.cr[0].eq {
	pc = 0x82411F20; continue 'dispatch;
	}
	// 82411F08: 4BFFE8B9  bl 0x824107c0
	ctx.lr = 0x82411F0C;
	sub_824107C0(ctx, base);
	// 82411F0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411F10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411F14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411F1C: 4E800421  bctrl
	ctx.lr = 0x82411F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411F24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411F30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411F40 size=92
    let mut pc: u32 = 0x82411F40;
    'dispatch: loop {
        match pc {
            0x82411F40 => {
    //   block [0x82411F40..0x82411F9C)
	// 82411F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411F48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411F4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411F58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411F5C: 480018CD  bl 0x82413828
	ctx.lr = 0x82411F60;
	sub_82413828(ctx, base);
	// 82411F60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411F64: 4182001C  beq 0x82411f80
	if ctx.cr[0].eq {
	pc = 0x82411F80; continue 'dispatch;
	}
	// 82411F68: 4BFFE859  bl 0x824107c0
	ctx.lr = 0x82411F6C;
	sub_824107C0(ctx, base);
	// 82411F6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411F70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411F74: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411F78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411F7C: 4E800421  bctrl
	ctx.lr = 0x82411F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411F84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411F90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82411FA0 size=92
    let mut pc: u32 = 0x82411FA0;
    'dispatch: loop {
        match pc {
            0x82411FA0 => {
    //   block [0x82411FA0..0x82411FFC)
	// 82411FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82411FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82411FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82411FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82411FB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82411FB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82411FB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82411FBC: 48002225  bl 0x824141e0
	ctx.lr = 0x82411FC0;
	sub_824141E0(ctx, base);
	// 82411FC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82411FC4: 4182001C  beq 0x82411fe0
	if ctx.cr[0].eq {
	pc = 0x82411FE0; continue 'dispatch;
	}
	// 82411FC8: 4BFFE7F9  bl 0x824107c0
	ctx.lr = 0x82411FCC;
	sub_824107C0(ctx, base);
	// 82411FCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411FD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82411FD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82411FD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82411FDC: 4E800421  bctrl
	ctx.lr = 0x82411FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82411FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82411FE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82411FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82411FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82411FF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82411FF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82411FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412000 size=108
    let mut pc: u32 = 0x82412000;
    'dispatch: loop {
        match pc {
            0x82412000 => {
    //   block [0x82412000..0x8241206C)
	// 82412000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412004: 481230B9  bl 0x825350bc
	ctx.lr = 0x82412008;
	sub_82535080(ctx, base);
	// 82412008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241200C: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82412010: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82412014: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412018: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8241201C: 419A002C  beq cr6, 0x82412048
	if ctx.cr[6].eq {
	pc = 0x82412048; continue 'dispatch;
	}
	// 82412020: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82412024: 409A003C  bne cr6, 0x82412060
	if !ctx.cr[6].eq {
	pc = 0x82412060; continue 'dispatch;
	}
	// 82412028: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8241202C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412030: 48056A91  bl 0x82468ac0
	ctx.lr = 0x82412034;
	sub_82468AC0(ctx, base);
	// 82412034: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82412038: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241203C: 40980024  bge cr6, 0x82412060
	if !ctx.cr[6].lt {
	pc = 0x82412060; continue 'dispatch;
	}
	// 82412040: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82412044: 48000014  b 0x82412058
	pc = 0x82412058; continue 'dispatch;
	// 82412048: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241204C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412050: 41820010  beq 0x82412060
	if ctx.cr[0].eq {
	pc = 0x82412060; continue 'dispatch;
	}
	// 82412054: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82412058: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8241205C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82412060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82412064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412068: 481230A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412070 size=8
    let mut pc: u32 = 0x82412070;
    'dispatch: loop {
        match pc {
            0x82412070 => {
    //   block [0x82412070..0x82412078)
	// 82412070: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82412074: 480024B4  b 0x82414528
	sub_82414528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412078 size=8
    let mut pc: u32 = 0x82412078;
    'dispatch: loop {
        match pc {
            0x82412078 => {
    //   block [0x82412078..0x82412080)
	// 82412078: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8241207C: 48056A44  b 0x82468ac0
	sub_82468AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412080 size=104
    let mut pc: u32 = 0x82412080;
    'dispatch: loop {
        match pc {
            0x82412080 => {
    //   block [0x82412080..0x824120E8)
	// 82412080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412084: 48123031  bl 0x825350b4
	ctx.lr = 0x82412088;
	sub_82535080(ctx, base);
	// 82412088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241208C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412090: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412094: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82412098: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8241209C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824120A0: 48056A21  bl 0x82468ac0
	ctx.lr = 0x824120A4;
	sub_82468AC0(ctx, base);
	// 824120A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824120A8: 4182002C  beq 0x824120d4
	if ctx.cr[0].eq {
	pc = 0x824120D4; continue 'dispatch;
	}
	// 824120AC: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 824120B0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824120B4: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824120B8: 48001AD1  bl 0x82413b88
	ctx.lr = 0x824120BC;
	sub_82413B88(ctx, base);
	// 824120BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824120C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824120C4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824120C8: 480569F9  bl 0x82468ac0
	ctx.lr = 0x824120CC;
	sub_82468AC0(ctx, base);
	// 824120CC: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824120D0: 4198FFE0  blt cr6, 0x824120b0
	if ctx.cr[6].lt {
	pc = 0x824120B0; continue 'dispatch;
	}
	// 824120D4: 937F004C  stw r27, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[27].u32 ) };
	// 824120D8: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 824120DC: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 824120E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824120E4: 48123020  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824120E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824120E8 size=108
    let mut pc: u32 = 0x824120E8;
    'dispatch: loop {
        match pc {
            0x824120E8 => {
    //   block [0x824120E8..0x82412154)
	// 824120E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824120EC: 48122FCD  bl 0x825350b8
	ctx.lr = 0x824120F0;
	sub_82535080(ctx, base);
	// 824120F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824120F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824120F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824120FC: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412100: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412104: 480569BD  bl 0x82468ac0
	ctx.lr = 0x82412108;
	sub_82468AC0(ctx, base);
	// 82412108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241210C: 41820038  beq 0x82412144
	if ctx.cr[0].eq {
	pc = 0x82412144; continue 'dispatch;
	}
	// 82412110: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412114: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412118: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8241211C: 419A0010  beq cr6, 0x8241212c
	if ctx.cr[6].eq {
	pc = 0x8241212C; continue 'dispatch;
	}
	// 82412120: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412124: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82412128: 48001A61  bl 0x82413b88
	ctx.lr = 0x8241212C;
	sub_82413B88(ctx, base);
	// 8241212C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412130: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82412134: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82412138: 48056989  bl 0x82468ac0
	ctx.lr = 0x8241213C;
	sub_82468AC0(ctx, base);
	// 8241213C: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412140: 4198FFD4  blt cr6, 0x82412114
	if ctx.cr[6].lt {
	pc = 0x82412114; continue 'dispatch;
	}
	// 82412144: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412148: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8241214C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412150: 48122FB8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412158 size=108
    let mut pc: u32 = 0x82412158;
    'dispatch: loop {
        match pc {
            0x82412158 => {
    //   block [0x82412158..0x824121C4)
	// 82412158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241215C: 48122F61  bl 0x825350bc
	ctx.lr = 0x82412160;
	sub_82535080(ctx, base);
	// 82412160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412168: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8241216C: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 82412170: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82412174: 4805694D  bl 0x82468ac0
	ctx.lr = 0x82412178;
	sub_82468AC0(ctx, base);
	// 82412178: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8241217C: 41980008  blt cr6, 0x82412184
	if ctx.cr[6].lt {
	pc = 0x82412184; continue 'dispatch;
	}
	// 82412180: 48000000  b 0x82412180
	pc = 0x82412180; continue 'dispatch;
	// 82412184: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412188: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241218C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82412190: 48002191  bl 0x82414320
	ctx.lr = 0x82412194;
	sub_82414320(ctx, base);
	// 82412194: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412198: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241219C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824121A0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824121A4: 48001755  bl 0x824138f8
	ctx.lr = 0x824121A8;
	sub_824138F8(ctx, base);
	// 824121A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824121AC: 41820010  beq 0x824121bc
	if ctx.cr[0].eq {
	pc = 0x824121BC; continue 'dispatch;
	}
	// 824121B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824121B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824121B8: 4BFFFE49  bl 0x82412000
	ctx.lr = 0x824121BC;
	sub_82412000(ctx, base);
	// 824121BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824121C0: 48122F4C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824121C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824121C8 size=28
    let mut pc: u32 = 0x824121C8;
    'dispatch: loop {
        match pc {
            0x824121C8 => {
    //   block [0x824121C8..0x824121E4)
	// 824121C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824121CC: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 824121D0: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824121D4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824121D8: 409A000C  bne cr6, 0x824121e4
	if !ctx.cr[6].eq {
		sub_824121E4(ctx, base);
		return;
	}
	// 824121DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824121E0: 48000014  b 0x824121f4
	sub_824121E4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824121E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824121E4 size=32
    let mut pc: u32 = 0x824121E4;
    'dispatch: loop {
        match pc {
            0x824121E4 => {
    //   block [0x824121E4..0x82412204)
	// 824121E4: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824121E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824121EC: 41980008  blt cr6, 0x824121f4
	if ctx.cr[6].lt {
	pc = 0x824121F4; continue 'dispatch;
	}
	// 824121F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824121F4: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824121F8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824121FC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82412200: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412204 size=8
    let mut pc: u32 = 0x82412204;
    'dispatch: loop {
        match pc {
            0x82412204 => {
    //   block [0x82412204..0x8241220C)
	// 82412204: 4BFFFEE4  b 0x824120e8
	sub_824120E8(ctx, base);
	return;
	// 82412208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82412210 size=4
    let mut pc: u32 = 0x82412210;
    'dispatch: loop {
        match pc {
            0x82412210 => {
    //   block [0x82412210..0x82412214)
	// 82412210: 4BFFFF48  b 0x82412158
	sub_82412158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412218 size=116
    let mut pc: u32 = 0x82412218;
    'dispatch: loop {
        match pc {
            0x82412218 => {
    //   block [0x82412218..0x8241228C)
	// 82412218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241221C: 48122E99  bl 0x825350b4
	ctx.lr = 0x82412220;
	sub_82535080(ctx, base);
	// 82412220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82412228: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241222C: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412230: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412234: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412238: 48056889  bl 0x82468ac0
	ctx.lr = 0x8241223C;
	sub_82468AC0(ctx, base);
	// 8241223C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412240: 41820038  beq 0x82412278
	if ctx.cr[0].eq {
	pc = 0x82412278; continue 'dispatch;
	}
	// 82412244: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82412248: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241224C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82412250: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82412254: 4800224D  bl 0x824144a0
	ctx.lr = 0x82412258;
	sub_824144A0(ctx, base);
	// 82412258: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241225C: 40820028  bne 0x82412284
	if !ctx.cr[0].eq {
	pc = 0x82412284; continue 'dispatch;
	}
	// 82412260: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412264: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412268: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8241226C: 48056855  bl 0x82468ac0
	ctx.lr = 0x82412270;
	sub_82468AC0(ctx, base);
	// 82412270: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412274: 4198FFD4  blt cr6, 0x82412248
	if ctx.cr[6].lt {
	pc = 0x82412248; continue 'dispatch;
	}
	// 82412278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241227C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412280: 48122E84  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82412284: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82412288: 4BFFFFF4  b 0x8241227c
	pc = 0x8241227C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412290 size=116
    let mut pc: u32 = 0x82412290;
    'dispatch: loop {
        match pc {
            0x82412290 => {
    //   block [0x82412290..0x82412304)
	// 82412290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412294: 48122E21  bl 0x825350b4
	ctx.lr = 0x82412298;
	sub_82535080(ctx, base);
	// 82412298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241229C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824122A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824122A4: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 824122A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824122AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824122B0: 48056811  bl 0x82468ac0
	ctx.lr = 0x824122B4;
	sub_82468AC0(ctx, base);
	// 824122B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824122B8: 41820038  beq 0x824122f0
	if ctx.cr[0].eq {
	pc = 0x824122F0; continue 'dispatch;
	}
	// 824122BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824122C0: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824122C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824122C8: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824122CC: 480021FD  bl 0x824144c8
	ctx.lr = 0x824122D0;
	sub_824144C8(ctx, base);
	// 824122D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824122D4: 40820028  bne 0x824122fc
	if !ctx.cr[0].eq {
	pc = 0x824122FC; continue 'dispatch;
	}
	// 824122D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824122DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824122E0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824122E4: 480567DD  bl 0x82468ac0
	ctx.lr = 0x824122E8;
	sub_82468AC0(ctx, base);
	// 824122E8: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824122EC: 4198FFD4  blt cr6, 0x824122c0
	if ctx.cr[6].lt {
	pc = 0x824122C0; continue 'dispatch;
	}
	// 824122F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824122F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824122F8: 48122E0C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824122FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82412300: 4BFFFFF4  b 0x824122f4
	pc = 0x824122F4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412308 size=128
    let mut pc: u32 = 0x82412308;
    'dispatch: loop {
        match pc {
            0x82412308 => {
    //   block [0x82412308..0x82412388)
	// 82412308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241230C: 48122DA9  bl 0x825350b4
	ctx.lr = 0x82412310;
	sub_82535080(ctx, base);
	// 82412310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412318: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241231C: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82412320: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82412324: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412328: 48056799  bl 0x82468ac0
	ctx.lr = 0x8241232C;
	sub_82468AC0(ctx, base);
	// 8241232C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412330: 41820038  beq 0x82412368
	if ctx.cr[0].eq {
	pc = 0x82412368; continue 'dispatch;
	}
	// 82412334: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82412338: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241233C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82412340: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82412344: 4800215D  bl 0x824144a0
	ctx.lr = 0x82412348;
	sub_824144A0(ctx, base);
	// 82412348: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241234C: 40820028  bne 0x82412374
	if !ctx.cr[0].eq {
	pc = 0x82412374; continue 'dispatch;
	}
	// 82412350: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412354: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82412358: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8241235C: 48056765  bl 0x82468ac0
	ctx.lr = 0x82412360;
	sub_82468AC0(ctx, base);
	// 82412360: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412364: 4198FFD4  blt cr6, 0x82412338
	if ctx.cr[6].lt {
	pc = 0x82412338; continue 'dispatch;
	}
	// 82412368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241236C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412370: 48122D94  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82412374: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412378: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8241237C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412380: 4BEE9E51  bl 0x822fc1d0
	ctx.lr = 0x82412384;
	sub_822FC1D0(ctx, base);
	// 82412384: 4BFFFFE8  b 0x8241236c
	pc = 0x8241236C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412388 size=128
    let mut pc: u32 = 0x82412388;
    'dispatch: loop {
        match pc {
            0x82412388 => {
    //   block [0x82412388..0x82412408)
	// 82412388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241238C: 48122D29  bl 0x825350b4
	ctx.lr = 0x82412390;
	sub_82535080(ctx, base);
	// 82412390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412398: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241239C: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 824123A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824123A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824123A8: 48056719  bl 0x82468ac0
	ctx.lr = 0x824123AC;
	sub_82468AC0(ctx, base);
	// 824123AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824123B0: 41820038  beq 0x824123e8
	if ctx.cr[0].eq {
	pc = 0x824123E8; continue 'dispatch;
	}
	// 824123B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824123B8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824123BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824123C0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824123C4: 48002105  bl 0x824144c8
	ctx.lr = 0x824123C8;
	sub_824144C8(ctx, base);
	// 824123C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824123CC: 40820028  bne 0x824123f4
	if !ctx.cr[0].eq {
	pc = 0x824123F4; continue 'dispatch;
	}
	// 824123D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824123D4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824123D8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824123DC: 480566E5  bl 0x82468ac0
	ctx.lr = 0x824123E0;
	sub_82468AC0(ctx, base);
	// 824123E0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824123E4: 4198FFD4  blt cr6, 0x824123b8
	if ctx.cr[6].lt {
	pc = 0x824123B8; continue 'dispatch;
	}
	// 824123E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824123EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824123F0: 48122D14  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824123F4: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824123F8: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824123FC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412400: 4BEE9DD1  bl 0x822fc1d0
	ctx.lr = 0x82412404;
	sub_822FC1D0(ctx, base);
	// 82412404: 4BFFFFE8  b 0x824123ec
	pc = 0x824123EC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412408 size=128
    let mut pc: u32 = 0x82412408;
    'dispatch: loop {
        match pc {
            0x82412408 => {
    //   block [0x82412408..0x82412488)
	// 82412408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241240C: 48122CA5  bl 0x825350b0
	ctx.lr = 0x82412410;
	sub_82535080(ctx, base);
	// 82412410: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82412418: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8241241C: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412420: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82412424: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412428: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8241242C: 48056695  bl 0x82468ac0
	ctx.lr = 0x82412430;
	sub_82468AC0(ctx, base);
	// 82412430: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412434: 41820034  beq 0x82412468
	if ctx.cr[0].eq {
	pc = 0x82412468; continue 'dispatch;
	}
	// 82412438: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8241243C: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412440: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82412444: 4BEE9D8D  bl 0x822fc1d0
	ctx.lr = 0x82412448;
	sub_822FC1D0(ctx, base);
	// 82412448: 7F03D840  cmplw cr6, r3, r27
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8241244C: 419A0028  beq cr6, 0x82412474
	if ctx.cr[6].eq {
	pc = 0x82412474; continue 'dispatch;
	}
	// 82412450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82412454: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82412458: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8241245C: 48056665  bl 0x82468ac0
	ctx.lr = 0x82412460;
	sub_82468AC0(ctx, base);
	// 82412460: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412464: 4198FFD8  blt cr6, 0x8241243c
	if ctx.cr[6].lt {
	pc = 0x8241243C; continue 'dispatch;
	}
	// 82412468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8241246C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82412470: 48122C90  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82412474: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82412478: 419A0008  beq cr6, 0x82412480
	if ctx.cr[6].eq {
	pc = 0x82412480; continue 'dispatch;
	}
	// 8241247C: 93BA0000  stw r29, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82412480: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82412484: 4BFFFFE8  b 0x8241246c
	pc = 0x8241246C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412488 size=84
    let mut pc: u32 = 0x82412488;
    'dispatch: loop {
        match pc {
            0x82412488 => {
    //   block [0x82412488..0x824124DC)
	// 82412488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241249C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824124A0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824124A4: 83DF004C  lwz r30, 0x4c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824124A8: 48056619  bl 0x82468ac0
	ctx.lr = 0x824124AC;
	sub_82468AC0(ctx, base);
	// 824124AC: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824124B0: 41980008  blt cr6, 0x824124b8
	if ctx.cr[6].lt {
	pc = 0x824124B8; continue 'dispatch;
	}
	// 824124B4: 48000000  b 0x824124b4
	pc = 0x824124B4; continue 'dispatch;
	// 824124B8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824124BC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824124C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824124C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824124C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824124CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824124D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824124D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824124D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824124E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824124E0 size=72
    let mut pc: u32 = 0x824124E0;
    'dispatch: loop {
        match pc {
            0x824124E0 => {
    //   block [0x824124E0..0x82412528)
	// 824124E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824124E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824124E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824124EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824124F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824124F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824124F8: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824124FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82412500: 4BFFFB01  bl 0x82412000
	ctx.lr = 0x82412504;
	sub_82412000(ctx, base);
	// 82412504: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82412508: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241250C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82412510: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412514: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241251C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412528 size=120
    let mut pc: u32 = 0x82412528;
    'dispatch: loop {
        match pc {
            0x82412528 => {
    //   block [0x82412528..0x824125A0)
	// 82412528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412538: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8241253C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82412540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412548: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8241254C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82412550: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82412554: 83DF004C  lwz r30, 0x4c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412558: 48056569  bl 0x82468ac0
	ctx.lr = 0x8241255C;
	sub_82468AC0(ctx, base);
	// 8241255C: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412560: 41980008  blt cr6, 0x82412568
	if ctx.cr[6].lt {
	pc = 0x82412568; continue 'dispatch;
	}
	// 82412564: 48000000  b 0x82412564
	pc = 0x82412564; continue 'dispatch;
	// 82412568: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8241256C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82412570: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82412574: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82412578: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8241257C: 48001475  bl 0x824139f0
	ctx.lr = 0x82412580;
	sub_824139F0(ctx, base);
	// 82412580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241258C: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82412590: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82412594: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82412598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8241259C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824125A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824125A0 size=76
    let mut pc: u32 = 0x824125A0;
    'dispatch: loop {
        match pc {
            0x824125A0 => {
    //   block [0x824125A0..0x824125EC)
	// 824125A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824125A4: 48122B19  bl 0x825350bc
	ctx.lr = 0x824125A8;
	sub_82535080(ctx, base);
	// 824125A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824125AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824125B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824125B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824125B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824125BC: 4BFFFE4D  bl 0x82412408
	ctx.lr = 0x824125C0;
	sub_82412408(ctx, base);
	// 824125C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824125C4: 41820020  beq 0x824125e4
	if ctx.cr[0].eq {
	pc = 0x824125E4; continue 'dispatch;
	}
	// 824125C8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824125CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824125D0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824125D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824125D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824125DC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824125E0: 48001A89  bl 0x82414068
	ctx.lr = 0x824125E4;
	sub_82414068(ctx, base);
	// 824125E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824125E8: 48122B24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824125F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824125F0 size=76
    let mut pc: u32 = 0x824125F0;
    'dispatch: loop {
        match pc {
            0x824125F0 => {
    //   block [0x824125F0..0x8241263C)
	// 824125F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824125F4: 48122AC9  bl 0x825350bc
	ctx.lr = 0x824125F8;
	sub_82535080(ctx, base);
	// 824125F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824125FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82412600: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82412604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412608: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8241260C: 4BFFFDFD  bl 0x82412408
	ctx.lr = 0x82412610;
	sub_82412408(ctx, base);
	// 82412610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412614: 41820020  beq 0x82412634
	if ctx.cr[0].eq {
	pc = 0x82412634; continue 'dispatch;
	}
	// 82412618: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241261C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82412620: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412628: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241262C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412630: 48001A89  bl 0x824140b8
	ctx.lr = 0x82412634;
	sub_824140B8(ctx, base);
	// 82412634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412638: 48122AD4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412640 size=76
    let mut pc: u32 = 0x82412640;
    'dispatch: loop {
        match pc {
            0x82412640 => {
    //   block [0x82412640..0x8241268C)
	// 82412640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412644: 48122A79  bl 0x825350bc
	ctx.lr = 0x82412648;
	sub_82535080(ctx, base);
	// 82412648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241264C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82412650: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82412654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412658: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8241265C: 4BFFFDAD  bl 0x82412408
	ctx.lr = 0x82412660;
	sub_82412408(ctx, base);
	// 82412660: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82412664: 41820020  beq 0x82412684
	if ctx.cr[0].eq {
	pc = 0x82412684; continue 'dispatch;
	}
	// 82412668: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8241266C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82412670: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82412674: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82412678: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8241267C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82412680: 48001A89  bl 0x82414108
	ctx.lr = 0x82412684;
	sub_82414108(ctx, base);
	// 82412684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412688: 48122A84  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412690 size=76
    let mut pc: u32 = 0x82412690;
    'dispatch: loop {
        match pc {
            0x82412690 => {
    //   block [0x82412690..0x824126DC)
	// 82412690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412694: 48122A29  bl 0x825350bc
	ctx.lr = 0x82412698;
	sub_82535080(ctx, base);
	// 82412698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241269C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824126A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824126A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824126A8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824126AC: 4BFFFD5D  bl 0x82412408
	ctx.lr = 0x824126B0;
	sub_82412408(ctx, base);
	// 824126B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824126B4: 41820020  beq 0x824126d4
	if ctx.cr[0].eq {
	pc = 0x824126D4; continue 'dispatch;
	}
	// 824126B8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824126BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824126C0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824126C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824126C8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824126CC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824126D0: 48001A89  bl 0x82414158
	ctx.lr = 0x824126D4;
	sub_82414158(ctx, base);
	// 824126D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824126D8: 48122A34  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824126E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824126E0 size=160
    let mut pc: u32 = 0x824126E0;
    'dispatch: loop {
        match pc {
            0x824126E0 => {
    //   block [0x824126E0..0x82412780)
	// 824126E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824126E4: 481229CD  bl 0x825350b0
	ctx.lr = 0x824126E8;
	sub_82535080(ctx, base);
	// 824126E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824126EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824126F0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824126F4: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 824126F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824126FC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82412700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412704: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82412708: 48002849  bl 0x82414f50
	ctx.lr = 0x8241270C;
	sub_82414F50(ctx, base);
	// 8241270C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82412710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412714: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82412718: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241271C: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82412720: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82412724: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82412728: 83AA0008  lwz r29, 8(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241272C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82412730: 48001DC1  bl 0x824144f0
	ctx.lr = 0x82412734;
	sub_824144F0(ctx, base);
	// 82412734: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82412738: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8241273C: 48001DE5  bl 0x82414520
	ctx.lr = 0x82412740;
	sub_82414520(ctx, base);
	// 82412740: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82412744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82412748: 48001DD1  bl 0x82414518
	ctx.lr = 0x8241274C;
	sub_82414518(ctx, base);
	// 8241274C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82412750: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82412754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82412758: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8241275C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82412760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82412764: 4E800421  bctrl
	ctx.lr = 0x82412768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82412768: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8241276C: 40820008  bne 0x82412774
	if !ctx.cr[0].eq {
	pc = 0x82412774; continue 'dispatch;
	}
	// 82412770: 48000000  b 0x82412770
	pc = 0x82412770; continue 'dispatch;
	// 82412774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412778: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8241277C: 48122984  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412780 size=152
    let mut pc: u32 = 0x82412780;
    'dispatch: loop {
        match pc {
            0x82412780 => {
    //   block [0x82412780..0x82412818)
	// 82412780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412784: 48122931  bl 0x825350b4
	ctx.lr = 0x82412788;
	sub_82535080(ctx, base);
	// 82412788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241278C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82412790: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82412794: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 82412798: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8241279C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824127A0: 48056321  bl 0x82468ac0
	ctx.lr = 0x824127A4;
	sub_82468AC0(ctx, base);
	// 824127A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824127A8: 4182004C  beq 0x824127f4
	if ctx.cr[0].eq {
	pc = 0x824127F4; continue 'dispatch;
	}
	// 824127AC: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 824127B0: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824127B4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824127B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824127BC: 419A0020  beq cr6, 0x824127dc
	if ctx.cr[6].eq {
	pc = 0x824127DC; continue 'dispatch;
	}
	// 824127C0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824127C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824127C8: 4182000C  beq 0x824127d4
	if ctx.cr[0].eq {
	pc = 0x824127D4; continue 'dispatch;
	}
	// 824127CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824127D0: 4BFFF7D1  bl 0x82411fa0
	ctx.lr = 0x824127D4;
	sub_82411FA0(ctx, base);
	// 824127D4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824127D8: 7F6BF92E  stwx r27, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[27].u32) };
	// 824127DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824127E0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824127E4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824127E8: 480562D9  bl 0x82468ac0
	ctx.lr = 0x824127EC;
	sub_82468AC0(ctx, base);
	// 824127EC: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 824127F0: 4198FFC0  blt cr6, 0x824127b0
	if ctx.cr[6].lt {
	pc = 0x824127B0; continue 'dispatch;
	}
	// 824127F4: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 824127F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824127FC: 4182000C  beq 0x82412808
	if ctx.cr[0].eq {
	pc = 0x82412808; continue 'dispatch;
	}
	// 82412800: 481203B9  bl 0x82532bb8
	ctx.lr = 0x82412804;
	sub_82532BB8(ctx, base);
	// 82412804: 937E0048  stw r27, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[27].u32 ) };
	// 82412808: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8241280C: 480025A5  bl 0x82414db0
	ctx.lr = 0x82412810;
	sub_82414DB0(ctx, base);
	// 82412810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82412814: 481228F0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412818 size=108
    let mut pc: u32 = 0x82412818;
    'dispatch: loop {
        match pc {
            0x82412818 => {
    //   block [0x82412818..0x82412884)
	// 82412818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241281C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241282C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82412830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82412834: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82412838: 48056289  bl 0x82468ac0
	ctx.lr = 0x8241283C;
	sub_82468AC0(ctx, base);
	// 8241283C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82412840: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82412844: 41980008  blt cr6, 0x8241284c
	if ctx.cr[6].lt {
	pc = 0x8241284C; continue 'dispatch;
	}
	// 82412848: 48000000  b 0x82412848
	pc = 0x82412848; continue 'dispatch;
	// 8241284C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82412850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82412854: 419A000C  beq cr6, 0x82412860
	if ctx.cr[6].eq {
	pc = 0x82412860; continue 'dispatch;
	}
	// 82412858: 4BFFF829  bl 0x82412080
	ctx.lr = 0x8241285C;
	sub_82412080(ctx, base);
	// 8241285C: 48000010  b 0x8241286c
	pc = 0x8241286C; continue 'dispatch;
	// 82412860: 4BFFF889  bl 0x824120e8
	ctx.lr = 0x82412864;
	sub_824120E8(ctx, base);
	// 82412864: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82412868: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8241286C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82412870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241287C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412888 size=116
    let mut pc: u32 = 0x82412888;
    'dispatch: loop {
        match pc {
            0x82412888 => {
    //   block [0x82412888..0x824128FC)
	// 82412888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241288C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82412894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241289C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824128A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824128A4: 4BFFFB65  bl 0x82412408
	ctx.lr = 0x824128A8;
	sub_82412408(ctx, base);
	// 824128A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824128AC: 41820038  beq 0x824128e4
	if ctx.cr[0].eq {
	pc = 0x824128E4; continue 'dispatch;
	}
	// 824128B0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824128B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824128B8: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824128BC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824128C0: 409A0014  bne cr6, 0x824128d4
	if !ctx.cr[6].eq {
	pc = 0x824128D4; continue 'dispatch;
	}
	// 824128C4: 4BFFF825  bl 0x824120e8
	ctx.lr = 0x824128C8;
	sub_824120E8(ctx, base);
	// 824128C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824128CC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824128D0: 48000008  b 0x824128d8
	pc = 0x824128D8; continue 'dispatch;
	// 824128D4: 4BFFF7AD  bl 0x82412080
	ctx.lr = 0x824128D8;
	sub_82412080(ctx, base);
	// 824128D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824128DC: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 824128E0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 824128E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824128E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824128EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824128F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824128F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824128F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412900 size=36
    let mut pc: u32 = 0x82412900;
    'dispatch: loop {
        match pc {
            0x82412900 => {
    //   block [0x82412900..0x82412924)
	// 82412900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82412904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241290C: 4BFFFB7D  bl 0x82412488
	ctx.lr = 0x82412910;
	sub_82412488(ctx, base);
	// 82412910: 48000FE9  bl 0x824138f8
	ctx.lr = 0x82412914;
	sub_824138F8(ctx, base);
	// 82412914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82412918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241291C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82412928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82412928 size=52
    let mut pc: u32 = 0x82412928;
    'dispatch: loop {
        match pc {
            0x82412928 => {
    //   block [0x82412928..0x8241295C)
	// 82412928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241292C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82412930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82412934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82412938: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8241293C: 4BFFFB4D  bl 0x82412488
	ctx.lr = 0x82412940;
	sub_82412488(ctx, base);
	// 82412940: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82412944: 480012DD  bl 0x82413c20
	ctx.lr = 0x82412948;
	sub_82413C20(ctx, base);
	// 82412948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241294C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82412950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82412954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82412958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


