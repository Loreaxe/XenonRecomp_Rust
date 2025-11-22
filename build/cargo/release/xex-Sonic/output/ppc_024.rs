pub fn sub_824532C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824532C0 size=80
    let mut pc: u32 = 0x824532C0;
    'dispatch: loop {
        match pc {
            0x824532C0 => {
    //   block [0x824532C0..0x82453310)
	// 824532C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824532C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824532C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824532CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824532D0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 824532D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824532D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824532DC: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 824532E0: 4899E919  bl 0x82df1bf8
	ctx.lr = 0x824532E4;
	sub_82DF1BF8(ctx, base);
	// 824532E4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824532E8: 486F3F59  bl 0x82b47240
	ctx.lr = 0x824532EC;
	sub_82B47240(ctx, base);
	// 824532EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824532F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824532F4: 4899E99D  bl 0x82df1c90
	ctx.lr = 0x824532F8;
	sub_82DF1C90(ctx, base);
	// 824532F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824532FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245330C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82453310 size=188
    let mut pc: u32 = 0x82453310;
    'dispatch: loop {
        match pc {
            0x82453310 => {
    //   block [0x82453310..0x824533CC)
	// 82453310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245331C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453324: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82453328: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8245332C: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82453330: 4BFFFF91  bl 0x824532c0
	ctx.lr = 0x82453334;
	sub_824532C0(ctx, base);
	// 82453334: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82453338: 391F0070  addi r8, r31, 0x70
	ctx.r[8].s64 = ctx.r[31].s64 + 112;
	// 8245333C: 396B3664  addi r11, r11, 0x3664
	ctx.r[11].s64 = ctx.r[11].s64 + 13924;
	// 82453340: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82453344: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82453348: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8245334C: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82453350: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82453358: 7D4859AE  stbx r10, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 8245335C: 7CEA5830  slw r10, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82453360: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453364: 7CA55038  and r5, r5, r10
	ctx.r[5].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	// 82453368: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8245336C: 409A0040  bne cr6, 0x824533ac
	if !ctx.cr[6].eq {
	pc = 0x824533AC; continue 'dispatch;
	}
	// 82453370: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82453374: 7CAA5039  and. r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82453378: 41820024  beq 0x8245339c
	if ctx.cr[0].eq {
	pc = 0x8245339C; continue 'dispatch;
	}
	// 8245337C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82453380: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82453384: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82453388: 7D8AFC2E  lfsx f12, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8245338C: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82453390: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82453394: 41990008  bgt cr6, 0x8245339c
	if ctx.cr[6].gt {
	pc = 0x8245339C; continue 'dispatch;
	}
	// 82453398: 7CE859AE  stbx r7, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u8) };
	// 8245339C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 824533A0: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824533A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824533A8: 7DABFD2E  stfsx f13, r11, r31
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 824533AC: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824533B0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824533B4: 4082FF9C  bne 0x82453350
	if !ctx.cr[0].eq {
	pc = 0x82453350; continue 'dispatch;
	}
	// 824533B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824533BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824533C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824533C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824533C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824533D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824533D0 size=168
    let mut pc: u32 = 0x824533D0;
    'dispatch: loop {
        match pc {
            0x824533D0 => {
    //   block [0x824533D0..0x82453478)
	// 824533D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824533D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824533D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824533DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824533E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824533E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824533E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824533EC: 4BFFFED5  bl 0x824532c0
	ctx.lr = 0x824533F0;
	sub_824532C0(ctx, base);
	// 824533F0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824533F4: 2B1F000C  cmplwi cr6, r31, 0xc
	ctx.cr[6].compare_u32(ctx.r[31].u32, 12 as u32, &mut ctx.xer);
	// 824533F8: 419A0014  beq cr6, 0x8245340c
	if ctx.cr[6].eq {
	pc = 0x8245340C; continue 'dispatch;
	}
	// 824533FC: 2B1F0015  cmplwi cr6, r31, 0x15
	ctx.cr[6].compare_u32(ctx.r[31].u32, 21 as u32, &mut ctx.xer);
	// 82453400: 409A002C  bne cr6, 0x8245342c
	if !ctx.cr[6].eq {
	pc = 0x8245342C; continue 'dispatch;
	}
	// 82453404: 887E0071  lbz r3, 0x71(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(113 as u32) ) } as u64;
	// 82453408: 48000058  b 0x82453460
	pc = 0x82453460; continue 'dispatch;
	// 8245340C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82453410: C1AA0028  lfs f13, 0x28(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82453414: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82453418: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8245341C: 41990040  bgt cr6, 0x8245345c
	if ctx.cr[6].gt {
	pc = 0x8245345C; continue 'dispatch;
	}
	// 82453420: C1AA002C  lfs f13, 0x2c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82453424: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82453428: 41990034  bgt cr6, 0x8245345c
	if ctx.cr[6].gt {
	pc = 0x8245345C; continue 'dispatch;
	}
	// 8245342C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453434: 4BFFFE05  bl 0x82453238
	ctx.lr = 0x82453438;
	sub_82453238(ctx, base);
	// 82453438: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245343C: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 82453440: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82453444: 409A0010  bne cr6, 0x82453454
	if !ctx.cr[6].eq {
	pc = 0x82453454; continue 'dispatch;
	}
	// 82453448: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245344C: 7D6B1839  and. r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82453450: 4082000C  bne 0x8245345c
	if !ctx.cr[0].eq {
	pc = 0x8245345C; continue 'dispatch;
	}
	// 82453454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82453458: 48000008  b 0x82453460
	pc = 0x82453460; continue 'dispatch;
	// 8245345C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82453460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245346C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82453470: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82453478 size=140
    let mut pc: u32 = 0x82453478;
    'dispatch: loop {
        match pc {
            0x82453478 => {
    //   block [0x82453478..0x82453504)
	// 82453478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245347C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82453484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82453488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245348C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82453494: 4BFFFE2D  bl 0x824532c0
	ctx.lr = 0x82453498;
	sub_824532C0(ctx, base);
	// 82453498: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8245349C: 2B1E000C  cmplwi cr6, r30, 0xc
	ctx.cr[6].compare_u32(ctx.r[30].u32, 12 as u32, &mut ctx.xer);
	// 824534A0: 409A0024  bne cr6, 0x824534c4
	if !ctx.cr[6].eq {
	pc = 0x824534C4; continue 'dispatch;
	}
	// 824534A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824534A8: C1AA0028  lfs f13, 0x28(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824534AC: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824534B0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824534B4: 41990034  bgt cr6, 0x824534e8
	if ctx.cr[6].gt {
	pc = 0x824534E8; continue 'dispatch;
	}
	// 824534B8: C1AA002C  lfs f13, 0x2c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824534BC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824534C0: 41990028  bgt cr6, 0x824534e8
	if ctx.cr[6].gt {
	pc = 0x824534E8; continue 'dispatch;
	}
	// 824534C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824534C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824534CC: 4BFFFD6D  bl 0x82453238
	ctx.lr = 0x824534D0;
	sub_82453238(ctx, base);
	// 824534D0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824534D4: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 824534D8: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 824534DC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824534E0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824534E4: 48000008  b 0x824534ec
	pc = 0x824534EC; continue 'dispatch;
	// 824534E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824534EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824534F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824534F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824534F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824534FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82453508 size=140
    let mut pc: u32 = 0x82453508;
    'dispatch: loop {
        match pc {
            0x82453508 => {
    //   block [0x82453508..0x82453594)
	// 82453508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245350C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82453514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82453518: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245351C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453520: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82453524: 4BFFFD9D  bl 0x824532c0
	ctx.lr = 0x82453528;
	sub_824532C0(ctx, base);
	// 82453528: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8245352C: 2B1E000C  cmplwi cr6, r30, 0xc
	ctx.cr[6].compare_u32(ctx.r[30].u32, 12 as u32, &mut ctx.xer);
	// 82453530: 409A0024  bne cr6, 0x82453554
	if !ctx.cr[6].eq {
	pc = 0x82453554; continue 'dispatch;
	}
	// 82453534: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82453538: C1AA0028  lfs f13, 0x28(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245353C: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82453540: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82453544: 41980034  blt cr6, 0x82453578
	if ctx.cr[6].lt {
	pc = 0x82453578; continue 'dispatch;
	}
	// 82453548: C1AA002C  lfs f13, 0x2c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245354C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82453550: 41980028  blt cr6, 0x82453578
	if ctx.cr[6].lt {
	pc = 0x82453578; continue 'dispatch;
	}
	// 82453554: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82453558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245355C: 4BFFFCDD  bl 0x82453238
	ctx.lr = 0x82453560;
	sub_82453238(ctx, base);
	// 82453560: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82453564: 7C6B5838  and r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[11].u64;
	// 82453568: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8245356C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82453570: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82453574: 48000008  b 0x8245357c
	pc = 0x8245357C; continue 'dispatch;
	// 82453578: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8245357C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245358C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453598 size=48
    let mut pc: u32 = 0x82453598;
    'dispatch: loop {
        match pc {
            0x82453598 => {
    //   block [0x82453598..0x824535C8)
	// 82453598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824535A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824535A4: 4BFFFD1D  bl 0x824532c0
	ctx.lr = 0x824535A8;
	sub_824532C0(ctx, base);
	// 824535A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824535AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824535B0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824535B4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 824535B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824535BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824535C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824535C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824535C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824535C8 size=52
    let mut pc: u32 = 0x824535C8;
    'dispatch: loop {
        match pc {
            0x824535C8 => {
    //   block [0x824535C8..0x824535FC)
	// 824535C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824535CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824535D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824535D4: 4BFFFCED  bl 0x824532c0
	ctx.lr = 0x824535D8;
	sub_824532C0(ctx, base);
	// 824535D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824535DC: 716B301B  andi. r11, r11, 0x301b
	ctx.r[11].u64 = ctx.r[11].u64 & 12315;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824535E0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824535E4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824535E8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 824535EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824535F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824535F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824535F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453600 size=68
    let mut pc: u32 = 0x82453600;
    'dispatch: loop {
        match pc {
            0x82453600 => {
    //   block [0x82453600..0x82453644)
	// 82453600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453608: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245360C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453614: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453618: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245361C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82453620: 4E800421  bctrl
	ctx.lr = 0x82453624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82453624: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82453628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245362C: 48A06115  bl 0x82e59740
	ctx.lr = 0x82453630;
	sub_82E59740(ctx, base);
	// 82453630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82453634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245363C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453648 size=52
    let mut pc: u32 = 0x82453648;
    'dispatch: loop {
        match pc {
            0x82453648 => {
    //   block [0x82453648..0x8245367C)
	// 82453648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82453654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453658: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245365C: 48D00FA5  bl 0x83154600
	ctx.lr = 0x82453660;
	sub_83154600(ctx, base);
	// 82453660: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453664: 4BEB253D  bl 0x82305ba0
	ctx.lr = 0x82453668;
	sub_82305BA0(ctx, base);
	// 82453668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245366C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453674: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453680 size=92
    let mut pc: u32 = 0x82453680;
    'dispatch: loop {
        match pc {
            0x82453680 => {
    //   block [0x82453680..0x824536DC)
	// 82453680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245368C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453694: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245369C: 409A0018  bne cr6, 0x824536b4
	if !ctx.cr[6].eq {
	pc = 0x824536B4; continue 'dispatch;
	}
	// 824536A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824536A4: 4BE7FF35  bl 0x822d35d8
	ctx.lr = 0x824536A8;
	sub_822D35D8(ctx, base);
	// 824536A8: 4BE6C959  bl 0x822c0000
	ctx.lr = 0x824536AC;
	sub_822C0000(ctx, base);
	// 824536AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824536B0: 4BE7F161  bl 0x822d2810
	ctx.lr = 0x824536B4;
	sub_822D2810(ctx, base);
	// 824536B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824536B8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824536BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824536C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824536C4: 4E800421  bctrl
	ctx.lr = 0x824536C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824536C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824536CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824536D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824536D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824536D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824536E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824536E0 size=196
    let mut pc: u32 = 0x824536E0;
    'dispatch: loop {
        match pc {
            0x824536E0 => {
    //   block [0x824536E0..0x824537A4)
	// 824536E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824536E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824536E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824536EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824536F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824536F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824536F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824536FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82453700: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82453704: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82453708: 4BE6D231  bl 0x822c0938
	ctx.lr = 0x8245370C;
	sub_822C0938(ctx, base);
	// 8245370C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82453710: 41820028  beq 0x82453738
	if ctx.cr[0].eq {
	pc = 0x82453738; continue 'dispatch;
	}
	// 82453714: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82453718: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8245371C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82453720: 392B7568  addi r9, r11, 0x7568
	ctx.r[9].s64 = ctx.r[11].s64 + 30056;
	// 82453724: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82453728: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245372C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82453730: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82453734: 48000008  b 0x8245373c
	pc = 0x8245373C; continue 'dispatch;
	// 82453738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245373C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82453740: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453744: 409A0044  bne cr6, 0x82453788
	if !ctx.cr[6].eq {
	pc = 0x82453788; continue 'dispatch;
	}
	// 82453748: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8245374C: 419A001C  beq cr6, 0x82453768
	if ctx.cr[6].eq {
	pc = 0x82453768; continue 'dispatch;
	}
	// 82453750: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453754: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82453758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245375C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82453760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82453764: 4E800421  bctrl
	ctx.lr = 0x82453768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82453768: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245376C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82453770: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82453774: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82453778: 816B36CC  lwz r11, 0x36cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14028 as u32) ) } as u64;
	// 8245377C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82453780: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82453784: 4BE6C87D  bl 0x822c0000
	ctx.lr = 0x82453788;
	sub_822C0000(ctx, base);
	// 82453788: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245378C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453798: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245379C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824537A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824537A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824537A8 size=196
    let mut pc: u32 = 0x824537A8;
    'dispatch: loop {
        match pc {
            0x824537A8 => {
    //   block [0x824537A8..0x8245386C)
	// 824537A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824537AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824537B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824537B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824537B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824537BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824537C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824537C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824537C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824537CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824537D0: 4BE6D169  bl 0x822c0938
	ctx.lr = 0x824537D4;
	sub_822C0938(ctx, base);
	// 824537D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824537D8: 41820028  beq 0x82453800
	if ctx.cr[0].eq {
	pc = 0x82453800; continue 'dispatch;
	}
	// 824537DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824537E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 824537E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824537E8: 392B757C  addi r9, r11, 0x757c
	ctx.r[9].s64 = ctx.r[11].s64 + 30076;
	// 824537EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824537F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824537F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824537F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824537FC: 48000008  b 0x82453804
	pc = 0x82453804; continue 'dispatch;
	// 82453800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82453804: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82453808: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245380C: 409A0044  bne cr6, 0x82453850
	if !ctx.cr[6].eq {
	pc = 0x82453850; continue 'dispatch;
	}
	// 82453810: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82453814: 419A001C  beq cr6, 0x82453830
	if ctx.cr[6].eq {
	pc = 0x82453830; continue 'dispatch;
	}
	// 82453818: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245381C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82453820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453824: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453828: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245382C: 4E800421  bctrl
	ctx.lr = 0x82453830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82453830: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82453834: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82453838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245383C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82453840: 816B36CC  lwz r11, 0x36cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14028 as u32) ) } as u64;
	// 82453844: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82453848: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245384C: 4BE6C7B5  bl 0x822c0000
	ctx.lr = 0x82453850;
	sub_822C0000(ctx, base);
	// 82453850: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245385C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82453864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453870 size=120
    let mut pc: u32 = 0x82453870;
    'dispatch: loop {
        match pc {
            0x82453870 => {
    //   block [0x82453870..0x824538E8)
	// 82453870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453874: 48D548F9  bl 0x831a816c
	ctx.lr = 0x82453878;
	sub_831A8130(ctx, base);
	// 82453878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245387C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82453880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82453884: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82453888: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8245388C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82453890: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82453894: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82453898: 4899EB51  bl 0x82df23e8
	ctx.lr = 0x8245389C;
	sub_82DF23E8(ctx, base);
	// 8245389C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824538A0: 41820014  beq 0x824538b4
	if ctx.cr[0].eq {
	pc = 0x824538B4; continue 'dispatch;
	}
	// 824538A4: 889F0000  lbz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824538A8: 486FDAF1  bl 0x82b51398
	ctx.lr = 0x824538AC;
	sub_82B51398(ctx, base);
	// 824538AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824538B0: 48000008  b 0x824538b8
	pc = 0x824538B8; continue 'dispatch;
	// 824538B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824538B8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824538BC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 824538C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824538C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824538C8: 4BFFFEE1  bl 0x824537a8
	ctx.lr = 0x824538CC;
	sub_824537A8(ctx, base);
	// 824538CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824538D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824538D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824538D8: 4BE6C729  bl 0x822c0000
	ctx.lr = 0x824538DC;
	sub_822C0000(ctx, base);
	// 824538DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824538E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824538E4: 48D548D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824538E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824538E8 size=144
    let mut pc: u32 = 0x824538E8;
    'dispatch: loop {
        match pc {
            0x824538E8 => {
    //   block [0x824538E8..0x82453978)
	// 824538E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824538EC: 48D5487D  bl 0x831a8168
	ctx.lr = 0x824538F0;
	sub_831A8130(ctx, base);
	// 824538F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824538F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824538F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824538FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82453900: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82453904: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82453908: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8245390C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82453910: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82453914: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82453918: 4899EAD1  bl 0x82df23e8
	ctx.lr = 0x8245391C;
	sub_82DF23E8(ctx, base);
	// 8245391C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82453920: 41820024  beq 0x82453944
	if ctx.cr[0].eq {
	pc = 0x82453944; continue 'dispatch;
	}
	// 82453924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82453928: 88DD0000  lbz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245392C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82453930: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82453934: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453938: 486FDCF9  bl 0x82b51630
	ctx.lr = 0x8245393C;
	sub_82B51630(ctx, base);
	// 8245393C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453940: 48000008  b 0x82453948
	pc = 0x82453948; continue 'dispatch;
	// 82453944: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82453948: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8245394C: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 82453950: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453954: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453958: 4BEAD801  bl 0x82301158
	ctx.lr = 0x8245395C;
	sub_82301158(ctx, base);
	// 8245395C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82453960: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453964: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453968: 4BE6C699  bl 0x822c0000
	ctx.lr = 0x8245396C;
	sub_822C0000(ctx, base);
	// 8245396C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82453970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82453974: 48D54844  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82453978 size=400
    let mut pc: u32 = 0x82453978;
    'dispatch: loop {
        match pc {
            0x82453978 => {
    //   block [0x82453978..0x82453B08)
	// 82453978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245397C: 48D547ED  bl 0x831a8168
	ctx.lr = 0x82453980;
	sub_831A8130(ctx, base);
	// 82453980: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453988: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8245398C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82453990: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82453994: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82453998: 409A000C  bne cr6, 0x824539a4
	if !ctx.cr[6].eq {
	pc = 0x824539A4; continue 'dispatch;
	}
	// 8245399C: 9BDF00DC  stb r30, 0xdc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u8 ) };
	// 824539A0: 9BDF00DD  stb r30, 0xdd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(221 as u32), ctx.r[30].u8 ) };
	// 824539A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824539A8: 409A00A0  bne cr6, 0x82453a48
	if !ctx.cr[6].eq {
	pc = 0x82453A48; continue 'dispatch;
	}
	// 824539AC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 824539B0: 4BEAF139  bl 0x82302ae8
	ctx.lr = 0x824539B4;
	sub_82302AE8(ctx, base);
	// 824539B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824539B8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824539BC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824539C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824539C4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824539C8: 419A0024  beq cr6, 0x824539ec
	if ctx.cr[6].eq {
	pc = 0x824539EC; continue 'dispatch;
	}
	// 824539CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824539D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 824539D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824539D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 824539DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824539E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 824539E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824539E8: 4082FFE8  bne 0x824539d0
	if !ctx.cr[0].eq {
	pc = 0x824539D0; continue 'dispatch;
	}
	// 824539EC: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 824539F0: 3B810058  addi r28, r1, 0x58
	ctx.r[28].s64 = ctx.r[1].s64 + 88;
	// 824539F4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 824539F8: 48BB55C1  bl 0x83008fb8
	ctx.lr = 0x824539FC;
	sub_83008FB8(ctx, base);
	// 824539FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82453A00: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82453A04: 388B7590  addi r4, r11, 0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + 30096;
	// 82453A08: 38A0007B  li r5, 0x7b
	ctx.r[5].s64 = 123;
	// 82453A0C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82453A10: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82453A14: 48A035D5  bl 0x82e56fe8
	ctx.lr = 0x82453A18;
	sub_82E56FE8(ctx, base);
	// 82453A18: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82453A1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453A20: 419A0008  beq cr6, 0x82453a28
	if ctx.cr[6].eq {
	pc = 0x82453A28; continue 'dispatch;
	}
	// 82453A24: 4BE6CE6D  bl 0x822c0890
	ctx.lr = 0x82453A28;
	sub_822C0890(ctx, base);
	// 82453A28: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82453A2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453A30: 419A0008  beq cr6, 0x82453a38
	if ctx.cr[6].eq {
	pc = 0x82453A38; continue 'dispatch;
	}
	// 82453A34: 4BE6CE5D  bl 0x822c0890
	ctx.lr = 0x82453A38;
	sub_822C0890(ctx, base);
	// 82453A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82453A3C: 9BDF00DD  stb r30, 0xdd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(221 as u32), ctx.r[30].u8 ) };
	// 82453A40: 997F00DC  stb r11, 0xdc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u8 ) };
	// 82453A44: 480000B8  b 0x82453afc
	pc = 0x82453AFC; continue 'dispatch;
	// 82453A48: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82453A4C: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 82453A50: 815F00D0  lwz r10, 0xd0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82453A54: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82453A58: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82453A5C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82453A60: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82453A64: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82453A68: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 82453A6C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82453A70: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82453A74: 4BFFFE75  bl 0x824538e8
	ctx.lr = 0x82453A78;
	sub_824538E8(ctx, base);
	// 82453A78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453A7C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82453A80: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82453A84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82453A8C: 419A0024  beq cr6, 0x82453ab0
	if ctx.cr[6].eq {
	pc = 0x82453AB0; continue 'dispatch;
	}
	// 82453A90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82453A94: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82453A98: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82453A9C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82453AA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82453AA4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82453AA8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82453AAC: 4082FFE8  bne 0x82453a94
	if !ctx.cr[0].eq {
	pc = 0x82453A94; continue 'dispatch;
	}
	// 82453AB0: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82453AB4: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 82453AB8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82453ABC: 48BB54FD  bl 0x83008fb8
	ctx.lr = 0x82453AC0;
	sub_83008FB8(ctx, base);
	// 82453AC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82453AC4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82453AC8: 388B7590  addi r4, r11, 0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + 30096;
	// 82453ACC: 38A00082  li r5, 0x82
	ctx.r[5].s64 = 130;
	// 82453AD0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82453AD4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82453AD8: 48A03511  bl 0x82e56fe8
	ctx.lr = 0x82453ADC;
	sub_82E56FE8(ctx, base);
	// 82453ADC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82453AE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453AE4: 419A0008  beq cr6, 0x82453aec
	if ctx.cr[6].eq {
	pc = 0x82453AEC; continue 'dispatch;
	}
	// 82453AE8: 4BE6CDA9  bl 0x822c0890
	ctx.lr = 0x82453AEC;
	sub_822C0890(ctx, base);
	// 82453AEC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82453AF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453AF4: 419A0008  beq cr6, 0x82453afc
	if ctx.cr[6].eq {
	pc = 0x82453AFC; continue 'dispatch;
	}
	// 82453AF8: 4BE6CD99  bl 0x822c0890
	ctx.lr = 0x82453AFC;
	sub_822C0890(ctx, base);
	// 82453AFC: 93BF00C8  stw r29, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[29].u32 ) };
	// 82453B00: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82453B04: 48D546B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453B08 size=372
    let mut pc: u32 = 0x82453B08;
    'dispatch: loop {
        match pc {
            0x82453B08 => {
    //   block [0x82453B08..0x82453C7C)
	// 82453B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453B0C: 48D54661  bl 0x831a816c
	ctx.lr = 0x82453B10;
	sub_831A8130(ctx, base);
	// 82453B10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453B14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453B18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82453B1C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82453B20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82453B24: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82453B28: 409A0008  bne cr6, 0x82453b30
	if !ctx.cr[6].eq {
	pc = 0x82453B30; continue 'dispatch;
	}
	// 82453B2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82453B30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453B34: 480B4C6D  bl 0x825087a0
	ctx.lr = 0x82453B38;
	sub_825087A0(ctx, base);
	// 82453B38: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82453B3C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82453B40: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 82453B44: 4899FEC5  bl 0x82df3a08
	ctx.lr = 0x82453B48;
	sub_82DF3A08(ctx, base);
	// 82453B48: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82453B4C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82453B50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453B54: 480B4C2D  bl 0x82508780
	ctx.lr = 0x82453B58;
	sub_82508780(ctx, base);
	// 82453B58: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82453B5C: 4899F8CD  bl 0x82df3428
	ctx.lr = 0x82453B60;
	sub_82DF3428(ctx, base);
	// 82453B60: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82453B64: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82453B68: 808BE26C  lwz r4, -0x1d94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7572 as u32) ) } as u64;
	// 82453B6C: 4899FE9D  bl 0x82df3a08
	ctx.lr = 0x82453B70;
	sub_82DF3A08(ctx, base);
	// 82453B70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82453B74: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82453B78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453B7C: 480B4C05  bl 0x82508780
	ctx.lr = 0x82453B80;
	sub_82508780(ctx, base);
	// 82453B80: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82453B84: 4899F8A5  bl 0x82df3428
	ctx.lr = 0x82453B88;
	sub_82DF3428(ctx, base);
	// 82453B88: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453B8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453B90: 388BFF6C  addi r4, r11, -0x94
	ctx.r[4].s64 = ctx.r[11].s64 + -148;
	// 82453B94: 409A0008  bne cr6, 0x82453b9c
	if !ctx.cr[6].eq {
	pc = 0x82453B9C; continue 'dispatch;
	}
	// 82453B98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82453B9C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82453BA0: 480D46F1  bl 0x82528290
	ctx.lr = 0x82453BA4;
	sub_82528290(ctx, base);
	// 82453BA4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82453BA8: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 82453BAC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82453BB0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82453BB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453BB8: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82453BBC: 4BE708A5  bl 0x822c4460
	ctx.lr = 0x82453BC0;
	sub_822C4460(ctx, base);
	// 82453BC0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82453BC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453BC8: 419A0008  beq cr6, 0x82453bd0
	if ctx.cr[6].eq {
	pc = 0x82453BD0; continue 'dispatch;
	}
	// 82453BCC: 4BE6CCC5  bl 0x822c0890
	ctx.lr = 0x82453BD0;
	sub_822C0890(ctx, base);
	// 82453BD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82453BD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82453BD8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82453BDC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82453BE0: 4BFFFC91  bl 0x82453870
	ctx.lr = 0x82453BE4;
	sub_82453870(ctx, base);
	// 82453BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453BE8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82453BEC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82453BF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453BF4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82453BF8: 419A0024  beq cr6, 0x82453c1c
	if ctx.cr[6].eq {
	pc = 0x82453C1C; continue 'dispatch;
	}
	// 82453BFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82453C00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82453C04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82453C08: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82453C0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82453C10: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82453C14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82453C18: 4082FFE8  bne 0x82453c00
	if !ctx.cr[0].eq {
	pc = 0x82453C00; continue 'dispatch;
	}
	// 82453C1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453C20: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 82453C24: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82453C28: 48BB5391  bl 0x83008fb8
	ctx.lr = 0x82453C2C;
	sub_83008FB8(ctx, base);
	// 82453C2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82453C30: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82453C34: 388B7590  addi r4, r11, 0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + 30096;
	// 82453C38: 38A00032  li r5, 0x32
	ctx.r[5].s64 = 50;
	// 82453C3C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82453C40: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82453C44: 48A033A5  bl 0x82e56fe8
	ctx.lr = 0x82453C48;
	sub_82E56FE8(ctx, base);
	// 82453C48: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82453C4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453C50: 419A0008  beq cr6, 0x82453c58
	if ctx.cr[6].eq {
	pc = 0x82453C58; continue 'dispatch;
	}
	// 82453C54: 4BE6CC3D  bl 0x822c0890
	ctx.lr = 0x82453C58;
	sub_822C0890(ctx, base);
	// 82453C58: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82453C5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453C60: 419A0008  beq cr6, 0x82453c68
	if ctx.cr[6].eq {
	pc = 0x82453C68; continue 'dispatch;
	}
	// 82453C64: 4BE6CC2D  bl 0x822c0890
	ctx.lr = 0x82453C68;
	sub_822C0890(ctx, base);
	// 82453C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453C6C: 809F00C8  lwz r4, 0xc8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82453C70: 4BFFFD09  bl 0x82453978
	ctx.lr = 0x82453C74;
	sub_82453978(ctx, base);
	// 82453C74: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82453C78: 48D54544  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453C80 size=388
    let mut pc: u32 = 0x82453C80;
    'dispatch: loop {
        match pc {
            0x82453C80 => {
    //   block [0x82453C80..0x82453E04)
	// 82453C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453C84: 48D544DD  bl 0x831a8160
	ctx.lr = 0x82453C88;
	sub_831A8130(ctx, base);
	// 82453C88: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453C8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82453C90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82453C98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82453C9C: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82453CA0: 4899DF59  bl 0x82df1bf8
	ctx.lr = 0x82453CA4;
	sub_82DF1BF8(ctx, base);
	// 82453CA4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82453CA8: 486F35B1  bl 0x82b47258
	ctx.lr = 0x82453CAC;
	sub_82B47258(ctx, base);
	// 82453CAC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82453CB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82453CB4: 4899DFDD  bl 0x82df1c90
	ctx.lr = 0x82453CB8;
	sub_82DF1C90(ctx, base);
	// 82453CB8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82453CBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453CC0: 419A0010  beq cr6, 0x82453cd0
	if ctx.cr[6].eq {
	pc = 0x82453CD0; continue 'dispatch;
	}
	// 82453CC4: 815F00D4  lwz r10, 0xd4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82453CC8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82453CCC: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82453CD0: 895F00DD  lbz r10, 0xdd(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(221 as u32) ) } as u64;
	// 82453CD4: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 82453CD8: 839F00C8  lwz r28, 0xc8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82453CDC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82453CE0: 418200E8  beq 0x82453dc8
	if ctx.cr[0].eq {
	pc = 0x82453DC8; continue 'dispatch;
	}
	// 82453CE4: 897F00DC  lbz r11, 0xdc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 82453CE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82453CEC: 4182006C  beq 0x82453d58
	if ctx.cr[0].eq {
	pc = 0x82453D58; continue 'dispatch;
	}
	// 82453CF0: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82453CF4: 815F00C0  lwz r10, 0xc0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82453CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453CFC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82453D00: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82453D04: 419A0024  beq cr6, 0x82453d28
	if ctx.cr[6].eq {
	pc = 0x82453D28; continue 'dispatch;
	}
	// 82453D08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82453D0C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82453D10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82453D14: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82453D18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82453D1C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82453D20: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82453D24: 4082FFE8  bne 0x82453d0c
	if !ctx.cr[0].eq {
	pc = 0x82453D0C; continue 'dispatch;
	}
	// 82453D28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453D2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82453D30: 480BB7E9  bl 0x8250f518
	ctx.lr = 0x82453D34;
	sub_8250F518(ctx, base);
	// 82453D34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453D38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453D3C: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82453D40: 409A0008  bne cr6, 0x82453d48
	if !ctx.cr[6].eq {
	pc = 0x82453D48; continue 'dispatch;
	}
	// 82453D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82453D48: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82453D4C: 480D45CD  bl 0x82528318
	ctx.lr = 0x82453D50;
	sub_82528318(ctx, base);
	// 82453D50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82453D54: 48000068  b 0x82453dbc
	pc = 0x82453DBC; continue 'dispatch;
	// 82453D58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453D5C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82453D60: 480BB7B9  bl 0x8250f518
	ctx.lr = 0x82453D64;
	sub_8250F518(ctx, base);
	// 82453D64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453D68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453D6C: 3BABFF6C  addi r29, r11, -0x94
	ctx.r[29].s64 = ctx.r[11].s64 + -148;
	// 82453D70: 409A0008  bne cr6, 0x82453d78
	if !ctx.cr[6].eq {
	pc = 0x82453D78; continue 'dispatch;
	}
	// 82453D74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82453D78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82453D7C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82453D80: 480BB799  bl 0x8250f518
	ctx.lr = 0x82453D84;
	sub_8250F518(ctx, base);
	// 82453D84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453D88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453D8C: 3BCBFF6C  addi r30, r11, -0x94
	ctx.r[30].s64 = ctx.r[11].s64 + -148;
	// 82453D90: 409A0008  bne cr6, 0x82453d98
	if !ctx.cr[6].eq {
	pc = 0x82453D98; continue 'dispatch;
	}
	// 82453D94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82453D98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82453D9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82453DA0: 480D4971  bl 0x82528710
	ctx.lr = 0x82453DA4;
	sub_82528710(ctx, base);
	// 82453DA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82453DA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82453DAC: 480D456D  bl 0x82528318
	ctx.lr = 0x82453DB0;
	sub_82528318(ctx, base);
	// 82453DB0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82453DB4: 4899DEDD  bl 0x82df1c90
	ctx.lr = 0x82453DB8;
	sub_82DF1C90(ctx, base);
	// 82453DB8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82453DBC: 4899DED5  bl 0x82df1c90
	ctx.lr = 0x82453DC0;
	sub_82DF1C90(ctx, base);
	// 82453DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82453DC4: 997F00DD  stb r11, 0xdd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(221 as u32), ctx.r[11].u8 ) };
	// 82453DC8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82453DCC: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82453DD0: 41820014  beq 0x82453de4
	if ctx.cr[0].eq {
	pc = 0x82453DE4; continue 'dispatch;
	}
	// 82453DD4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82453DD8: 7F1BE000  cmpw cr6, r27, r28
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82453DDC: 41990008  bgt cr6, 0x82453de4
	if ctx.cr[6].gt {
	pc = 0x82453DE4; continue 'dispatch;
	}
	// 82453DE0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82453DE4: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82453DE8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82453DEC: 419A0010  beq cr6, 0x82453dfc
	if ctx.cr[6].eq {
	pc = 0x82453DFC; continue 'dispatch;
	}
	// 82453DF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82453DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453DF8: 4BFFFB81  bl 0x82453978
	ctx.lr = 0x82453DFC;
	sub_82453978(ctx, base);
	// 82453DFC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82453E00: 48D543B0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453E08 size=108
    let mut pc: u32 = 0x82453E08;
    'dispatch: loop {
        match pc {
            0x82453E08 => {
    //   block [0x82453E08..0x82453E74)
	// 82453E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453E10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82453E14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453E18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453E1C: 480BD2D5  bl 0x825110f0
	ctx.lr = 0x82453E20;
	sub_825110F0(ctx, base);
	// 82453E20: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82453E24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82453E28: 392B75EC  addi r9, r11, 0x75ec
	ctx.r[9].s64 = ctx.r[11].s64 + 30188;
	// 82453E2C: 394A75D8  addi r10, r10, 0x75d8
	ctx.r[10].s64 = ctx.r[10].s64 + 30168;
	// 82453E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82453E34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82453E38: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82453E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453E40: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82453E44: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82453E48: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82453E4C: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82453E50: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 82453E54: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 82453E58: 997F00DC  stb r11, 0xdc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u8 ) };
	// 82453E5C: 997F00DD  stb r11, 0xdd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(221 as u32), ctx.r[11].u8 ) };
	// 82453E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82453E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453E78 size=8
    let mut pc: u32 = 0x82453E78;
    'dispatch: loop {
        match pc {
            0x82453E78 => {
    //   block [0x82453E78..0x82453E80)
	// 82453E78: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82453E7C: 48000004  b 0x82453e80
	sub_82453E80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453E80 size=104
    let mut pc: u32 = 0x82453E80;
    'dispatch: loop {
        match pc {
            0x82453E80 => {
    //   block [0x82453E80..0x82453EE8)
	// 82453E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82453E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82453E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453E98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82453E9C: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 82453EA0: 4805D401  bl 0x824b12a0
	ctx.lr = 0x82453EA4;
	sub_824B12A0(ctx, base);
	// 82453EA4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82453EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453EAC: 419A0008  beq cr6, 0x82453eb4
	if ctx.cr[6].eq {
	pc = 0x82453EB4; continue 'dispatch;
	}
	// 82453EB0: 4BE6C9E1  bl 0x822c0890
	ctx.lr = 0x82453EB4;
	sub_822C0890(ctx, base);
	// 82453EB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453EB8: 480BD2E1  bl 0x82511198
	ctx.lr = 0x82453EBC;
	sub_82511198(ctx, base);
	// 82453EBC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82453EC0: 4182000C  beq 0x82453ecc
	if ctx.cr[0].eq {
	pc = 0x82453ECC; continue 'dispatch;
	}
	// 82453EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453EC8: 4899E511  bl 0x82df23d8
	ctx.lr = 0x82453ECC;
	sub_82DF23D8(ctx, base);
	// 82453ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82453ED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453EDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82453EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82453EE8 size=108
    let mut pc: u32 = 0x82453EE8;
    'dispatch: loop {
        match pc {
            0x82453EE8 => {
    //   block [0x82453EE8..0x82453F54)
	// 82453EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82453EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82453EF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82453EF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82453EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82453EFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82453F00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82453F04: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82453F08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82453F0C: 4BFFF7D5  bl 0x824536e0
	ctx.lr = 0x82453F10;
	sub_824536E0(ctx, base);
	// 82453F10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82453F14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82453F18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82453F1C: 4BE6C0E5  bl 0x822c0000
	ctx.lr = 0x82453F20;
	sub_822C0000(ctx, base);
	// 82453F20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82453F24: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 82453F28: 4875F1E1  bl 0x82bb3108
	ctx.lr = 0x82453F2C;
	sub_82BB3108(ctx, base);
	// 82453F2C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82453F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82453F34: 419A0008  beq cr6, 0x82453f3c
	if ctx.cr[6].eq {
	pc = 0x82453F3C; continue 'dispatch;
	}
	// 82453F38: 4BE6C959  bl 0x822c0890
	ctx.lr = 0x82453F3C;
	sub_822C0890(ctx, base);
	// 82453F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82453F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82453F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82453F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82453F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82453F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453F58 size=84
    let mut pc: u32 = 0x82453F58;
    'dispatch: loop {
        match pc {
            0x82453F58 => {
    //   block [0x82453F58..0x82453FAC)
	// 82453F58: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82453F5C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82453F60: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82453F64: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82453F68: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82453F6C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82453F70: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82453F74: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82453F78: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82453F7C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82453F80: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82453F84: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82453F88: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82453F8C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82453F90: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82453F94: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82453F98: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82453F9C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82453FA0: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82453FA4: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82453FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453FB0 size=8
    let mut pc: u32 = 0x82453FB0;
    'dispatch: loop {
        match pc {
            0x82453FB0 => {
    //   block [0x82453FB0..0x82453FB8)
	// 82453FB0: 98830000  stb r4, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 82453FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453FB8 size=20
    let mut pc: u32 = 0x82453FB8;
    'dispatch: loop {
        match pc {
            0x82453FB8 => {
    //   block [0x82453FB8..0x82453FCC)
	// 82453FB8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82453FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82453FC0: 409A000C  bne cr6, 0x82453fcc
	if !ctx.cr[6].eq {
		sub_82453FCC(ctx, base);
		return;
	}
	// 82453FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82453FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453FCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453FCC size=16
    let mut pc: u32 = 0x82453FCC;
    'dispatch: loop {
        match pc {
            0x82453FCC => {
    //   block [0x82453FCC..0x82453FDC)
	// 82453FCC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82453FD0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82453FD4: 7D632E70  srawi r3, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82453FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453FE0 size=16
    let mut pc: u32 = 0x82453FE0;
    'dispatch: loop {
        match pc {
            0x82453FE0 => {
    //   block [0x82453FE0..0x82453FF0)
	// 82453FE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82453FE4: 548A2834  slwi r10, r4, 5
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82453FE8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82453FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82453FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82453FF0 size=20
    let mut pc: u32 = 0x82453FF0;
    'dispatch: loop {
        match pc {
            0x82453FF0 => {
    //   block [0x82453FF0..0x82454004)
	// 82453FF0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82453FF4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82453FF8: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82453FFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82454000: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454004 size=40
    let mut pc: u32 = 0x82454004;
    'dispatch: loop {
        match pc {
            0x82454004 => {
    //   block [0x82454004..0x8245402C)
	// 82454004: E9240000  ld r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82454008: E94B0010  ld r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 8245400C: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82454010: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82454014: 41980008  blt cr6, 0x8245401c
	if ctx.cr[6].lt {
	pc = 0x8245401C; continue 'dispatch;
	}
	// 82454018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245401C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82454020: 4182000C  beq 0x8245402c
	if ctx.cr[0].eq {
		sub_8245402C(ctx, base);
		return;
	}
	// 82454024: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454028: 4800000C  b 0x82454034
	sub_8245402C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245402C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245402C size=24
    let mut pc: u32 = 0x8245402C;
    'dispatch: loop {
        match pc {
            0x8245402C => {
    //   block [0x8245402C..0x82454044)
	// 8245402C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82454030: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454034: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82454038: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245403C: 419AFFCC  beq cr6, 0x82454008
	if ctx.cr[6].eq {
		sub_82454004(ctx, base);
		return;
	}
	// 82454040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454048 size=64
    let mut pc: u32 = 0x82454048;
    'dispatch: loop {
        match pc {
            0x82454048 => {
    //   block [0x82454048..0x82454088)
	// 82454048: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245404C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454050: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82454054: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454058: 892A0029  lbz r9, 0x29(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 8245405C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454060: 409A0008  bne cr6, 0x82454068
	if !ctx.cr[6].eq {
	pc = 0x82454068; continue 'dispatch;
	}
	// 82454064: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82454068: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245406C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82454070: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454074: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454078: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245407C: 409A000C  bne cr6, 0x82454088
	if !ctx.cr[6].eq {
		sub_82454088(ctx, base);
		return;
	}
	// 82454080: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82454084: 48000020  b 0x824540a4
	sub_824540A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454088 size=24
    let mut pc: u32 = 0x82454088;
    'dispatch: loop {
        match pc {
            0x82454088 => {
    //   block [0x82454088..0x824540A0)
	// 82454088: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245408C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454090: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82454094: 409A000C  bne cr6, 0x824540a0
	if !ctx.cr[6].eq {
		sub_824540A0(ctx, base);
		return;
	}
	// 82454098: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245409C: 48000008  b 0x824540a4
	sub_824540A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824540A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824540A0 size=16
    let mut pc: u32 = 0x824540A0;
    'dispatch: loop {
        match pc {
            0x824540A0 => {
    //   block [0x824540A0..0x824540B0)
	// 824540A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824540A4: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824540A8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824540AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824540B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824540B0 size=64
    let mut pc: u32 = 0x824540B0;
    'dispatch: loop {
        match pc {
            0x824540B0 => {
    //   block [0x824540B0..0x824540F0)
	// 824540B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824540B4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824540B8: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824540BC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824540C0: 892A0029  lbz r9, 0x29(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 824540C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824540C8: 409A0008  bne cr6, 0x824540d0
	if !ctx.cr[6].eq {
	pc = 0x824540D0; continue 'dispatch;
	}
	// 824540CC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824540D0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824540D4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824540D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824540DC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824540E0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824540E4: 409A000C  bne cr6, 0x824540f0
	if !ctx.cr[6].eq {
		sub_824540F0(ctx, base);
		return;
	}
	// 824540E8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824540EC: 48000020  b 0x8245410c
	sub_82454108(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824540F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824540F0 size=24
    let mut pc: u32 = 0x824540F0;
    'dispatch: loop {
        match pc {
            0x824540F0 => {
    //   block [0x824540F0..0x82454108)
	// 824540F0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824540F4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824540F8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824540FC: 409A000C  bne cr6, 0x82454108
	if !ctx.cr[6].eq {
		sub_82454108(ctx, base);
		return;
	}
	// 82454100: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454104: 48000008  b 0x8245410c
	sub_82454108(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454108 size=16
    let mut pc: u32 = 0x82454108;
    'dispatch: loop {
        match pc {
            0x82454108 => {
    //   block [0x82454108..0x82454118)
	// 82454108: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245410C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82454110: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82454114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454118 size=64
    let mut pc: u32 = 0x82454118;
    'dispatch: loop {
        match pc {
            0x82454118 => {
    //   block [0x82454118..0x82454158)
	// 82454118: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245411C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454120: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82454124: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454128: 892A0021  lbz r9, 0x21(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 8245412C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454130: 409A0008  bne cr6, 0x82454138
	if !ctx.cr[6].eq {
	pc = 0x82454138; continue 'dispatch;
	}
	// 82454134: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82454138: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245413C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82454140: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454144: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454148: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245414C: 409A000C  bne cr6, 0x82454158
	if !ctx.cr[6].eq {
		sub_82454158(ctx, base);
		return;
	}
	// 82454150: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82454154: 48000020  b 0x82454174
	sub_82454170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454158 size=24
    let mut pc: u32 = 0x82454158;
    'dispatch: loop {
        match pc {
            0x82454158 => {
    //   block [0x82454158..0x82454170)
	// 82454158: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245415C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454160: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82454164: 409A000C  bne cr6, 0x82454170
	if !ctx.cr[6].eq {
		sub_82454170(ctx, base);
		return;
	}
	// 82454168: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245416C: 48000008  b 0x82454174
	sub_82454170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454170 size=16
    let mut pc: u32 = 0x82454170;
    'dispatch: loop {
        match pc {
            0x82454170 => {
    //   block [0x82454170..0x82454180)
	// 82454170: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454174: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82454178: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245417C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454180 size=12
    let mut pc: u32 = 0x82454180;
    'dispatch: loop {
        match pc {
            0x82454180 => {
    //   block [0x82454180..0x8245418C)
	// 82454180: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454184: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82454188: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245418C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245418C size=40
    //   switch @ 0x824541A4: r11 with 10 label(s)
    //       case  0 → 0x824541B4
    //       case  1 → 0x824541C4
    //       case  2 → 0x824541D4
    //       case  3 → 0x824541E4
    //       case  4 → 0x824541B4
    //       case  5 → 0x824541C4
    //       case  6 → 0x824541D4
    //       case  7 → 0x824541E4
    //       case  8 → 0x824541F4
    //       case  9 → 0x8245420C
    let mut pc: u32 = 0x8245418C;
    'dispatch: loop {
        match pc {
            0x8245418C => {
    //   block [0x8245418C..0x824541B4)
	// 8245418C: 3D808202  lis r12, -0x7dfe
	ctx.r[12].s64 = -2113798144;
	// 82454190: 398C7628  addi r12, r12, 0x7628
	ctx.r[12].s64 = ctx.r[12].s64 + 30248;
	// 82454194: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82454198: 3D808245  lis r12, -0x7dbb
	ctx.r[12].s64 = -2109407232;
	// 8245419C: 398C41B4  addi r12, r12, 0x41b4
	ctx.r[12].s64 = ctx.r[12].s64 + 16820;
	// 824541A0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 824541A4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 824541A8: 60000000  nop
	// 824541AC: 60000000  nop
	// 824541B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824541B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824541B4 size=16
    let mut pc: u32 = 0x824541B4;
    'dispatch: loop {
        match pc {
            0x824541B4 => {
    //   block [0x824541B4..0x824541C4)
	// 824541B4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824541B8: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824541BC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824541C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824541C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824541C4 size=16
    let mut pc: u32 = 0x824541C4;
    'dispatch: loop {
        match pc {
            0x824541C4 => {
    //   block [0x824541C4..0x824541D4)
	// 824541C4: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824541C8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824541CC: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 824541D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824541D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824541D4 size=16
    let mut pc: u32 = 0x824541D4;
    'dispatch: loop {
        match pc {
            0x824541D4 => {
    //   block [0x824541D4..0x824541E4)
	// 824541D4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824541D8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824541DC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824541E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824541E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824541E4 size=16
    let mut pc: u32 = 0x824541E4;
    'dispatch: loop {
        match pc {
            0x824541E4 => {
    //   block [0x824541E4..0x824541F4)
	// 824541E4: E9640000  ld r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 824541E8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824541EC: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 824541F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824541F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824541F4 size=24
    let mut pc: u32 = 0x824541F4;
    'dispatch: loop {
        match pc {
            0x824541F4 => {
    //   block [0x824541F4..0x8245420C)
	// 824541F4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824541F8: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824541FC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82454200: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82454204: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82454208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245420C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245420C size=72
    let mut pc: u32 = 0x8245420C;
    'dispatch: loop {
        match pc {
            0x8245420C => {
    //   block [0x8245420C..0x82454254)
	// 8245420C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454210: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454214: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82454218: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8245421C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82454220: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454224: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82454228: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8245422C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82454230: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454234: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82454238: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8245423C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82454240: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454244: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82454248: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8245424C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82454250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82454258 size=268
    //   switch @ 0x82454290: r10 with 10 label(s)
    //       case  0 → 0x824542A0
    //       case  1 → 0x824542B0
    //       case  2 → 0x824542C8
    //       case  3 → 0x824542F0
    //       case  4 → 0x824542A0
    //       case  5 → 0x824542B0
    //       case  6 → 0x824542C8
    //       case  7 → 0x824542F0
    //       case  8 → 0x824542C8
    //       case  9 → 0x82454300
    let mut pc: u32 = 0x82454258;
    'dispatch: loop {
        match pc {
            0x82454258 => {
    //   block [0x82454258..0x824542A0)
	// 82454258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245425C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454264: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82454268: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8245426C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454270: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 82454274: 419900E0  bgt cr6, 0x82454354
	if ctx.cr[6].gt {
	pc = 0x82454354; continue 'dispatch;
	}
	// 82454278: 3D808202  lis r12, -0x7dfe
	ctx.r[12].s64 = -2113798144;
	// 8245427C: 398C7638  addi r12, r12, 0x7638
	ctx.r[12].s64 = ctx.r[12].s64 + 30264;
	// 82454280: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82454284: 3D808245  lis r12, -0x7dbb
	ctx.r[12].s64 = -2109407232;
	// 82454288: 398C42A0  addi r12, r12, 0x42a0
	ctx.r[12].s64 = ctx.r[12].s64 + 17056;
	// 8245428C: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82454290: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82454294: 60000000  nop
	// 82454298: 60000000  nop
	// 8245429C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x824542A0 => {
    //   block [0x824542A0..0x824542B0)
	// 824542A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824542A4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824542A8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824542AC: 480000A8  b 0x82454354
	pc = 0x82454354; continue 'dispatch;
            }
            0x824542B0 => {
    //   block [0x824542B0..0x824542C8)
	// 824542B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824542B4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824542B8: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824542BC: 896B0001  lbz r11, 1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824542C0: 99630001  stb r11, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 824542C4: 48000090  b 0x82454354
	pc = 0x82454354; continue 'dispatch;
            }
            0x824542C8 => {
    //   block [0x824542C8..0x824542F0)
	// 824542C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824542CC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824542D0: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824542D4: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 824542D8: 99430001  stb r10, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 824542DC: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 824542E0: 99430002  stb r10, 2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 824542E4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824542E8: 99630003  stb r11, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 824542EC: 48000068  b 0x82454354
	pc = 0x82454354; continue 'dispatch;
            }
            0x824542F0 => {
    //   block [0x824542F0..0x82454300)
	// 824542F0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824542F4: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824542F8: 48D54219  bl 0x831a8510
	ctx.lr = 0x824542FC;
	sub_831A8510(ctx, base);
	// 824542FC: 48000058  b 0x82454354
	pc = 0x82454354; continue 'dispatch;
            }
            0x82454300 => {
    //   block [0x82454300..0x82454364)
	// 82454300: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454304: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454308: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8245430C: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82454310: 99430001  stb r10, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82454314: 894B0002  lbz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82454318: 99430002  stb r10, 2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 8245431C: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82454320: 99430003  stb r10, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82454324: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454328: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8245432C: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454330: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82454334: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454338: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8245433C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454340: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82454344: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82454348: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245434C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82454350: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82454354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82454358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245435C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82454360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454368 size=44
    let mut pc: u32 = 0x82454368;
    'dispatch: loop {
        match pc {
            0x82454368 => {
    //   block [0x82454368..0x82454394)
	// 82454368: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245436C: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82454370: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82454374: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454378: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245437C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454380: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454384: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82454388: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245438C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82454390: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82454394 size=36
    let mut pc: u32 = 0x82454394;
    'dispatch: loop {
        match pc {
            0x82454394 => {
    //   block [0x82454394..0x824543B8)
	// 82454394: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82454398: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8245439C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824543A0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 824543A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824543A8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 824543AC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824543B0: 4082FFE8  bne 0x82454398
	if !ctx.cr[0].eq {
	pc = 0x82454398; continue 'dispatch;
	}
	// 824543B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824543B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824543B8 size=108
    let mut pc: u32 = 0x824543B8;
    'dispatch: loop {
        match pc {
            0x824543B8 => {
    //   block [0x824543B8..0x82454424)
	// 824543B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824543BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824543C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824543C4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 824543C8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824543CC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 824543D0: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 824543D4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 824543D8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 824543DC: 4899DCED  bl 0x82df20c8
	ctx.lr = 0x824543E0;
	sub_82DF20C8(ctx, base);
	// 824543E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824543E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824543E8: 41820008  beq 0x824543f0
	if ctx.cr[0].eq {
	pc = 0x824543F0; continue 'dispatch;
	}
	// 824543EC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824543F0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824543F4: 41820008  beq 0x824543fc
	if ctx.cr[0].eq {
	pc = 0x824543FC; continue 'dispatch;
	}
	// 824543F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824543FC: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82454400: 41820008  beq 0x82454408
	if ctx.cr[0].eq {
	pc = 0x82454408; continue 'dispatch;
	}
	// 82454404: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82454408: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8245440C: 99430029  stb r10, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 82454410: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82454414: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82454418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245441C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82454420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454428 size=92
    let mut pc: u32 = 0x82454428;
    'dispatch: loop {
        match pc {
            0x82454428 => {
    //   block [0x82454428..0x82454484)
	// 82454428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245442C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82454434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454438: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8245443C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82454440: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82454444: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 82454448: 38890004  addi r4, r9, 4
	ctx.r[4].s64 = ctx.r[9].s64 + 4;
	// 8245444C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454450: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82454454: 991F0000  stb r8, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82454458: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245445C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82454460: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454464: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454468: 4BE6FFF9  bl 0x822c4460
	ctx.lr = 0x8245446C;
	sub_822C4460(ctx, base);
	// 8245446C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82454474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82454478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245447C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82454480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454488 size=100
    let mut pc: u32 = 0x82454488;
    'dispatch: loop {
        match pc {
            0x82454488 => {
    //   block [0x82454488..0x824544EC)
	// 82454488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82454494: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454498: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8245449C: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 824544A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824544A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824544A8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 824544AC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824544B0: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824544B4: 4BFC7995  bl 0x8241be48
	ctx.lr = 0x824544B8;
	sub_8241BE48(ctx, base);
	// 824544B8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824544BC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824544C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824544C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824544C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824544CC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824544D0: 419A0008  beq cr6, 0x824544d8
	if ctx.cr[6].eq {
	pc = 0x824544D8; continue 'dispatch;
	}
	// 824544D4: 4BE6C3BD  bl 0x822c0890
	ctx.lr = 0x824544D8;
	sub_822C0890(ctx, base);
	// 824544D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824544DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824544E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824544E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824544E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824544F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824544F0 size=120
    let mut pc: u32 = 0x824544F0;
    'dispatch: loop {
        match pc {
            0x824544F0 => {
    //   block [0x824544F0..0x82454568)
	// 824544F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824544F4: 48D53C79  bl 0x831a816c
	ctx.lr = 0x824544F8;
	sub_831A8130(ctx, base);
	// 824544F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824544FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82454500: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82454504: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82454508: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245450C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82454510: 4BFFFAE1  bl 0x82453ff0
	ctx.lr = 0x82454514;
	sub_82453FF0(ctx, base);
	// 82454514: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454518: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8245451C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82454520: 419A002C  beq cr6, 0x8245454c
	if ctx.cr[6].eq {
	pc = 0x8245454C; continue 'dispatch;
	}
	// 82454524: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82454528: E9230010  ld r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8245452C: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82454530: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82454534: 41980008  blt cr6, 0x8245453c
	if ctx.cr[6].lt {
	pc = 0x8245453C; continue 'dispatch;
	}
	// 82454538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245453C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82454540: 4082000C  bne 0x8245454c
	if !ctx.cr[0].eq {
	pc = 0x8245454C; continue 'dispatch;
	}
	// 82454544: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82454548: 4800000C  b 0x82454554
	pc = 0x82454554; continue 'dispatch;
	// 8245454C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82454550: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82454554: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454558: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245455C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82454560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82454564: 48D53C58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82454568 size=52
    let mut pc: u32 = 0x82454568;
    'dispatch: loop {
        match pc {
            0x82454568 => {
    //   block [0x82454568..0x8245459C)
	// 82454568: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245456C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454570: 892B0021  lbz r9, 0x21(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82454574: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454578: 409A0038  bne cr6, 0x824545b0
	if !ctx.cr[6].eq {
		sub_8245459C(ctx, base);
		return;
	}
	// 8245457C: 89250000  lbz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454580: 890B000C  lbz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82454584: 7D094010  subfc r8, r9, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[9].u32;
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82454588: 7D084110  subfe r8, r8, r8
	let x = (!ctx.r[8].u32);
	let y = ctx.r[8].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8245458C: 550807FF  clrlwi. r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82454590: 4182000C  beq 0x8245459c
	if ctx.cr[0].eq {
		sub_8245459C(ctx, base);
		return;
	}
	// 82454594: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454598: 4800000C  b 0x824545a4
	sub_8245459C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245459C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245459C size=68
    let mut pc: u32 = 0x8245459C;
    'dispatch: loop {
        match pc {
            0x8245459C => {
    //   block [0x8245459C..0x824545E0)
	// 8245459C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824545A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824545A4: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 824545A8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824545AC: 419AFFD4  beq cr6, 0x82454580
	if ctx.cr[6].eq {
		sub_82454568(ctx, base);
		return;
	}
	// 824545B0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824545B4: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 824545B8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824545BC: 419A0024  beq cr6, 0x824545e0
	if ctx.cr[6].eq {
		sub_824545E0(ctx, base);
		return;
	}
	// 824545C0: 894A000C  lbz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824545C4: 89250000  lbz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824545C8: 7D4A4810  subfc r10, r10, r9
	ctx.xer.ca = ctx.r[9].u32 >= ctx.r[10].u32;
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 824545CC: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824545D0: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824545D4: 4082000C  bne 0x824545e0
	if !ctx.cr[0].eq {
		sub_824545E0(ctx, base);
		return;
	}
	// 824545D8: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 824545DC: 4800000C  b 0x824545e8
	sub_824545E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824545E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824545E0 size=20
    let mut pc: u32 = 0x824545E0;
    'dispatch: loop {
        match pc {
            0x824545E0 => {
    //   block [0x824545E0..0x824545F4)
	// 824545E0: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 824545E4: 3961FFF4  addi r11, r1, -0xc
	ctx.r[11].s64 = ctx.r[1].s64 + -12;
	// 824545E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824545EC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824545F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824545F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824545F8 size=88
    let mut pc: u32 = 0x824545F8;
    'dispatch: loop {
        match pc {
            0x824545F8 => {
    //   block [0x824545F8..0x82454650)
	// 824545F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824545FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82454604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245460C: 4BFFFDAD  bl 0x824543b8
	ctx.lr = 0x82454610;
	sub_824543B8(ctx, base);
	// 82454610: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82454614: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82454618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245461C: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82454620: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454624: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82454628: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245462C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82454630: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454634: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454638: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8245463C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82454640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82454644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82454648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245464C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454650 size=96
    let mut pc: u32 = 0x82454650;
    'dispatch: loop {
        match pc {
            0x82454650 => {
    //   block [0x82454650..0x824546B0)
	// 82454650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245465C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454660: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82454664: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82454668: 548A063E  clrlwi r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8245466C: 396B7618  addi r11, r11, 0x7618
	ctx.r[11].s64 = ctx.r[11].s64 + 30232;
	// 82454670: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82454674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82454678: 989F0000  stb r4, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8245467C: 38897644  addi r4, r9, 0x7644
	ctx.r[4].s64 = ctx.r[9].s64 + 30276;
	// 82454680: 38A0003B  li r5, 0x3b
	ctx.r[5].s64 = 59;
	// 82454684: 7C6A58AE  lbzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82454688: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8245468C: 4BE6BD4D  bl 0x822c03d8
	ctx.lr = 0x82454690;
	sub_822C03D8(ctx, base);
	// 82454690: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82454694: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82454698: 4BFFFDF1  bl 0x82454488
	ctx.lr = 0x8245469C;
	sub_82454488(ctx, base);
	// 8245469C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824546A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824546A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824546A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824546AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824546B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824546B0 size=104
    let mut pc: u32 = 0x824546B0;
    'dispatch: loop {
        match pc {
            0x824546B0 => {
    //   block [0x824546B0..0x82454718)
	// 824546B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824546B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824546B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824546BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824546C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824546C4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824546C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824546CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824546D0: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 824546D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824546D8: 99490000  stb r10, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 824546DC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824546E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824546E4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 824546E8: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824546EC: 4BFC775D  bl 0x8241be48
	ctx.lr = 0x824546F0;
	sub_8241BE48(ctx, base);
	// 824546F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824546F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824546F8: 4BFFFF59  bl 0x82454650
	ctx.lr = 0x824546FC;
	sub_82454650(ctx, base);
	// 824546FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454700: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82454704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82454708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245470C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82454710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82454714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454718 size=104
    let mut pc: u32 = 0x82454718;
    'dispatch: loop {
        match pc {
            0x82454718 => {
    //   block [0x82454718..0x82454780)
	// 82454718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245471C: 48D53A45  bl 0x831a8160
	ctx.lr = 0x82454720;
	sub_831A8130(ctx, base);
	// 82454720: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454724: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82454728: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8245472C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82454730: 7F1C3040  cmplw cr6, r28, r6
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82454734: 419A003C  beq cr6, 0x82454770
	if ctx.cr[6].eq {
	pc = 0x82454770; continue 'dispatch;
	}
	// 82454738: 83BB0008  lwz r29, 8(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245473C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82454740: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82454744: 7F06E840  cmplw cr6, r6, r29
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454748: 419A0024  beq cr6, 0x8245476c
	if ctx.cr[6].eq {
	pc = 0x8245476C; continue 'dispatch;
	}
	// 8245474C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82454750: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82454754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82454758: 48D53DB9  bl 0x831a8510
	ctx.lr = 0x8245475C;
	sub_831A8510(ctx, base);
	// 8245475C: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 82454760: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82454764: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454768: 409AFFE4  bne cr6, 0x8245474c
	if !ctx.cr[6].eq {
	pc = 0x8245474C; continue 'dispatch;
	}
	// 8245476C: 93DB0008  stw r30, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82454770: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82454774: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82454778: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8245477C: 48D53A34  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454780 size=84
    let mut pc: u32 = 0x82454780;
    'dispatch: loop {
        match pc {
            0x82454780 => {
    //   block [0x82454780..0x824547D4)
	// 82454780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245478C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82454790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82454798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245479C: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824547A0: 419A001C  beq cr6, 0x824547bc
	if ctx.cr[6].eq {
	pc = 0x824547BC; continue 'dispatch;
	}
	// 824547A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824547A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824547AC: 4817092D  bl 0x825c50d8
	ctx.lr = 0x824547B0;
	sub_825C50D8(ctx, base);
	// 824547B0: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 824547B4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824547B8: 409AFFEC  bne cr6, 0x824547a4
	if !ctx.cr[6].eq {
	pc = 0x824547A4; continue 'dispatch;
	}
	// 824547BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824547C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824547C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824547C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824547CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824547D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824547D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824547D8 size=96
    let mut pc: u32 = 0x824547D8;
    'dispatch: loop {
        match pc {
            0x824547D8 => {
    //   block [0x824547D8..0x82454838)
	// 824547D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824547DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824547E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824547E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824547E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824547EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824547F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824547F4: 419A0020  beq cr6, 0x82454814
	if ctx.cr[6].eq {
	pc = 0x82454814; continue 'dispatch;
	}
	// 824547F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824547FC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454800: 4BFFFF81  bl 0x82454780
	ctx.lr = 0x82454804;
	sub_82454780(ctx, base);
	// 82454804: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82454808: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245480C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82454810: 4899D979  bl 0x82df2188
	ctx.lr = 0x82454814;
	sub_82DF2188(ctx, base);
	// 82454814: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454818: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245481C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454820: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82454824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82454828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82454830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82454834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454838 size=88
    let mut pc: u32 = 0x82454838;
    'dispatch: loop {
        match pc {
            0x82454838 => {
    //   block [0x82454838..0x82454890)
	// 82454838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245483C: 48D53929  bl 0x831a8164
	ctx.lr = 0x82454840;
	sub_831A8130(ctx, base);
	// 82454840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454844: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82454848: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8245484C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82454850: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82454854: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82454858: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8245485C: 419A0024  beq cr6, 0x82454880
	if ctx.cr[6].eq {
	pc = 0x82454880; continue 'dispatch;
	}
	// 82454860: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82454864: 419A0010  beq cr6, 0x82454874
	if ctx.cr[6].eq {
	pc = 0x82454874; continue 'dispatch;
	}
	// 82454868: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8245486C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82454870: 4BFFFAF9  bl 0x82454368
	ctx.lr = 0x82454874;
	sub_82454368(ctx, base);
	// 82454874: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82454878: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8245487C: 4082FFE4  bne 0x82454860
	if !ctx.cr[0].eq {
	pc = 0x82454860; continue 'dispatch;
	}
	// 82454880: 57EB2036  slwi r11, r31, 4
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82454884: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82454888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245488C: 48D53928  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454890 size=156
    let mut pc: u32 = 0x82454890;
    'dispatch: loop {
        match pc {
            0x82454890 => {
    //   block [0x82454890..0x8245492C)
	// 82454890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82454898: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245489C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824548A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824548A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824548A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824548AC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824548B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824548B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824548B8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824548BC: 409A000C  bne cr6, 0x824548c8
	if !ctx.cr[6].eq {
	pc = 0x824548C8; continue 'dispatch;
	}
	// 824548C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824548C4: 48000050  b 0x82454914
	pc = 0x82454914; continue 'dispatch;
	// 824548C8: 3D600666  lis r11, 0x666
	ctx.r[11].s64 = 107347968;
	// 824548CC: 616B6666  ori r11, r11, 0x6666
	ctx.r[11].u64 = ctx.r[11].u64 | 26214;
	// 824548D0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824548D4: 4099000C  ble cr6, 0x824548e0
	if !ctx.cr[6].gt {
	pc = 0x824548E0; continue 'dispatch;
	}
	// 824548D8: 48763449  bl 0x82bb7d20
	ctx.lr = 0x824548DC;
	sub_82BB7D20(ctx, base);
	// 824548DC: 48000034  b 0x82454910
	pc = 0x82454910; continue 'dispatch;
	// 824548E0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 824548E4: 1FC40028  mulli r30, r4, 0x28
	ctx.r[30].s64 = ctx.r[4].s64 * 40;
	// 824548E8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 824548EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824548F0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824548F4: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 824548F8: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 824548FC: 4899D7CD  bl 0x82df20c8
	ctx.lr = 0x82454900;
	sub_82DF20C8(ctx, base);
	// 82454900: 7D7E1A14  add r11, r30, r3
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 82454904: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82454908: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8245490C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82454910: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82454914: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82454918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245491C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82454920: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82454924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82454928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454930 size=540
    let mut pc: u32 = 0x82454930;
    'dispatch: loop {
        match pc {
            0x82454930 => {
    //   block [0x82454930..0x82454B4C)
	// 82454930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454934: 48D5382D  bl 0x831a8160
	ctx.lr = 0x82454938;
	sub_831A8130(ctx, base);
	// 82454938: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245493C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82454940: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82454944: 7F1AE040  cmplw cr6, r26, r28
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82454948: 419A01F8  beq cr6, 0x82454b40
	if ctx.cr[6].eq {
	pc = 0x82454B40; continue 'dispatch;
	}
	// 8245494C: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454950: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82454954: 419A0018  beq cr6, 0x8245496c
	if ctx.cr[6].eq {
	pc = 0x8245496C; continue 'dispatch;
	}
	// 82454958: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245495C: 3B600028  li r27, 0x28
	ctx.r[27].s64 = 40;
	// 82454960: 7D7FE850  subf r11, r31, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82454964: 7D4BDBD7  divw. r10, r11, r27
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82454968: 4082001C  bne 0x82454984
	if !ctx.cr[0].eq {
	pc = 0x82454984; continue 'dispatch;
	}
	// 8245496C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82454970: 80DA0008  lwz r6, 8(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454974: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82454978: 80BA0004  lwz r5, 4(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245497C: 4BFFFD9D  bl 0x82454718
	ctx.lr = 0x82454980;
	sub_82454718(ctx, base);
	// 82454980: 480001C0  b 0x82454b40
	pc = 0x82454B40; continue 'dispatch;
	// 82454984: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454988: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8245498C: 409A000C  bne cr6, 0x82454998
	if !ctx.cr[6].eq {
	pc = 0x82454998; continue 'dispatch;
	}
	// 82454990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454994: 48000010  b 0x824549a4
	pc = 0x824549A4; continue 'dispatch;
	// 82454998: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245499C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 824549A0: 7D6BDBD6  divw r11, r11, r27
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	// 824549A4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824549A8: 41990058  bgt cr6, 0x82454a00
	if ctx.cr[6].gt {
	pc = 0x82454A00; continue 'dispatch;
	}
	// 824549AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824549B0: 4800001C  b 0x824549cc
	pc = 0x824549CC; continue 'dispatch;
	// 824549B4: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 824549B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824549BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824549C0: 48D53B51  bl 0x831a8510
	ctx.lr = 0x824549C4;
	sub_831A8510(ctx, base);
	// 824549C4: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 824549C8: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 824549CC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 824549D0: 409AFFE4  bne cr6, 0x824549b4
	if !ctx.cr[6].eq {
	pc = 0x824549B4; continue 'dispatch;
	}
	// 824549D4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824549D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824549DC: 419A0010  beq cr6, 0x824549ec
	if ctx.cr[6].eq {
	pc = 0x824549EC; continue 'dispatch;
	}
	// 824549E0: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824549E4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824549E8: 7D6BDBD6  divw r11, r11, r27
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	// 824549EC: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824549F0: 1D6B0028  mulli r11, r11, 0x28
	ctx.r[11].s64 = ctx.r[11].s64 * 40;
	// 824549F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824549F8: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824549FC: 48000144  b 0x82454b40
	pc = 0x82454B40; continue 'dispatch;
	// 82454A00: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82454A04: 409A000C  bne cr6, 0x82454a10
	if !ctx.cr[6].eq {
	pc = 0x82454A10; continue 'dispatch;
	}
	// 82454A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454A0C: 48000010  b 0x82454a1c
	pc = 0x82454A1C; continue 'dispatch;
	// 82454A10: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82454A14: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82454A18: 7D6BDBD6  divw r11, r11, r27
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	// 82454A1C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82454A20: 419900A0  bgt cr6, 0x82454ac0
	if ctx.cr[6].gt {
	pc = 0x82454AC0; continue 'dispatch;
	}
	// 82454A24: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82454A28: 409A000C  bne cr6, 0x82454a34
	if !ctx.cr[6].eq {
	pc = 0x82454A34; continue 'dispatch;
	}
	// 82454A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82454A30: 48000010  b 0x82454a40
	pc = 0x82454A40; continue 'dispatch;
	// 82454A34: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454A38: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82454A3C: 7D4BDBD6  divw r10, r11, r27
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	// 82454A40: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454A44: 1D4A0028  mulli r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 * 40;
	// 82454A48: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82454A4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82454A50: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82454A54: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82454A58: 419A0024  beq cr6, 0x82454a7c
	if ctx.cr[6].eq {
	pc = 0x82454A7C; continue 'dispatch;
	}
	// 82454A5C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82454A60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82454A64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82454A68: 48D53AA9  bl 0x831a8510
	ctx.lr = 0x82454A6C;
	sub_831A8510(ctx, base);
	// 82454A6C: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82454A70: 3BBD0028  addi r29, r29, 0x28
	ctx.r[29].s64 = ctx.r[29].s64 + 40;
	// 82454A74: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82454A78: 409AFFE4  bne cr6, 0x82454a5c
	if !ctx.cr[6].eq {
	pc = 0x82454A5C; continue 'dispatch;
	}
	// 82454A7C: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454A80: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82454A84: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454A88: 83FA0008  lwz r31, 8(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454A8C: 419A002C  beq cr6, 0x82454ab8
	if ctx.cr[6].eq {
	pc = 0x82454AB8; continue 'dispatch;
	}
	// 82454A90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82454A94: 419A0014  beq cr6, 0x82454aa8
	if ctx.cr[6].eq {
	pc = 0x82454AA8; continue 'dispatch;
	}
	// 82454A98: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82454A9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82454AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454AA4: 48D53A6D  bl 0x831a8510
	ctx.lr = 0x82454AA8;
	sub_831A8510(ctx, base);
	// 82454AA8: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82454AAC: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 82454AB0: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454AB4: 409AFFDC  bne cr6, 0x82454a90
	if !ctx.cr[6].eq {
	pc = 0x82454A90; continue 'dispatch;
	}
	// 82454AB8: 93FA0008  stw r31, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82454ABC: 48000084  b 0x82454b40
	pc = 0x82454B40; continue 'dispatch;
	// 82454AC0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82454AC4: 419A0010  beq cr6, 0x82454ad4
	if ctx.cr[6].eq {
	pc = 0x82454AD4; continue 'dispatch;
	}
	// 82454AC8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82454ACC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82454AD0: 4899D6B9  bl 0x82df2188
	ctx.lr = 0x82454AD4;
	sub_82DF2188(ctx, base);
	// 82454AD4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82454ADC: 409A000C  bne cr6, 0x82454ae8
	if !ctx.cr[6].eq {
	pc = 0x82454AE8; continue 'dispatch;
	}
	// 82454AE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82454AE4: 48000010  b 0x82454af4
	pc = 0x82454AF4; continue 'dispatch;
	// 82454AE8: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454AEC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82454AF0: 7C8BDBD6  divw r4, r11, r27
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[27].s32;
	// 82454AF4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82454AF8: 4BFFFD99  bl 0x82454890
	ctx.lr = 0x82454AFC;
	sub_82454890(ctx, base);
	// 82454AFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82454B00: 41820040  beq 0x82454b40
	if ctx.cr[0].eq {
	pc = 0x82454B40; continue 'dispatch;
	}
	// 82454B04: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454B08: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454B0C: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454B10: 48000024  b 0x82454b34
	pc = 0x82454B34; continue 'dispatch;
	// 82454B14: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82454B18: 419A0014  beq cr6, 0x82454b2c
	if ctx.cr[6].eq {
	pc = 0x82454B2C; continue 'dispatch;
	}
	// 82454B1C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82454B20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82454B24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82454B28: 48D539E9  bl 0x831a8510
	ctx.lr = 0x82454B2C;
	sub_831A8510(ctx, base);
	// 82454B2C: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 82454B30: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82454B34: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454B38: 409AFFDC  bne cr6, 0x82454b14
	if !ctx.cr[6].eq {
	pc = 0x82454B14; continue 'dispatch;
	}
	// 82454B3C: 93DA0008  stw r30, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82454B40: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82454B44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82454B48: 48D53668  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454B50 size=112
    let mut pc: u32 = 0x82454B50;
    'dispatch: loop {
        match pc {
            0x82454B50 => {
    //   block [0x82454B50..0x82454BC0)
	// 82454B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454B54: 48D5360D  bl 0x831a8160
	ctx.lr = 0x82454B58;
	sub_831A8130(ctx, base);
	// 82454B58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454B5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82454B60: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82454B64: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82454B68: 3BFD0010  addi r31, r29, 0x10
	ctx.r[31].s64 = ctx.r[29].s64 + 16;
	// 82454B6C: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454B70: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82454B74: 419A0020  beq cr6, 0x82454b94
	if ctx.cr[6].eq {
	pc = 0x82454B94; continue 'dispatch;
	}
	// 82454B78: 7F5FE850  subf r26, r31, r29
	ctx.r[26].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82454B7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82454B80: 7C7AFA14  add r3, r26, r31
	ctx.r[3].u64 = ctx.r[26].u64 + ctx.r[31].u64;
	// 82454B84: 4BFFF8A5  bl 0x82454428
	ctx.lr = 0x82454B88;
	sub_82454428(ctx, base);
	// 82454B88: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82454B8C: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82454B90: 409AFFEC  bne cr6, 0x82454b7c
	if !ctx.cr[6].eq {
	pc = 0x82454B7C; continue 'dispatch;
	}
	// 82454B94: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454B98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82454B9C: 3864FFF0  addi r3, r4, -0x10
	ctx.r[3].s64 = ctx.r[4].s64 + -16;
	// 82454BA0: 4BFFFBE1  bl 0x82454780
	ctx.lr = 0x82454BA4;
	sub_82454780(ctx, base);
	// 82454BA4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454BA8: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82454BAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82454BB0: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82454BB4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454BB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82454BBC: 48D535F4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454BC0 size=152
    let mut pc: u32 = 0x82454BC0;
    'dispatch: loop {
        match pc {
            0x82454BC0 => {
    //   block [0x82454BC0..0x82454C58)
	// 82454BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454BC4: 48D535A5  bl 0x831a8168
	ctx.lr = 0x82454BC8;
	sub_831A8130(ctx, base);
	// 82454BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454BCC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82454BD0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82454BD4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454BD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82454BDC: 409A000C  bne cr6, 0x82454be8
	if !ctx.cr[6].eq {
	pc = 0x82454BE8; continue 'dispatch;
	}
	// 82454BE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82454BE4: 48000014  b 0x82454bf8
	pc = 0x82454BF8; continue 'dispatch;
	// 82454BE8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454BEC: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 82454BF0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82454BF4: 7C8B4BD6  divw r4, r11, r9
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82454BF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82454BFC: 4BFFFC95  bl 0x82454890
	ctx.lr = 0x82454C00;
	sub_82454890(ctx, base);
	// 82454C00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82454C04: 41820048  beq 0x82454c4c
	if ctx.cr[0].eq {
	pc = 0x82454C4C; continue 'dispatch;
	}
	// 82454C08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454C0C: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454C10: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454C14: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82454C18: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454C1C: 419A002C  beq cr6, 0x82454c48
	if ctx.cr[6].eq {
	pc = 0x82454C48; continue 'dispatch;
	}
	// 82454C20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82454C24: 419A0014  beq cr6, 0x82454c38
	if ctx.cr[6].eq {
	pc = 0x82454C38; continue 'dispatch;
	}
	// 82454C28: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82454C2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82454C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454C34: 48D538DD  bl 0x831a8510
	ctx.lr = 0x82454C38;
	sub_831A8510(ctx, base);
	// 82454C38: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82454C3C: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 82454C40: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454C44: 409AFFDC  bne cr6, 0x82454c20
	if !ctx.cr[6].eq {
	pc = 0x82454C20; continue 'dispatch;
	}
	// 82454C48: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82454C4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82454C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82454C54: 48D53564  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454C58 size=172
    let mut pc: u32 = 0x82454C58;
    'dispatch: loop {
        match pc {
            0x82454C58 => {
    //   block [0x82454C58..0x82454D04)
	// 82454C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454C5C: 48D5350D  bl 0x831a8168
	ctx.lr = 0x82454C60;
	sub_831A8130(ctx, base);
	// 82454C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454C64: F8810098  std r4, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[4].u64 ) };
	// 82454C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82454C6C: 98A100A7  stb r5, 0xa7(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(167 as u32), ctx.r[5].u8 ) };
	// 82454C70: 38A10098  addi r5, r1, 0x98
	ctx.r[5].s64 = ctx.r[1].s64 + 152;
	// 82454C74: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82454C78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82454C7C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82454C80: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82454C84: 4BFFF86D  bl 0x824544f0
	ctx.lr = 0x82454C88;
	sub_824544F0(ctx, base);
	// 82454C88: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82454C8C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82454C90: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82454C94: 409A000C  bne cr6, 0x82454ca0
	if !ctx.cr[6].eq {
	pc = 0x82454CA0; continue 'dispatch;
	}
	// 82454C98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82454C9C: 48000060  b 0x82454cfc
	pc = 0x82454CFC; continue 'dispatch;
	// 82454CA0: 38A100A7  addi r5, r1, 0xa7
	ctx.r[5].s64 = ctx.r[1].s64 + 167;
	// 82454CA4: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 82454CA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82454CAC: 4BFFF8BD  bl 0x82454568
	ctx.lr = 0x82454CB0;
	sub_82454568(ctx, base);
	// 82454CB0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82454CB4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82454CB8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82454CBC: 419AFFDC  beq cr6, 0x82454c98
	if ctx.cr[6].eq {
	pc = 0x82454C98; continue 'dispatch;
	}
	// 82454CC0: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82454CC4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82454CC8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82454CCC: 419AFFCC  beq cr6, 0x82454c98
	if ctx.cr[6].eq {
	pc = 0x82454C98; continue 'dispatch;
	}
	// 82454CD0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82454CD4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82454CD8: 409AFFC0  bne cr6, 0x82454c98
	if !ctx.cr[6].eq {
	pc = 0x82454C98; continue 'dispatch;
	}
	// 82454CDC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82454CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454CE4: 4BFFF575  bl 0x82454258
	ctx.lr = 0x82454CE8;
	sub_82454258(ctx, base);
	// 82454CE8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82454CEC: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82454CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82454CF4: 4BFFFE5D  bl 0x82454b50
	ctx.lr = 0x82454CF8;
	sub_82454B50(ctx, base);
	// 82454CF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82454CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82454D00: 48D534B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454D08 size=144
    let mut pc: u32 = 0x82454D08;
    'dispatch: loop {
        match pc {
            0x82454D08 => {
    //   block [0x82454D08..0x82454D98)
	// 82454D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454D0C: 48D5345D  bl 0x831a8168
	ctx.lr = 0x82454D10;
	sub_831A8130(ctx, base);
	// 82454D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454D14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82454D18: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82454D1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82454D24: 409A000C  bne cr6, 0x82454d30
	if !ctx.cr[6].eq {
	pc = 0x82454D30; continue 'dispatch;
	}
	// 82454D28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82454D2C: 48000010  b 0x82454d3c
	pc = 0x82454D3C; continue 'dispatch;
	// 82454D30: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454D34: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82454D38: 7D642670  srawi r4, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454D3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82454D40: 48040209  bl 0x82494f48
	ctx.lr = 0x82454D44;
	sub_82494F48(ctx, base);
	// 82454D44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82454D48: 41820044  beq 0x82454d8c
	if ctx.cr[0].eq {
	pc = 0x82454D8C; continue 'dispatch;
	}
	// 82454D4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454D50: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454D54: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454D58: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82454D5C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454D60: 419A0028  beq cr6, 0x82454d88
	if ctx.cr[6].eq {
	pc = 0x82454D88; continue 'dispatch;
	}
	// 82454D64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82454D68: 419A0010  beq cr6, 0x82454d78
	if ctx.cr[6].eq {
	pc = 0x82454D78; continue 'dispatch;
	}
	// 82454D6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82454D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454D74: 4BFFF5F5  bl 0x82454368
	ctx.lr = 0x82454D78;
	sub_82454368(ctx, base);
	// 82454D78: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82454D7C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82454D80: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454D84: 409AFFE0  bne cr6, 0x82454d64
	if !ctx.cr[6].eq {
	pc = 0x82454D64; continue 'dispatch;
	}
	// 82454D88: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82454D8C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82454D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82454D94: 48D53424  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82454D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82454D98 size=856
    let mut pc: u32 = 0x82454D98;
    'dispatch: loop {
        match pc {
            0x82454D98 => {
    //   block [0x82454D98..0x824550F0)
	// 82454D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82454D9C: 48D533B9  bl 0x831a8154
	ctx.lr = 0x82454DA0;
	sub_831A8130(ctx, base);
	// 82454DA0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82454DA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82454DA8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82454DAC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82454DB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82454DB4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82454DB8: 4BFFF5B1  bl 0x82454368
	ctx.lr = 0x82454DBC;
	sub_82454368(ctx, base);
	// 82454DBC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454DC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454DC4: 409A000C  bne cr6, 0x82454dd0
	if !ctx.cr[6].eq {
	pc = 0x82454DD0; continue 'dispatch;
	}
	// 82454DC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82454DCC: 48000010  b 0x82454ddc
	pc = 0x82454DDC; continue 'dispatch;
	// 82454DD0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82454DD4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82454DD8: 7D682670  srawi r8, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454DDC: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82454DE0: 419A02F8  beq cr6, 0x824550d8
	if ctx.cr[6].eq {
	pc = 0x824550D8; continue 'dispatch;
	}
	// 82454DE4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454DE8: 409A000C  bne cr6, 0x82454df4
	if !ctx.cr[6].eq {
	pc = 0x82454DF4; continue 'dispatch;
	}
	// 82454DEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454DF0: 48000010  b 0x82454e00
	pc = 0x82454E00; continue 'dispatch;
	// 82454DF4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454DF8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82454DFC: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454E00: 3D400FFF  lis r10, 0xfff
	ctx.r[10].s64 = 268369920;
	// 82454E04: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82454E08: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82454E0C: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82454E10: 4098000C  bge cr6, 0x82454e1c
	if !ctx.cr[6].lt {
	pc = 0x82454E1C; continue 'dispatch;
	}
	// 82454E14: 48762F0D  bl 0x82bb7d20
	ctx.lr = 0x82454E18;
	sub_82BB7D20(ctx, base);
	// 82454E18: 480002C0  b 0x824550d8
	pc = 0x824550D8; continue 'dispatch;
	// 82454E1C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454E20: 409A000C  bne cr6, 0x82454e2c
	if !ctx.cr[6].eq {
	pc = 0x82454E2C; continue 'dispatch;
	}
	// 82454E24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454E28: 48000010  b 0x82454e38
	pc = 0x82454E38; continue 'dispatch;
	// 82454E2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454E30: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82454E34: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454E38: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82454E3C: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82454E40: 40980168  bge cr6, 0x82454fa8
	if !ctx.cr[6].lt {
	pc = 0x82454FA8; continue 'dispatch;
	}
	// 82454E44: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82454E48: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82454E4C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82454E50: 4098000C  bge cr6, 0x82454e5c
	if !ctx.cr[6].lt {
	pc = 0x82454E5C; continue 'dispatch;
	}
	// 82454E54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454E58: 48000008  b 0x82454e60
	pc = 0x82454E60; continue 'dispatch;
	// 82454E5C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82454E60: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454E64: 409A000C  bne cr6, 0x82454e70
	if !ctx.cr[6].eq {
	pc = 0x82454E70; continue 'dispatch;
	}
	// 82454E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82454E6C: 48000010  b 0x82454e7c
	pc = 0x82454E7C; continue 'dispatch;
	// 82454E70: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454E74: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82454E78: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82454E7C: 7D4ABA14  add r10, r10, r23
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[23].u64;
	// 82454E80: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82454E84: 40980024  bge cr6, 0x82454ea8
	if !ctx.cr[6].lt {
	pc = 0x82454EA8; continue 'dispatch;
	}
	// 82454E88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82454E8C: 409A000C  bne cr6, 0x82454e98
	if !ctx.cr[6].eq {
	pc = 0x82454E98; continue 'dispatch;
	}
	// 82454E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454E94: 48000010  b 0x82454ea4
	pc = 0x82454EA4; continue 'dispatch;
	// 82454E98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454E9C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82454EA0: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454EA4: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82454EA8: 55782036  slwi r24, r11, 4
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82454EAC: 3F208335  lis r25, -0x7ccb
	ctx.r[25].s64 = -2093678592;
	// 82454EB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82454EB4: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82454EB8: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 82454EBC: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82454EC0: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82454EC4: 4899D205  bl 0x82df20c8
	ctx.lr = 0x82454EC8;
	sub_82DF20C8(ctx, base);
	// 82454EC8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82454ECC: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454ED0: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82454ED4: 48000020  b 0x82454ef4
	pc = 0x82454EF4; continue 'dispatch;
	// 82454ED8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82454EDC: 419A0010  beq cr6, 0x82454eec
	if ctx.cr[6].eq {
	pc = 0x82454EEC; continue 'dispatch;
	}
	// 82454EE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82454EE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82454EE8: 4BFFF481  bl 0x82454368
	ctx.lr = 0x82454EEC;
	sub_82454368(ctx, base);
	// 82454EEC: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82454EF0: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82454EF4: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82454EF8: 409AFFE0  bne cr6, 0x82454ed8
	if !ctx.cr[6].eq {
	pc = 0x82454ED8; continue 'dispatch;
	}
	// 82454EFC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82454F00: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82454F04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82454F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82454F0C: 4BFFF92D  bl 0x82454838
	ctx.lr = 0x82454F10;
	sub_82454838(ctx, base);
	// 82454F10: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454F14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82454F18: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454F1C: 419A002C  beq cr6, 0x82454f48
	if ctx.cr[6].eq {
	pc = 0x82454F48; continue 'dispatch;
	}
	// 82454F20: 7F9ED050  subf r28, r30, r26
	ctx.r[28].s64 = ctx.r[26].s64 - ctx.r[30].s64;
	// 82454F24: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82454F28: 419A0010  beq cr6, 0x82454f38
	if ctx.cr[6].eq {
	pc = 0x82454F38; continue 'dispatch;
	}
	// 82454F2C: 7C9CF214  add r4, r28, r30
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82454F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82454F34: 4BFFF435  bl 0x82454368
	ctx.lr = 0x82454F38;
	sub_82454368(ctx, base);
	// 82454F38: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82454F3C: 7D7CF214  add r11, r28, r30
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82454F40: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82454F44: 409AFFE0  bne cr6, 0x82454f24
	if !ctx.cr[6].eq {
	pc = 0x82454F24; continue 'dispatch;
	}
	// 82454F48: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454F4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82454F50: 409A000C  bne cr6, 0x82454f5c
	if !ctx.cr[6].eq {
	pc = 0x82454F5C; continue 'dispatch;
	}
	// 82454F54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82454F58: 48000010  b 0x82454f68
	pc = 0x82454F68; continue 'dispatch;
	// 82454F5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454F60: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82454F64: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454F68: 7FCBBA14  add r30, r11, r23
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82454F6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82454F70: 419A001C  beq cr6, 0x82454f8c
	if ctx.cr[6].eq {
	pc = 0x82454F8C; continue 'dispatch;
	}
	// 82454F74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82454F78: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454F7C: 4BFFF805  bl 0x82454780
	ctx.lr = 0x82454F80;
	sub_82454780(ctx, base);
	// 82454F80: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82454F84: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82454F88: 4899D201  bl 0x82df2188
	ctx.lr = 0x82454F8C;
	sub_82DF2188(ctx, base);
	// 82454F8C: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82454F90: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82454F94: 7D58DA14  add r10, r24, r27
	ctx.r[10].u64 = ctx.r[24].u64 + ctx.r[27].u64;
	// 82454F98: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82454F9C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82454FA0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82454FA4: 48000134  b 0x824550d8
	pc = 0x824550D8; continue 'dispatch;
	// 82454FA8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454FAC: 7D7AF050  subf r11, r26, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 82454FB0: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82454FB4: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82454FB8: 40980090  bge cr6, 0x82455048
	if !ctx.cr[6].lt {
	pc = 0x82455048; continue 'dispatch;
	}
	// 82454FBC: 56FD2036  slwi r29, r23, 4
	ctx.r[29].u32 = ctx.r[23].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82454FC0: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82454FC4: 7F9DD214  add r28, r29, r26
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 82454FC8: 419A002C  beq cr6, 0x82454ff4
	if ctx.cr[6].eq {
	pc = 0x82454FF4; continue 'dispatch;
	}
	// 82454FCC: 7F7DE050  subf r27, r29, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82454FD0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82454FD4: 419A0010  beq cr6, 0x82454fe4
	if ctx.cr[6].eq {
	pc = 0x82454FE4; continue 'dispatch;
	}
	// 82454FD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82454FDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82454FE0: 4BFFF389  bl 0x82454368
	ctx.lr = 0x82454FE4;
	sub_82454368(ctx, base);
	// 82454FE4: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 82454FE8: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 82454FEC: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82454FF0: 409AFFE0  bne cr6, 0x82454fd0
	if !ctx.cr[6].eq {
	pc = 0x82454FD0; continue 'dispatch;
	}
	// 82454FF4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82454FF8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82454FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455000: 7D7A2050  subf r11, r26, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[26].s64;
	// 82455004: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82455008: 7CABB850  subf r5, r11, r23
	ctx.r[5].s64 = ctx.r[23].s64 - ctx.r[11].s64;
	// 8245500C: 4BFFF82D  bl 0x82454838
	ctx.lr = 0x82455010;
	sub_82454838(ctx, base);
	// 82455010: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455014: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82455018: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8245501C: 7FDD5850  subf r30, r29, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82455020: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82455024: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82455028: 419A00B0  beq cr6, 0x824550d8
	if ctx.cr[6].eq {
	pc = 0x824550D8; continue 'dispatch;
	}
	// 8245502C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82455030: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82455034: 4BFFF3F5  bl 0x82454428
	ctx.lr = 0x82455038;
	sub_82454428(ctx, base);
	// 82455038: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 8245503C: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82455040: 409AFFEC  bne cr6, 0x8245502c
	if !ctx.cr[6].eq {
	pc = 0x8245502C; continue 'dispatch;
	}
	// 82455044: 48000094  b 0x824550d8
	pc = 0x824550D8; continue 'dispatch;
	// 82455048: 56F92036  slwi r25, r23, 4
	ctx.r[25].u32 = ctx.r[23].u32.wrapping_shl(4);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 8245504C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82455050: 7FB9F050  subf r29, r25, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[25].s64;
	// 82455054: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82455058: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8245505C: 419A0028  beq cr6, 0x82455084
	if ctx.cr[6].eq {
	pc = 0x82455084; continue 'dispatch;
	}
	// 82455060: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82455064: 419A0010  beq cr6, 0x82455074
	if ctx.cr[6].eq {
	pc = 0x82455074; continue 'dispatch;
	}
	// 82455068: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8245506C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82455070: 4BFFF2F9  bl 0x82454368
	ctx.lr = 0x82455074;
	sub_82454368(ctx, base);
	// 82455074: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 82455078: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 8245507C: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82455080: 409AFFE0  bne cr6, 0x82455060
	if !ctx.cr[6].eq {
	pc = 0x82455060; continue 'dispatch;
	}
	// 82455084: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82455088: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8245508C: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82455090: 419A0020  beq cr6, 0x824550b0
	if ctx.cr[6].eq {
	pc = 0x824550B0; continue 'dispatch;
	}
	// 82455094: 7FDDF050  subf r30, r29, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82455098: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 8245509C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824550A0: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 824550A4: 4BFFF385  bl 0x82454428
	ctx.lr = 0x824550A8;
	sub_82454428(ctx, base);
	// 824550A8: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 824550AC: 409AFFEC  bne cr6, 0x82455098
	if !ctx.cr[6].eq {
	pc = 0x82455098; continue 'dispatch;
	}
	// 824550B0: 7FD9D214  add r30, r25, r26
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 824550B4: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 824550B8: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824550BC: 419A001C  beq cr6, 0x824550d8
	if ctx.cr[6].eq {
	pc = 0x824550D8; continue 'dispatch;
	}
	// 824550C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824550C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824550C8: 4BFFF361  bl 0x82454428
	ctx.lr = 0x824550CC;
	sub_82454428(ctx, base);
	// 824550CC: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 824550D0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824550D4: 409AFFEC  bne cr6, 0x824550c0
	if !ctx.cr[6].eq {
	pc = 0x824550C0; continue 'dispatch;
	}
	// 824550D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824550DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824550E0: 419A0008  beq cr6, 0x824550e8
	if ctx.cr[6].eq {
	pc = 0x824550E8; continue 'dispatch;
	}
	// 824550E4: 4BE6B7AD  bl 0x822c0890
	ctx.lr = 0x824550E8;
	sub_822C0890(ctx, base);
	// 824550E8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824550EC: 48D530B8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824550F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824550F0 size=108
    let mut pc: u32 = 0x824550F0;
    'dispatch: loop {
        match pc {
            0x824550F0 => {
    //   block [0x824550F0..0x8245515C)
	// 824550F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824550F4: 48D53079  bl 0x831a816c
	ctx.lr = 0x824550F8;
	sub_831A8130(ctx, base);
	// 824550F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824550FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82455100: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82455104: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82455108: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245510C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455110: 419A0014  beq cr6, 0x82455124
	if ctx.cr[6].eq {
	pc = 0x82455124; continue 'dispatch;
	}
	// 82455114: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455118: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8245511C: 7D4A2671  srawi. r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82455120: 4082000C  bne 0x8245512c
	if !ctx.cr[0].eq {
	pc = 0x8245512C; continue 'dispatch;
	}
	// 82455124: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82455128: 4800000C  b 0x82455134
	pc = 0x82455134; continue 'dispatch;
	// 8245512C: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82455130: 7D7D2670  srawi r29, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82455134: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82455138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245513C: 4BFFFC5D  bl 0x82454d98
	ctx.lr = 0x82455140;
	sub_82454D98(ctx, base);
	// 82455140: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455144: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82455148: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245514C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82455150: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82455154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82455158: 48D53064  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455160 size=160
    let mut pc: u32 = 0x82455160;
    'dispatch: loop {
        match pc {
            0x82455160 => {
    //   block [0x82455160..0x82455200)
	// 82455160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82455168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245516C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82455170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455178: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245517C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455180: 409A000C  bne cr6, 0x8245518c
	if !ctx.cr[6].eq {
	pc = 0x8245518C; continue 'dispatch;
	}
	// 82455184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82455188: 48000010  b 0x82455198
	pc = 0x82455198; continue 'dispatch;
	// 8245518C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455190: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82455194: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82455198: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245519C: 419A0038  beq cr6, 0x824551d4
	if ctx.cr[6].eq {
	pc = 0x824551D4; continue 'dispatch;
	}
	// 824551A0: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824551A4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824551A8: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 824551AC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824551B0: 40980024  bge cr6, 0x824551d4
	if !ctx.cr[6].lt {
	pc = 0x824551D4; continue 'dispatch;
	}
	// 824551B4: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824551B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824551BC: 419A000C  beq cr6, 0x824551c8
	if ctx.cr[6].eq {
	pc = 0x824551C8; continue 'dispatch;
	}
	// 824551C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824551C4: 4BFFF1A5  bl 0x82454368
	ctx.lr = 0x824551C8;
	sub_82454368(ctx, base);
	// 824551C8: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 824551CC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824551D0: 48000018  b 0x824551e8
	pc = 0x824551E8; continue 'dispatch;
	// 824551D4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 824551D8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824551DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824551E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824551E4: 4BFFFF0D  bl 0x824550f0
	ctx.lr = 0x824551E8;
	sub_824550F0(ctx, base);
	// 824551E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824551EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824551F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824551F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824551F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824551FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455200 size=104
    let mut pc: u32 = 0x82455200;
    'dispatch: loop {
        match pc {
            0x82455200 => {
    //   block [0x82455200..0x82455268)
	// 82455200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82455208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245520C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82455210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455218: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 8245521C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82455220: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82455224: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82455228: 38870004  addi r4, r7, 4
	ctx.r[4].s64 = ctx.r[7].s64 + 4;
	// 8245522C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82455230: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82455234: 89470000  lbz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455238: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 8245523C: 4BFFFACD  bl 0x82454d08
	ctx.lr = 0x82455240;
	sub_82454D08(ctx, base);
	// 82455240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82455244: 9BDF0020  stb r30, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82455248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245524C: 997F0021  stb r11, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82455250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82455254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82455258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245525C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82455260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82455264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455268 size=108
    let mut pc: u32 = 0x82455268;
    'dispatch: loop {
        match pc {
            0x82455268 => {
    //   block [0x82455268..0x824552D4)
	// 82455268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245526C: 48D52EF5  bl 0x831a8160
	ctx.lr = 0x82455270;
	sub_831A8130(ctx, base);
	// 82455270: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455274: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82455278: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8245527C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82455280: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82455284: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82455288: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8245528C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82455290: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82455294: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82455298: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8245529C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 824552A0: 4899CE29  bl 0x82df20c8
	ctx.lr = 0x824552A4;
	sub_82DF20C8(ctx, base);
	// 824552A4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824552A8: 41820020  beq 0x824552c8
	if ctx.cr[0].eq {
	pc = 0x824552C8; continue 'dispatch;
	}
	// 824552AC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 824552B0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 824552B4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824552B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824552BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824552C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824552C4: 4BFFFF3D  bl 0x82455200
	ctx.lr = 0x824552C8;
	sub_82455200(ctx, base);
	// 824552C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824552CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824552D0: 48D52EE0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824552D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824552D8 size=548
    let mut pc: u32 = 0x824552D8;
    'dispatch: loop {
        match pc {
            0x824552D8 => {
    //   block [0x824552D8..0x824554FC)
	// 824552D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824552DC: 48D52E85  bl 0x831a8160
	ctx.lr = 0x824552E0;
	sub_831A8130(ctx, base);
	// 824552E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824552E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824552E8: 3D600CCC  lis r11, 0xccc
	ctx.r[11].s64 = 214695936;
	// 824552EC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824552F0: 616BCCCB  ori r11, r11, 0xcccb
	ctx.r[11].u64 = ctx.r[11].u64 | 52427;
	// 824552F4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824552F8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824552FC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82455300: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82455304: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455308: 41980048  blt cr6, 0x82455350
	if ctx.cr[6].lt {
	pc = 0x82455350; continue 'dispatch;
	}
	// 8245530C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82455310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455314: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82455318: 4BE705B1  bl 0x822c58c8
	ctx.lr = 0x8245531C;
	sub_822C58C8(ctx, base);
	// 8245531C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82455320: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455324: 4BE704F5  bl 0x822c5818
	ctx.lr = 0x82455328;
	sub_822C5818(ctx, base);
	// 82455328: 4BE6EF89  bl 0x822c42b0
	ctx.lr = 0x8245532C;
	sub_822C42B0(ctx, base);
	// 8245532C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82455330: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455334: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82455338: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8245533C: 4BE70135  bl 0x822c5470
	ctx.lr = 0x82455340;
	sub_822C5470(ctx, base);
	// 82455340: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82455344: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82455348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245534C: 4BE6F995  bl 0x822c4ce0
	ctx.lr = 0x82455350;
	sub_822C4CE0(ctx, base);
	// 82455350: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82455358: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245535C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82455360: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82455364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455368: 4BFFFF01  bl 0x82455268
	ctx.lr = 0x8245536C;
	sub_82455268(ctx, base);
	// 8245536C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455370: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455374: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82455378: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8245537C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455380: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82455384: 409A0018  bne cr6, 0x8245539c
	if !ctx.cr[6].eq {
	pc = 0x8245539C; continue 'dispatch;
	}
	// 82455388: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8245538C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455390: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82455394: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455398: 4800003C  b 0x824553d4
	pc = 0x824553D4; continue 'dispatch;
	// 8245539C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824553A0: 41820020  beq 0x824553c0
	if ctx.cr[0].eq {
	pc = 0x824553C0; continue 'dispatch;
	}
	// 824553A4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824553A8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824553AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824553B0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824553B4: 409A0024  bne cr6, 0x824553d8
	if !ctx.cr[6].eq {
	pc = 0x824553D8; continue 'dispatch;
	}
	// 824553B8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824553BC: 4800001C  b 0x824553d8
	pc = 0x824553D8; continue 'dispatch;
	// 824553C0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824553C4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824553C8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824553CC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824553D0: 409A0008  bne cr6, 0x824553d8
	if !ctx.cr[6].eq {
	pc = 0x824553D8; continue 'dispatch;
	}
	// 824553D4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824553D8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824553DC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 824553E0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 824553E4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 824553E8: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 824553EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824553F0: 409A00F0  bne cr6, 0x824554e0
	if !ctx.cr[6].eq {
	pc = 0x824554E0; continue 'dispatch;
	}
	// 824553F4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824553F8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824553FC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455400: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455404: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82455408: 409A0054  bne cr6, 0x8245545c
	if !ctx.cr[6].eq {
	pc = 0x8245545C; continue 'dispatch;
	}
	// 8245540C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455410: 892A0020  lbz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455414: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82455418: 419A0054  beq cr6, 0x8245546c
	if ctx.cr[6].eq {
	pc = 0x8245546C; continue 'dispatch;
	}
	// 8245541C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455420: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455424: 409A0010  bne cr6, 0x82455434
	if !ctx.cr[6].eq {
	pc = 0x82455434; continue 'dispatch;
	}
	// 82455428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245542C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82455430: 4BFFECE9  bl 0x82454118
	ctx.lr = 0x82455434;
	sub_82454118(ctx, base);
	// 82455434: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245543C: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82455440: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455444: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455448: 9B6B0020  stb r27, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[27].u8 ) };
	// 8245544C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455450: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455454: 48373C55  bl 0x827c90a8
	ctx.lr = 0x82455458;
	sub_827C90A8(ctx, base);
	// 82455458: 48000074  b 0x824554cc
	pc = 0x824554CC; continue 'dispatch;
	// 8245545C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455460: 892A0020  lbz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455464: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82455468: 409A0028  bne cr6, 0x82455490
	if !ctx.cr[6].eq {
	pc = 0x82455490; continue 'dispatch;
	}
	// 8245546C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455470: 9BA90020  stb r29, 0x20(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82455474: 9BAA0020  stb r29, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82455478: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245547C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455480: 9B6A0020  stb r27, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[27].u8 ) };
	// 82455484: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455488: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245548C: 48000040  b 0x824554cc
	pc = 0x824554CC; continue 'dispatch;
	// 82455490: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455494: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455498: 409A0010  bne cr6, 0x824554a8
	if !ctx.cr[6].eq {
	pc = 0x824554A8; continue 'dispatch;
	}
	// 8245549C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824554A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824554A4: 48373C05  bl 0x827c90a8
	ctx.lr = 0x824554A8;
	sub_827C90A8(ctx, base);
	// 824554A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824554B0: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 824554B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554BC: 9B6B0020  stb r27, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[27].u8 ) };
	// 824554C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554C4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554C8: 4BFFEC51  bl 0x82454118
	ctx.lr = 0x824554CC;
	sub_82454118(ctx, base);
	// 824554CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 824554D4: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 824554D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824554DC: 419AFF1C  beq cr6, 0x824553f8
	if ctx.cr[6].eq {
	pc = 0x824553F8; continue 'dispatch;
	}
	// 824554E0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824554E8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824554EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824554F0: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 824554F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824554F8: 48D52CB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455500 size=1024
    let mut pc: u32 = 0x82455500;
    'dispatch: loop {
        match pc {
            0x82455500 => {
    //   block [0x82455500..0x82455900)
	// 82455500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455504: 48D52C55  bl 0x831a8158
	ctx.lr = 0x82455508;
	sub_831A8130(ctx, base);
	// 82455508: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245550C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82455510: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82455514: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82455518: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8245551C: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455524: 419A0048  beq cr6, 0x8245556c
	if ctx.cr[6].eq {
	pc = 0x8245556C; continue 'dispatch;
	}
	// 82455528: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245552C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455530: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82455534: 4BE70395  bl 0x822c58c8
	ctx.lr = 0x82455538;
	sub_822C58C8(ctx, base);
	// 82455538: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245553C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455540: 4BE74971  bl 0x822c9eb0
	ctx.lr = 0x82455544;
	sub_822C9EB0(ctx, base);
	// 82455544: 4BE6ED6D  bl 0x822c42b0
	ctx.lr = 0x82455548;
	sub_822C42B0(ctx, base);
	// 82455548: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245554C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455550: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82455554: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82455558: 4BE6FF19  bl 0x822c5470
	ctx.lr = 0x8245555C;
	sub_822C5470(ctx, base);
	// 8245555C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82455560: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82455564: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455568: 4BE6F779  bl 0x822c4ce0
	ctx.lr = 0x8245556C;
	sub_822C4CE0(ctx, base);
	// 8245556C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82455570: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82455574: 48373ABD  bl 0x827c9030
	ctx.lr = 0x82455578;
	sub_827C9030(ctx, base);
	// 82455578: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245557C: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455580: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82455584: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82455588: 419A000C  beq cr6, 0x82455594
	if ctx.cr[6].eq {
	pc = 0x82455594; continue 'dispatch;
	}
	// 8245558C: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455590: 48000028  b 0x824555b8
	pc = 0x824555B8; continue 'dispatch;
	// 82455594: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455598: 894A0021  lbz r10, 0x21(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 8245559C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824555A0: 419A000C  beq cr6, 0x824555ac
	if ctx.cr[6].eq {
	pc = 0x824555AC; continue 'dispatch;
	}
	// 824555A4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 824555A8: 48000010  b 0x824555b8
	pc = 0x824555B8; continue 'dispatch;
	// 824555AC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 824555B0: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824555B4: 409A00DC  bne cr6, 0x82455690
	if !ctx.cr[6].eq {
	pc = 0x82455690; continue 'dispatch;
	}
	// 824555B8: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 824555BC: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824555C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824555C4: 409A0008  bne cr6, 0x824555cc
	if !ctx.cr[6].eq {
	pc = 0x824555CC; continue 'dispatch;
	}
	// 824555C8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824555CC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824555D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824555D4: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824555D8: 409A000C  bne cr6, 0x824555e4
	if !ctx.cr[6].eq {
	pc = 0x824555E4; continue 'dispatch;
	}
	// 824555DC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 824555E0: 4800001C  b 0x824555fc
	pc = 0x824555FC; continue 'dispatch;
	// 824555E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824555E8: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824555EC: 409A000C  bne cr6, 0x824555f8
	if !ctx.cr[6].eq {
	pc = 0x824555F8; continue 'dispatch;
	}
	// 824555F0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824555F4: 48000008  b 0x824555fc
	pc = 0x824555FC; continue 'dispatch;
	// 824555F8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824555FC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455600: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455604: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82455608: 409A003C  bne cr6, 0x82455644
	if !ctx.cr[6].eq {
	pc = 0x82455644; continue 'dispatch;
	}
	// 8245560C: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455610: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455614: 419A000C  beq cr6, 0x82455620
	if ctx.cr[6].eq {
	pc = 0x82455620; continue 'dispatch;
	}
	// 82455618: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8245561C: 48000024  b 0x82455640
	pc = 0x82455640; continue 'dispatch;
	// 82455620: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455624: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82455628: 4800000C  b 0x82455634
	pc = 0x82455634; continue 'dispatch;
	// 8245562C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82455630: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455634: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455638: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8245563C: 419AFFF0  beq cr6, 0x8245562c
	if ctx.cr[6].eq {
	pc = 0x8245562C; continue 'dispatch;
	}
	// 82455640: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82455644: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455648: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245564C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82455650: 409A00D4  bne cr6, 0x82455724
	if !ctx.cr[6].eq {
	pc = 0x82455724; continue 'dispatch;
	}
	// 82455654: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245565C: 419A000C  beq cr6, 0x82455668
	if ctx.cr[6].eq {
	pc = 0x82455668; continue 'dispatch;
	}
	// 82455660: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82455664: 48000024  b 0x82455688
	pc = 0x82455688; continue 'dispatch;
	// 82455668: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245566C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82455670: 4800000C  b 0x8245567c
	pc = 0x8245567C; continue 'dispatch;
	// 82455674: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82455678: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245567C: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455680: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82455684: 419AFFF0  beq cr6, 0x82455674
	if ctx.cr[6].eq {
	pc = 0x82455674; continue 'dispatch;
	}
	// 82455688: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8245568C: 48000098  b 0x82455724
	pc = 0x82455724; continue 'dispatch;
	// 82455690: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82455694: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455698: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245569C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824556A0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824556A4: 409A000C  bne cr6, 0x824556b0
	if !ctx.cr[6].eq {
	pc = 0x824556B0; continue 'dispatch;
	}
	// 824556A8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 824556AC: 4800002C  b 0x824556d8
	pc = 0x824556D8; continue 'dispatch;
	// 824556B0: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 824556B4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 824556B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824556BC: 409A0008  bne cr6, 0x824556c4
	if !ctx.cr[6].eq {
	pc = 0x824556C4; continue 'dispatch;
	}
	// 824556C0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824556C4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824556C8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824556CC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824556D0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824556D4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 824556D8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824556DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824556E0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824556E4: 409A000C  bne cr6, 0x824556f0
	if !ctx.cr[6].eq {
	pc = 0x824556F0; continue 'dispatch;
	}
	// 824556E8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 824556EC: 48000020  b 0x8245570c
	pc = 0x8245570C; continue 'dispatch;
	// 824556F0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824556F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824556F8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824556FC: 409A000C  bne cr6, 0x82455708
	if !ctx.cr[6].eq {
	pc = 0x82455708; continue 'dispatch;
	}
	// 82455700: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82455704: 48000008  b 0x8245570c
	pc = 0x8245570C; continue 'dispatch;
	// 82455708: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8245570C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455710: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82455714: 897B0020  lbz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455718: 89590020  lbz r10, 0x20(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245571C: 99790020  stb r11, 0x20(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82455720: 995B0020  stb r10, 0x20(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82455724: 897B0020  lbz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455728: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8245572C: 409A0198  bne cr6, 0x824558c4
	if !ctx.cr[6].eq {
	pc = 0x824558C4; continue 'dispatch;
	}
	// 82455730: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455734: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82455738: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245573C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455740: 419A0180  beq cr6, 0x824558c0
	if ctx.cr[6].eq {
	pc = 0x824558C0; continue 'dispatch;
	}
	// 82455744: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82455748: 897C0020  lbz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245574C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82455750: 409A0170  bne cr6, 0x824558c0
	if !ctx.cr[6].eq {
	pc = 0x824558C0; continue 'dispatch;
	}
	// 82455754: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455758: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245575C: 409A00A8  bne cr6, 0x82455804
	if !ctx.cr[6].eq {
	pc = 0x82455804; continue 'dispatch;
	}
	// 82455760: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455764: 894B0020  lbz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455768: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245576C: 409A001C  bne cr6, 0x82455788
	if !ctx.cr[6].eq {
	pc = 0x82455788; continue 'dispatch;
	}
	// 82455770: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82455774: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82455778: 9BBF0020  stb r29, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 8245577C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82455780: 4BFFE999  bl 0x82454118
	ctx.lr = 0x82455784;
	sub_82454118(ctx, base);
	// 82455784: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455788: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 8245578C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82455790: 409A00C8  bne cr6, 0x82455858
	if !ctx.cr[6].eq {
	pc = 0x82455858; continue 'dispatch;
	}
	// 82455794: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455798: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245579C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824557A0: 409A0014  bne cr6, 0x824557b4
	if !ctx.cr[6].eq {
	pc = 0x824557B4; continue 'dispatch;
	}
	// 824557A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824557A8: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 824557AC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824557B0: 419A00A4  beq cr6, 0x82455854
	if ctx.cr[6].eq {
	pc = 0x82455854; continue 'dispatch;
	}
	// 824557B4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824557B8: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 824557BC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824557C0: 409A0020  bne cr6, 0x824557e0
	if !ctx.cr[6].eq {
	pc = 0x824557E0; continue 'dispatch;
	}
	// 824557C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824557C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824557CC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824557D0: 9BCA0020  stb r30, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 824557D4: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 824557D8: 483738D1  bl 0x827c90a8
	ctx.lr = 0x824557DC;
	sub_827C90A8(ctx, base);
	// 824557DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824557E0: 895F0020  lbz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824557E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824557E8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824557EC: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 824557F0: 9BDF0020  stb r30, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 824557F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824557F8: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 824557FC: 4BFFE91D  bl 0x82454118
	ctx.lr = 0x82455800;
	sub_82454118(ctx, base);
	// 82455800: 480000C0  b 0x824558c0
	pc = 0x824558C0; continue 'dispatch;
	// 82455804: 894B0020  lbz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455808: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245580C: 409A001C  bne cr6, 0x82455828
	if !ctx.cr[6].eq {
	pc = 0x82455828; continue 'dispatch;
	}
	// 82455810: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82455814: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82455818: 9BBF0020  stb r29, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 8245581C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82455820: 48373889  bl 0x827c90a8
	ctx.lr = 0x82455824;
	sub_827C90A8(ctx, base);
	// 82455824: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455828: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 8245582C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82455830: 409A0028  bne cr6, 0x82455858
	if !ctx.cr[6].eq {
	pc = 0x82455858; continue 'dispatch;
	}
	// 82455834: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455838: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245583C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82455840: 409A0034  bne cr6, 0x82455874
	if !ctx.cr[6].eq {
	pc = 0x82455874; continue 'dispatch;
	}
	// 82455844: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455848: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245584C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82455850: 409A0024  bne cr6, 0x82455874
	if !ctx.cr[6].eq {
	pc = 0x82455874; continue 'dispatch;
	}
	// 82455854: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82455858: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245585C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82455860: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455864: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455868: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245586C: 409AFEDC  bne cr6, 0x82455748
	if !ctx.cr[6].eq {
	pc = 0x82455748; continue 'dispatch;
	}
	// 82455870: 48000050  b 0x824558c0
	pc = 0x824558C0; continue 'dispatch;
	// 82455874: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455878: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245587C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82455880: 409A0020  bne cr6, 0x824558a0
	if !ctx.cr[6].eq {
	pc = 0x824558A0; continue 'dispatch;
	}
	// 82455884: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455888: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8245588C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82455890: 9BCA0020  stb r30, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82455894: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82455898: 4BFFE881  bl 0x82454118
	ctx.lr = 0x8245589C;
	sub_82454118(ctx, base);
	// 8245589C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824558A0: 895F0020  lbz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824558A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824558A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824558AC: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 824558B0: 9BDF0020  stb r30, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 824558B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824558B8: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 824558BC: 483737ED  bl 0x827c90a8
	ctx.lr = 0x824558C0;
	sub_827C90A8(ctx, base);
	// 824558C0: 9BDC0020  stb r30, 0x20(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 824558C4: 387B0010  addi r3, r27, 0x10
	ctx.r[3].s64 = ctx.r[27].s64 + 16;
	// 824558C8: 4BFFEF11  bl 0x824547d8
	ctx.lr = 0x824558CC;
	sub_824547D8(ctx, base);
	// 824558CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 824558D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824558D4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 824558D8: 4899C8B1  bl 0x82df2188
	ctx.lr = 0x824558DC;
	sub_82DF2188(ctx, base);
	// 824558DC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824558E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824558E4: 419A000C  beq cr6, 0x824558f0
	if ctx.cr[6].eq {
	pc = 0x824558F0; continue 'dispatch;
	}
	// 824558E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824558EC: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824558F0: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 824558F4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824558F8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824558FC: 48D528AC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455900 size=100
    let mut pc: u32 = 0x82455900;
    'dispatch: loop {
        match pc {
            0x82455900 => {
    //   block [0x82455900..0x82455964)
	// 82455900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455904: 48D52865  bl 0x831a8168
	ctx.lr = 0x82455908;
	sub_831A8130(ctx, base);
	// 82455908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245590C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82455910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82455914: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82455918: 897E0021  lbz r11, 0x21(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(33 as u32) ) } as u64;
	// 8245591C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455920: 409A003C  bne cr6, 0x8245595c
	if !ctx.cr[6].eq {
	pc = 0x8245595C; continue 'dispatch;
	}
	// 82455924: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82455928: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8245592C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455930: 4BFFFFD1  bl 0x82455900
	ctx.lr = 0x82455934;
	sub_82455900(ctx, base);
	// 82455934: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82455938: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245593C: 4BFFEE9D  bl 0x824547d8
	ctx.lr = 0x82455940;
	sub_824547D8(ctx, base);
	// 82455940: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82455944: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82455948: 4899C841  bl 0x82df2188
	ctx.lr = 0x8245594C;
	sub_82DF2188(ctx, base);
	// 8245594C: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455950: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82455954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455958: 419AFFD0  beq cr6, 0x82455928
	if ctx.cr[6].eq {
	pc = 0x82455928; continue 'dispatch;
	}
	// 8245595C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82455960: 48D52858  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82455968 size=248
    let mut pc: u32 = 0x82455968;
    'dispatch: loop {
        match pc {
            0x82455968 => {
    //   block [0x82455968..0x82455A60)
	// 82455968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245596C: 48D527F5  bl 0x831a8160
	ctx.lr = 0x82455970;
	sub_831A8130(ctx, base);
	// 82455970: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455974: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82455978: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8245597C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82455980: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82455984: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82455988: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245598C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455990: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455994: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82455998: 409A003C  bne cr6, 0x824559d4
	if !ctx.cr[6].eq {
	pc = 0x824559D4; continue 'dispatch;
	}
	// 8245599C: 893B0000  lbz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824559A0: 894B000C  lbz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824559A4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824559A8: 7D4A4810  subfc r10, r10, r9
	ctx.xer.ca = ctx.r[9].u32 >= ctx.r[10].u32;
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 824559AC: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824559B0: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824559B4: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 824559B8: 4182000C  beq 0x824559c4
	if ctx.cr[0].eq {
	pc = 0x824559C4; continue 'dispatch;
	}
	// 824559BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824559C0: 48000008  b 0x824559c8
	pc = 0x824559C8; continue 'dispatch;
	// 824559C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824559C8: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 824559CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824559D0: 419AFFD0  beq cr6, 0x824559a0
	if ctx.cr[6].eq {
	pc = 0x824559A0; continue 'dispatch;
	}
	// 824559D4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824559D8: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824559DC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824559E0: 41820044  beq 0x82455a24
	if ctx.cr[0].eq {
	pc = 0x82455A24; continue 'dispatch;
	}
	// 824559E4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824559E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824559EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824559F0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824559F4: 409A0028  bne cr6, 0x82455a1c
	if !ctx.cr[6].eq {
	pc = 0x82455A1C; continue 'dispatch;
	}
	// 824559F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824559FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82455A00: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82455A04: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82455A08: 4BFFF8D1  bl 0x824552d8
	ctx.lr = 0x82455A0C;
	sub_824552D8(ctx, base);
	// 82455A0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82455A10: 9B5E0004  stb r26, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82455A14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455A18: 48000038  b 0x82455a50
	pc = 0x82455A50; continue 'dispatch;
	// 82455A1C: 4873893D  bl 0x82b8e358
	ctx.lr = 0x82455A20;
	sub_82B8E358(ctx, base);
	// 82455A20: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82455A24: 894B000C  lbz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82455A28: 893B0000  lbz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455A2C: 7D495010  subfc r10, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82455A30: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82455A34: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82455A38: 41820010  beq 0x82455a48
	if ctx.cr[0].eq {
	pc = 0x82455A48; continue 'dispatch;
	}
	// 82455A3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82455A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455A44: 4BFFFFB8  b 0x824559fc
	pc = 0x824559FC; continue 'dispatch;
	// 82455A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82455A4C: 995E0004  stb r10, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82455A50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82455A54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455A58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82455A5C: 48D52754  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455A60 size=128
    let mut pc: u32 = 0x82455A60;
    'dispatch: loop {
        match pc {
            0x82455A60 => {
    //   block [0x82455A60..0x82455AE0)
	// 82455A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455A64: 48D52705  bl 0x831a8168
	ctx.lr = 0x82455A68;
	sub_831A8130(ctx, base);
	// 82455A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455A6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82455A70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455A74: 897E0021  lbz r11, 0x21(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455A78: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455A7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455A80: 409A0054  bne cr6, 0x82455ad4
	if !ctx.cr[6].eq {
	pc = 0x82455AD4; continue 'dispatch;
	}
	// 82455A84: 5784003E  slwi r4, r28, 0
	ctx.r[4].u32 = ctx.r[28].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82455A88: 891E0020  lbz r8, 0x20(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82455A8C: 38FE000C  addi r7, r30, 0xc
	ctx.r[7].s64 = ctx.r[30].s64 + 12;
	// 82455A90: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82455A94: 4BFFF7D5  bl 0x82455268
	ctx.lr = 0x82455A98;
	sub_82455268(ctx, base);
	// 82455A98: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455A9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82455AA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455AA4: 419A0008  beq cr6, 0x82455aac
	if ctx.cr[6].eq {
	pc = 0x82455AAC; continue 'dispatch;
	}
	// 82455AA8: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82455AAC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82455AB0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455AB8: 4BFFFFA9  bl 0x82455a60
	ctx.lr = 0x82455ABC;
	sub_82455A60(ctx, base);
	// 82455ABC: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82455AC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82455AC4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455ACC: 4BFFFF95  bl 0x82455a60
	ctx.lr = 0x82455AD0;
	sub_82455A60(ctx, base);
	// 82455AD0: 907D0008  stw r3, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82455AD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82455AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82455ADC: 48D526DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455AE0 size=84
    let mut pc: u32 = 0x82455AE0;
    'dispatch: loop {
        match pc {
            0x82455AE0 => {
    //   block [0x82455AE0..0x82455B34)
	// 82455AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82455AE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82455AEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455AF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455AF8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455AFC: 4BFFFE05  bl 0x82455900
	ctx.lr = 0x82455B00;
	sub_82455900(ctx, base);
	// 82455B00: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82455B08: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82455B0C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82455B10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B14: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82455B18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B1C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82455B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82455B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82455B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82455B2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82455B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455B38 size=168
    let mut pc: u32 = 0x82455B38;
    'dispatch: loop {
        match pc {
            0x82455B38 => {
    //   block [0x82455B38..0x82455BE0)
	// 82455B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455B3C: 48D52631  bl 0x831a816c
	ctx.lr = 0x82455B40;
	sub_831A8130(ctx, base);
	// 82455B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455B44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82455B48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455B4C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B50: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B54: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82455B58: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B5C: 4BFFFF05  bl 0x82455a60
	ctx.lr = 0x82455B60;
	sub_82455A60(ctx, base);
	// 82455B60: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82455B64: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B68: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455B6C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82455B70: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455B74: 896A0021  lbz r11, 0x21(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455B7C: 409A0050  bne cr6, 0x82455bcc
	if !ctx.cr[6].eq {
	pc = 0x82455BCC; continue 'dispatch;
	}
	// 82455B80: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455B84: 4800000C  b 0x82455b90
	pc = 0x82455B90; continue 'dispatch;
	// 82455B88: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82455B8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455B90: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455B94: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82455B98: 419AFFF0  beq cr6, 0x82455b88
	if ctx.cr[6].eq {
	pc = 0x82455B88; continue 'dispatch;
	}
	// 82455B9C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82455BA0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455BA4: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455BA8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455BAC: 4800000C  b 0x82455bb8
	pc = 0x82455BB8; continue 'dispatch;
	// 82455BB0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82455BB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455BB8: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82455BBC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82455BC0: 419AFFF0  beq cr6, 0x82455bb0
	if ctx.cr[6].eq {
	pc = 0x82455BB0; continue 'dispatch;
	}
	// 82455BC4: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82455BC8: 48000010  b 0x82455bd8
	pc = 0x82455BD8; continue 'dispatch;
	// 82455BCC: 91290000  stw r9, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82455BD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455BD4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82455BD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82455BDC: 48D525E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455BE0 size=132
    let mut pc: u32 = 0x82455BE0;
    'dispatch: loop {
        match pc {
            0x82455BE0 => {
    //   block [0x82455BE0..0x82455C64)
	// 82455BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455BE4: 48D52585  bl 0x831a8168
	ctx.lr = 0x82455BE8;
	sub_831A8130(ctx, base);
	// 82455BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455BEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82455BF0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82455BF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82455BF8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82455BFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455C00: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455C04: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82455C08: 409A0044  bne cr6, 0x82455c4c
	if !ctx.cr[6].eq {
	pc = 0x82455C4C; continue 'dispatch;
	}
	// 82455C0C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455C10: 409A003C  bne cr6, 0x82455c4c
	if !ctx.cr[6].eq {
	pc = 0x82455C4C; continue 'dispatch;
	}
	// 82455C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455C18: 4BFFFEC9  bl 0x82455ae0
	ctx.lr = 0x82455C1C;
	sub_82455AE0(ctx, base);
	// 82455C1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455C20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455C24: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82455C28: 48000030  b 0x82455c58
	pc = 0x82455C58; continue 'dispatch;
	// 82455C2C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82455C30: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82455C34: 483733FD  bl 0x827c9030
	ctx.lr = 0x82455C38;
	sub_827C9030(ctx, base);
	// 82455C38: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82455C3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82455C40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455C44: 4BFFF8BD  bl 0x82455500
	ctx.lr = 0x82455C48;
	sub_82455500(ctx, base);
	// 82455C48: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82455C4C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82455C50: 409AFFDC  bne cr6, 0x82455c2c
	if !ctx.cr[6].eq {
	pc = 0x82455C2C; continue 'dispatch;
	}
	// 82455C54: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82455C58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82455C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82455C60: 48D52558  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455C68 size=88
    let mut pc: u32 = 0x82455C68;
    'dispatch: loop {
        match pc {
            0x82455C68 => {
    //   block [0x82455C68..0x82455CC0)
	// 82455C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82455C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82455C74: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455C7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455C80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82455C84: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455C88: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455C8C: 4BFFFF55  bl 0x82455be0
	ctx.lr = 0x82455C90;
	sub_82455BE0(ctx, base);
	// 82455C90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82455C94: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455C98: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82455C9C: 4899C4ED  bl 0x82df2188
	ctx.lr = 0x82455CA0;
	sub_82DF2188(ctx, base);
	// 82455CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82455CA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82455CA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82455CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82455CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82455CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82455CB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82455CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455CC0 size=96
    let mut pc: u32 = 0x82455CC0;
    'dispatch: loop {
        match pc {
            0x82455CC0 => {
    //   block [0x82455CC0..0x82455D20)
	// 82455CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455CC4: 48D524A5  bl 0x831a8168
	ctx.lr = 0x82455CC8;
	sub_831A8130(ctx, base);
	// 82455CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82455CD0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82455CD4: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82455CD8: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82455CDC: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82455CE0: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82455CE4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82455CE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455CEC: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82455CF0: E95D0000  ld r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 82455CF4: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82455CF8: 487398A1  bl 0x82b8f598
	ctx.lr = 0x82455CFC;
	sub_82B8F598(ctx, base);
	// 82455CFC: 389D0008  addi r4, r29, 8
	ctx.r[4].s64 = ctx.r[29].s64 + 8;
	// 82455D00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455D04: 4BFFFE35  bl 0x82455b38
	ctx.lr = 0x82455D08;
	sub_82455B38(ctx, base);
	// 82455D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82455D0C: 9B9F0028  stb r28, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u8 ) };
	// 82455D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455D14: 997F0029  stb r11, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82455D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82455D1C: 48D5249C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455D20 size=108
    let mut pc: u32 = 0x82455D20;
    'dispatch: loop {
        match pc {
            0x82455D20 => {
    //   block [0x82455D20..0x82455D8C)
	// 82455D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455D24: 48D5243D  bl 0x831a8160
	ctx.lr = 0x82455D28;
	sub_831A8130(ctx, base);
	// 82455D28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455D2C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82455D30: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82455D34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82455D38: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82455D3C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82455D40: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82455D44: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82455D48: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82455D4C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82455D50: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82455D54: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82455D58: 4899C371  bl 0x82df20c8
	ctx.lr = 0x82455D5C;
	sub_82DF20C8(ctx, base);
	// 82455D5C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82455D60: 41820020  beq 0x82455d80
	if ctx.cr[0].eq {
	pc = 0x82455D80; continue 'dispatch;
	}
	// 82455D64: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82455D68: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82455D6C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82455D70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82455D74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82455D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455D7C: 4BFFFF45  bl 0x82455cc0
	ctx.lr = 0x82455D80;
	sub_82455CC0(ctx, base);
	// 82455D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82455D84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82455D88: 48D52428  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455D90 size=548
    let mut pc: u32 = 0x82455D90;
    'dispatch: loop {
        match pc {
            0x82455D90 => {
    //   block [0x82455D90..0x82455FB4)
	// 82455D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455D94: 48D523CD  bl 0x831a8160
	ctx.lr = 0x82455D98;
	sub_831A8130(ctx, base);
	// 82455D98: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455D9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82455DA0: 3D600AAA  lis r11, 0xaaa
	ctx.r[11].s64 = 178913280;
	// 82455DA4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82455DA8: 616BAAA9  ori r11, r11, 0xaaa9
	ctx.r[11].u64 = ctx.r[11].u64 | 43689;
	// 82455DAC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82455DB0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455DB4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82455DB8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82455DBC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455DC0: 41980048  blt cr6, 0x82455e08
	if ctx.cr[6].lt {
	pc = 0x82455E08; continue 'dispatch;
	}
	// 82455DC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82455DC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455DCC: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82455DD0: 4BE6FAF9  bl 0x822c58c8
	ctx.lr = 0x82455DD4;
	sub_822C58C8(ctx, base);
	// 82455DD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82455DD8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455DDC: 4BE6FA3D  bl 0x822c5818
	ctx.lr = 0x82455DE0;
	sub_822C5818(ctx, base);
	// 82455DE0: 4BE6E4D1  bl 0x822c42b0
	ctx.lr = 0x82455DE4;
	sub_822C42B0(ctx, base);
	// 82455DE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82455DE8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455DEC: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82455DF0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82455DF4: 4BE6F67D  bl 0x822c5470
	ctx.lr = 0x82455DF8;
	sub_822C5470(ctx, base);
	// 82455DF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82455DFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82455E00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455E04: 4BE6EEDD  bl 0x822c4ce0
	ctx.lr = 0x82455E08;
	sub_822C4CE0(ctx, base);
	// 82455E08: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82455E10: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82455E14: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82455E18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82455E1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455E20: 4BFFFF01  bl 0x82455d20
	ctx.lr = 0x82455E24;
	sub_82455D20(ctx, base);
	// 82455E24: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455E28: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E2C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82455E30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82455E34: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455E38: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82455E3C: 409A0018  bne cr6, 0x82455e54
	if !ctx.cr[6].eq {
	pc = 0x82455E54; continue 'dispatch;
	}
	// 82455E40: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82455E44: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E48: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82455E4C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E50: 4800003C  b 0x82455e8c
	pc = 0x82455E8C; continue 'dispatch;
	// 82455E54: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82455E58: 41820020  beq 0x82455e78
	if ctx.cr[0].eq {
	pc = 0x82455E78; continue 'dispatch;
	}
	// 82455E5C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82455E60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455E68: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82455E6C: 409A0024  bne cr6, 0x82455e90
	if !ctx.cr[6].eq {
	pc = 0x82455E90; continue 'dispatch;
	}
	// 82455E70: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82455E74: 4800001C  b 0x82455e90
	pc = 0x82455E90; continue 'dispatch;
	// 82455E78: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82455E7C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E80: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455E84: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82455E88: 409A0008  bne cr6, 0x82455e90
	if !ctx.cr[6].eq {
	pc = 0x82455E90; continue 'dispatch;
	}
	// 82455E8C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82455E90: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455E94: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82455E98: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82455E9C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82455EA0: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82455EA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82455EA8: 409A00F0  bne cr6, 0x82455f98
	if !ctx.cr[6].eq {
	pc = 0x82455F98; continue 'dispatch;
	}
	// 82455EAC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82455EB0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455EB4: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455EB8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455EBC: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82455EC0: 409A0054  bne cr6, 0x82455f14
	if !ctx.cr[6].eq {
	pc = 0x82455F14; continue 'dispatch;
	}
	// 82455EC4: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455EC8: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82455ECC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82455ED0: 419A0054  beq cr6, 0x82455f24
	if ctx.cr[6].eq {
	pc = 0x82455F24; continue 'dispatch;
	}
	// 82455ED4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82455ED8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455EDC: 409A0010  bne cr6, 0x82455eec
	if !ctx.cr[6].eq {
	pc = 0x82455EEC; continue 'dispatch;
	}
	// 82455EE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455EE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82455EE8: 4BFFE161  bl 0x82454048
	ctx.lr = 0x82455EEC;
	sub_82454048(ctx, base);
	// 82455EEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455EF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455EF4: 9BAB0028  stb r29, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82455EF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455EFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F00: 9B6B0028  stb r27, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82455F04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F08: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F0C: 4BFFE1A5  bl 0x824540b0
	ctx.lr = 0x82455F10;
	sub_824540B0(ctx, base);
	// 82455F10: 48000074  b 0x82455f84
	pc = 0x82455F84; continue 'dispatch;
	// 82455F14: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455F18: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82455F1C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82455F20: 409A0028  bne cr6, 0x82455f48
	if !ctx.cr[6].eq {
	pc = 0x82455F48; continue 'dispatch;
	}
	// 82455F24: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455F28: 9BA90028  stb r29, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82455F2C: 9BAA0028  stb r29, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82455F30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455F34: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F38: 9B6A0028  stb r27, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82455F3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455F40: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F44: 48000040  b 0x82455f84
	pc = 0x82455F84; continue 'dispatch;
	// 82455F48: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82455F4C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82455F50: 409A0010  bne cr6, 0x82455f60
	if !ctx.cr[6].eq {
	pc = 0x82455F60; continue 'dispatch;
	}
	// 82455F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455F58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82455F5C: 4BFFE155  bl 0x824540b0
	ctx.lr = 0x82455F60;
	sub_824540B0(ctx, base);
	// 82455F60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82455F68: 9BAB0028  stb r29, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82455F6C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F74: 9B6B0028  stb r27, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82455F78: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F7C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F80: 4BFFE0C9  bl 0x82454048
	ctx.lr = 0x82455F84;
	sub_82454048(ctx, base);
	// 82455F84: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F88: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82455F8C: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82455F90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82455F94: 419AFF1C  beq cr6, 0x82455eb0
	if ctx.cr[6].eq {
	pc = 0x82455EB0; continue 'dispatch;
	}
	// 82455F98: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455F9C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82455FA0: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82455FA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82455FA8: 9BAB0028  stb r29, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82455FAC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82455FB0: 48D52200  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82455FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82455FB8 size=1024
    let mut pc: u32 = 0x82455FB8;
    'dispatch: loop {
        match pc {
            0x82455FB8 => {
    //   block [0x82455FB8..0x824563B8)
	// 82455FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82455FBC: 48D5219D  bl 0x831a8158
	ctx.lr = 0x82455FC0;
	sub_831A8130(ctx, base);
	// 82455FC0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82455FC4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82455FC8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82455FCC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82455FD0: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82455FD4: 897F0029  lbz r11, 0x29(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(41 as u32) ) } as u64;
	// 82455FD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82455FDC: 419A0048  beq cr6, 0x82456024
	if ctx.cr[6].eq {
	pc = 0x82456024; continue 'dispatch;
	}
	// 82455FE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82455FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82455FE8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82455FEC: 4BE6F8DD  bl 0x822c58c8
	ctx.lr = 0x82455FF0;
	sub_822C58C8(ctx, base);
	// 82455FF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82455FF4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82455FF8: 4BE73EB9  bl 0x822c9eb0
	ctx.lr = 0x82455FFC;
	sub_822C9EB0(ctx, base);
	// 82455FFC: 4BE6E2B5  bl 0x822c42b0
	ctx.lr = 0x82456000;
	sub_822C42B0(ctx, base);
	// 82456000: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82456004: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82456008: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8245600C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82456010: 4BE6F461  bl 0x822c5470
	ctx.lr = 0x82456014;
	sub_822C5470(ctx, base);
	// 82456014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82456018: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245601C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456020: 4BE6ECC1  bl 0x822c4ce0
	ctx.lr = 0x82456024;
	sub_822C4CE0(ctx, base);
	// 82456024: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82456028: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8245602C: 480930DD  bl 0x824e9108
	ctx.lr = 0x82456030;
	sub_824E9108(ctx, base);
	// 82456030: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456034: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456038: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245603C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82456040: 419A000C  beq cr6, 0x8245604c
	if ctx.cr[6].eq {
	pc = 0x8245604C; continue 'dispatch;
	}
	// 82456044: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456048: 48000028  b 0x82456070
	pc = 0x82456070; continue 'dispatch;
	// 8245604C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456050: 894A0029  lbz r10, 0x29(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456054: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82456058: 419A000C  beq cr6, 0x82456064
	if ctx.cr[6].eq {
	pc = 0x82456064; continue 'dispatch;
	}
	// 8245605C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82456060: 48000010  b 0x82456070
	pc = 0x82456070; continue 'dispatch;
	// 82456064: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456068: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245606C: 409A00DC  bne cr6, 0x82456148
	if !ctx.cr[6].eq {
	pc = 0x82456148; continue 'dispatch;
	}
	// 82456070: 897C0029  lbz r11, 0x29(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456074: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245607C: 409A0008  bne cr6, 0x82456084
	if !ctx.cr[6].eq {
	pc = 0x82456084; continue 'dispatch;
	}
	// 82456080: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82456084: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456088: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245608C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82456090: 409A000C  bne cr6, 0x8245609c
	if !ctx.cr[6].eq {
	pc = 0x8245609C; continue 'dispatch;
	}
	// 82456094: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82456098: 4800001C  b 0x824560b4
	pc = 0x824560B4; continue 'dispatch;
	// 8245609C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824560A0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824560A4: 409A000C  bne cr6, 0x824560b0
	if !ctx.cr[6].eq {
	pc = 0x824560B0; continue 'dispatch;
	}
	// 824560A8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824560AC: 48000008  b 0x824560b4
	pc = 0x824560B4; continue 'dispatch;
	// 824560B0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 824560B4: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824560B8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824560BC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824560C0: 409A003C  bne cr6, 0x824560fc
	if !ctx.cr[6].eq {
	pc = 0x824560FC; continue 'dispatch;
	}
	// 824560C4: 897C0029  lbz r11, 0x29(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(41 as u32) ) } as u64;
	// 824560C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824560CC: 419A000C  beq cr6, 0x824560d8
	if ctx.cr[6].eq {
	pc = 0x824560D8; continue 'dispatch;
	}
	// 824560D0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 824560D4: 48000024  b 0x824560f8
	pc = 0x824560F8; continue 'dispatch;
	// 824560D8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824560DC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 824560E0: 4800000C  b 0x824560ec
	pc = 0x824560EC; continue 'dispatch;
	// 824560E4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824560E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824560EC: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 824560F0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824560F4: 419AFFF0  beq cr6, 0x824560e4
	if ctx.cr[6].eq {
	pc = 0x824560E4; continue 'dispatch;
	}
	// 824560F8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824560FC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456100: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456104: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82456108: 409A00D4  bne cr6, 0x824561dc
	if !ctx.cr[6].eq {
	pc = 0x824561DC; continue 'dispatch;
	}
	// 8245610C: 897C0029  lbz r11, 0x29(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82456114: 419A000C  beq cr6, 0x82456120
	if ctx.cr[6].eq {
	pc = 0x82456120; continue 'dispatch;
	}
	// 82456118: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8245611C: 48000024  b 0x82456140
	pc = 0x82456140; continue 'dispatch;
	// 82456120: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456124: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82456128: 4800000C  b 0x82456134
	pc = 0x82456134; continue 'dispatch;
	// 8245612C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82456130: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456134: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456138: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8245613C: 419AFFF0  beq cr6, 0x8245612c
	if ctx.cr[6].eq {
	pc = 0x8245612C; continue 'dispatch;
	}
	// 82456140: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82456144: 48000098  b 0x824561dc
	pc = 0x824561DC; continue 'dispatch;
	// 82456148: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8245614C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456150: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82456154: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456158: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245615C: 409A000C  bne cr6, 0x82456168
	if !ctx.cr[6].eq {
	pc = 0x82456168; continue 'dispatch;
	}
	// 82456160: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82456164: 4800002C  b 0x82456190
	pc = 0x82456190; continue 'dispatch;
	// 82456168: 897C0029  lbz r11, 0x29(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(41 as u32) ) } as u64;
	// 8245616C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82456174: 409A0008  bne cr6, 0x8245617c
	if !ctx.cr[6].eq {
	pc = 0x8245617C; continue 'dispatch;
	}
	// 82456178: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8245617C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82456180: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456184: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82456188: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245618C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82456190: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456194: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456198: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245619C: 409A000C  bne cr6, 0x824561a8
	if !ctx.cr[6].eq {
	pc = 0x824561A8; continue 'dispatch;
	}
	// 824561A0: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 824561A4: 48000020  b 0x824561c4
	pc = 0x824561C4; continue 'dispatch;
	// 824561A8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824561AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824561B0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 824561B4: 409A000C  bne cr6, 0x824561c0
	if !ctx.cr[6].eq {
	pc = 0x824561C0; continue 'dispatch;
	}
	// 824561B8: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 824561BC: 48000008  b 0x824561c4
	pc = 0x824561C4; continue 'dispatch;
	// 824561C0: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 824561C4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824561C8: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824561CC: 897B0028  lbz r11, 0x28(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(40 as u32) ) } as u64;
	// 824561D0: 89590028  lbz r10, 0x28(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(40 as u32) ) } as u64;
	// 824561D4: 99790028  stb r11, 0x28(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 824561D8: 995B0028  stb r10, 0x28(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 824561DC: 897B0028  lbz r11, 0x28(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(40 as u32) ) } as u64;
	// 824561E0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824561E4: 409A0198  bne cr6, 0x8245637c
	if !ctx.cr[6].eq {
	pc = 0x8245637C; continue 'dispatch;
	}
	// 824561E8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824561EC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824561F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824561F4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824561F8: 419A0180  beq cr6, 0x82456378
	if ctx.cr[6].eq {
	pc = 0x82456378; continue 'dispatch;
	}
	// 824561FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82456200: 897C0028  lbz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456204: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82456208: 409A0170  bne cr6, 0x82456378
	if !ctx.cr[6].eq {
	pc = 0x82456378; continue 'dispatch;
	}
	// 8245620C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456210: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82456214: 409A00A8  bne cr6, 0x824562bc
	if !ctx.cr[6].eq {
	pc = 0x824562BC; continue 'dispatch;
	}
	// 82456218: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245621C: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456220: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82456224: 409A001C  bne cr6, 0x82456240
	if !ctx.cr[6].eq {
	pc = 0x82456240; continue 'dispatch;
	}
	// 82456228: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 8245622C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456230: 9BBF0028  stb r29, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82456234: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82456238: 4BFFDE11  bl 0x82454048
	ctx.lr = 0x8245623C;
	sub_82454048(ctx, base);
	// 8245623C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456240: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456244: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82456248: 409A00C8  bne cr6, 0x82456310
	if !ctx.cr[6].eq {
	pc = 0x82456310; continue 'dispatch;
	}
	// 8245624C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456250: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456254: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82456258: 409A0014  bne cr6, 0x8245626c
	if !ctx.cr[6].eq {
	pc = 0x8245626C; continue 'dispatch;
	}
	// 8245625C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456260: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456264: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82456268: 419A00A4  beq cr6, 0x8245630c
	if ctx.cr[6].eq {
	pc = 0x8245630C; continue 'dispatch;
	}
	// 8245626C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456270: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456274: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82456278: 409A0020  bne cr6, 0x82456298
	if !ctx.cr[6].eq {
	pc = 0x82456298; continue 'dispatch;
	}
	// 8245627C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456280: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82456284: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82456288: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 8245628C: 9BAB0028  stb r29, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82456290: 4BFFDE21  bl 0x824540b0
	ctx.lr = 0x82456294;
	sub_824540B0(ctx, base);
	// 82456294: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456298: 895F0028  lbz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8245629C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824562A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824562A4: 994B0028  stb r10, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 824562A8: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 824562AC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824562B0: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 824562B4: 4BFFDD95  bl 0x82454048
	ctx.lr = 0x824562B8;
	sub_82454048(ctx, base);
	// 824562B8: 480000C0  b 0x82456378
	pc = 0x82456378; continue 'dispatch;
	// 824562BC: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824562C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824562C4: 409A001C  bne cr6, 0x824562e0
	if !ctx.cr[6].eq {
	pc = 0x824562E0; continue 'dispatch;
	}
	// 824562C8: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 824562CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824562D0: 9BBF0028  stb r29, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 824562D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824562D8: 4BFFDDD9  bl 0x824540b0
	ctx.lr = 0x824562DC;
	sub_824540B0(ctx, base);
	// 824562DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824562E0: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 824562E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824562E8: 409A0028  bne cr6, 0x82456310
	if !ctx.cr[6].eq {
	pc = 0x82456310; continue 'dispatch;
	}
	// 824562EC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824562F0: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 824562F4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 824562F8: 409A0034  bne cr6, 0x8245632c
	if !ctx.cr[6].eq {
	pc = 0x8245632C; continue 'dispatch;
	}
	// 824562FC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456300: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456304: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82456308: 409A0024  bne cr6, 0x8245632c
	if !ctx.cr[6].eq {
	pc = 0x8245632C; continue 'dispatch;
	}
	// 8245630C: 9BAB0028  stb r29, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82456310: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456314: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82456318: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245631C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456320: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82456324: 409AFEDC  bne cr6, 0x82456200
	if !ctx.cr[6].eq {
	pc = 0x82456200; continue 'dispatch;
	}
	// 82456328: 48000050  b 0x82456378
	pc = 0x82456378; continue 'dispatch;
	// 8245632C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456330: 894A0028  lbz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456334: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82456338: 409A0020  bne cr6, 0x82456358
	if !ctx.cr[6].eq {
	pc = 0x82456358; continue 'dispatch;
	}
	// 8245633C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456340: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82456344: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82456348: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 8245634C: 9BAB0028  stb r29, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 82456350: 4BFFDCF9  bl 0x82454048
	ctx.lr = 0x82456354;
	sub_82454048(ctx, base);
	// 82456354: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456358: 895F0028  lbz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8245635C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456360: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82456364: 994B0028  stb r10, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82456368: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 8245636C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456370: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82456374: 4BFFDD3D  bl 0x824540b0
	ctx.lr = 0x82456378;
	sub_824540B0(ctx, base);
	// 82456378: 9BDC0028  stb r30, 0x28(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 8245637C: 387B0018  addi r3, r27, 0x18
	ctx.r[3].s64 = ctx.r[27].s64 + 24;
	// 82456380: 4BFFF8E9  bl 0x82455c68
	ctx.lr = 0x82456384;
	sub_82455C68(ctx, base);
	// 82456384: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82456388: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8245638C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82456390: 4899BDF9  bl 0x82df2188
	ctx.lr = 0x82456394;
	sub_82DF2188(ctx, base);
	// 82456394: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245639C: 419A000C  beq cr6, 0x824563a8
	if ctx.cr[6].eq {
	pc = 0x824563A8; continue 'dispatch;
	}
	// 824563A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824563A4: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824563A8: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 824563AC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824563B0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824563B4: 48D51DF4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824563B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824563B8 size=264
    let mut pc: u32 = 0x824563B8;
    'dispatch: loop {
        match pc {
            0x824563B8 => {
    //   block [0x824563B8..0x824564C0)
	// 824563B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824563BC: 48D51DA5  bl 0x831a8160
	ctx.lr = 0x824563C0;
	sub_831A8130(ctx, base);
	// 824563C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824563C4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824563C8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 824563CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824563D0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824563D4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 824563D8: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824563DC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824563E0: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 824563E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824563E8: 409A0040  bne cr6, 0x82456428
	if !ctx.cr[6].eq {
	pc = 0x82456428; continue 'dispatch;
	}
	// 824563EC: E93B0000  ld r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 824563F0: E94B0010  ld r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 824563F4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824563F8: 7F295040  cmpld cr6, r9, r10
	ctx.cr[6].compare_u64(ctx.r[9].u64, ctx.r[10].u64, &mut ctx.xer);
	// 824563FC: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82456400: 41980008  blt cr6, 0x82456408
	if ctx.cr[6].lt {
	pc = 0x82456408; continue 'dispatch;
	}
	// 82456404: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82456408: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8245640C: 4182000C  beq 0x82456418
	if ctx.cr[0].eq {
	pc = 0x82456418; continue 'dispatch;
	}
	// 82456410: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456414: 48000008  b 0x8245641c
	pc = 0x8245641C; continue 'dispatch;
	// 82456418: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245641C: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456420: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82456424: 419AFFCC  beq cr6, 0x824563f0
	if ctx.cr[6].eq {
	pc = 0x824563F0; continue 'dispatch;
	}
	// 82456428: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8245642C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82456430: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82456434: 41820048  beq 0x8245647c
	if ctx.cr[0].eq {
	pc = 0x8245647C; continue 'dispatch;
	}
	// 82456438: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245643C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456440: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456444: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82456448: 409A002C  bne cr6, 0x82456474
	if !ctx.cr[6].eq {
	pc = 0x82456474; continue 'dispatch;
	}
	// 8245644C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82456450: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82456454: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82456458: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8245645C: 4BFFF935  bl 0x82455d90
	ctx.lr = 0x82456460;
	sub_82455D90(ctx, base);
	// 82456460: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82456464: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82456468: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245646C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82456470: 48000044  b 0x824564b4
	pc = 0x824564B4; continue 'dispatch;
	// 82456474: 480F8BAD  bl 0x8254f020
	ctx.lr = 0x82456478;
	sub_8254F020(ctx, base);
	// 82456478: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245647C: E96A0010  ld r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	// 82456480: E93B0000  ld r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 82456484: 7F2B4840  cmpld cr6, r11, r9
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82456488: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 8245648C: 41980008  blt cr6, 0x82456494
	if ctx.cr[6].lt {
	pc = 0x82456494; continue 'dispatch;
	}
	// 82456490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456494: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82456498: 41820010  beq 0x824564a8
	if ctx.cr[0].eq {
	pc = 0x824564A8; continue 'dispatch;
	}
	// 8245649C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824564A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824564A4: 4BFFFFAC  b 0x82456450
	pc = 0x82456450; continue 'dispatch;
	// 824564A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824564AC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824564B0: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 824564B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824564B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824564BC: 48D51CF4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824564C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824564C0 size=100
    let mut pc: u32 = 0x824564C0;
    'dispatch: loop {
        match pc {
            0x824564C0 => {
    //   block [0x824564C0..0x82456524)
	// 824564C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824564C4: 48D51CA5  bl 0x831a8168
	ctx.lr = 0x824564C8;
	sub_831A8130(ctx, base);
	// 824564C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824564CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824564D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824564D4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 824564D8: 897E0029  lbz r11, 0x29(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(41 as u32) ) } as u64;
	// 824564DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824564E0: 409A003C  bne cr6, 0x8245651c
	if !ctx.cr[6].eq {
	pc = 0x8245651C; continue 'dispatch;
	}
	// 824564E4: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 824564E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824564EC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824564F0: 4BFFFFD1  bl 0x824564c0
	ctx.lr = 0x824564F4;
	sub_824564C0(ctx, base);
	// 824564F4: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 824564F8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824564FC: 4BFFF76D  bl 0x82455c68
	ctx.lr = 0x82456500;
	sub_82455C68(ctx, base);
	// 82456500: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456504: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82456508: 4899BC81  bl 0x82df2188
	ctx.lr = 0x8245650C;
	sub_82DF2188(ctx, base);
	// 8245650C: 897F0029  lbz r11, 0x29(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456510: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82456514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82456518: 419AFFD0  beq cr6, 0x824564e8
	if ctx.cr[6].eq {
	pc = 0x824564E8; continue 'dispatch;
	}
	// 8245651C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82456520: 48D51C98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456528 size=128
    let mut pc: u32 = 0x82456528;
    'dispatch: loop {
        match pc {
            0x82456528 => {
    //   block [0x82456528..0x824565A8)
	// 82456528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245652C: 48D51C3D  bl 0x831a8168
	ctx.lr = 0x82456530;
	sub_831A8130(ctx, base);
	// 82456530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456534: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456538: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245653C: 897E0029  lbz r11, 0x29(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456540: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82456548: 409A0054  bne cr6, 0x8245659c
	if !ctx.cr[6].eq {
	pc = 0x8245659C; continue 'dispatch;
	}
	// 8245654C: 5784003E  slwi r4, r28, 0
	ctx.r[4].u32 = ctx.r[28].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82456550: 891E0028  lbz r8, 0x28(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82456554: 38FE0010  addi r7, r30, 0x10
	ctx.r[7].s64 = ctx.r[30].s64 + 16;
	// 82456558: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245655C: 4BFFF7C5  bl 0x82455d20
	ctx.lr = 0x82456560;
	sub_82455D20(ctx, base);
	// 82456560: 897C0029  lbz r11, 0x29(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456564: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82456568: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245656C: 419A0008  beq cr6, 0x82456574
	if ctx.cr[6].eq {
	pc = 0x82456574; continue 'dispatch;
	}
	// 82456570: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82456574: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82456578: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245657C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456580: 4BFFFFA9  bl 0x82456528
	ctx.lr = 0x82456584;
	sub_82456528(ctx, base);
	// 82456584: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82456588: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8245658C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456594: 4BFFFF95  bl 0x82456528
	ctx.lr = 0x82456598;
	sub_82456528(ctx, base);
	// 82456598: 907D0008  stw r3, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8245659C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824565A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824565A4: 48D51C14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824565A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824565A8 size=460
    let mut pc: u32 = 0x824565A8;
    'dispatch: loop {
        match pc {
            0x824565A8 => {
    //   block [0x824565A8..0x82456774)
	// 824565A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824565AC: 48D51BB5  bl 0x831a8160
	ctx.lr = 0x824565B0;
	sub_831A8130(ctx, base);
	// 824565B0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824565B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824565B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824565BC: FBA10148  std r29, 0x148(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[29].u64 ) };
	// 824565C0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824565C4: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 824565C8: 38A10148  addi r5, r1, 0x148
	ctx.r[5].s64 = ctx.r[1].s64 + 328;
	// 824565CC: 9B810157  stb r28, 0x157(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(343 as u32), ctx.r[28].u8 ) };
	// 824565D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824565D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824565D8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 824565DC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 824565E0: 4BFFDF11  bl 0x824544f0
	ctx.lr = 0x824565E4;
	sub_824544F0(ctx, base);
	// 824565E4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824565E8: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824565EC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824565F0: 409A009C  bne cr6, 0x8245668c
	if !ctx.cr[6].eq {
	pc = 0x8245668C; continue 'dispatch;
	}
	// 824565F4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824565F8: 48738FA1  bl 0x82b8f598
	ctx.lr = 0x824565FC;
	sub_82B8F598(ctx, base);
	// 824565FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82456600: 48738F99  bl 0x82b8f598
	ctx.lr = 0x82456604;
	sub_82B8F598(ctx, base);
	// 82456604: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82456608: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8245660C: 4BFFF52D  bl 0x82455b38
	ctx.lr = 0x82456610;
	sub_82455B38(ctx, base);
	// 82456610: FBA100C0  std r29, 0xc0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[29].u64 ) };
	// 82456614: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82456618: 48738F81  bl 0x82b8f598
	ctx.lr = 0x8245661C;
	sub_82B8F598(ctx, base);
	// 8245661C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82456620: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82456624: 4BFFF515  bl 0x82455b38
	ctx.lr = 0x82456628;
	sub_82455B38(ctx, base);
	// 82456628: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8245662C: 4BFFF63D  bl 0x82455c68
	ctx.lr = 0x82456630;
	sub_82455C68(ctx, base);
	// 82456630: E96100C0  ld r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) };
	// 82456634: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82456638: F96100E0  std r11, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[11].u64 ) };
	// 8245663C: 48738F5D  bl 0x82b8f598
	ctx.lr = 0x82456640;
	sub_82B8F598(ctx, base);
	// 82456640: 388100C8  addi r4, r1, 0xc8
	ctx.r[4].s64 = ctx.r[1].s64 + 200;
	// 82456644: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82456648: 4BFFF4F1  bl 0x82455b38
	ctx.lr = 0x8245664C;
	sub_82455B38(ctx, base);
	// 8245664C: 38A100E0  addi r5, r1, 0xe0
	ctx.r[5].s64 = ctx.r[1].s64 + 224;
	// 82456650: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456654: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456658: 4BFFFD61  bl 0x824563b8
	ctx.lr = 0x8245665C;
	sub_824563B8(ctx, base);
	// 8245665C: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82456660: 4BFFF609  bl 0x82455c68
	ctx.lr = 0x82456664;
	sub_82455C68(ctx, base);
	// 82456664: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82456668: 4BFFF601  bl 0x82455c68
	ctx.lr = 0x8245666C;
	sub_82455C68(ctx, base);
	// 8245666C: 38A10148  addi r5, r1, 0x148
	ctx.r[5].s64 = ctx.r[1].s64 + 328;
	// 82456670: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456678: 4BFFDE79  bl 0x824544f0
	ctx.lr = 0x8245667C;
	sub_824544F0(ctx, base);
	// 8245667C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82456680: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82456684: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456688: 4BFFF5E1  bl 0x82455c68
	ctx.lr = 0x8245668C;
	sub_82455C68(ctx, base);
	// 8245668C: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 82456690: 38A10157  addi r5, r1, 0x157
	ctx.r[5].s64 = ctx.r[1].s64 + 343;
	// 82456694: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456698: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245669C: 4BFFDECD  bl 0x82454568
	ctx.lr = 0x824566A0;
	sub_82454568(ctx, base);
	// 824566A0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824566A4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824566A8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824566AC: 409A008C  bne cr6, 0x82456738
	if !ctx.cr[6].eq {
	pc = 0x82456738; continue 'dispatch;
	}
	// 824566B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824566B4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824566B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824566BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824566C0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824566C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 824566C8: 4BFFE641  bl 0x82454d08
	ctx.lr = 0x824566CC;
	sub_82454D08(ctx, base);
	// 824566CC: 9B810080  stb r28, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u8 ) };
	// 824566D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824566D4: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 824566D8: 4BFFE631  bl 0x82454d08
	ctx.lr = 0x824566DC;
	sub_82454D08(ctx, base);
	// 824566DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824566E0: 4BFFE0F9  bl 0x824547d8
	ctx.lr = 0x824566E4;
	sub_824547D8(ctx, base);
	// 824566E4: 89610080  lbz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824566E8: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 824566EC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 824566F0: 996100A0  stb r11, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u8 ) };
	// 824566F4: 4BFFE615  bl 0x82454d08
	ctx.lr = 0x824566F8;
	sub_82454D08(ctx, base);
	// 824566F8: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 824566FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456700: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456704: 4BFFF265  bl 0x82455968
	ctx.lr = 0x82456708;
	sub_82455968(ctx, base);
	// 82456708: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8245670C: 4BFFE0CD  bl 0x824547d8
	ctx.lr = 0x82456710;
	sub_824547D8(ctx, base);
	// 82456710: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82456714: 4BFFE0C5  bl 0x824547d8
	ctx.lr = 0x82456718;
	sub_824547D8(ctx, base);
	// 82456718: 38A10157  addi r5, r1, 0x157
	ctx.r[5].s64 = ctx.r[1].s64 + 343;
	// 8245671C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456720: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456724: 4BFFDE45  bl 0x82454568
	ctx.lr = 0x82456728;
	sub_82454568(ctx, base);
	// 82456728: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245672C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82456730: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456734: 4BFFE0A5  bl 0x824547d8
	ctx.lr = 0x82456738;
	sub_824547D8(ctx, base);
	// 82456738: 5744063E  clrlwi r4, r26, 0x18
	ctx.r[4].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 8245673C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82456740: 4BFFDF71  bl 0x824546b0
	ctx.lr = 0x82456744;
	sub_824546B0(ctx, base);
	// 82456744: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82456748: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8245674C: 4BFFDA35  bl 0x82454180
	ctx.lr = 0x82456750;
	sub_82454180(ctx, base);
	// 82456750: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82456754: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82456758: 4BFFEA09  bl 0x82455160
	ctx.lr = 0x8245675C;
	sub_82455160(ctx, base);
	// 8245675C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82456760: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82456764: 419A0008  beq cr6, 0x8245676c
	if ctx.cr[6].eq {
	pc = 0x8245676C; continue 'dispatch;
	}
	// 82456768: 4BE6A129  bl 0x822c0890
	ctx.lr = 0x8245676C;
	sub_822C0890(ctx, base);
	// 8245676C: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82456770: 48D51A40  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456778 size=432
    let mut pc: u32 = 0x82456778;
    'dispatch: loop {
        match pc {
            0x82456778 => {
    //   block [0x82456778..0x82456928)
	// 82456778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245677C: 48D519E9  bl 0x831a8164
	ctx.lr = 0x82456780;
	sub_831A8130(ctx, base);
	// 82456780: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456784: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82456788: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245678C: FBA10148  std r29, 0x148(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[29].u64 ) };
	// 82456790: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82456794: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 82456798: 38A10148  addi r5, r1, 0x148
	ctx.r[5].s64 = ctx.r[1].s64 + 328;
	// 8245679C: 9B810157  stb r28, 0x157(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(343 as u32), ctx.r[28].u8 ) };
	// 824567A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824567A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824567A8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 824567AC: 4BFFDD45  bl 0x824544f0
	ctx.lr = 0x824567B0;
	sub_824544F0(ctx, base);
	// 824567B0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 824567B4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824567B8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824567BC: 409A009C  bne cr6, 0x82456858
	if !ctx.cr[6].eq {
	pc = 0x82456858; continue 'dispatch;
	}
	// 824567C0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824567C4: 48738DD5  bl 0x82b8f598
	ctx.lr = 0x824567C8;
	sub_82B8F598(ctx, base);
	// 824567C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824567CC: 48738DCD  bl 0x82b8f598
	ctx.lr = 0x824567D0;
	sub_82B8F598(ctx, base);
	// 824567D0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824567D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824567D8: 4BFFF361  bl 0x82455b38
	ctx.lr = 0x824567DC;
	sub_82455B38(ctx, base);
	// 824567DC: FBA100C0  std r29, 0xc0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[29].u64 ) };
	// 824567E0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 824567E4: 48738DB5  bl 0x82b8f598
	ctx.lr = 0x824567E8;
	sub_82B8F598(ctx, base);
	// 824567E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824567EC: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 824567F0: 4BFFF349  bl 0x82455b38
	ctx.lr = 0x824567F4;
	sub_82455B38(ctx, base);
	// 824567F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824567F8: 4BFFF471  bl 0x82455c68
	ctx.lr = 0x824567FC;
	sub_82455C68(ctx, base);
	// 824567FC: E96100C0  ld r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) };
	// 82456800: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82456804: F96100E0  std r11, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[11].u64 ) };
	// 82456808: 48738D91  bl 0x82b8f598
	ctx.lr = 0x8245680C;
	sub_82B8F598(ctx, base);
	// 8245680C: 388100C8  addi r4, r1, 0xc8
	ctx.r[4].s64 = ctx.r[1].s64 + 200;
	// 82456810: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82456814: 4BFFF325  bl 0x82455b38
	ctx.lr = 0x82456818;
	sub_82455B38(ctx, base);
	// 82456818: 38A100E0  addi r5, r1, 0xe0
	ctx.r[5].s64 = ctx.r[1].s64 + 224;
	// 8245681C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456824: 4BFFFB95  bl 0x824563b8
	ctx.lr = 0x82456828;
	sub_824563B8(ctx, base);
	// 82456828: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8245682C: 4BFFF43D  bl 0x82455c68
	ctx.lr = 0x82456830;
	sub_82455C68(ctx, base);
	// 82456830: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82456834: 4BFFF435  bl 0x82455c68
	ctx.lr = 0x82456838;
	sub_82455C68(ctx, base);
	// 82456838: 38A10148  addi r5, r1, 0x148
	ctx.r[5].s64 = ctx.r[1].s64 + 328;
	// 8245683C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456840: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456844: 4BFFDCAD  bl 0x824544f0
	ctx.lr = 0x82456848;
	sub_824544F0(ctx, base);
	// 82456848: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245684C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82456850: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456854: 4BFFF415  bl 0x82455c68
	ctx.lr = 0x82456858;
	sub_82455C68(ctx, base);
	// 82456858: 3BFE0018  addi r31, r30, 0x18
	ctx.r[31].s64 = ctx.r[30].s64 + 24;
	// 8245685C: 38A10157  addi r5, r1, 0x157
	ctx.r[5].s64 = ctx.r[1].s64 + 343;
	// 82456860: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456868: 4BFFDD01  bl 0x82454568
	ctx.lr = 0x8245686C;
	sub_82454568(ctx, base);
	// 8245686C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82456870: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82456874: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82456878: 409A008C  bne cr6, 0x82456904
	if !ctx.cr[6].eq {
	pc = 0x82456904; continue 'dispatch;
	}
	// 8245687C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456880: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82456884: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82456888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245688C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82456890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82456894: 4BFFE475  bl 0x82454d08
	ctx.lr = 0x82456898;
	sub_82454D08(ctx, base);
	// 82456898: 9B810080  stb r28, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u8 ) };
	// 8245689C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824568A0: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 824568A4: 4BFFE465  bl 0x82454d08
	ctx.lr = 0x824568A8;
	sub_82454D08(ctx, base);
	// 824568A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824568AC: 4BFFDF2D  bl 0x824547d8
	ctx.lr = 0x824568B0;
	sub_824547D8(ctx, base);
	// 824568B0: 89610080  lbz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824568B4: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 824568B8: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 824568BC: 996100A0  stb r11, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u8 ) };
	// 824568C0: 4BFFE449  bl 0x82454d08
	ctx.lr = 0x824568C4;
	sub_82454D08(ctx, base);
	// 824568C4: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 824568C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824568CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824568D0: 4BFFF099  bl 0x82455968
	ctx.lr = 0x824568D4;
	sub_82455968(ctx, base);
	// 824568D4: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 824568D8: 4BFFDF01  bl 0x824547d8
	ctx.lr = 0x824568DC;
	sub_824547D8(ctx, base);
	// 824568DC: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 824568E0: 4BFFDEF9  bl 0x824547d8
	ctx.lr = 0x824568E4;
	sub_824547D8(ctx, base);
	// 824568E4: 38A10157  addi r5, r1, 0x157
	ctx.r[5].s64 = ctx.r[1].s64 + 343;
	// 824568E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824568EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824568F0: 4BFFDC79  bl 0x82454568
	ctx.lr = 0x824568F4;
	sub_82454568(ctx, base);
	// 824568F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824568F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824568FC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456900: 4BFFDED9  bl 0x824547d8
	ctx.lr = 0x82456904;
	sub_824547D8(ctx, base);
	// 82456904: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82456908: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 8245690C: 4BFFE855  bl 0x82455160
	ctx.lr = 0x82456910;
	sub_82455160(ctx, base);
	// 82456910: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82456914: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82456918: 419A0008  beq cr6, 0x82456920
	if ctx.cr[6].eq {
	pc = 0x82456920; continue 'dispatch;
	}
	// 8245691C: 4BE69F75  bl 0x822c0890
	ctx.lr = 0x82456920;
	sub_822C0890(ctx, base);
	// 82456920: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82456924: 48D51890  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456928 size=84
    let mut pc: u32 = 0x82456928;
    'dispatch: loop {
        match pc {
            0x82456928 => {
    //   block [0x82456928..0x8245697C)
	// 82456928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245693C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456940: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456944: 4BFFFB7D  bl 0x824564c0
	ctx.lr = 0x82456948;
	sub_824564C0(ctx, base);
	// 82456948: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245694C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456950: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82456954: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82456958: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245695C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82456960: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456964: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82456968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245696C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456980 size=168
    let mut pc: u32 = 0x82456980;
    'dispatch: loop {
        match pc {
            0x82456980 => {
    //   block [0x82456980..0x82456A28)
	// 82456980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456984: 48D517E9  bl 0x831a816c
	ctx.lr = 0x82456988;
	sub_831A8130(ctx, base);
	// 82456988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245698C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82456994: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456998: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245699C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824569A0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824569A4: 4BFFFB85  bl 0x82456528
	ctx.lr = 0x824569A8;
	sub_82456528(ctx, base);
	// 824569A8: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 824569AC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824569B0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824569B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824569B8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824569BC: 896A0029  lbz r11, 0x29(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 824569C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824569C4: 409A0050  bne cr6, 0x82456a14
	if !ctx.cr[6].eq {
	pc = 0x82456A14; continue 'dispatch;
	}
	// 824569C8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824569CC: 4800000C  b 0x824569d8
	pc = 0x824569D8; continue 'dispatch;
	// 824569D0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824569D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824569D8: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 824569DC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824569E0: 419AFFF0  beq cr6, 0x824569d0
	if ctx.cr[6].eq {
	pc = 0x824569D0; continue 'dispatch;
	}
	// 824569E4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824569E8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824569EC: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824569F0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824569F4: 4800000C  b 0x82456a00
	pc = 0x82456A00; continue 'dispatch;
	// 824569F8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824569FC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456A00: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82456A04: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82456A08: 419AFFF0  beq cr6, 0x824569f8
	if ctx.cr[6].eq {
	pc = 0x824569F8; continue 'dispatch;
	}
	// 82456A0C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82456A10: 48000010  b 0x82456a20
	pc = 0x82456A20; continue 'dispatch;
	// 82456A14: 91290000  stw r9, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82456A18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456A1C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82456A20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82456A24: 48D51798  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456A28 size=132
    let mut pc: u32 = 0x82456A28;
    'dispatch: loop {
        match pc {
            0x82456A28 => {
    //   block [0x82456A28..0x82456AAC)
	// 82456A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456A2C: 48D5173D  bl 0x831a8168
	ctx.lr = 0x82456A30;
	sub_831A8130(ctx, base);
	// 82456A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456A34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82456A38: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82456A3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82456A40: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82456A44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456A48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456A4C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82456A50: 409A0044  bne cr6, 0x82456a94
	if !ctx.cr[6].eq {
	pc = 0x82456A94; continue 'dispatch;
	}
	// 82456A54: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82456A58: 409A003C  bne cr6, 0x82456a94
	if !ctx.cr[6].eq {
	pc = 0x82456A94; continue 'dispatch;
	}
	// 82456A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456A60: 4BFFFEC9  bl 0x82456928
	ctx.lr = 0x82456A64;
	sub_82456928(ctx, base);
	// 82456A64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456A68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456A6C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82456A70: 48000030  b 0x82456aa0
	pc = 0x82456AA0; continue 'dispatch;
	// 82456A74: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82456A78: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82456A7C: 4809268D  bl 0x824e9108
	ctx.lr = 0x82456A80;
	sub_824E9108(ctx, base);
	// 82456A80: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82456A84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456A88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456A8C: 4BFFF52D  bl 0x82455fb8
	ctx.lr = 0x82456A90;
	sub_82455FB8(ctx, base);
	// 82456A90: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82456A94: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82456A98: 409AFFDC  bne cr6, 0x82456a74
	if !ctx.cr[6].eq {
	pc = 0x82456A74; continue 'dispatch;
	}
	// 82456A9C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82456AA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82456AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82456AA8: 48D51710  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456AB0 size=96
    let mut pc: u32 = 0x82456AB0;
    'dispatch: loop {
        match pc {
            0x82456AB0 => {
    //   block [0x82456AB0..0x82456B10)
	// 82456AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456AB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82456ABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456AC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82456AC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456ACC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82456AD0: 419A0024  beq cr6, 0x82456af4
	if ctx.cr[6].eq {
	pc = 0x82456AF4; continue 'dispatch;
	}
	// 82456AD4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456AD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456ADC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456AE0: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456AE4: 4BFFFF45  bl 0x82456a28
	ctx.lr = 0x82456AE8;
	sub_82456A28(ctx, base);
	// 82456AE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456AF0: 4BFFFE91  bl 0x82456980
	ctx.lr = 0x82456AF4;
	sub_82456980(ctx, base);
	// 82456AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82456AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456B04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82456B08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82456B10 size=132
    let mut pc: u32 = 0x82456B10;
    'dispatch: loop {
        match pc {
            0x82456B10 => {
    //   block [0x82456B10..0x82456B94)
	// 82456B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456B14: 48D51659  bl 0x831a816c
	ctx.lr = 0x82456B18;
	sub_831A8130(ctx, base);
	// 82456B18: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456B1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82456B20: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82456B24: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82456B28: 486F0659  bl 0x82b47180
	ctx.lr = 0x82456B2C;
	sub_82B47180(ctx, base);
	// 82456B2C: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82456B30: 486F0651  bl 0x82b47180
	ctx.lr = 0x82456B34;
	sub_82B47180(ctx, base);
	// 82456B34: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82456B38: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 82456B3C: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82456B40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456B44: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456B48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456B4C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82456B50: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82456B54: 4BFFDBC5  bl 0x82454718
	ctx.lr = 0x82456B58;
	sub_82454718(ctx, base);
	// 82456B58: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82456B5C: 4BFFFDCD  bl 0x82456928
	ctx.lr = 0x82456B60;
	sub_82456928(ctx, base);
	// 82456B60: 57BD063F  clrlwi. r29, r29, 0x18
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82456B64: 41820028  beq 0x82456b8c
	if ctx.cr[0].eq {
	pc = 0x82456B8C; continue 'dispatch;
	}
	// 82456B68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82456B6C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82456B70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456B74: 4803B15D  bl 0x82491cd0
	ctx.lr = 0x82456B78;
	sub_82491CD0(ctx, base);
	// 82456B78: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82456B7C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82456B80: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82456B84: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82456B88: 4198FFE4  blt cr6, 0x82456b6c
	if ctx.cr[6].lt {
	pc = 0x82456B6C; continue 'dispatch;
	}
	// 82456B8C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82456B90: 48D5162C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456B98 size=88
    let mut pc: u32 = 0x82456B98;
    'dispatch: loop {
        match pc {
            0x82456B98 => {
    //   block [0x82456B98..0x82456BF0)
	// 82456B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456BA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456BA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456BA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82456BAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82456BB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456BB4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456BB8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82456BBC: 4BFFFE6D  bl 0x82456a28
	ctx.lr = 0x82456BC0;
	sub_82456A28(ctx, base);
	// 82456BC0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82456BC4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456BC8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82456BCC: 4899B5BD  bl 0x82df2188
	ctx.lr = 0x82456BD0;
	sub_82DF2188(ctx, base);
	// 82456BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456BD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82456BD8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82456BDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82456BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456BE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82456BF0 size=88
    let mut pc: u32 = 0x82456BF0;
    'dispatch: loop {
        match pc {
            0x82456BF0 => {
    //   block [0x82456BF0..0x82456C48)
	// 82456BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456BF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82456BFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456C04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82456C0C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82456C10: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82456C14: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82456C18: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82456C1C: 4BFFDD15  bl 0x82454930
	ctx.lr = 0x82456C20;
	sub_82454930(ctx, base);
	// 82456C20: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82456C24: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 82456C28: 4BFFFE89  bl 0x82456ab0
	ctx.lr = 0x82456C2C;
	sub_82456AB0(ctx, base);
	// 82456C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456C30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82456C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456C3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82456C40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82456C48 size=104
    let mut pc: u32 = 0x82456C48;
    'dispatch: loop {
        match pc {
            0x82456C48 => {
    //   block [0x82456C48..0x82456CB0)
	// 82456C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456C50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82456C54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82456C60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456C64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82456C68: 419A0030  beq cr6, 0x82456c98
	if ctx.cr[6].eq {
	pc = 0x82456C98; continue 'dispatch;
	}
	// 82456C6C: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82456C70: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82456C74: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82456C78: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82456C7C: 4BFFDF45  bl 0x82454bc0
	ctx.lr = 0x82456C80;
	sub_82454BC0(ctx, base);
	// 82456C80: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 82456C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456C88: 4BFFD971  bl 0x824545f8
	ctx.lr = 0x82456C8C;
	sub_824545F8(ctx, base);
	// 82456C8C: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 82456C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456C94: 4BFFFCED  bl 0x82456980
	ctx.lr = 0x82456C98;
	sub_82456980(ctx, base);
	// 82456C98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82456C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456CA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82456CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456CB0 size=92
    let mut pc: u32 = 0x82456CB0;
    'dispatch: loop {
        match pc {
            0x82456CB0 => {
    //   block [0x82456CB0..0x82456D0C)
	// 82456CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456CB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82456CBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456CC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456CC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456CC8: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82456CCC: 419A0028  beq cr6, 0x82456cf4
	if ctx.cr[6].eq {
	pc = 0x82456CF4; continue 'dispatch;
	}
	// 82456CD0: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82456CD4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82456CD8: 4BFFFEC1  bl 0x82456b98
	ctx.lr = 0x82456CDC;
	sub_82456B98(ctx, base);
	// 82456CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456CE0: 48013321  bl 0x8246a000
	ctx.lr = 0x82456CE4;
	sub_8246A000(ctx, base);
	// 82456CE4: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82456CE8: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82456CEC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82456CF0: 409AFFE4  bne cr6, 0x82456cd4
	if !ctx.cr[6].eq {
	pc = 0x82456CD4; continue 'dispatch;
	}
	// 82456CF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82456CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456D00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82456D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456D10 size=116
    let mut pc: u32 = 0x82456D10;
    'dispatch: loop {
        match pc {
            0x82456D10 => {
    //   block [0x82456D10..0x82456D84)
	// 82456D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456D14: 48D5144D  bl 0x831a8160
	ctx.lr = 0x82456D18;
	sub_831A8130(ctx, base);
	// 82456D18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456D1C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82456D20: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82456D24: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82456D28: 7F1B3040  cmplw cr6, r27, r6
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82456D2C: 419A0048  beq cr6, 0x82456d74
	if ctx.cr[6].eq {
	pc = 0x82456D74; continue 'dispatch;
	}
	// 82456D30: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456D34: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82456D38: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82456D3C: 7F06E840  cmplw cr6, r6, r29
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82456D40: 419A0020  beq cr6, 0x82456d60
	if ctx.cr[6].eq {
	pc = 0x82456D60; continue 'dispatch;
	}
	// 82456D44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82456D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456D4C: 4BFFFEA5  bl 0x82456bf0
	ctx.lr = 0x82456D50;
	sub_82456BF0(ctx, base);
	// 82456D50: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 82456D54: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82456D58: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82456D5C: 409AFFE8  bne cr6, 0x82456d44
	if !ctx.cr[6].eq {
	pc = 0x82456D44; continue 'dispatch;
	}
	// 82456D60: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82456D64: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82456D6C: 4BFFFF45  bl 0x82456cb0
	ctx.lr = 0x82456D70;
	sub_82456CB0(ctx, base);
	// 82456D70: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82456D74: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82456D78: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82456D7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82456D80: 48D51430  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82456D88 size=96
    let mut pc: u32 = 0x82456D88;
    'dispatch: loop {
        match pc {
            0x82456D88 => {
    //   block [0x82456D88..0x82456DE8)
	// 82456D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82456D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82456D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82456D9C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456DA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82456DA4: 419A0020  beq cr6, 0x82456dc4
	if ctx.cr[6].eq {
	pc = 0x82456DC4; continue 'dispatch;
	}
	// 82456DA8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82456DAC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456DB0: 4BFFFF01  bl 0x82456cb0
	ctx.lr = 0x82456DB4;
	sub_82456CB0(ctx, base);
	// 82456DB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82456DB8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456DBC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82456DC0: 4899B3C9  bl 0x82df2188
	ctx.lr = 0x82456DC4;
	sub_82DF2188(ctx, base);
	// 82456DC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456DC8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82456DCC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82456DD0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82456DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82456DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82456DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82456DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82456DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82456DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82456DE8 size=888
    let mut pc: u32 = 0x82456DE8;
    'dispatch: loop {
        match pc {
            0x82456DE8 => {
    //   block [0x82456DE8..0x82457160)
	// 82456DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82456DEC: 48D51369  bl 0x831a8154
	ctx.lr = 0x82456DF0;
	sub_831A8130(ctx, base);
	// 82456DF0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82456DF4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82456DF8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82456DFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82456E00: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82456E04: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82456E08: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82456E0C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82456E10: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82456E14: 4BFFDDAD  bl 0x82454bc0
	ctx.lr = 0x82456E18;
	sub_82454BC0(ctx, base);
	// 82456E18: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82456E1C: 4BFFD7DD  bl 0x824545f8
	ctx.lr = 0x82456E20;
	sub_824545F8(ctx, base);
	// 82456E20: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82456E24: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82456E28: 4BFFFB59  bl 0x82456980
	ctx.lr = 0x82456E2C;
	sub_82456980(ctx, base);
	// 82456E2C: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456E30: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82456E34: 409A000C  bne cr6, 0x82456e40
	if !ctx.cr[6].eq {
	pc = 0x82456E40; continue 'dispatch;
	}
	// 82456E38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82456E3C: 48000010  b 0x82456e4c
	pc = 0x82456E4C; continue 'dispatch;
	// 82456E40: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82456E44: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82456E48: 7D682E70  srawi r8, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82456E4C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82456E50: 419A02F8  beq cr6, 0x82457148
	if ctx.cr[6].eq {
	pc = 0x82457148; continue 'dispatch;
	}
	// 82456E54: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82456E58: 409A000C  bne cr6, 0x82456e64
	if !ctx.cr[6].eq {
	pc = 0x82456E64; continue 'dispatch;
	}
	// 82456E5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456E60: 48000010  b 0x82456e70
	pc = 0x82456E70; continue 'dispatch;
	// 82456E64: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456E68: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82456E6C: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82456E70: 3D4007FF  lis r10, 0x7ff
	ctx.r[10].s64 = 134152192;
	// 82456E74: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82456E78: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82456E7C: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82456E80: 4098000C  bge cr6, 0x82456e8c
	if !ctx.cr[6].lt {
	pc = 0x82456E8C; continue 'dispatch;
	}
	// 82456E84: 48760E9D  bl 0x82bb7d20
	ctx.lr = 0x82456E88;
	sub_82BB7D20(ctx, base);
	// 82456E88: 480002C0  b 0x82457148
	pc = 0x82457148; continue 'dispatch;
	// 82456E8C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82456E90: 409A000C  bne cr6, 0x82456e9c
	if !ctx.cr[6].eq {
	pc = 0x82456E9C; continue 'dispatch;
	}
	// 82456E94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456E98: 48000010  b 0x82456ea8
	pc = 0x82456EA8; continue 'dispatch;
	// 82456E9C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456EA0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82456EA4: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82456EA8: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82456EAC: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82456EB0: 40980170  bge cr6, 0x82457020
	if !ctx.cr[6].lt {
	pc = 0x82457020; continue 'dispatch;
	}
	// 82456EB4: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82456EB8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82456EBC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82456EC0: 4098000C  bge cr6, 0x82456ecc
	if !ctx.cr[6].lt {
	pc = 0x82456ECC; continue 'dispatch;
	}
	// 82456EC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456EC8: 48000008  b 0x82456ed0
	pc = 0x82456ED0; continue 'dispatch;
	// 82456ECC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82456ED0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82456ED4: 409A000C  bne cr6, 0x82456ee0
	if !ctx.cr[6].eq {
	pc = 0x82456EE0; continue 'dispatch;
	}
	// 82456ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82456EDC: 48000010  b 0x82456eec
	pc = 0x82456EEC; continue 'dispatch;
	// 82456EE0: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456EE4: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82456EE8: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 82456EEC: 7D4ACA14  add r10, r10, r25
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 82456EF0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82456EF4: 40980024  bge cr6, 0x82456f18
	if !ctx.cr[6].lt {
	pc = 0x82456F18; continue 'dispatch;
	}
	// 82456EF8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82456EFC: 409A000C  bne cr6, 0x82456f08
	if !ctx.cr[6].eq {
	pc = 0x82456F08; continue 'dispatch;
	}
	// 82456F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456F04: 48000010  b 0x82456f14
	pc = 0x82456F14; continue 'dispatch;
	// 82456F08: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456F0C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82456F10: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82456F14: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82456F18: 55772834  slwi r23, r11, 5
	ctx.r[23].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82456F1C: 3F008335  lis r24, -0x7ccb
	ctx.r[24].s64 = -2093678592;
	// 82456F20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82456F24: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82456F28: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 82456F2C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82456F30: 8078110C  lwz r3, 0x110c(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82456F34: 4899B195  bl 0x82df20c8
	ctx.lr = 0x82456F38;
	sub_82DF20C8(ctx, base);
	// 82456F38: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82456F3C: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456F40: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82456F44: 48000018  b 0x82456f5c
	pc = 0x82456F5C; continue 'dispatch;
	// 82456F48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456F4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82456F50: 4BFFFCF9  bl 0x82456c48
	ctx.lr = 0x82456F54;
	sub_82456C48(ctx, base);
	// 82456F54: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82456F58: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 82456F5C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82456F60: 409AFFE8  bne cr6, 0x82456f48
	if !ctx.cr[6].eq {
	pc = 0x82456F48; continue 'dispatch;
	}
	// 82456F64: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82456F68: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82456F6C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82456F70: 419A001C  beq cr6, 0x82456f8c
	if ctx.cr[6].eq {
	pc = 0x82456F8C; continue 'dispatch;
	}
	// 82456F74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82456F78: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82456F7C: 4BFFFCCD  bl 0x82456c48
	ctx.lr = 0x82456F80;
	sub_82456C48(ctx, base);
	// 82456F80: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82456F84: 3B7B0020  addi r27, r27, 0x20
	ctx.r[27].s64 = ctx.r[27].s64 + 32;
	// 82456F88: 4082FFEC  bne 0x82456f74
	if !ctx.cr[0].eq {
	pc = 0x82456F74; continue 'dispatch;
	}
	// 82456F8C: 572B2834  slwi r11, r25, 5
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82456F90: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82456F94: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82456F98: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456F9C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82456FA0: 419A0020  beq cr6, 0x82456fc0
	if ctx.cr[6].eq {
	pc = 0x82456FC0; continue 'dispatch;
	}
	// 82456FA4: 7FDE5850  subf r30, r30, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82456FA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82456FAC: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82456FB0: 4BFFFC99  bl 0x82456c48
	ctx.lr = 0x82456FB4;
	sub_82456C48(ctx, base);
	// 82456FB4: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82456FB8: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82456FBC: 409AFFEC  bne cr6, 0x82456fa8
	if !ctx.cr[6].eq {
	pc = 0x82456FA8; continue 'dispatch;
	}
	// 82456FC0: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82456FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82456FC8: 409A000C  bne cr6, 0x82456fd4
	if !ctx.cr[6].eq {
	pc = 0x82456FD4; continue 'dispatch;
	}
	// 82456FCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82456FD0: 48000010  b 0x82456fe0
	pc = 0x82456FE0; continue 'dispatch;
	// 82456FD4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456FD8: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82456FDC: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82456FE0: 7FEBCA14  add r31, r11, r25
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82456FE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82456FE8: 419A001C  beq cr6, 0x82457004
	if ctx.cr[6].eq {
	pc = 0x82457004; continue 'dispatch;
	}
	// 82456FEC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82456FF0: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82456FF4: 4BFFFCBD  bl 0x82456cb0
	ctx.lr = 0x82456FF8;
	sub_82456CB0(ctx, base);
	// 82456FF8: 8078110C  lwz r3, 0x110c(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82456FFC: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457000: 4899B189  bl 0x82df2188
	ctx.lr = 0x82457004;
	sub_82DF2188(ctx, base);
	// 82457004: 57EB2834  slwi r11, r31, 5
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82457008: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 8245700C: 7D57D214  add r10, r23, r26
	ctx.r[10].u64 = ctx.r[23].u64 + ctx.r[26].u64;
	// 82457010: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82457014: 915C000C  stw r10, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82457018: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245701C: 4800012C  b 0x82457148
	pc = 0x82457148; continue 'dispatch;
	// 82457020: 83FC0008  lwz r31, 8(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457024: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82457028: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 8245702C: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82457030: 40980094  bge cr6, 0x824570c4
	if !ctx.cr[6].lt {
	pc = 0x824570C4; continue 'dispatch;
	}
	// 82457034: 573B2834  slwi r27, r25, 5
	ctx.r[27].u32 = ctx.r[25].u32.wrapping_shl(5);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82457038: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8245703C: 7D7BF214  add r11, r27, r30
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 82457040: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82457044: 419A0020  beq cr6, 0x82457064
	if ctx.cr[6].eq {
	pc = 0x82457064; continue 'dispatch;
	}
	// 82457048: 7F5E5850  subf r26, r30, r11
	ctx.r[26].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8245704C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82457050: 7C7AEA14  add r3, r26, r29
	ctx.r[3].u64 = ctx.r[26].u64 + ctx.r[29].u64;
	// 82457054: 4BFFFBF5  bl 0x82456c48
	ctx.lr = 0x82457058;
	sub_82456C48(ctx, base);
	// 82457058: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 8245705C: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82457060: 409AFFEC  bne cr6, 0x8245704c
	if !ctx.cr[6].eq {
	pc = 0x8245704C; continue 'dispatch;
	}
	// 82457064: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457068: 7D5E5850  subf r10, r30, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8245706C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82457070: 7D4B2E70  srawi r11, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 82457074: 7FEBC851  subf. r31, r11, r25
	ctx.r[31].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82457078: 4182001C  beq 0x82457094
	if ctx.cr[0].eq {
	pc = 0x82457094; continue 'dispatch;
	}
	// 8245707C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82457080: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82457084: 4BFFFBC5  bl 0x82456c48
	ctx.lr = 0x82457088;
	sub_82456C48(ctx, base);
	// 82457088: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245708C: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 82457090: 4082FFEC  bne 0x8245707c
	if !ctx.cr[0].eq {
	pc = 0x8245707C; continue 'dispatch;
	}
	// 82457094: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457098: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 8245709C: 7FFB5850  subf r31, r27, r11
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 824570A0: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824570A4: 48000014  b 0x824570b8
	pc = 0x824570B8; continue 'dispatch;
	// 824570A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824570AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824570B0: 4BFFFB41  bl 0x82456bf0
	ctx.lr = 0x824570B4;
	sub_82456BF0(ctx, base);
	// 824570B4: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 824570B8: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824570BC: 409AFFEC  bne cr6, 0x824570a8
	if !ctx.cr[6].eq {
	pc = 0x824570A8; continue 'dispatch;
	}
	// 824570C0: 48000088  b 0x82457148
	pc = 0x82457148; continue 'dispatch;
	// 824570C4: 57392834  slwi r25, r25, 5
	ctx.r[25].u32 = ctx.r[25].u32.wrapping_shl(5);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 824570C8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 824570CC: 7FB9F850  subf r29, r25, r31
	ctx.r[29].s64 = ctx.r[31].s64 - ctx.r[25].s64;
	// 824570D0: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 824570D4: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824570D8: 419A0020  beq cr6, 0x824570f8
	if ctx.cr[6].eq {
	pc = 0x824570F8; continue 'dispatch;
	}
	// 824570DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 824570E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824570E4: 4BFFFB65  bl 0x82456c48
	ctx.lr = 0x824570E8;
	sub_82456C48(ctx, base);
	// 824570E8: 3B5A0020  addi r26, r26, 0x20
	ctx.r[26].s64 = ctx.r[26].s64 + 32;
	// 824570EC: 3B7B0020  addi r27, r27, 0x20
	ctx.r[27].s64 = ctx.r[27].s64 + 32;
	// 824570F0: 7F1AF840  cmplw cr6, r26, r31
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824570F4: 409AFFE8  bne cr6, 0x824570dc
	if !ctx.cr[6].eq {
	pc = 0x824570DC; continue 'dispatch;
	}
	// 824570F8: 937C0008  stw r27, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 824570FC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82457100: 419A0020  beq cr6, 0x82457120
	if ctx.cr[6].eq {
	pc = 0x82457120; continue 'dispatch;
	}
	// 82457104: 7FFDF850  subf r31, r29, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 82457108: 3BBDFFE0  addi r29, r29, -0x20
	ctx.r[29].s64 = ctx.r[29].s64 + -32;
	// 8245710C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82457110: 7C7FEA14  add r3, r31, r29
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82457114: 4BFFFADD  bl 0x82456bf0
	ctx.lr = 0x82457118;
	sub_82456BF0(ctx, base);
	// 82457118: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8245711C: 409AFFEC  bne cr6, 0x82457108
	if !ctx.cr[6].eq {
	pc = 0x82457108; continue 'dispatch;
	}
	// 82457120: 7FB9F214  add r29, r25, r30
	ctx.r[29].u64 = ctx.r[25].u64 + ctx.r[30].u64;
	// 82457124: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82457128: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8245712C: 419A001C  beq cr6, 0x82457148
	if ctx.cr[6].eq {
	pc = 0x82457148; continue 'dispatch;
	}
	// 82457130: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82457134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457138: 4BFFFAB9  bl 0x82456bf0
	ctx.lr = 0x8245713C;
	sub_82456BF0(ctx, base);
	// 8245713C: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82457140: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82457144: 409AFFEC  bne cr6, 0x82457130
	if !ctx.cr[6].eq {
	pc = 0x82457130; continue 'dispatch;
	}
	// 82457148: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8245714C: 4BFFFA4D  bl 0x82456b98
	ctx.lr = 0x82457150;
	sub_82456B98(ctx, base);
	// 82457150: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82457154: 48012EAD  bl 0x8246a000
	ctx.lr = 0x82457158;
	sub_8246A000(ctx, base);
	// 82457158: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8245715C: 48D51048  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457160 size=60
    let mut pc: u32 = 0x82457160;
    'dispatch: loop {
        match pc {
            0x82457160 => {
    //   block [0x82457160..0x8245719C)
	// 82457160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82457164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82457168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245716C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82457170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82457174: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82457178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245717C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82457180: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82457184: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457188: 4BFFFB89  bl 0x82456d10
	ctx.lr = 0x8245718C;
	sub_82456D10(ctx, base);
	// 8245718C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82457190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82457194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82457198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824571A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824571A0 size=108
    let mut pc: u32 = 0x824571A0;
    'dispatch: loop {
        match pc {
            0x824571A0 => {
    //   block [0x824571A0..0x8245720C)
	// 824571A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824571A4: 48D50FC9  bl 0x831a816c
	ctx.lr = 0x824571A8;
	sub_831A8130(ctx, base);
	// 824571A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824571AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824571B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824571B4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824571B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824571BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824571C0: 419A0014  beq cr6, 0x824571d4
	if ctx.cr[6].eq {
	pc = 0x824571D4; continue 'dispatch;
	}
	// 824571C4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824571C8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824571CC: 7D4A2E71  srawi. r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824571D0: 4082000C  bne 0x824571dc
	if !ctx.cr[0].eq {
	pc = 0x824571DC; continue 'dispatch;
	}
	// 824571D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824571D8: 4800000C  b 0x824571e4
	pc = 0x824571E4; continue 'dispatch;
	// 824571DC: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 824571E0: 7D7D2E70  srawi r29, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 824571E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824571E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824571EC: 4BFFFBFD  bl 0x82456de8
	ctx.lr = 0x824571F0;
	sub_82456DE8(ctx, base);
	// 824571F0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824571F4: 57AB2834  slwi r11, r29, 5
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824571F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824571FC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82457200: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82457204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82457208: 48D50FB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457210 size=24
    let mut pc: u32 = 0x82457210;
    'dispatch: loop {
        match pc {
            0x82457210 => {
    //   block [0x82457210..0x82457228)
	// 82457210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82457214: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82457218: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245721C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82457220: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82457224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457228 size=152
    let mut pc: u32 = 0x82457228;
    'dispatch: loop {
        match pc {
            0x82457228 => {
    //   block [0x82457228..0x824572C0)
	// 82457228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82457230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82457234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82457238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245723C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82457240: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82457248: 409A000C  bne cr6, 0x82457254
	if !ctx.cr[6].eq {
	pc = 0x82457254; continue 'dispatch;
	}
	// 8245724C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82457250: 48000010  b 0x82457260
	pc = 0x82457260; continue 'dispatch;
	// 82457254: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457258: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8245725C: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 82457260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82457264: 419A0030  beq cr6, 0x82457294
	if ctx.cr[6].eq {
	pc = 0x82457294; continue 'dispatch;
	}
	// 82457268: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245726C: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82457270: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82457274: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82457278: 4098001C  bge cr6, 0x82457294
	if !ctx.cr[6].lt {
	pc = 0x82457294; continue 'dispatch;
	}
	// 8245727C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82457284: 4BFFF9C5  bl 0x82456c48
	ctx.lr = 0x82457288;
	sub_82456C48(ctx, base);
	// 82457288: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 8245728C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82457290: 48000018  b 0x824572a8
	pc = 0x824572A8; continue 'dispatch;
	// 82457294: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82457298: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245729C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824572A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824572A4: 4BFFFEFD  bl 0x824571a0
	ctx.lr = 0x824572A8;
	sub_824571A0(ctx, base);
	// 824572A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824572AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824572B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824572B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824572B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824572BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824572C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824572C0 size=8
    let mut pc: u32 = 0x824572C0;
    'dispatch: loop {
        match pc {
            0x824572C0 => {
    //   block [0x824572C0..0x824572C8)
	// 824572C0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 824572C4: 4BFFFF64  b 0x82457228
	sub_82457228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824572C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824572C8 size=80
    let mut pc: u32 = 0x824572C8;
    'dispatch: loop {
        match pc {
            0x824572C8 => {
    //   block [0x824572C8..0x82457318)
	// 824572C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824572CC: 48D50EA1  bl 0x831a816c
	ctx.lr = 0x824572D0;
	sub_831A8130(ctx, base);
	// 824572D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824572D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824572D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824572DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824572E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824572E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824572E8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824572EC: 4899A90D  bl 0x82df1bf8
	ctx.lr = 0x824572F0;
	sub_82DF1BF8(ctx, base);
	// 824572F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 824572F4: E89F0008  ld r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 824572F8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824572FC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82457300: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82457304: 48000A9D  bl 0x82457da0
	ctx.lr = 0x82457308;
	sub_82457DA0(ctx, base);
	// 82457308: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245730C: 4899A985  bl 0x82df1c90
	ctx.lr = 0x82457310;
	sub_82DF1C90(ctx, base);
	// 82457310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82457314: 48D50EA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457318 size=120
    let mut pc: u32 = 0x82457318;
    'dispatch: loop {
        match pc {
            0x82457318 => {
    //   block [0x82457318..0x82457390)
	// 82457318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82457320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82457324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82457328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245732C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82457330: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82457334: 48000030  b 0x82457364
	pc = 0x82457364; continue 'dispatch;
	// 82457338: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245733C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82457344: 4899A8B5  bl 0x82df1bf8
	ctx.lr = 0x82457348;
	sub_82DF1BF8(ctx, base);
	// 82457348: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245734C: 88BE0000  lbz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457350: E89F0008  ld r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82457354: 48000E65  bl 0x824581b8
	ctx.lr = 0x82457358;
	sub_824581B8(ctx, base);
	// 82457358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245735C: 4899A935  bl 0x82df1c90
	ctx.lr = 0x82457360;
	sub_82DF1C90(ctx, base);
	// 82457360: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82457364: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82457368: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245736C: 409AFFCC  bne cr6, 0x82457338
	if !ctx.cr[6].eq {
	pc = 0x82457338; continue 'dispatch;
	}
	// 82457370: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82457374: 4803D6DD  bl 0x82494a50
	ctx.lr = 0x82457378;
	sub_82494A50(ctx, base);
	// 82457378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245737C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82457380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82457384: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82457388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245738C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457390 size=52
    let mut pc: u32 = 0x82457390;
    'dispatch: loop {
        match pc {
            0x82457390 => {
    //   block [0x82457390..0x824573C4)
	// 82457390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82457394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82457398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245739C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824573A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824573A4: 4BFFFF75  bl 0x82457318
	ctx.lr = 0x824573A8;
	sub_82457318(ctx, base);
	// 824573A8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 824573AC: 48012C55  bl 0x8246a000
	ctx.lr = 0x824573B0;
	sub_8246A000(ctx, base);
	// 824573B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824573B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824573B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824573BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824573C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824573C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824573C8 size=28
    let mut pc: u32 = 0x824573C8;
    'dispatch: loop {
        match pc {
            0x824573C8 => {
    //   block [0x824573C8..0x824573E4)
	// 824573C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824573CC: F8A30008  std r5, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 824573D0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824573D4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824573D8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824573DC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824573E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824573E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824573E8 size=184
    let mut pc: u32 = 0x824573E8;
    'dispatch: loop {
        match pc {
            0x824573E8 => {
    //   block [0x824573E8..0x824574A0)
	// 824573E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824573EC: 48D50D7D  bl 0x831a8168
	ctx.lr = 0x824573F0;
	sub_831A8130(ctx, base);
	// 824573F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824573F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824573F8: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824573FC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82457400: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82457404: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82457408: 4BFC7961  bl 0x8241ed68
	ctx.lr = 0x8245740C;
	sub_8241ED68(ctx, base);
	// 8245740C: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82457410: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82457414: 409A001C  bne cr6, 0x82457430
	if !ctx.cr[6].eq {
	pc = 0x82457430; continue 'dispatch;
	}
	// 82457418: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8245741C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82457420: 419A0008  beq cr6, 0x82457428
	if ctx.cr[6].eq {
	pc = 0x82457428; continue 'dispatch;
	}
	// 82457424: 4BE6946D  bl 0x822c0890
	ctx.lr = 0x82457428;
	sub_822C0890(ctx, base);
	// 82457428: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245742C: 4800006C  b 0x82457498
	pc = 0x82457498; continue 'dispatch;
	// 82457430: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82457434: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82457438: 48A434D1  bl 0x82e9a908
	ctx.lr = 0x8245743C;
	sub_82E9A908(ctx, base);
	// 8245743C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82457440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82457444: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457448: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245744C: 48BB0CDD  bl 0x83008128
	ctx.lr = 0x82457450;
	sub_83008128(ctx, base);
	// 82457450: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82457454: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457458: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245745C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457460: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82457464: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82457468: 4E800421  bctrl
	ctx.lr = 0x8245746C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245746C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82457470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82457474: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82457478: 419A000C  beq cr6, 0x82457484
	if ctx.cr[6].eq {
	pc = 0x82457484; continue 'dispatch;
	}
	// 8245747C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82457480: 4BE69411  bl 0x822c0890
	ctx.lr = 0x82457484;
	sub_822C0890(ctx, base);
	// 82457484: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82457488: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245748C: 419A0008  beq cr6, 0x82457494
	if ctx.cr[6].eq {
	pc = 0x82457494; continue 'dispatch;
	}
	// 82457490: 4BE69401  bl 0x822c0890
	ctx.lr = 0x82457494;
	sub_822C0890(ctx, base);
	// 82457494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457498: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8245749C: 48D50D1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824574A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824574A0 size=1060
    let mut pc: u32 = 0x824574A0;
    'dispatch: loop {
        match pc {
            0x824574A0 => {
    //   block [0x824574A0..0x824578C4)
	// 824574A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824574A4: 48D50CAD  bl 0x831a8150
	ctx.lr = 0x824574A8;
	sub_831A8130(ctx, base);
	// 824574A8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824574AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824574B0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824574B4: 3FE04000  lis r31, 0x4000
	ctx.r[31].s64 = 1073741824;
	// 824574B8: 4899BCF9  bl 0x82df31b0
	ctx.lr = 0x824574BC;
	sub_82DF31B0(ctx, base);
	// 824574BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824574C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824574C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824574C8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 824574CC: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 824574D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824574D4: 48775785  bl 0x82bccc58
	ctx.lr = 0x824574D8;
	sub_82BCCC58(ctx, base);
	// 824574D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824574DC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 824574E0: 409A000C  bne cr6, 0x824574ec
	if !ctx.cr[6].eq {
	pc = 0x824574EC; continue 'dispatch;
	}
	// 824574E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824574E8: 480003D4  b 0x824578bc
	pc = 0x824578BC; continue 'dispatch;
	// 824574EC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824574F0: 4BFFCAC9  bl 0x82453fb8
	ctx.lr = 0x824574F4;
	sub_82453FB8(ctx, base);
	// 824574F4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824574F8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824574FC: 48274DBD  bl 0x826cc2b8
	ctx.lr = 0x82457500;
	sub_826CC2B8(ctx, base);
	// 82457500: 98610050  stb r3, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u8 ) };
	// 82457504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457508: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8245750C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82457510: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82457514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457518: 48775939  bl 0x82bcce50
	ctx.lr = 0x8245751C;
	sub_82BCCE50(ctx, base);
	// 8245751C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82457520: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82457524: 419A038C  beq cr6, 0x824578b0
	if ctx.cr[6].eq {
	pc = 0x824578B0; continue 'dispatch;
	}
	// 82457528: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8245752C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82457530: 4BFFCAB1  bl 0x82453fe0
	ctx.lr = 0x82457534;
	sub_82453FE0(ctx, base);
	// 82457534: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82457538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8245753C: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457540: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82457544: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82457548: C01B0000  lfs f0, 0(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245754C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457550: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82457554: 487758FD  bl 0x82bcce50
	ctx.lr = 0x82457558;
	sub_82BCCE50(ctx, base);
	// 82457558: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245755C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82457560: 48274D59  bl 0x826cc2b8
	ctx.lr = 0x82457564;
	sub_826CC2B8(ctx, base);
	// 82457564: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82457568: 418201AC  beq 0x82457714
	if ctx.cr[0].eq {
	pc = 0x82457714; continue 'dispatch;
	}
	// 8245756C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82457570: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82457574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457578: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8245757C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82457580: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82457584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457588: 7D4BE02E  lwzx r10, r11, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8245758C: 7FCBE214  add r30, r11, r28
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82457590: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82457594: 487758BD  bl 0x82bcce50
	ctx.lr = 0x82457598;
	sub_82BCCE50(ctx, base);
	// 82457598: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824575A0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824575A4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824575A8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824575AC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824575B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824575B4: 4877589D  bl 0x82bcce50
	ctx.lr = 0x824575B8;
	sub_82BCCE50(ctx, base);
	// 824575B8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824575BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824575C0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824575C4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824575C8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824575CC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824575D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824575D4: 4877587D  bl 0x82bcce50
	ctx.lr = 0x824575D8;
	sub_82BCCE50(ctx, base);
	// 824575D8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 824575DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824575E0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824575E4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824575E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824575EC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824575F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824575F4: 4877585D  bl 0x82bcce50
	ctx.lr = 0x824575F8;
	sub_82BCCE50(ctx, base);
	// 824575F8: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824575FC: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82457600: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457604: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457608: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8245760C: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82457610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457614: 4877583D  bl 0x82bcce50
	ctx.lr = 0x82457618;
	sub_82BCCE50(ctx, base);
	// 82457618: C01E0014  lfs f0, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245761C: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82457620: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457624: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457628: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8245762C: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82457630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457634: 4877581D  bl 0x82bcce50
	ctx.lr = 0x82457638;
	sub_82BCCE50(ctx, base);
	// 82457638: A17E0018  lhz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8245763C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457640: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457644: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82457648: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8245764C: B1610054  sth r11, 0x54(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u16 ) };
	// 82457650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457654: 487757FD  bl 0x82bcce50
	ctx.lr = 0x82457658;
	sub_82BCCE50(ctx, base);
	// 82457658: A17E001A  lhz r11, 0x1a(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(26 as u32) ) } as u64;
	// 8245765C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457660: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457664: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82457668: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8245766C: B1610054  sth r11, 0x54(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u16 ) };
	// 82457670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457674: 487757DD  bl 0x82bcce50
	ctx.lr = 0x82457678;
	sub_82BCCE50(ctx, base);
	// 82457678: C01E001C  lfs f0, 0x1c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245767C: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82457680: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457684: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457688: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8245768C: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82457690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457694: 487757BD  bl 0x82bcce50
	ctx.lr = 0x82457698;
	sub_82BCCE50(ctx, base);
	// 82457698: C01E0020  lfs f0, 0x20(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245769C: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824576A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824576A4: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824576A8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824576AC: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 824576B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824576B4: 4877579D  bl 0x82bcce50
	ctx.lr = 0x824576B8;
	sub_82BCCE50(ctx, base);
	// 824576B8: A17E0024  lhz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 824576BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824576C0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824576C4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824576C8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824576CC: B1610054  sth r11, 0x54(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u16 ) };
	// 824576D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824576D4: 4877577D  bl 0x82bcce50
	ctx.lr = 0x824576D8;
	sub_82BCCE50(ctx, base);
	// 824576D8: A17E0026  lhz r11, 0x26(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(38 as u32) ) } as u64;
	// 824576DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824576E0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824576E4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824576E8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824576EC: B1610054  sth r11, 0x54(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u16 ) };
	// 824576F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824576F4: 4877575D  bl 0x82bcce50
	ctx.lr = 0x824576F8;
	sub_82BCCE50(ctx, base);
	// 824576F8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824576FC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82457700: 3B9C0028  addi r28, r28, 0x28
	ctx.r[28].s64 = ctx.r[28].s64 + 40;
	// 82457704: 48274BB5  bl 0x826cc2b8
	ctx.lr = 0x82457708;
	sub_826CC2B8(ctx, base);
	// 82457708: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8245770C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82457710: 4198FE60  blt cr6, 0x82457570
	if ctx.cr[6].lt {
	pc = 0x82457570; continue 'dispatch;
	}
	// 82457714: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82457718: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8245771C: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457720: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82457724: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 82457728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245772C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82457730: 48775721  bl 0x82bcce50
	ctx.lr = 0x82457734;
	sub_82BCCE50(ctx, base);
	// 82457734: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82457738: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245773C: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82457740: 4800015C  b 0x8245789c
	pc = 0x8245789C; continue 'dispatch;
	// 82457744: E97E0010  ld r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	// 82457748: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8245774C: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457750: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82457754: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82457758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245775C: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 82457760: 3B9E0018  addi r28, r30, 0x18
	ctx.r[28].s64 = ctx.r[30].s64 + 24;
	// 82457764: 487756ED  bl 0x82bcce50
	ctx.lr = 0x82457768;
	sub_82BCCE50(ctx, base);
	// 82457768: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8245776C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457770: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82457774: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82457778: 38810056  addi r4, r1, 0x56
	ctx.r[4].s64 = ctx.r[1].s64 + 86;
	// 8245777C: B1610056  sth r11, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u16 ) };
	// 82457780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457784: 487756CD  bl 0x82bcce50
	ctx.lr = 0x82457788;
	sub_82BCCE50(ctx, base);
	// 82457788: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8245778C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457790: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82457794: 480000F0  b 0x82457884
	pc = 0x82457884; continue 'dispatch;
	// 82457798: 894B000C  lbz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245779C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824577A0: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824577A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824577A8: 38810051  addi r4, r1, 0x51
	ctx.r[4].s64 = ctx.r[1].s64 + 81;
	// 824577AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824577B0: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 824577B4: 3BAB0010  addi r29, r11, 0x10
	ctx.r[29].s64 = ctx.r[11].s64 + 16;
	// 824577B8: 48775699  bl 0x82bcce50
	ctx.lr = 0x824577BC;
	sub_82BCCE50(ctx, base);
	// 824577BC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824577C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824577C4: 419A0010  beq cr6, 0x824577d4
	if ctx.cr[6].eq {
	pc = 0x824577D4; continue 'dispatch;
	}
	// 824577C8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824577CC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824577D0: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 824577D4: B1610058  sth r11, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u16 ) };
	// 824577D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824577DC: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 824577E0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824577E4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824577E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824577EC: 48775665  bl 0x82bcce50
	ctx.lr = 0x824577F0;
	sub_82BCCE50(ctx, base);
	// 824577F0: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824577F4: 48000074  b 0x82457868
	pc = 0x82457868; continue 'dispatch;
	// 824577F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824577FC: 48274ABD  bl 0x826cc2b8
	ctx.lr = 0x82457800;
	sub_826CC2B8(ctx, base);
	// 82457800: 98610052  stb r3, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[3].u8 ) };
	// 82457804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457808: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8245780C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82457810: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 82457814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457818: 48775639  bl 0x82bcce50
	ctx.lr = 0x8245781C;
	sub_82BCCE50(ctx, base);
	// 8245781C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82457820: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82457824: 484EAEAD  bl 0x829426d0
	ctx.lr = 0x82457828;
	sub_829426D0(ctx, base);
	// 82457828: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245782C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82457830: 3AE1005C  addi r23, r1, 0x5c
	ctx.r[23].s64 = ctx.r[1].s64 + 92;
	// 82457834: 82CB0000  lwz r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457838: 484E76E1  bl 0x8293ef18
	ctx.lr = 0x8245783C;
	sub_8293EF18(ctx, base);
	// 8245783C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82457840: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82457844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457848: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 8245784C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82457850: 48775601  bl 0x82bcce50
	ctx.lr = 0x82457854;
	sub_82BCCE50(ctx, base);
	// 82457854: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82457858: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245785C: 419A0008  beq cr6, 0x82457864
	if ctx.cr[6].eq {
	pc = 0x82457864; continue 'dispatch;
	}
	// 82457860: 4BE69031  bl 0x822c0890
	ctx.lr = 0x82457864;
	sub_822C0890(ctx, base);
	// 82457864: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82457868: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245786C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82457870: 409AFF88  bne cr6, 0x824577f8
	if !ctx.cr[6].eq {
	pc = 0x824577F8; continue 'dispatch;
	}
	// 82457874: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82457878: 483717B9  bl 0x827c9030
	ctx.lr = 0x8245787C;
	sub_827C9030(ctx, base);
	// 8245787C: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457880: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82457884: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82457888: 409AFF10  bne cr6, 0x82457798
	if !ctx.cr[6].eq {
	pc = 0x82457798; continue 'dispatch;
	}
	// 8245788C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82457890: 48091879  bl 0x824e9108
	ctx.lr = 0x82457894;
	sub_824E9108(ctx, base);
	// 82457894: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82457898: 83C1006C  lwz r30, 0x6c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8245789C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824578A0: 409AFEA4  bne cr6, 0x82457744
	if !ctx.cr[6].eq {
	pc = 0x82457744; continue 'dispatch;
	}
	// 824578A4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 824578A8: 7F19C040  cmplw cr6, r25, r24
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[24].u32, &mut ctx.xer);
	// 824578AC: 4198FC7C  blt cr6, 0x82457528
	if ctx.cr[6].lt {
	pc = 0x82457528; continue 'dispatch;
	}
	// 824578B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824578B4: 4877516D  bl 0x82bcca20
	ctx.lr = 0x824578B8;
	sub_82BCCA20(ctx, base);
	// 824578B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824578BC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 824578C0: 48D508E0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824578C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824578C8 size=1188
    let mut pc: u32 = 0x824578C8;
    'dispatch: loop {
        match pc {
            0x824578C8 => {
    //   block [0x824578C8..0x82457D6C)
	// 824578C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824578CC: 48D50885  bl 0x831a8150
	ctx.lr = 0x824578D0;
	sub_831A8130(ctx, base);
	// 824578D0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824578D4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 824578D8: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 824578DC: 93010098  stw r24, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[24].u32 ) };
	// 824578E0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824578E4: 9301009C  stw r24, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[24].u32 ) };
	// 824578E8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 824578EC: 930100A0  stw r24, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[24].u32 ) };
	// 824578F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824578F4: 4BFFCD05  bl 0x824545f8
	ctx.lr = 0x824578F8;
	sub_824545F8(ctx, base);
	// 824578F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824578FC: 4BFFF865  bl 0x82457160
	ctx.lr = 0x82457900;
	sub_82457160(ctx, base);
	// 82457900: 895B0000  lbz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457904: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82457908: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8245790C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82457910: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82457914: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82457918: 4BFFC699  bl 0x82453fb0
	ctx.lr = 0x8245791C;
	sub_82453FB0(ctx, base);
	// 8245791C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82457920: 48274999  bl 0x826cc2b8
	ctx.lr = 0x82457924;
	sub_826CC2B8(ctx, base);
	// 82457924: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82457928: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8245792C: 4BFFF1E5  bl 0x82456b10
	ctx.lr = 0x82457930;
	sub_82456B10(ctx, base);
	// 82457930: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82457934: 4099041C  ble cr6, 0x82457d50
	if !ctx.cr[6].gt {
	pc = 0x82457D50; continue 'dispatch;
	}
	// 82457938: 7D7FDA14  add r11, r31, r27
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 8245793C: 7D5FD8AE  lbzx r10, r31, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82457940: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82457944: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82457948: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 8245794C: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457950: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457954: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457958: 99490000  stb r10, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8245795C: 99090001  stb r8, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 82457960: 98E90002  stb r7, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[7].u8 ) };
	// 82457964: 99690003  stb r11, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82457968: 48274951  bl 0x826cc2b8
	ctx.lr = 0x8245796C;
	sub_826CC2B8(ctx, base);
	// 8245796C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82457970: 41820258  beq 0x82457bc8
	if ctx.cr[0].eq {
	pc = 0x82457BC8; continue 'dispatch;
	}
	// 82457974: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82457978: 7FFDDA14  add r31, r29, r27
	ctx.r[31].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 8245797C: 81410098  lwz r10, 0x98(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457980: 1D7E0028  mulli r11, r30, 0x28
	ctx.r[11].s64 = ctx.r[30].s64 * 40;
	// 82457984: 893F0000  lbz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457988: 7D2B51AE  stbx r9, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 8245798C: 891F0001  lbz r8, 1(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457990: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82457994: 395D0004  addi r10, r29, 4
	ctx.r[10].s64 = ctx.r[29].s64 + 4;
	// 82457998: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8245799C: 99090001  stb r8, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 824579A0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824579A4: 88FF0002  lbz r7, 2(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 824579A8: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 824579AC: 98E90002  stb r7, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[7].u8 ) };
	// 824579B0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 824579B4: 88FF0003  lbz r7, 3(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 824579B8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824579BC: 98E90003  stb r7, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 824579C0: 39280004  addi r9, r8, 4
	ctx.r[9].s64 = ctx.r[8].s64 + 4;
	// 824579C4: 88DF0004  lbz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824579C8: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 824579CC: 81010098  lwz r8, 0x98(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 824579D0: 7D2B4214  add r9, r11, r8
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824579D4: 98C90004  stb r6, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u8 ) };
	// 824579D8: 39090004  addi r8, r9, 4
	ctx.r[8].s64 = ctx.r[9].s64 + 4;
	// 824579DC: 891F0005  lbz r8, 5(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 824579E0: 99090005  stb r8, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[8].u8 ) };
	// 824579E4: 891F0006  lbz r8, 6(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824579E8: 99090006  stb r8, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[8].u8 ) };
	// 824579EC: 891F0007  lbz r8, 7(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 824579F0: 99090007  stb r8, 7(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(7 as u32), ctx.r[8].u8 ) };
	// 824579F4: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824579F8: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 824579FC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457A00: 99090008  stb r8, 8(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82457A04: 39090008  addi r8, r9, 8
	ctx.r[8].s64 = ctx.r[9].s64 + 8;
	// 82457A08: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457A0C: 99090009  stb r8, 9(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(9 as u32), ctx.r[8].u8 ) };
	// 82457A10: 890A0002  lbz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457A14: 9909000A  stb r8, 0xa(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(10 as u32), ctx.r[8].u8 ) };
	// 82457A18: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457A1C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82457A20: 9909000B  stb r8, 0xb(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(11 as u32), ctx.r[8].u8 ) };
	// 82457A24: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457A28: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457A2C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457A30: 9909000C  stb r8, 0xc(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82457A34: 3909000C  addi r8, r9, 0xc
	ctx.r[8].s64 = ctx.r[9].s64 + 12;
	// 82457A38: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457A3C: 9909000D  stb r8, 0xd(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(13 as u32), ctx.r[8].u8 ) };
	// 82457A40: 890A0002  lbz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457A44: 9909000E  stb r8, 0xe(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(14 as u32), ctx.r[8].u8 ) };
	// 82457A48: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457A4C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82457A50: 9909000F  stb r8, 0xf(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(15 as u32), ctx.r[8].u8 ) };
	// 82457A54: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457A58: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457A5C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457A60: 99090010  stb r8, 0x10(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[8].u8 ) };
	// 82457A64: 39090010  addi r8, r9, 0x10
	ctx.r[8].s64 = ctx.r[9].s64 + 16;
	// 82457A68: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457A6C: 99090011  stb r8, 0x11(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(17 as u32), ctx.r[8].u8 ) };
	// 82457A70: 890A0002  lbz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457A74: 99090012  stb r8, 0x12(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(18 as u32), ctx.r[8].u8 ) };
	// 82457A78: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457A7C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82457A80: 99090013  stb r8, 0x13(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(19 as u32), ctx.r[8].u8 ) };
	// 82457A84: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457A88: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457A8C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457A90: 99090014  stb r8, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82457A94: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 82457A98: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457A9C: 99090015  stb r8, 0x15(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(21 as u32), ctx.r[8].u8 ) };
	// 82457AA0: 890A0002  lbz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457AA4: 99090016  stb r8, 0x16(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(22 as u32), ctx.r[8].u8 ) };
	// 82457AA8: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457AAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82457AB0: 99090017  stb r8, 0x17(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(23 as u32), ctx.r[8].u8 ) };
	// 82457AB4: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457AB8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457ABC: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457AC0: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82457AC4: 98C90018  stb r6, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[6].u8 ) };
	// 82457AC8: 39070002  addi r8, r7, 2
	ctx.r[8].s64 = ctx.r[7].s64 + 2;
	// 82457ACC: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457AD0: 98E90019  stb r7, 0x19(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(25 as u32), ctx.r[7].u8 ) };
	// 82457AD4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82457AD8: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457ADC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457AE0: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457AE4: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82457AE8: 98E9001A  stb r7, 0x1a(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(26 as u32), ctx.r[7].u8 ) };
	// 82457AEC: 38DE0001  addi r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 1;
	// 82457AF0: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457AF4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82457AF8: 98E9001B  stb r7, 0x1b(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(27 as u32), ctx.r[7].u8 ) };
	// 82457AFC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82457B00: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82457B04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82457B08: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82457B0C: 54DE063E  clrlwi r30, r6, 0x18
	ctx.r[30].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82457B10: 3BA80002  addi r29, r8, 2
	ctx.r[29].s64 = ctx.r[8].s64 + 2;
	// 82457B14: 3909001A  addi r8, r9, 0x1a
	ctx.r[8].s64 = ctx.r[9].s64 + 26;
	// 82457B18: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457B1C: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457B20: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457B24: 9909001C  stb r8, 0x1c(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), ctx.r[8].u8 ) };
	// 82457B28: 3909001C  addi r8, r9, 0x1c
	ctx.r[8].s64 = ctx.r[9].s64 + 28;
	// 82457B2C: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457B30: 9909001D  stb r8, 0x1d(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(29 as u32), ctx.r[8].u8 ) };
	// 82457B34: 890A0002  lbz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457B38: 9909001E  stb r8, 0x1e(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(30 as u32), ctx.r[8].u8 ) };
	// 82457B3C: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457B40: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82457B44: 9909001F  stb r8, 0x1f(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(31 as u32), ctx.r[8].u8 ) };
	// 82457B48: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457B4C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457B50: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457B54: 39090020  addi r8, r9, 0x20
	ctx.r[8].s64 = ctx.r[9].s64 + 32;
	// 82457B58: 98E90020  stb r7, 0x20(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[7].u8 ) };
	// 82457B5C: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457B60: 99090021  stb r8, 0x21(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(33 as u32), ctx.r[8].u8 ) };
	// 82457B64: 890A0002  lbz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457B68: 99090022  stb r8, 0x22(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(34 as u32), ctx.r[8].u8 ) };
	// 82457B6C: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457B70: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82457B74: 99090023  stb r8, 0x23(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(35 as u32), ctx.r[8].u8 ) };
	// 82457B78: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457B7C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457B80: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457B84: 39090024  addi r8, r9, 0x24
	ctx.r[8].s64 = ctx.r[9].s64 + 36;
	// 82457B88: 98E90024  stb r7, 0x24(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), ctx.r[7].u8 ) };
	// 82457B8C: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457B90: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82457B94: 99090025  stb r8, 0x25(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(37 as u32), ctx.r[8].u8 ) };
	// 82457B98: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82457B9C: 3BEA0002  addi r31, r10, 2
	ctx.r[31].s64 = ctx.r[10].s64 + 2;
	// 82457BA0: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457BA4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82457BA8: 990B0026  stb r8, 0x26(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(38 as u32), ctx.r[8].u8 ) };
	// 82457BAC: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457BB0: 392B0026  addi r9, r11, 0x26
	ctx.r[9].s64 = ctx.r[11].s64 + 38;
	// 82457BB4: 994B0027  stb r10, 0x27(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(39 as u32), ctx.r[10].u8 ) };
	// 82457BB8: 48274701  bl 0x826cc2b8
	ctx.lr = 0x82457BBC;
	sub_826CC2B8(ctx, base);
	// 82457BBC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82457BC0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82457BC4: 4198FDB8  blt cr6, 0x8245797c
	if ctx.cr[6].lt {
	pc = 0x8245797C; continue 'dispatch;
	}
	// 82457BC8: 7D7DDA14  add r11, r29, r27
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 82457BCC: 7D5DD8AE  lbzx r10, r29, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82457BD0: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 82457BD4: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 82457BD8: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457BDC: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82457BE0: 99490000  stb r10, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82457BE4: 896B0003  lbz r11, 3(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 82457BE8: 99090001  stb r8, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 82457BEC: 98E90002  stb r7, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[7].u8 ) };
	// 82457BF0: 99690003  stb r11, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82457BF4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82457BF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82457BFC: 419A012C  beq cr6, 0x82457d28
	if ctx.cr[6].eq {
	pc = 0x82457D28; continue 'dispatch;
	}
	// 82457C00: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82457C04: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82457C08: 7C9FDA14  add r4, r31, r27
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 82457C0C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82457C10: 48D50901  bl 0x831a8510
	ctx.lr = 0x82457C14;
	sub_831A8510(ctx, base);
	// 82457C14: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82457C18: 39210052  addi r9, r1, 0x52
	ctx.r[9].s64 = ctx.r[1].s64 + 82;
	// 82457C1C: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82457C20: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 82457C24: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82457C28: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457C2C: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82457C30: 99490001  stb r10, 1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82457C34: A1610052  lhz r11, 0x52(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 82457C38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82457C3C: 418200E4  beq 0x82457d20
	if ctx.cr[0].eq {
	pc = 0x82457D20; continue 'dispatch;
	}
	// 82457C40: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82457C44: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82457C48: 7D3FD8AE  lbzx r9, r31, r27
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82457C4C: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82457C50: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82457C54: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 82457C58: 38E10051  addi r7, r1, 0x51
	ctx.r[7].s64 = ctx.r[1].s64 + 81;
	// 82457C5C: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82457C60: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82457C64: 99270000  stb r9, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82457C68: 99680000  stb r11, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82457C6C: 99480001  stb r10, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82457C70: A1610054  lhz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82457C74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82457C78: 418200A0  beq 0x82457d18
	if ctx.cr[0].eq {
	pc = 0x82457D18; continue 'dispatch;
	}
	// 82457C7C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82457C80: 7D7FD8AE  lbzx r11, r31, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82457C84: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82457C88: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82457C8C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82457C90: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82457C94: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82457C98: 4BFFCA19  bl 0x824546b0
	ctx.lr = 0x82457C9C;
	sub_824546B0(ctx, base);
	// 82457C9C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82457CA0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82457CA4: 484EAA2D  bl 0x829426d0
	ctx.lr = 0x82457CA8;
	sub_829426D0(ctx, base);
	// 82457CA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82457CAC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82457CB0: 82CB0000  lwz r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457CB4: 484E7265  bl 0x8293ef18
	ctx.lr = 0x82457CB8;
	sub_8293EF18(ctx, base);
	// 82457CB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82457CBC: 7C9FDA14  add r4, r31, r27
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 82457CC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82457CC4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82457CC8: 48D50849  bl 0x831a8510
	ctx.lr = 0x82457CCC;
	sub_831A8510(ctx, base);
	// 82457CCC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82457CD0: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82457CD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82457CD8: 419A0008  beq cr6, 0x82457ce0
	if ctx.cr[6].eq {
	pc = 0x82457CE0; continue 'dispatch;
	}
	// 82457CDC: 4BE68BB5  bl 0x822c0890
	ctx.lr = 0x82457CE0;
	sub_822C0890(ctx, base);
	// 82457CE0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82457CE4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82457CE8: 4BFFC681  bl 0x82454368
	ctx.lr = 0x82457CEC;
	sub_82454368(ctx, base);
	// 82457CEC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82457CF0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82457CF4: 88A10051  lbz r5, 0x51(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82457CF8: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82457CFC: 4BFFEA7D  bl 0x82456778
	ctx.lr = 0x82457D00;
	sub_82456778(ctx, base);
	// 82457D00: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82457D04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82457D08: 419A0008  beq cr6, 0x82457d10
	if ctx.cr[6].eq {
	pc = 0x82457D10; continue 'dispatch;
	}
	// 82457D0C: 4BE68B85  bl 0x822c0890
	ctx.lr = 0x82457D10;
	sub_822C0890(ctx, base);
	// 82457D10: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82457D14: 4082FF6C  bne 0x82457c80
	if !ctx.cr[0].eq {
	pc = 0x82457C80; continue 'dispatch;
	}
	// 82457D18: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82457D1C: 4082FF28  bne 0x82457c44
	if !ctx.cr[0].eq {
	pc = 0x82457C44; continue 'dispatch;
	}
	// 82457D20: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82457D24: 4082FEE0  bne 0x82457c04
	if !ctx.cr[0].eq {
	pc = 0x82457C04; continue 'dispatch;
	}
	// 82457D28: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82457D2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82457D30: 4BFFF591  bl 0x824572c0
	ctx.lr = 0x82457D34;
	sub_824572C0(ctx, base);
	// 82457D34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82457D38: 48274581  bl 0x826cc2b8
	ctx.lr = 0x82457D3C;
	sub_826CC2B8(ctx, base);
	// 82457D3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82457D40: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82457D44: 4BFFEDCD  bl 0x82456b10
	ctx.lr = 0x82457D48;
	sub_82456B10(ctx, base);
	// 82457D48: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82457D4C: 4198FBEC  blt cr6, 0x82457938
	if ctx.cr[6].lt {
	pc = 0x82457938; continue 'dispatch;
	}
	// 82457D50: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82457D54: 4BFFEE45  bl 0x82456b98
	ctx.lr = 0x82457D58;
	sub_82456B98(ctx, base);
	// 82457D58: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 82457D5C: 480122A5  bl 0x8246a000
	ctx.lr = 0x82457D60;
	sub_8246A000(ctx, base);
	// 82457D60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82457D64: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82457D68: 48D50438  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457D70 size=16
    let mut pc: u32 = 0x82457D70;
    'dispatch: loop {
        match pc {
            0x82457D70 => {
    //   block [0x82457D70..0x82457D80)
	// 82457D70: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82457D74: 396B7680  addi r11, r11, 0x7680
	ctx.r[11].s64 = ctx.r[11].s64 + 30336;
	// 82457D78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82457D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457D80 size=12
    let mut pc: u32 = 0x82457D80;
    'dispatch: loop {
        match pc {
            0x82457D80 => {
    //   block [0x82457D80..0x82457D8C)
	// 82457D80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82457D84: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82457D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457D90 size=12
    let mut pc: u32 = 0x82457D90;
    'dispatch: loop {
        match pc {
            0x82457D90 => {
    //   block [0x82457D90..0x82457D9C)
	// 82457D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82457D94: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82457D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DA0 size=12
    let mut pc: u32 = 0x82457DA0;
    'dispatch: loop {
        match pc {
            0x82457DA0 => {
    //   block [0x82457DA0..0x82457DAC)
	// 82457DA0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82457DA8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DAC size=12
    let mut pc: u32 = 0x82457DAC;
    'dispatch: loop {
        match pc {
            0x82457DAC => {
    //   block [0x82457DAC..0x82457DB8)
	// 82457DAC: 8943002C  lbz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82457DB0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82457DB4: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DB8 size=16
    let mut pc: u32 = 0x82457DB8;
    'dispatch: loop {
        match pc {
            0x82457DB8 => {
    //   block [0x82457DB8..0x82457DC8)
	// 82457DB8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82457DBC: 409A000C  bne cr6, 0x82457dc8
	if !ctx.cr[6].eq {
		sub_82457DC8(ctx, base);
		return;
	}
	// 82457DC0: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82457DC4: 4BFFE7E4  b 0x824565a8
	sub_824565A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DC8 size=8
    let mut pc: u32 = 0x82457DC8;
    'dispatch: loop {
        match pc {
            0x82457DC8 => {
    //   block [0x82457DC8..0x82457DD0)
	// 82457DC8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82457DCC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DD0 size=8
    let mut pc: u32 = 0x82457DD0;
    'dispatch: loop {
        match pc {
            0x82457DD0 => {
    //   block [0x82457DD0..0x82457DD8)
	// 82457DD0: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82457DD4: 4BFFCE84  b 0x82454c58
	sub_82454C58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DD8 size=4
    let mut pc: u32 = 0x82457DD8;
    'dispatch: loop {
        match pc {
            0x82457DD8 => {
    //   block [0x82457DD8..0x82457DDC)
	// 82457DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457DE0 size=40
    let mut pc: u32 = 0x82457DE0;
    'dispatch: loop {
        match pc {
            0x82457DE0 => {
    //   block [0x82457DE0..0x82457E08)
	// 82457DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82457DE4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457DE8: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82457DEC: 0CC90000  twi 6, r9, 0
	// 82457DF0: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82457DF4: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82457DF8: 7D0A4B96  divwu r8, r10, r9
	ctx.r[8].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 82457DFC: 7D2849D6  mullw r9, r8, r9
	ctx.r[9].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82457E00: 7D495051  subf. r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82457E04: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457E08 size=28
    let mut pc: u32 = 0x82457E08;
    'dispatch: loop {
        match pc {
            0x82457E08 => {
    //   block [0x82457E08..0x82457E24)
	// 82457E08: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82457E0C: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82457E10: 80C30014  lwz r6, 0x14(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82457E14: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82457E18: 88A30010  lbz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82457E1C: E8830008  ld r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82457E20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457E24 size=12
    let mut pc: u32 = 0x82457E24;
    'dispatch: loop {
        match pc {
            0x82457E24 => {
    //   block [0x82457E24..0x82457E30)
	// 82457E24: 892B002C  lbz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82457E28: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82457E2C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457E30 size=16
    let mut pc: u32 = 0x82457E30;
    'dispatch: loop {
        match pc {
            0x82457E30 => {
    //   block [0x82457E30..0x82457E40)
	// 82457E30: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82457E34: 409A000C  bne cr6, 0x82457e40
	if !ctx.cr[6].eq {
		sub_82457E40(ctx, base);
		return;
	}
	// 82457E38: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82457E3C: 4BFFE76C  b 0x824565a8
	sub_824565A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457E40 size=8
    let mut pc: u32 = 0x82457E40;
    'dispatch: loop {
        match pc {
            0x82457E40 => {
    //   block [0x82457E40..0x82457E48)
	// 82457E40: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82457E44: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457E48 size=8
    let mut pc: u32 = 0x82457E48;
    'dispatch: loop {
        match pc {
            0x82457E48 => {
    //   block [0x82457E48..0x82457E50)
	// 82457E48: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82457E4C: 4BFFCE0C  b 0x82454c58
	sub_82454C58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457E50 size=4
    let mut pc: u32 = 0x82457E50;
    'dispatch: loop {
        match pc {
            0x82457E50 => {
    //   block [0x82457E50..0x82457E54)
	// 82457E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457E58 size=88
    let mut pc: u32 = 0x82457E58;
    'dispatch: loop {
        match pc {
            0x82457E58 => {
    //   block [0x82457E58..0x82457EB0)
	// 82457E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82457E5C: 48D50309  bl 0x831a8164
	ctx.lr = 0x82457E60;
	sub_831A8130(ctx, base);
	// 82457E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82457E64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82457E68: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82457E6C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82457E70: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82457E74: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82457E78: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82457E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82457E80: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82457E84: 4BFFF2DD  bl 0x82457160
	ctx.lr = 0x82457E88;
	sub_82457160(ctx, base);
	// 82457E88: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82457E8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82457E90: 4BFFC121  bl 0x82453fb0
	ctx.lr = 0x82457E94;
	sub_82453FB0(ctx, base);
	// 82457E94: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82457E98: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82457E9C: 4BFFEC75  bl 0x82456b10
	ctx.lr = 0x82457EA0;
	sub_82456B10(ctx, base);
	// 82457EA0: 9B9F002C  stb r28, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u8 ) };
	// 82457EA4: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82457EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82457EAC: 48D50308  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457EB0 size=32
    let mut pc: u32 = 0x82457EB0;
    'dispatch: loop {
        match pc {
            0x82457EB0 => {
    //   block [0x82457EB0..0x82457ED0)
	// 82457EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82457EB4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82457EB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82457EBC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82457EC0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82457EC4: 9923002C  stb r9, 0x2c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[9].u8 ) };
	// 82457EC8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82457ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457ED0 size=20
    let mut pc: u32 = 0x82457ED0;
    'dispatch: loop {
        match pc {
            0x82457ED0 => {
    //   block [0x82457ED0..0x82457EE4)
	// 82457ED0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82457ED4: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82457ED8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82457EDC: 409A0008  bne cr6, 0x82457ee4
	if !ctx.cr[6].eq {
		sub_82457EE4(ctx, base);
		return;
	}
	// 82457EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457EE4 size=8
    let mut pc: u32 = 0x82457EE4;
    'dispatch: loop {
        match pc {
            0x82457EE4 => {
    //   block [0x82457EE4..0x82457EEC)
	// 82457EE4: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82457EE8: 488785F8  b 0x82cd04e0
	sub_82CD04E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457EF0 size=20
    let mut pc: u32 = 0x82457EF0;
    'dispatch: loop {
        match pc {
            0x82457EF0 => {
    //   block [0x82457EF0..0x82457F04)
	// 82457EF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82457EF4: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82457EF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82457EFC: 409A0008  bne cr6, 0x82457f04
	if !ctx.cr[6].eq {
		sub_82457F04(ctx, base);
		return;
	}
	// 82457F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457F04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82457F04 size=8
    let mut pc: u32 = 0x82457F04;
    'dispatch: loop {
        match pc {
            0x82457F04 => {
    //   block [0x82457F04..0x82457F0C)
	// 82457F04: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82457F08: 4BFFF4E0  b 0x824573e8
	sub_824573E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457F10 size=172
    let mut pc: u32 = 0x82457F10;
    'dispatch: loop {
        match pc {
            0x82457F10 => {
    //   block [0x82457F10..0x82457FBC)
	// 82457F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82457F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82457F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82457F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82457F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82457F24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82457F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82457F2C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82457F30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82457F34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82457F38: 4BE68A01  bl 0x822c0938
	ctx.lr = 0x82457F3C;
	sub_822C0938(ctx, base);
	// 82457F3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82457F40: 41820028  beq 0x82457f68
	if ctx.cr[0].eq {
	pc = 0x82457F68; continue 'dispatch;
	}
	// 82457F44: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82457F48: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82457F4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82457F50: 392B76AC  addi r9, r11, 0x76ac
	ctx.r[9].s64 = ctx.r[11].s64 + 30380;
	// 82457F54: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82457F58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82457F5C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82457F60: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82457F64: 48000008  b 0x82457f6c
	pc = 0x82457F6C; continue 'dispatch;
	// 82457F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82457F6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82457F70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82457F74: 409A002C  bne cr6, 0x82457fa0
	if !ctx.cr[6].eq {
	pc = 0x82457FA0; continue 'dispatch;
	}
	// 82457F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82457F7C: 4BE682ED  bl 0x822c0268
	ctx.lr = 0x82457F80;
	sub_822C0268(ctx, base);
	// 82457F80: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82457F84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82457F88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82457F8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82457F90: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82457F94: 816B37F8  lwz r11, 0x37f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14328 as u32) ) } as u64;
	// 82457F98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82457F9C: 4BE68065  bl 0x822c0000
	ctx.lr = 0x82457FA0;
	sub_822C0000(ctx, base);
	// 82457FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82457FA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82457FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82457FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82457FB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82457FB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82457FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82457FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82457FC0 size=188
    let mut pc: u32 = 0x82457FC0;
    'dispatch: loop {
        match pc {
            0x82457FC0 => {
    //   block [0x82457FC0..0x8245807C)
	// 82457FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82457FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82457FC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82457FCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82457FD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82457FD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82457FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82457FDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82457FE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82457FE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82457FE8: 4BE68951  bl 0x822c0938
	ctx.lr = 0x82457FEC;
	sub_822C0938(ctx, base);
	// 82457FEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82457FF0: 41820028  beq 0x82458018
	if ctx.cr[0].eq {
	pc = 0x82458018; continue 'dispatch;
	}
	// 82457FF4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82457FF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82457FFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82458000: 392B7698  addi r9, r11, 0x7698
	ctx.r[9].s64 = ctx.r[11].s64 + 30360;
	// 82458004: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82458008: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245800C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82458010: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82458014: 48000008  b 0x8245801c
	pc = 0x8245801C; continue 'dispatch;
	// 82458018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245801C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458020: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82458024: 409A003C  bne cr6, 0x82458060
	if !ctx.cr[6].eq {
	pc = 0x82458060; continue 'dispatch;
	}
	// 82458028: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8245802C: 419A0014  beq cr6, 0x82458040
	if ctx.cr[6].eq {
	pc = 0x82458040; continue 'dispatch;
	}
	// 82458030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458034: 4BFFF35D  bl 0x82457390
	ctx.lr = 0x82458038;
	sub_82457390(ctx, base);
	// 82458038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245803C: 4BE6822D  bl 0x822c0268
	ctx.lr = 0x82458040;
	sub_822C0268(ctx, base);
	// 82458040: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82458044: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82458048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245804C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82458050: 816B37F8  lwz r11, 0x37f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14328 as u32) ) } as u64;
	// 82458054: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82458058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245805C: 4BE67FA5  bl 0x822c0000
	ctx.lr = 0x82458060;
	sub_822C0000(ctx, base);
	// 82458060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82458068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245806C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458070: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82458074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458080 size=64
    let mut pc: u32 = 0x82458080;
    'dispatch: loop {
        match pc {
            0x82458080 => {
    //   block [0x82458080..0x824580C0)
	// 82458080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245808C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458090: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82458094: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82458098: 419A0014  beq cr6, 0x824580ac
	if ctx.cr[6].eq {
	pc = 0x824580AC; continue 'dispatch;
	}
	// 8245809C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824580A0: 4BFFF2F1  bl 0x82457390
	ctx.lr = 0x824580A4;
	sub_82457390(ctx, base);
	// 824580A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824580A8: 4BE681C1  bl 0x822c0268
	ctx.lr = 0x824580AC;
	sub_822C0268(ctx, base);
	// 824580AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824580B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824580B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824580B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824580BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824580C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824580C0 size=112
    let mut pc: u32 = 0x824580C0;
    'dispatch: loop {
        match pc {
            0x824580C0 => {
    //   block [0x824580C0..0x82458130)
	// 824580C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824580C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824580C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824580CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824580D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824580D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824580D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824580DC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 824580E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 824580E4: 4BFFFE2D  bl 0x82457f10
	ctx.lr = 0x824580E8;
	sub_82457F10(ctx, base);
	// 824580E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824580EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824580F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 824580F4: 4BE67F0D  bl 0x822c0000
	ctx.lr = 0x824580F8;
	sub_822C0000(ctx, base);
	// 824580F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824580FC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82458100: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458104: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82458108: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245810C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82458110: 419A0008  beq cr6, 0x82458118
	if ctx.cr[6].eq {
	pc = 0x82458118; continue 'dispatch;
	}
	// 82458114: 4BE6877D  bl 0x822c0890
	ctx.lr = 0x82458118;
	sub_822C0890(ctx, base);
	// 82458118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245811C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458124: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82458128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82458130 size=8
    let mut pc: u32 = 0x82458130;
    'dispatch: loop {
        match pc {
            0x82458130 => {
    //   block [0x82458130..0x82458138)
	// 82458130: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 82458134: 4BFFFF8C  b 0x824580c0
	sub_824580C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458138 size=128
    let mut pc: u32 = 0x82458138;
    'dispatch: loop {
        match pc {
            0x82458138 => {
    //   block [0x82458138..0x824581B8)
	// 82458138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245813C: 48D50031  bl 0x831a816c
	ctx.lr = 0x82458140;
	sub_831A8130(ctx, base);
	// 82458140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458144: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458148: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8245814C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82458150: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82458154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82458158: 388B76BC  addi r4, r11, 0x76bc
	ctx.r[4].s64 = ctx.r[11].s64 + 30396;
	// 8245815C: 38A000F3  li r5, 0xf3
	ctx.r[5].s64 = 243;
	// 82458160: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82458164: 4BE68275  bl 0x822c03d8
	ctx.lr = 0x82458168;
	sub_822C03D8(ctx, base);
	// 82458168: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8245816C: 41820018  beq 0x82458184
	if ctx.cr[0].eq {
	pc = 0x82458184; continue 'dispatch;
	}
	// 82458170: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82458174: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82458178: 4BFFF251  bl 0x824573c8
	ctx.lr = 0x8245817C;
	sub_824573C8(ctx, base);
	// 8245817C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82458180: 48000008  b 0x82458188
	pc = 0x82458188; continue 'dispatch;
	// 82458184: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82458188: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8245818C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82458190: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82458194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458198: 4BFFFE29  bl 0x82457fc0
	ctx.lr = 0x8245819C;
	sub_82457FC0(ctx, base);
	// 8245819C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824581A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824581A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824581A8: 4BE67E59  bl 0x822c0000
	ctx.lr = 0x824581AC;
	sub_822C0000(ctx, base);
	// 824581AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824581B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824581B4: 48D50008  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824581B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824581B8 size=156
    let mut pc: u32 = 0x824581B8;
    'dispatch: loop {
        match pc {
            0x824581B8 => {
    //   block [0x824581B8..0x82458254)
	// 824581B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824581BC: 48D4FFA9  bl 0x831a8164
	ctx.lr = 0x824581C0;
	sub_831A8130(ctx, base);
	// 824581C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824581C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824581C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824581CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824581D0: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824581D4: 4800006C  b 0x82458240
	pc = 0x82458240; continue 'dispatch;
	// 824581D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824581DC: E94B0008  ld r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 824581E0: 7F2AE040  cmpld cr6, r10, r28
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[28].u64, &mut ctx.xer);
	// 824581E4: 409A0018  bne cr6, 0x824581fc
	if !ctx.cr[6].eq {
	pc = 0x824581FC; continue 'dispatch;
	}
	// 824581E8: 896B0010  lbz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824581EC: 576A063E  clrlwi r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 824581F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824581F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824581F8: 419A0008  beq cr6, 0x82458200
	if ctx.cr[6].eq {
	pc = 0x82458200; continue 'dispatch;
	}
	// 824581FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82458200: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82458204: 41820038  beq 0x8245823c
	if ctx.cr[0].eq {
	pc = 0x8245823C; continue 'dispatch;
	}
	// 82458208: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8245820C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82458210: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82458214: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 82458218: 4BE6D139  bl 0x822c5350
	ctx.lr = 0x8245821C;
	sub_822C5350(ctx, base);
	// 8245821C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82458220: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82458224: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82458228: 4837A121  bl 0x827d2348
	ctx.lr = 0x8245822C;
	sub_827D2348(ctx, base);
	// 8245822C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82458230: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82458234: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82458238: 48000008  b 0x82458240
	pc = 0x82458240; continue 'dispatch;
	// 8245823C: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82458240: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82458244: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82458248: 409AFF90  bne cr6, 0x824581d8
	if !ctx.cr[6].eq {
	pc = 0x824581D8; continue 'dispatch;
	}
	// 8245824C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82458250: 48D4FF64  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458258 size=76
    let mut pc: u32 = 0x82458258;
    'dispatch: loop {
        match pc {
            0x82458258 => {
    //   block [0x82458258..0x824582A4)
	// 82458258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245825C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82458264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245826C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82458270: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82458274: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82458278: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8245827C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82458280: 4BE6E929  bl 0x822c6ba8
	ctx.lr = 0x82458284;
	sub_822C6BA8(ctx, base);
	// 82458284: 9BDF002C  stb r30, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82458288: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 8245828C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82458290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245829C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824582A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824582A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824582A8 size=568
    let mut pc: u32 = 0x824582A8;
    'dispatch: loop {
        match pc {
            0x824582A8 => {
    //   block [0x824582A8..0x824584E0)
	// 824582A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824582AC: 48D4FEB9  bl 0x831a8164
	ctx.lr = 0x824582B0;
	sub_831A8130(ctx, base);
	// 824582B0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824582B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824582B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824582BC: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824582C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824582C4: 40820214  bne 0x824584d8
	if !ctx.cr[0].eq {
	pc = 0x824584D8; continue 'dispatch;
	}
	// 824582C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824582CC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824582D0: 419A0114  beq cr6, 0x824583e4
	if ctx.cr[6].eq {
	pc = 0x824583E4; continue 'dispatch;
	}
	// 824582D4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 824582D8: 409A0200  bne cr6, 0x824584d8
	if !ctx.cr[6].eq {
	pc = 0x824584D8; continue 'dispatch;
	}
	// 824582DC: 897F002C  lbz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 824582E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824582E4: 40820014  bne 0x824582f8
	if !ctx.cr[0].eq {
	pc = 0x824582F8; continue 'dispatch;
	}
	// 824582E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824582EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824582F0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824582F4: 4800000C  b 0x82458300
	pc = 0x82458300; continue 'dispatch;
	// 824582F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824582FC: 997F002C  stb r11, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 82458300: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 82458304: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82458308: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8245830C: 4BFFBCAD  bl 0x82453fb8
	ctx.lr = 0x82458310;
	sub_82453FB8(ctx, base);
	// 82458310: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82458314: 409800C4  bge cr6, 0x824583d8
	if !ctx.cr[6].lt {
	pc = 0x824583D8; continue 'dispatch;
	}
	// 82458318: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8245831C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82458320: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82458324: 4BFFBCBD  bl 0x82453fe0
	ctx.lr = 0x82458328;
	sub_82453FE0(ctx, base);
	// 82458328: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8245832C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458330: 4BFFE8C1  bl 0x82456bf0
	ctx.lr = 0x82458334;
	sub_82456BF0(ctx, base);
	// 82458334: C01F0030  lfs f0, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82458338: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8245833C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82458340: 486EEE61  bl 0x82b471a0
	ctx.lr = 0x82458344;
	sub_82B471A0(ctx, base);
	// 82458344: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82458348: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8245834C: 48273F6D  bl 0x826cc2b8
	ctx.lr = 0x82458350;
	sub_826CC2B8(ctx, base);
	// 82458350: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82458354: 41820060  beq 0x824583b4
	if ctx.cr[0].eq {
	pc = 0x824583B4; continue 'dispatch;
	}
	// 82458358: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8245835C: 3F608335  lis r27, -0x7ccb
	ctx.r[27].s64 = -2093678592;
	// 82458360: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82458364: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82458368: 7C9C5A14  add r4, r28, r11
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 8245836C: 4BFFBBED  bl 0x82453f58
	ctx.lr = 0x82458370;
	sub_82453F58(ctx, base);
	// 82458370: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82458374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458378: 809B167C  lwz r4, 0x167c(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(5756 as u32) ) } as u64;
	// 8245837C: 4899987D  bl 0x82df1bf8
	ctx.lr = 0x82458380;
	sub_82DF1BF8(ctx, base);
	// 82458380: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82458384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82458388: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245838C: 486EEEED  bl 0x82b47278
	ctx.lr = 0x82458390;
	sub_82B47278(ctx, base);
	// 82458390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458394: 489998FD  bl 0x82df1c90
	ctx.lr = 0x82458398;
	sub_82DF1C90(ctx, base);
	// 82458398: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8245839C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824583A0: 3B9C0028  addi r28, r28, 0x28
	ctx.r[28].s64 = ctx.r[28].s64 + 40;
	// 824583A4: 48273F15  bl 0x826cc2b8
	ctx.lr = 0x824583A8;
	sub_826CC2B8(ctx, base);
	// 824583A8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 824583AC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824583B0: 4198FFB0  blt cr6, 0x82458360
	if ctx.cr[6].lt {
	pc = 0x82458360; continue 'dispatch;
	}
	// 824583B4: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824583B8: 48000010  b 0x824583c8
	pc = 0x824583C8; continue 'dispatch;
	// 824583BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824583C0: 4BFFFA21  bl 0x82457de0
	ctx.lr = 0x824583C4;
	sub_82457DE0(ctx, base);
	// 824583C4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 824583C8: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824583CC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824583D0: 409AFFEC  bne cr6, 0x824583bc
	if !ctx.cr[6].eq {
	pc = 0x824583BC; continue 'dispatch;
	}
	// 824583D4: 48000104  b 0x824584d8
	pc = 0x824584D8; continue 'dispatch;
	// 824583D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824583DC: 4BFFFE7D  bl 0x82458258
	ctx.lr = 0x824583E0;
	sub_82458258(ctx, base);
	// 824583E0: 480000F8  b 0x824584d8
	pc = 0x824584D8; continue 'dispatch;
	// 824583E4: 897F002C  lbz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 824583E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824583EC: 4082003C  bne 0x82458428
	if !ctx.cr[0].eq {
	pc = 0x82458428; continue 'dispatch;
	}
	// 824583F0: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 824583F4: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 824583F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824583FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82458400: 4BFFEEC1  bl 0x824572c0
	ctx.lr = 0x82458404;
	sub_824572C0(ctx, base);
	// 82458404: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82458408: 48273EB1  bl 0x826cc2b8
	ctx.lr = 0x8245840C;
	sub_826CC2B8(ctx, base);
	// 8245840C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82458410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458414: 4BFFE6FD  bl 0x82456b10
	ctx.lr = 0x82458418;
	sub_82456B10(ctx, base);
	// 82458418: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245841C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82458420: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82458424: 4800000C  b 0x82458430
	pc = 0x82458430; continue 'dispatch;
	// 82458428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245842C: 997F002C  stb r11, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 82458430: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82458434: 486EED4D  bl 0x82b47180
	ctx.lr = 0x82458438;
	sub_82B47180(ctx, base);
	// 82458438: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 8245843C: 486EED45  bl 0x82b47180
	ctx.lr = 0x82458440;
	sub_82B47180(ctx, base);
	// 82458440: C01B0000  lfs f0, 0(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82458444: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82458448: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 8245844C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82458450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82458454: 48273E65  bl 0x826cc2b8
	ctx.lr = 0x82458458;
	sub_826CC2B8(ctx, base);
	// 82458458: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245845C: 4182005C  beq 0x824584b8
	if ctx.cr[0].eq {
	pc = 0x824584B8; continue 'dispatch;
	}
	// 82458460: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82458464: 3F608335  lis r27, -0x7ccb
	ctx.r[27].s64 = -2093678592;
	// 82458468: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245846C: 809B167C  lwz r4, 0x167c(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82458470: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82458474: 48999785  bl 0x82df1bf8
	ctx.lr = 0x82458478;
	sub_82DF1BF8(ctx, base);
	// 82458478: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8245847C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82458480: 486EEDB1  bl 0x82b47230
	ctx.lr = 0x82458484;
	sub_82B47230(ctx, base);
	// 82458484: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82458488: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8245848C: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82458490: 4BFFBAC9  bl 0x82453f58
	ctx.lr = 0x82458494;
	sub_82453F58(ctx, base);
	// 82458494: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82458498: 489997F9  bl 0x82df1c90
	ctx.lr = 0x8245849C;
	sub_82DF1C90(ctx, base);
	// 8245849C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824584A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824584A4: 3BBD0028  addi r29, r29, 0x28
	ctx.r[29].s64 = ctx.r[29].s64 + 40;
	// 824584A8: 48273E11  bl 0x826cc2b8
	ctx.lr = 0x824584AC;
	sub_826CC2B8(ctx, base);
	// 824584AC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 824584B0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824584B4: 4198FFB4  blt cr6, 0x82458468
	if ctx.cr[6].lt {
	pc = 0x82458468; continue 'dispatch;
	}
	// 824584B8: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824584BC: 48000010  b 0x824584cc
	pc = 0x824584CC; continue 'dispatch;
	// 824584C0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824584C4: 4BFFF91D  bl 0x82457de0
	ctx.lr = 0x824584C8;
	sub_82457DE0(ctx, base);
	// 824584C8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 824584CC: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824584D0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824584D4: 409AFFEC  bne cr6, 0x824584c0
	if !ctx.cr[6].eq {
	pc = 0x824584C0; continue 'dispatch;
	}
	// 824584D8: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824584DC: 48D4FCD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824584E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824584E0 size=128
    let mut pc: u32 = 0x824584E0;
    'dispatch: loop {
        match pc {
            0x824584E0 => {
    //   block [0x824584E0..0x82458560)
	// 824584E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824584E4: 48D4FC85  bl 0x831a8168
	ctx.lr = 0x824584E8;
	sub_831A8130(ctx, base);
	// 824584E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824584EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824584F0: 48999FE9  bl 0x82df24d8
	ctx.lr = 0x824584F4;
	sub_82DF24D8(ctx, base);
	// 824584F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824584F8: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 824584FC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82458500: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82458504: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82458508: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8245850C: 4BFFED05  bl 0x82457210
	ctx.lr = 0x82458510;
	sub_82457210(ctx, base);
	// 82458510: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82458514: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82458518: 3B9F0030  addi r28, r31, 0x30
	ctx.r[28].s64 = ctx.r[31].s64 + 48;
	// 8245851C: 9BDF002C  stb r30, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82458520: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82458524: 387C0014  addi r3, r28, 0x14
	ctx.r[3].s64 = ctx.r[28].s64 + 20;
	// 82458528: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8245852C: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82458530: 4BFFC0C9  bl 0x824545f8
	ctx.lr = 0x82458534;
	sub_824545F8(ctx, base);
	// 82458534: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82458538: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8245853C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82458540: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82458544: 4BFFEC1D  bl 0x82457160
	ctx.lr = 0x82458548;
	sub_82457160(ctx, base);
	// 82458548: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8245854C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82458550: 4BFFE5C1  bl 0x82456b10
	ctx.lr = 0x82458554;
	sub_82456B10(ctx, base);
	// 82458554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245855C: 48D4FC5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458560 size=164
    let mut pc: u32 = 0x82458560;
    'dispatch: loop {
        match pc {
            0x82458560 => {
    //   block [0x82458560..0x82458604)
	// 82458560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458564: 48D4FC09  bl 0x831a816c
	ctx.lr = 0x82458568;
	sub_831A8130(ctx, base);
	// 82458568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245856C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82458608 size=80
    let mut pc: u32 = 0x82458608;
    'dispatch: loop {
        match pc {
            0x82458608 => {
    //   block [0x82458608..0x82458658)
	// 82458608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245860C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458614: 48CFBFED  bl 0x83154600
	ctx.lr = 0x82458618;
	sub_83154600(ctx, base);
	// 82458618: 39600130  li r11, 0x130
	ctx.r[11].s64 = 304;
	// 8245861C: 39400150  li r10, 0x150
	ctx.r[10].s64 = 336;
	// 82458620: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82458624: 38E30164  addi r7, r3, 0x164
	ctx.r[7].s64 = ctx.r[3].s64 + 356;
	// 82458628: 38C30168  addi r6, r3, 0x168
	ctx.r[6].s64 = ctx.r[3].s64 + 360;
	// 8245862C: 13E358C7  vcmpequd (lvx128) v31, v3, v11
	tmp.u32 = ctx.r[3].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82458630: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82458634: 13C350C7  vcmpequd (lvx128) v30, v3, v10
	tmp.u32 = ctx.r[3].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82458638: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82458658 size=148
    let mut pc: u32 = 0x82458658;
    'dispatch: loop {
        match pc {
            0x82458658 => {
    //   block [0x82458658..0x824586EC)
	// 82458658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245865C: 48D4FB11  bl 0x831a816c
	ctx.lr = 0x82458660;
	sub_831A8130(ctx, base);
	// 82458660: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82458664: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82458668: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245866C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82458670: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82458674: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82458678: 48CFBF89  bl 0x83154600
	ctx.lr = 0x8245867C;
	sub_83154600(ctx, base);
	// 8245867C: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82458680: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82458684: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82458688: 39400130  li r10, 0x130
	ctx.r[10].s64 = 304;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824586F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824586F0 size=196
    let mut pc: u32 = 0x824586F0;
    'dispatch: loop {
        match pc {
            0x824586F0 => {
    //   block [0x824586F0..0x824587B4)
	// 824586F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824586F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824586F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824586FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458704: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82458708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245870C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82458710: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82458714: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458718: 4BE68221  bl 0x822c0938
	ctx.lr = 0x8245871C;
	sub_822C0938(ctx, base);
	// 8245871C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82458720: 41820028  beq 0x82458748
	if ctx.cr[0].eq {
	pc = 0x82458748; continue 'dispatch;
	}
	// 82458724: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458728: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8245872C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82458730: 392B7790  addi r9, r11, 0x7790
	ctx.r[9].s64 = ctx.r[11].s64 + 30608;
	// 82458734: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82458738: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245873C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82458740: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82458744: 48000008  b 0x8245874c
	pc = 0x8245874C; continue 'dispatch;
	// 82458748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245874C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82458754: 409A0044  bne cr6, 0x82458798
	if !ctx.cr[6].eq {
	pc = 0x82458798; continue 'dispatch;
	}
	// 82458758: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8245875C: 419A001C  beq cr6, 0x82458778
	if ctx.cr[6].eq {
	pc = 0x82458778; continue 'dispatch;
	}
	// 82458760: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82458764: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82458768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245876C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82458770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82458774: 4E800421  bctrl
	ctx.lr = 0x82458778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82458778: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245877C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82458780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458784: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82458788: 816B3894  lwz r11, 0x3894(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14484 as u32) ) } as u64;
	// 8245878C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82458790: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82458794: 4BE6786D  bl 0x822c0000
	ctx.lr = 0x82458798;
	sub_822C0000(ctx, base);
	// 82458798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245879C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824587A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824587A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824587A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824587AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824587B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824587B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824587B8 size=196
    let mut pc: u32 = 0x824587B8;
    'dispatch: loop {
        match pc {
            0x824587B8 => {
    //   block [0x824587B8..0x8245887C)
	// 824587B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824587BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824587C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824587C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824587C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824587CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824587D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824587D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824587D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824587DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824587E0: 4BE68159  bl 0x822c0938
	ctx.lr = 0x824587E4;
	sub_822C0938(ctx, base);
	// 824587E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824587E8: 41820028  beq 0x82458810
	if ctx.cr[0].eq {
	pc = 0x82458810; continue 'dispatch;
	}
	// 824587EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824587F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 824587F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824587F8: 392B77A4  addi r9, r11, 0x77a4
	ctx.r[9].s64 = ctx.r[11].s64 + 30628;
	// 824587FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82458800: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82458804: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82458808: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8245880C: 48000008  b 0x82458814
	pc = 0x82458814; continue 'dispatch;
	// 82458810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82458814: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245881C: 409A0044  bne cr6, 0x82458860
	if !ctx.cr[6].eq {
	pc = 0x82458860; continue 'dispatch;
	}
	// 82458820: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82458824: 419A001C  beq cr6, 0x82458840
	if ctx.cr[6].eq {
	pc = 0x82458840; continue 'dispatch;
	}
	// 82458828: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245882C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82458830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458834: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82458838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245883C: 4E800421  bctrl
	ctx.lr = 0x82458840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82458840: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82458844: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82458848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245884C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82458850: 816B3894  lwz r11, 0x3894(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14484 as u32) ) } as u64;
	// 82458854: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82458858: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245885C: 4BE677A5  bl 0x822c0000
	ctx.lr = 0x82458860;
	sub_822C0000(ctx, base);
	// 82458860: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458864: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82458868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245886C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82458874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458880 size=196
    let mut pc: u32 = 0x82458880;
    'dispatch: loop {
        match pc {
            0x82458880 => {
    //   block [0x82458880..0x82458944)
	// 82458880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458888: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245888C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458890: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458894: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82458898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245889C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824588A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824588A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824588A8: 4BE68091  bl 0x822c0938
	ctx.lr = 0x824588AC;
	sub_822C0938(ctx, base);
	// 824588AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824588B0: 41820028  beq 0x824588d8
	if ctx.cr[0].eq {
	pc = 0x824588D8; continue 'dispatch;
	}
	// 824588B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824588B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 824588BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824588C0: 392B77B8  addi r9, r11, 0x77b8
	ctx.r[9].s64 = ctx.r[11].s64 + 30648;
	// 824588C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824588C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824588CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824588D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824588D4: 48000008  b 0x824588dc
	pc = 0x824588DC; continue 'dispatch;
	// 824588D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824588DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824588E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824588E4: 409A0044  bne cr6, 0x82458928
	if !ctx.cr[6].eq {
	pc = 0x82458928; continue 'dispatch;
	}
	// 824588E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824588EC: 419A001C  beq cr6, 0x82458908
	if ctx.cr[6].eq {
	pc = 0x82458908; continue 'dispatch;
	}
	// 824588F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824588F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824588F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824588FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82458900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82458904: 4E800421  bctrl
	ctx.lr = 0x82458908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82458908: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245890C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82458910: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458914: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82458918: 816B3894  lwz r11, 0x3894(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14484 as u32) ) } as u64;
	// 8245891C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82458920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82458924: 4BE676DD  bl 0x822c0000
	ctx.lr = 0x82458928;
	sub_822C0000(ctx, base);
	// 82458928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245892C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82458930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245893C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458948 size=108
    let mut pc: u32 = 0x82458948;
    'dispatch: loop {
        match pc {
            0x82458948 => {
    //   block [0x82458948..0x824589B4)
	// 82458948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245894C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82458954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245895C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82458960: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82458964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458968: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245896C: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82458970: 48999289  bl 0x82df1bf8
	ctx.lr = 0x82458974;
	sub_82DF1BF8(ctx, base);
	// 82458974: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82458978: 486EE8E1  bl 0x82b47258
	ctx.lr = 0x8245897C;
	sub_82B47258(ctx, base);
	// 8245897C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82458980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458984: 4899930D  bl 0x82df1c90
	ctx.lr = 0x82458988;
	sub_82DF1C90(ctx, base);
	// 82458988: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245898C: 7D6BF838  and r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[31].u64;
	// 82458990: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82458994: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82458998: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8245899C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824589A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824589A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824589A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824589AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824589B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824589B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824589B8 size=108
    let mut pc: u32 = 0x824589B8;
    'dispatch: loop {
        match pc {
            0x824589B8 => {
    //   block [0x824589B8..0x82458A24)
	// 824589B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824589BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824589C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824589C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824589C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824589CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 824589D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824589D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824589D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824589DC: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 824589E0: 48999219  bl 0x82df1bf8
	ctx.lr = 0x824589E4;
	sub_82DF1BF8(ctx, base);
	// 824589E4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824589E8: 486EE871  bl 0x82b47258
	ctx.lr = 0x824589EC;
	sub_82B47258(ctx, base);
	// 824589EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824589F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824589F4: 4899929D  bl 0x82df1c90
	ctx.lr = 0x824589F8;
	sub_82DF1C90(ctx, base);
	// 824589F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824589FC: 7D6BF838  and r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[31].u64;
	// 82458A00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82458A04: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82458A08: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82458A0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82458A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458A18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82458A1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82458A28 size=80
    let mut pc: u32 = 0x82458A28;
    'dispatch: loop {
        match pc {
            0x82458A28 => {
    //   block [0x82458A28..0x82458A78)
	// 82458A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458A34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458A38: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82458A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82458A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458A44: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82458A48: 489991B1  bl 0x82df1bf8
	ctx.lr = 0x82458A4C;
	sub_82DF1BF8(ctx, base);
	// 82458A4C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82458A50: 486EE809  bl 0x82b47258
	ctx.lr = 0x82458A54;
	sub_82B47258(ctx, base);
	// 82458A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82458A58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458A5C: 48999235  bl 0x82df1c90
	ctx.lr = 0x82458A60;
	sub_82DF1C90(ctx, base);
	// 82458A60: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82458A64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82458A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82458A78 size=432
    let mut pc: u32 = 0x82458A78;
    'dispatch: loop {
        match pc {
            0x82458A78 => {
    //   block [0x82458A78..0x82458C28)
	// 82458A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458A80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82458A84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458A88: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82458A8C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82458A90: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82458A98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82458A9C: 419A016C  beq cr6, 0x82458c08
	if ctx.cr[6].eq {
	pc = 0x82458C08; continue 'dispatch;
	}
	// 82458AA0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458AA4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458AA8: 388B780C  addi r4, r11, 0x780c
	ctx.r[4].s64 = ctx.r[11].s64 + 30732;
	// 82458AAC: 4899AF5D  bl 0x82df3a08
	ctx.lr = 0x82458AB0;
	sub_82DF3A08(ctx, base);
	// 82458AB0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458AB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458AB8: 388B7800  addi r4, r11, 0x7800
	ctx.r[4].s64 = ctx.r[11].s64 + 30720;
	// 82458ABC: 4899AF4D  bl 0x82df3a08
	ctx.lr = 0x82458AC0;
	sub_82DF3A08(ctx, base);
	// 82458AC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82458AC4: 3D408327  lis r10, -0x7cd9
	ctx.r[10].s64 = -2094596096;
	// 82458AC8: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82458ACC: 3BCA388C  addi r30, r10, 0x388c
	ctx.r[30].s64 = ctx.r[10].s64 + 14476;
	// 82458AD0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82458AD4: C3EB9528  lfs f31, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82458AD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82458ADC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82458AE0: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82458AE4: C3C9DD6C  lfs f30, -0x2294(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82458AE8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82458AEC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82458AF0: 4814A7B9  bl 0x825a32a8
	ctx.lr = 0x82458AF4;
	sub_825A32A8(ctx, base);
	// 82458AF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82458AF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82458AFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458B00: 48148C71  bl 0x825a1770
	ctx.lr = 0x82458B04;
	sub_825A1770(ctx, base);
	// 82458B04: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82458B08: 4899A921  bl 0x82df3428
	ctx.lr = 0x82458B0C;
	sub_82DF3428(ctx, base);
	// 82458B0C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82458B10: 4BE701A9  bl 0x822c8cb8
	ctx.lr = 0x82458B14;
	sub_822C8CB8(ctx, base);
	// 82458B14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458B18: 4899A911  bl 0x82df3428
	ctx.lr = 0x82458B1C;
	sub_82DF3428(ctx, base);
	// 82458B1C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458B20: 4899A909  bl 0x82df3428
	ctx.lr = 0x82458B24;
	sub_82DF3428(ctx, base);
	// 82458B24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82458B28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458B2C: 388B2BF0  addi r4, r11, 0x2bf0
	ctx.r[4].s64 = ctx.r[11].s64 + 11248;
	// 82458B30: 4899AED9  bl 0x82df3a08
	ctx.lr = 0x82458B34;
	sub_82DF3A08(ctx, base);
	// 82458B34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458B38: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458B3C: 388B77F4  addi r4, r11, 0x77f4
	ctx.r[4].s64 = ctx.r[11].s64 + 30708;
	// 82458B40: 4899AEC9  bl 0x82df3a08
	ctx.lr = 0x82458B44;
	sub_82DF3A08(ctx, base);
	// 82458B44: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 82458B48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82458B4C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82458B50: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82458B54: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82458B58: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82458B5C: 4814A74D  bl 0x825a32a8
	ctx.lr = 0x82458B60;
	sub_825A32A8(ctx, base);
	// 82458B60: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82458B64: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82458B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458B6C: 48148C05  bl 0x825a1770
	ctx.lr = 0x82458B70;
	sub_825A1770(ctx, base);
	// 82458B70: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82458B74: 4899A8B5  bl 0x82df3428
	ctx.lr = 0x82458B78;
	sub_82DF3428(ctx, base);
	// 82458B78: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82458B7C: 4BE7013D  bl 0x822c8cb8
	ctx.lr = 0x82458B80;
	sub_822C8CB8(ctx, base);
	// 82458B80: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458B84: 4899A8A5  bl 0x82df3428
	ctx.lr = 0x82458B88;
	sub_82DF3428(ctx, base);
	// 82458B88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458B8C: 4899A89D  bl 0x82df3428
	ctx.lr = 0x82458B90;
	sub_82DF3428(ctx, base);
	// 82458B90: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458B94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458B98: 388B77D8  addi r4, r11, 0x77d8
	ctx.r[4].s64 = ctx.r[11].s64 + 30680;
	// 82458B9C: 4899AE6D  bl 0x82df3a08
	ctx.lr = 0x82458BA0;
	sub_82DF3A08(ctx, base);
	// 82458BA0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458BA4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458BA8: 388B77C8  addi r4, r11, 0x77c8
	ctx.r[4].s64 = ctx.r[11].s64 + 30664;
	// 82458BAC: 4899AE5D  bl 0x82df3a08
	ctx.lr = 0x82458BB0;
	sub_82DF3A08(ctx, base);
	// 82458BB0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458BB4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82458BB8: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82458BBC: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 82458BC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82458BC4: 38A96230  addi r5, r9, 0x6230
	ctx.r[5].s64 = ctx.r[9].s64 + 25136;
	// 82458BC8: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82458BCC: C04B6218  lfs f2, 0x6218(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82458BD0: C02A0790  lfs f1, 0x790(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1936 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82458BD4: 4814A6D5  bl 0x825a32a8
	ctx.lr = 0x82458BD8;
	sub_825A32A8(ctx, base);
	// 82458BD8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82458BDC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82458BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458BE4: 48148B8D  bl 0x825a1770
	ctx.lr = 0x82458BE8;
	sub_825A1770(ctx, base);
	// 82458BE8: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 82458BEC: 4899A83D  bl 0x82df3428
	ctx.lr = 0x82458BF0;
	sub_82DF3428(ctx, base);
	// 82458BF0: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 82458BF4: 4BE700C5  bl 0x822c8cb8
	ctx.lr = 0x82458BF8;
	sub_822C8CB8(ctx, base);
	// 82458BF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458BFC: 4899A82D  bl 0x82df3428
	ctx.lr = 0x82458C00;
	sub_82DF3428(ctx, base);
	// 82458C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82458C04: 4899A825  bl 0x82df3428
	ctx.lr = 0x82458C08;
	sub_82DF3428(ctx, base);
	// 82458C08: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82458C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458C14: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82458C18: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82458C1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82458C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82458C28 size=436
    let mut pc: u32 = 0x82458C28;
    'dispatch: loop {
        match pc {
            0x82458C28 => {
    //   block [0x82458C28..0x82458DDC)
	// 82458C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458C2C: 48D4F53D  bl 0x831a8168
	ctx.lr = 0x82458C30;
	sub_831A8130(ctx, base);
	// 82458C30: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82458C34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458C38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82458C3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82458C40: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82458C44: 48010B15  bl 0x82469758
	ctx.lr = 0x82458C48;
	sub_82469758(ctx, base);
	// 82458C48: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458C4C: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 82458C50: 396B7864  addi r11, r11, 0x7864
	ctx.r[11].s64 = ctx.r[11].s64 + 30820;
	// 82458C54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458C58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458C5C: 48A04C75  bl 0x82e5d8d0
	ctx.lr = 0x82458C60;
	sub_82E5D8D0(ctx, base);
	// 82458C60: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458C64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82458C68: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82458C6C: 39000130  li r8, 0x130
	ctx.r[8].s64 = 304;
	// 82458C70: 396B76FC  addi r11, r11, 0x76fc
	ctx.r[11].s64 = ctx.r[11].s64 + 30460;
	// 82458C74: 38E00150  li r7, 0x150
	ctx.r[7].s64 = 336;
	// 82458C78: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82458C7C: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82458C80: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82458C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82458C88: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82458C8C: 395F017C  addi r10, r31, 0x17c
	ctx.r[10].s64 = ctx.r[31].s64 + 380;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82458DE0 size=4
    let mut pc: u32 = 0x82458DE0;
    'dispatch: loop {
        match pc {
            0x82458DE0 => {
    //   block [0x82458DE0..0x82458DE4)
	// 82458DE0: 48A00970  b 0x82e59750
	sub_82E59750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82458DE8 size=100
    let mut pc: u32 = 0x82458DE8;
    'dispatch: loop {
        match pc {
            0x82458DE8 => {
    //   block [0x82458DE8..0x82458E4C)
	// 82458DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458DF8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82458DFC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82458E00: 3BEB6234  addi r31, r11, 0x6234
	ctx.r[31].s64 = ctx.r[11].s64 + 25140;
	// 82458E04: 816A6238  lwz r11, 0x6238(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25144 as u32) ) } as u64;
	// 82458E08: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82458E0C: 40820028  bne 0x82458e34
	if !ctx.cr[0].eq {
	pc = 0x82458E34; continue 'dispatch;
	}
	// 82458E10: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82458E14: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82458E18: 916A6238  stw r11, 0x6238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25144 as u32), ctx.r[11].u32 ) };
	// 82458E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458E20: 38897894  addi r4, r9, 0x7894
	ctx.r[4].s64 = ctx.r[9].s64 + 30868;
	// 82458E24: 4899ABE5  bl 0x82df3a08
	ctx.lr = 0x82458E28;
	sub_82DF3A08(ctx, base);
	// 82458E28: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82458E2C: 386BF3A8  addi r3, r11, -0xc58
	ctx.r[3].s64 = ctx.r[11].s64 + -3160;
	// 82458E30: 48D4F6A9  bl 0x831a84d8
	ctx.lr = 0x82458E34;
	sub_831A84D8(ctx, base);
	// 82458E34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82458E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82458E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82458E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82458E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82458E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82458E50 size=536
    let mut pc: u32 = 0x82458E50;
    'dispatch: loop {
        match pc {
            0x82458E50 => {
    //   block [0x82458E50..0x82459068)
	// 82458E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82458E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82458E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82458E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82458E60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82458E64: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 82458E68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82458E6C: 4BFFFB4D  bl 0x824589b8
	ctx.lr = 0x82458E70;
	sub_824589B8(ctx, base);
	// 82458E70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82458E74: 418200D0  beq 0x82458f44
	if ctx.cr[0].eq {
	pc = 0x82458F44; continue 'dispatch;
	}
	// 82458E78: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82458E80: 388B7818  addi r4, r11, 0x7818
	ctx.r[4].s64 = ctx.r[11].s64 + 30744;
	// 82458E84: 38A000C5  li r5, 0xc5
	ctx.r[5].s64 = 197;
	// 82458E88: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82458E8C: 4899955D  bl 0x82df23e8
	ctx.lr = 0x82458E90;
	sub_82DF23E8(ctx, base);
	// 82458E90: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82458E94: 4182001C  beq 0x82458eb0
	if ctx.cr[0].eq {
	pc = 0x82458EB0; continue 'dispatch;
	}
	// 82458E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458E9C: 48A03A8D  bl 0x82e5c928
	ctx.lr = 0x82458EA0;
	sub_82E5C928(ctx, base);
	// 82458EA0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458EA4: 396B7740  addi r11, r11, 0x7740
	ctx.r[11].s64 = ctx.r[11].s64 + 30528;
	// 82458EA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458EAC: 48000008  b 0x82458eb4
	pc = 0x82458EB4; continue 'dispatch;
	// 82458EB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82458EB4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82458EB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82458EBC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458EC0: 4BFFF831  bl 0x824586f0
	ctx.lr = 0x82458EC4;
	sub_824586F0(ctx, base);
	// 82458EC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82458EC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82458ECC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458ED0: 4BE67131  bl 0x822c0000
	ctx.lr = 0x82458ED4;
	sub_822C0000(ctx, base);
	// 82458ED4: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82458ED8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82458EDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82458EE0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82458EE4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82458EE8: 419A0024  beq cr6, 0x82458f0c
	if ctx.cr[6].eq {
	pc = 0x82458F0C; continue 'dispatch;
	}
	// 82458EEC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82458EF0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82458EF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82458EF8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82458EFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82458F00: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82458F04: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82458F08: 4082FFE8  bne 0x82458ef0
	if !ctx.cr[0].eq {
	pc = 0x82458EF0; continue 'dispatch;
	}
	// 82458F0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82458F10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82458F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82458F18: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82458F1C: 389E00D0  addi r4, r30, 0xd0
	ctx.r[4].s64 = ctx.r[30].s64 + 208;
	// 82458F20: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82458F24: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82458F28: 48A05781  bl 0x82e5e6a8
	ctx.lr = 0x82458F2C;
	sub_82E5E6A8(ctx, base);
	// 82458F2C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82458F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82458F34: 419A0008  beq cr6, 0x82458f3c
	if ctx.cr[6].eq {
	pc = 0x82458F3C; continue 'dispatch;
	}
	// 82458F38: 4BE67959  bl 0x822c0890
	ctx.lr = 0x82458F3C;
	sub_822C0890(ctx, base);
	// 82458F3C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82458F40: 480000E0  b 0x82459020
	pc = 0x82459020; continue 'dispatch;
	// 82458F44: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82458F48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82458F4C: 4BFFFA6D  bl 0x824589b8
	ctx.lr = 0x82458F50;
	sub_824589B8(ctx, base);
	// 82458F50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82458F54: 418200E8  beq 0x8245903c
	if ctx.cr[0].eq {
	pc = 0x8245903C; continue 'dispatch;
	}
	// 82458F58: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82458F60: 388B7818  addi r4, r11, 0x7818
	ctx.r[4].s64 = ctx.r[11].s64 + 30744;
	// 82458F64: 38A000C9  li r5, 0xc9
	ctx.r[5].s64 = 201;
	// 82458F68: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82458F6C: 4899947D  bl 0x82df23e8
	ctx.lr = 0x82458F70;
	sub_82DF23E8(ctx, base);
	// 82458F70: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82458F74: 4182001C  beq 0x82458f90
	if ctx.cr[0].eq {
	pc = 0x82458F90; continue 'dispatch;
	}
	// 82458F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82458F7C: 48A039AD  bl 0x82e5c928
	ctx.lr = 0x82458F80;
	sub_82E5C928(ctx, base);
	// 82458F80: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82458F84: 396B7768  addi r11, r11, 0x7768
	ctx.r[11].s64 = ctx.r[11].s64 + 30568;
	// 82458F88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82458F8C: 48000008  b 0x82458f94
	pc = 0x82458F94; continue 'dispatch;
	// 82458F90: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82458F94: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82458F98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82458F9C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458FA0: 4BFFF819  bl 0x824587b8
	ctx.lr = 0x82458FA4;
	sub_824587B8(ctx, base);
	// 82458FA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82458FA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82458FAC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82458FB0: 4BE67051  bl 0x822c0000
	ctx.lr = 0x82458FB4;
	sub_822C0000(ctx, base);
	// 82458FB4: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82458FB8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82458FBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82458FC0: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82458FC4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82458FC8: 419A0024  beq cr6, 0x82458fec
	if ctx.cr[6].eq {
	pc = 0x82458FEC; continue 'dispatch;
	}
	// 82458FCC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82458FD0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82458FD4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82458FD8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82458FDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82458FE0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82458FE4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82458FE8: 4082FFE8  bne 0x82458fd0
	if !ctx.cr[0].eq {
	pc = 0x82458FD0; continue 'dispatch;
	}
	// 82458FEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82458FF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82458FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82458FF8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82458FFC: 389E00D0  addi r4, r30, 0xd0
	ctx.r[4].s64 = ctx.r[30].s64 + 208;
	// 82459000: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82459004: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82459008: 48A056A1  bl 0x82e5e6a8
	ctx.lr = 0x8245900C;
	sub_82E5E6A8(ctx, base);
	// 8245900C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82459010: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82459014: 419A0008  beq cr6, 0x8245901c
	if ctx.cr[6].eq {
	pc = 0x8245901C; continue 'dispatch;
	}
	// 82459018: 4BE67879  bl 0x822c0890
	ctx.lr = 0x8245901C;
	sub_822C0890(ctx, base);
	// 8245901C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82459020: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82459024: 419A0008  beq cr6, 0x8245902c
	if ctx.cr[6].eq {
	pc = 0x8245902C; continue 'dispatch;
	}
	// 82459028: 4BE67869  bl 0x822c0890
	ctx.lr = 0x8245902C;
	sub_822C0890(ctx, base);
	// 8245902C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82459030: 419A000C  beq cr6, 0x8245903c
	if ctx.cr[6].eq {
	pc = 0x8245903C; continue 'dispatch;
	}
	// 82459034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459038: 4BE67859  bl 0x822c0890
	ctx.lr = 0x8245903C;
	sub_822C0890(ctx, base);
	// 8245903C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82459040: 48A00729  bl 0x82e59768
	ctx.lr = 0x82459044;
	sub_82E59768(ctx, base);
	// 82459044: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82459048: 387E00D0  addi r3, r30, 0xd0
	ctx.r[3].s64 = ctx.r[30].s64 + 208;
	// 8245904C: 48A03E2D  bl 0x82e5ce78
	ctx.lr = 0x82459050;
	sub_82E5CE78(ctx, base);
	// 82459050: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82459054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245905C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82459060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459068 size=296
    let mut pc: u32 = 0x82459068;
    'dispatch: loop {
        match pc {
            0x82459068 => {
    //   block [0x82459068..0x82459190)
	// 82459068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82459074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459078: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8245907C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82459084: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 82459088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245908C: 4BFFF8BD  bl 0x82458948
	ctx.lr = 0x82459090;
	sub_82458948(ctx, base);
	// 82459090: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82459094: 408200AC  bne 0x82459140
	if !ctx.cr[0].eq {
	pc = 0x82459140; continue 'dispatch;
	}
	// 82459098: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245909C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824590A0: 41820044  beq 0x824590e4
	if ctx.cr[0].eq {
	pc = 0x824590E4; continue 'dispatch;
	}
	// 824590A4: 480009BD  bl 0x82459a60
	ctx.lr = 0x824590A8;
	sub_82459A60(ctx, base);
	// 824590A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824590AC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824590B0: 48A006C9  bl 0x82e59778
	ctx.lr = 0x824590B4;
	sub_82E59778(ctx, base);
	// 824590B4: ED9F0072  fmuls f12, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 824590B8: 3FC08327  lis r30, -0x7cd9
	ctx.r[30].s64 = -2094596096;
	// 824590BC: C17F0168  lfs f11, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824590C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824590C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824590C8: C01E3890  lfs f0, 0x3890(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14480 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824590CC: C1AB9524  lfs f13, -0x6adc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824590D0: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 824590D4: EC005B7C  fnmsubs f0, f0, f13, f11
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 824590D8: D01F0168  stfs f0, 0x168(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 824590DC: 480009D5  bl 0x82459ab0
	ctx.lr = 0x824590E0;
	sub_82459AB0(ctx, base);
	// 824590E0: 48000040  b 0x82459120
	pc = 0x82459120; continue 'dispatch;
	// 824590E4: 4BFFF945  bl 0x82458a28
	ctx.lr = 0x824590E8;
	sub_82458A28(ctx, base);
	// 824590E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824590EC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824590F0: 48A00689  bl 0x82e59778
	ctx.lr = 0x824590F4;
	sub_82E59778(ctx, base);
	// 824590F4: ED9F0072  fmuls f12, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 824590F8: 3FC08327  lis r30, -0x7cd9
	ctx.r[30].s64 = -2094596096;
	// 824590FC: C17F0168  lfs f11, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82459100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459108: C01E3890  lfs f0, 0x3890(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14480 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245910C: C1AB9524  lfs f13, -0x6adc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82459110: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82459114: EC005B7C  fnmsubs f0, f0, f13, f11
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 82459118: D01F0168  stfs f0, 0x168(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 8245911C: 480008F5  bl 0x82459a10
	ctx.lr = 0x82459120;
	sub_82459A10(ctx, base);
	// 82459120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459124: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82459128: 48A00651  bl 0x82e59778
	ctx.lr = 0x8245912C;
	sub_82E59778(ctx, base);
	// 8245912C: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 82459130: C19F0164  lfs f12, 0x164(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82459134: C01E3890  lfs f0, 0x3890(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14480 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82459138: EC0D603A  fmadds f0, f13, f0, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 8245913C: D01F0164  stfs f0, 0x164(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82459140: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459144: C01F0164  lfs f0, 0x164(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82459148: C1AB9584  lfs f13, -0x6a7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245914C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82459150: 41990014  bgt cr6, 0x82459164
	if ctx.cr[6].gt {
	pc = 0x82459164; continue 'dispatch;
	}
	// 82459154: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459158: C1AB78A8  lfs f13, 0x78a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30888 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245915C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82459160: 40980008  bge cr6, 0x82459168
	if !ctx.cr[6].lt {
	pc = 0x82459168; continue 'dispatch;
	}
	// 82459164: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82459168: D01F0164  stfs f0, 0x164(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 8245916C: 387F0168  addi r3, r31, 0x168
	ctx.r[3].s64 = ctx.r[31].s64 + 360;
	// 82459170: 4803ECA9  bl 0x82497e18
	ctx.lr = 0x82459174;
	sub_82497E18(ctx, base);
	// 82459174: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245917C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459180: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82459184: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82459188: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245918C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459190 size=384
    let mut pc: u32 = 0x82459190;
    'dispatch: loop {
        match pc {
            0x82459190 => {
    //   block [0x82459190..0x82459310)
	// 82459190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459194: 48D4EFD9  bl 0x831a816c
	ctx.lr = 0x82459198;
	sub_831A8130(ctx, base);
	// 82459198: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8245919C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824591A0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 824591A4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824591A8: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 824591AC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824591B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824591B4: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 824591B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824591BC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82459310 size=120
    let mut pc: u32 = 0x82459310;
    'dispatch: loop {
        match pc {
            0x82459310 => {
    //   block [0x82459310..0x82459388)
	// 82459310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459314: 48D4EE59  bl 0x831a816c
	ctx.lr = 0x82459318;
	sub_831A8130(ctx, base);
	// 82459318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245931C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459320: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82459324: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82459328: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8245932C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82459330: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82459334: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82459338: 489990B1  bl 0x82df23e8
	ctx.lr = 0x8245933C;
	sub_82DF23E8(ctx, base);
	// 8245933C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82459340: 41820014  beq 0x82459354
	if ctx.cr[0].eq {
	pc = 0x82459354; continue 'dispatch;
	}
	// 82459344: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82459348: 486FF529  bl 0x82b58870
	ctx.lr = 0x8245934C;
	sub_82B58870(ctx, base);
	// 8245934C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459350: 48000008  b 0x82459358
	pc = 0x82459358; continue 'dispatch;
	// 82459354: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82459358: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8245935C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82459360: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82459364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82459368: 4BFFF519  bl 0x82458880
	ctx.lr = 0x8245936C;
	sub_82458880(ctx, base);
	// 8245936C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82459370: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82459374: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82459378: 4BE66C89  bl 0x822c0000
	ctx.lr = 0x8245937C;
	sub_822C0000(ctx, base);
	// 8245937C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82459380: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459384: 48D4EE38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459388 size=536
    let mut pc: u32 = 0x82459388;
    'dispatch: loop {
        match pc {
            0x82459388 => {
    //   block [0x82459388..0x824595A0)
	// 82459388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245938C: 48D4EDD9  bl 0x831a8164
	ctx.lr = 0x82459390;
	sub_831A8130(ctx, base);
	// 82459390: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82459394: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459398: 48CFB269  bl 0x83154600
	ctx.lr = 0x8245939C;
	sub_83154600(ctx, base);
	// 8245939C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824593A0: 48CFB261  bl 0x83154600
	ctx.lr = 0x824593A4;
	sub_83154600(ctx, base);
	// 824593A4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824593A8: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 824593AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824593B0: 4BFFF609  bl 0x824589b8
	ctx.lr = 0x824593B4;
	sub_824589B8(ctx, base);
	// 824593B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824593B8: 418200F0  beq 0x824594a8
	if ctx.cr[0].eq {
	pc = 0x824594A8; continue 'dispatch;
	}
	// 824593BC: 3BBF0130  addi r29, r31, 0x130
	ctx.r[29].s64 = ctx.r[31].s64 + 304;
	// 824593C0: 3BDF0150  addi r30, r31, 0x150
	ctx.r[30].s64 = ctx.r[31].s64 + 336;
	// 824593C4: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 824593C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824593CC: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824593D0: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824595A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824595A0 size=644
    let mut pc: u32 = 0x824595A0;
    'dispatch: loop {
        match pc {
            0x824595A0 => {
    //   block [0x824595A0..0x82459824)
	// 824595A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824595A4: 48D4EBBD  bl 0x831a8160
	ctx.lr = 0x824595A8;
	sub_831A8130(ctx, base);
	// 824595A8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 824595AC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824595B0: 48CFB051  bl 0x83154600
	ctx.lr = 0x824595B4;
	sub_83154600(ctx, base);
	// 824595B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824595B8: 48CFB049  bl 0x83154600
	ctx.lr = 0x824595BC;
	sub_83154600(ctx, base);
	// 824595BC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824595C0: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 824595C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824595C8: 4BFFF3F1  bl 0x824589b8
	ctx.lr = 0x824595CC;
	sub_824589B8(ctx, base);
	// 824595CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824595D0: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824595D4: 3B4B7818  addi r26, r11, 0x7818
	ctx.r[26].s64 = ctx.r[11].s64 + 30744;
	// 824595D8: 418200EC  beq 0x824596c4
	if ctx.cr[0].eq {
	pc = 0x824596C4; continue 'dispatch;
	}
	// 824595DC: 3BBF0130  addi r29, r31, 0x130
	ctx.r[29].s64 = ctx.r[31].s64 + 304;
	// 824595E0: 3BDF0150  addi r30, r31, 0x150
	ctx.r[30].s64 = ctx.r[31].s64 + 336;
	// 824595E4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 824595E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824595EC: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824595F0: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82459828 size=72
    let mut pc: u32 = 0x82459828;
    'dispatch: loop {
        match pc {
            0x82459828 => {
    //   block [0x82459828..0x82459870)
	// 82459828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245982C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245983C: 48CFADC5  bl 0x83154600
	ctx.lr = 0x82459840;
	sub_83154600(ctx, base);
	// 82459840: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82459844: 38BF0150  addi r5, r31, 0x150
	ctx.r[5].s64 = ctx.r[31].s64 + 336;
	// 82459848: 389F0130  addi r4, r31, 0x130
	ctx.r[4].s64 = ctx.r[31].s64 + 304;
	// 8245984C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459850: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82459854: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82459858: 4E800421  bctrl
	ctx.lr = 0x8245985C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245985C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82459860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82459870 size=216
    let mut pc: u32 = 0x82459870;
    'dispatch: loop {
        match pc {
            0x82459870 => {
    //   block [0x82459870..0x82459948)
	// 82459870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459874: 48D4E8F9  bl 0x831a816c
	ctx.lr = 0x82459878;
	sub_831A8130(ctx, base);
	// 82459878: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8245987C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82459880: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459888: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8245988C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82459890: 48CFAD71  bl 0x83154600
	ctx.lr = 0x82459894;
	sub_83154600(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82459948 size=196
    let mut pc: u32 = 0x82459948;
    'dispatch: loop {
        match pc {
            0x82459948 => {
    //   block [0x82459948..0x82459A0C)
	// 82459948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245994C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82459954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245995C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82459960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82459964: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82459968: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245996C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82459970: 4BE66FC9  bl 0x822c0938
	ctx.lr = 0x82459974;
	sub_822C0938(ctx, base);
	// 82459974: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82459978: 41820028  beq 0x824599a0
	if ctx.cr[0].eq {
	pc = 0x824599A0; continue 'dispatch;
	}
	// 8245997C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459980: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82459984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82459988: 392B78E8  addi r9, r11, 0x78e8
	ctx.r[9].s64 = ctx.r[11].s64 + 30952;
	// 8245998C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82459990: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82459994: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82459998: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8245999C: 48000008  b 0x824599a4
	pc = 0x824599A4; continue 'dispatch;
	// 824599A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824599A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824599A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824599AC: 409A0044  bne cr6, 0x824599f0
	if !ctx.cr[6].eq {
	pc = 0x824599F0; continue 'dispatch;
	}
	// 824599B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824599B4: 419A001C  beq cr6, 0x824599d0
	if ctx.cr[6].eq {
	pc = 0x824599D0; continue 'dispatch;
	}
	// 824599B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824599BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824599C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824599C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824599C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824599CC: 4E800421  bctrl
	ctx.lr = 0x824599D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824599D0: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 824599D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824599D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824599DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 824599E0: 816B3B40  lwz r11, 0x3b40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15168 as u32) ) } as u64;
	// 824599E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824599E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824599EC: 4BE66615  bl 0x822c0000
	ctx.lr = 0x824599F0;
	sub_822C0000(ctx, base);
	// 824599F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824599F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824599F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824599FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459A00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82459A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459A10 size=80
    let mut pc: u32 = 0x82459A10;
    'dispatch: loop {
        match pc {
            0x82459A10 => {
    //   block [0x82459A10..0x82459A60)
	// 82459A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459A18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459A1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459A20: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82459A24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82459A28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459A2C: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82459A30: 489981C9  bl 0x82df1bf8
	ctx.lr = 0x82459A34;
	sub_82DF1BF8(ctx, base);
	// 82459A34: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82459A38: 486ED821  bl 0x82b47258
	ctx.lr = 0x82459A3C;
	sub_82B47258(ctx, base);
	// 82459A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459A44: 4899824D  bl 0x82df1c90
	ctx.lr = 0x82459A48;
	sub_82DF1C90(ctx, base);
	// 82459A48: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82459A4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459A60 size=80
    let mut pc: u32 = 0x82459A60;
    'dispatch: loop {
        match pc {
            0x82459A60 => {
    //   block [0x82459A60..0x82459AB0)
	// 82459A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459A68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459A6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459A70: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82459A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82459A78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459A7C: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82459A80: 48998179  bl 0x82df1bf8
	ctx.lr = 0x82459A84;
	sub_82DF1BF8(ctx, base);
	// 82459A84: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82459A88: 486ED7D1  bl 0x82b47258
	ctx.lr = 0x82459A8C;
	sub_82B47258(ctx, base);
	// 82459A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459A90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459A94: 489981FD  bl 0x82df1c90
	ctx.lr = 0x82459A98;
	sub_82DF1C90(ctx, base);
	// 82459A98: C03F001C  lfs f1, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82459A9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459AA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459AB0 size=80
    let mut pc: u32 = 0x82459AB0;
    'dispatch: loop {
        match pc {
            0x82459AB0 => {
    //   block [0x82459AB0..0x82459B00)
	// 82459AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459ABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459AC0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82459AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82459AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459ACC: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82459AD0: 48998129  bl 0x82df1bf8
	ctx.lr = 0x82459AD4;
	sub_82DF1BF8(ctx, base);
	// 82459AD4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82459AD8: 486ED781  bl 0x82b47258
	ctx.lr = 0x82459ADC;
	sub_82B47258(ctx, base);
	// 82459ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459AE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459AE4: 489981AD  bl 0x82df1c90
	ctx.lr = 0x82459AE8;
	sub_82DF1C90(ctx, base);
	// 82459AE8: C03F0020  lfs f1, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82459AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459B00 size=312
    let mut pc: u32 = 0x82459B00;
    'dispatch: loop {
        match pc {
            0x82459B00 => {
    //   block [0x82459B00..0x82459C38)
	// 82459B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459B08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82459B0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459B10: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82459B14: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82459B18: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459B20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82459B24: 419A00F4  beq cr6, 0x82459c18
	if ctx.cr[6].eq {
	pc = 0x82459C18; continue 'dispatch;
	}
	// 82459B28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459B2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459B30: 388B2BF0  addi r4, r11, 0x2bf0
	ctx.r[4].s64 = ctx.r[11].s64 + 11248;
	// 82459B34: 48999ED5  bl 0x82df3a08
	ctx.lr = 0x82459B38;
	sub_82DF3A08(ctx, base);
	// 82459B38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459B3C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82459B40: 388B77F4  addi r4, r11, 0x77f4
	ctx.r[4].s64 = ctx.r[11].s64 + 30708;
	// 82459B44: 48999EC5  bl 0x82df3a08
	ctx.lr = 0x82459B48;
	sub_82DF3A08(ctx, base);
	// 82459B48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459B4C: 3D408327  lis r10, -0x7cd9
	ctx.r[10].s64 = -2094596096;
	// 82459B50: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82459B54: 3BCA3B38  addi r30, r10, 0x3b38
	ctx.r[30].s64 = ctx.r[10].s64 + 15160;
	// 82459B58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82459B5C: C3EB9528  lfs f31, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82459B60: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 82459B64: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82459B68: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82459B6C: C3C9DD6C  lfs f30, -0x2294(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82459B70: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82459B74: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82459B78: 48149731  bl 0x825a32a8
	ctx.lr = 0x82459B7C;
	sub_825A32A8(ctx, base);
	// 82459B7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82459B80: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82459B84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459B88: 48147BE9  bl 0x825a1770
	ctx.lr = 0x82459B8C;
	sub_825A1770(ctx, base);
	// 82459B8C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82459B90: 48999899  bl 0x82df3428
	ctx.lr = 0x82459B94;
	sub_82DF3428(ctx, base);
	// 82459B94: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82459B98: 4BE6F121  bl 0x822c8cb8
	ctx.lr = 0x82459B9C;
	sub_822C8CB8(ctx, base);
	// 82459B9C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82459BA0: 48999889  bl 0x82df3428
	ctx.lr = 0x82459BA4;
	sub_82DF3428(ctx, base);
	// 82459BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459BA8: 48999881  bl 0x82df3428
	ctx.lr = 0x82459BAC;
	sub_82DF3428(ctx, base);
	// 82459BAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459BB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82459BB4: 388B7904  addi r4, r11, 0x7904
	ctx.r[4].s64 = ctx.r[11].s64 + 30980;
	// 82459BB8: 48999E51  bl 0x82df3a08
	ctx.lr = 0x82459BBC;
	sub_82DF3A08(ctx, base);
	// 82459BBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459BC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459BC4: 388B78F8  addi r4, r11, 0x78f8
	ctx.r[4].s64 = ctx.r[11].s64 + 30968;
	// 82459BC8: 48999E41  bl 0x82df3a08
	ctx.lr = 0x82459BCC;
	sub_82DF3A08(ctx, base);
	// 82459BCC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82459BD0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82459BD4: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82459BD8: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82459BDC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82459BE0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82459BE4: 481496C5  bl 0x825a32a8
	ctx.lr = 0x82459BE8;
	sub_825A32A8(ctx, base);
	// 82459BE8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82459BEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82459BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459BF4: 48147B7D  bl 0x825a1770
	ctx.lr = 0x82459BF8;
	sub_825A1770(ctx, base);
	// 82459BF8: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82459BFC: 4899982D  bl 0x82df3428
	ctx.lr = 0x82459C00;
	sub_82DF3428(ctx, base);
	// 82459C00: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82459C04: 4BE6F0B5  bl 0x822c8cb8
	ctx.lr = 0x82459C08;
	sub_822C8CB8(ctx, base);
	// 82459C08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82459C0C: 4899981D  bl 0x82df3428
	ctx.lr = 0x82459C10;
	sub_82DF3428(ctx, base);
	// 82459C10: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82459C14: 48999815  bl 0x82df3428
	ctx.lr = 0x82459C18;
	sub_82DF3428(ctx, base);
	// 82459C18: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82459C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459C24: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82459C28: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82459C2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82459C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459C38 size=440
    let mut pc: u32 = 0x82459C38;
    'dispatch: loop {
        match pc {
            0x82459C38 => {
    //   block [0x82459C38..0x82459DF0)
	// 82459C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459C3C: 48D4E52D  bl 0x831a8168
	ctx.lr = 0x82459C40;
	sub_831A8130(ctx, base);
	// 82459C40: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82459C44: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459C4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82459C50: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82459C54: 4800FB05  bl 0x82469758
	ctx.lr = 0x82459C58;
	sub_82469758(ctx, base);
	// 82459C58: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459C5C: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 82459C60: 396B7960  addi r11, r11, 0x7960
	ctx.r[11].s64 = ctx.r[11].s64 + 31072;
	// 82459C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82459C68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82459C6C: 48A03C65  bl 0x82e5d8d0
	ctx.lr = 0x82459C70;
	sub_82E5D8D0(ctx, base);
	// 82459C70: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459C74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82459C78: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82459C7C: 39000130  li r8, 0x130
	ctx.r[8].s64 = 304;
	// 82459C80: 396B78B0  addi r11, r11, 0x78b0
	ctx.r[11].s64 = ctx.r[11].s64 + 30896;
	// 82459C84: 38E00150  li r7, 0x150
	ctx.r[7].s64 = 336;
	// 82459C88: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82459C8C: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82459C90: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82459C94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82459DF0 size=100
    let mut pc: u32 = 0x82459DF0;
    'dispatch: loop {
        match pc {
            0x82459DF0 => {
    //   block [0x82459DF0..0x82459E54)
	// 82459DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459DF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459DFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459E00: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82459E04: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82459E08: 3BEB623C  addi r31, r11, 0x623c
	ctx.r[31].s64 = ctx.r[11].s64 + 25148;
	// 82459E0C: 816A6240  lwz r11, 0x6240(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25152 as u32) ) } as u64;
	// 82459E10: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82459E14: 40820028  bne 0x82459e3c
	if !ctx.cr[0].eq {
	pc = 0x82459E3C; continue 'dispatch;
	}
	// 82459E18: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82459E1C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82459E20: 916A6240  stw r11, 0x6240(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25152 as u32), ctx.r[11].u32 ) };
	// 82459E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459E28: 38897990  addi r4, r9, 0x7990
	ctx.r[4].s64 = ctx.r[9].s64 + 31120;
	// 82459E2C: 48999BDD  bl 0x82df3a08
	ctx.lr = 0x82459E30;
	sub_82DF3A08(ctx, base);
	// 82459E30: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82459E34: 386BF3B8  addi r3, r11, -0xc48
	ctx.r[3].s64 = ctx.r[11].s64 + -3144;
	// 82459E38: 48D4E6A1  bl 0x831a84d8
	ctx.lr = 0x82459E3C;
	sub_831A84D8(ctx, base);
	// 82459E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82459E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459E4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82459E58 size=100
    let mut pc: u32 = 0x82459E58;
    'dispatch: loop {
        match pc {
            0x82459E58 => {
    //   block [0x82459E58..0x82459EBC)
	// 82459E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459E60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82459E64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459E68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459E6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459E70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82459E74: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82459E78: 48A039D1  bl 0x82e5d848
	ctx.lr = 0x82459E7C;
	sub_82E5D848(ctx, base);
	// 82459E7C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459E84: 396B770C  addi r11, r11, 0x770c
	ctx.r[11].s64 = ctx.r[11].s64 + 30476;
	// 82459E88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82459E8C: 48A026FD  bl 0x82e5c588
	ctx.lr = 0x82459E90;
	sub_82E5C588(ctx, base);
	// 82459E90: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82459E94: 4182000C  beq 0x82459ea0
	if ctx.cr[0].eq {
	pc = 0x82459EA0; continue 'dispatch;
	}
	// 82459E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459E9C: 4899853D  bl 0x82df23d8
	ctx.lr = 0x82459EA0;
	sub_82DF23D8(ctx, base);
	// 82459EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459EA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459EB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82459EB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459EC0 size=296
    let mut pc: u32 = 0x82459EC0;
    'dispatch: loop {
        match pc {
            0x82459EC0 => {
    //   block [0x82459EC0..0x82459FE8)
	// 82459EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82459ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459ED0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82459ED4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82459ED8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82459EDC: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 82459EE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82459EE4: 4BFFEA65  bl 0x82458948
	ctx.lr = 0x82459EE8;
	sub_82458948(ctx, base);
	// 82459EE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82459EEC: 408200AC  bne 0x82459f98
	if !ctx.cr[0].eq {
	pc = 0x82459F98; continue 'dispatch;
	}
	// 82459EF0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82459EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459EF8: 41820044  beq 0x82459f3c
	if ctx.cr[0].eq {
	pc = 0x82459F3C; continue 'dispatch;
	}
	// 82459EFC: 4BFFFB65  bl 0x82459a60
	ctx.lr = 0x82459F00;
	sub_82459A60(ctx, base);
	// 82459F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459F04: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82459F08: 489FF871  bl 0x82e59778
	ctx.lr = 0x82459F0C;
	sub_82E59778(ctx, base);
	// 82459F0C: ED9F0072  fmuls f12, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 82459F10: 3FC08327  lis r30, -0x7cd9
	ctx.r[30].s64 = -2094596096;
	// 82459F14: C17F0168  lfs f11, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82459F18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459F20: C01E3B3C  lfs f0, 0x3b3c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(15164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82459F24: C1AB9524  lfs f13, -0x6adc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82459F28: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82459F2C: EC005B7C  fnmsubs f0, f0, f13, f11
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 82459F30: D01F0168  stfs f0, 0x168(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 82459F34: 4BFFFB7D  bl 0x82459ab0
	ctx.lr = 0x82459F38;
	sub_82459AB0(ctx, base);
	// 82459F38: 48000040  b 0x82459f78
	pc = 0x82459F78; continue 'dispatch;
	// 82459F3C: 4BFFEAED  bl 0x82458a28
	ctx.lr = 0x82459F40;
	sub_82458A28(ctx, base);
	// 82459F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459F44: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82459F48: 489FF831  bl 0x82e59778
	ctx.lr = 0x82459F4C;
	sub_82E59778(ctx, base);
	// 82459F4C: ED9F0072  fmuls f12, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 82459F50: 3FC08327  lis r30, -0x7cd9
	ctx.r[30].s64 = -2094596096;
	// 82459F54: C17F0168  lfs f11, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82459F58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459F60: C01E3B3C  lfs f0, 0x3b3c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(15164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82459F64: C1AB9524  lfs f13, -0x6adc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82459F68: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82459F6C: EC005B7C  fnmsubs f0, f0, f13, f11
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 82459F70: D01F0168  stfs f0, 0x168(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 82459F74: 4BFFFA9D  bl 0x82459a10
	ctx.lr = 0x82459F78;
	sub_82459A10(ctx, base);
	// 82459F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82459F7C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82459F80: 489FF7F9  bl 0x82e59778
	ctx.lr = 0x82459F84;
	sub_82E59778(ctx, base);
	// 82459F84: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 82459F88: C19F0164  lfs f12, 0x164(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82459F8C: C01E3B3C  lfs f0, 0x3b3c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(15164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82459F90: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82459F94: D01F0164  stfs f0, 0x164(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82459F98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82459F9C: C01F0164  lfs f0, 0x164(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82459FA0: C1AB9584  lfs f13, -0x6a7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82459FA4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82459FA8: 41990014  bgt cr6, 0x82459fbc
	if ctx.cr[6].gt {
	pc = 0x82459FBC; continue 'dispatch;
	}
	// 82459FAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82459FB0: C1AB78A8  lfs f13, 0x78a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30888 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82459FB4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82459FB8: 40980008  bge cr6, 0x82459fc0
	if !ctx.cr[6].lt {
	pc = 0x82459FC0; continue 'dispatch;
	}
	// 82459FBC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82459FC0: D01F0164  stfs f0, 0x164(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82459FC4: 387F0168  addi r3, r31, 0x168
	ctx.r[3].s64 = ctx.r[31].s64 + 360;
	// 82459FC8: 4803DE51  bl 0x82497e18
	ctx.lr = 0x82459FCC;
	sub_82497E18(ctx, base);
	// 82459FCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82459FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82459FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82459FD8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82459FDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82459FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82459FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82459FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82459FE8 size=188
    let mut pc: u32 = 0x82459FE8;
    'dispatch: loop {
        match pc {
            0x82459FE8 => {
    //   block [0x82459FE8..0x8245A0A4)
	// 82459FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82459FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82459FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82459FF4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82459FF8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82459FFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A000: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 8245A004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A008: 4BFFE941  bl 0x82458948
	ctx.lr = 0x8245A00C;
	sub_82458948(ctx, base);
	// 8245A00C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245A010: 41820078  beq 0x8245a088
	if ctx.cr[0].eq {
	pc = 0x8245A088; continue 'dispatch;
	}
	// 8245A014: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8245A018: C01F016C  lfs f0, 0x16c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A01C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245A020: C1AA89AC  lfs f13, -0x7654(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245A024: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8245A028: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8245A02C: 41980010  blt cr6, 0x8245a03c
	if ctx.cr[6].lt {
	pc = 0x8245A03C; continue 'dispatch;
	}
	// 8245A030: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A034: C1AB964C  lfs f13, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245A038: EFE00372  fmuls f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8245A03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A040: 4BFFFA71  bl 0x82459ab0
	ctx.lr = 0x8245A044;
	sub_82459AB0(ctx, base);
	// 8245A044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A048: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8245A04C: 489FF72D  bl 0x82e59778
	ctx.lr = 0x8245A050;
	sub_82E59778(ctx, base);
	// 8245A050: ED9E0072  fmuls f12, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (((ctx.f[30].f64 * ctx.f[1].f64) as f32) as f64);
	// 8245A054: C17F0160  lfs f11, 0x160(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8245A058: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245A05C: C15F016C  lfs f10, 0x16c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8245A060: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8245A064: C1AB3B38  lfs f13, 0x3b38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245A068: C00A9528  lfs f0, -0x6ad8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A06C: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 8245A070: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8245A074: EDAD57FC  fnmsubs f13, f13, f31, f10
	ctx.f[13].f64 = -(((ctx.f[13].f64 * ctx.f[31].f64 - ctx.f[10].f64) as f32) as f64);
	// 8245A078: D1BF016C  stfs f13, 0x16c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8245A07C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8245A080: 40980008  bge cr6, 0x8245a088
	if !ctx.cr[6].lt {
	pc = 0x8245A088; continue 'dispatch;
	}
	// 8245A084: D01F016C  stfs f0, 0x16c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8245A088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A094: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8245A098: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A09C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245A0A8 size=88
    let mut pc: u32 = 0x8245A0A8;
    'dispatch: loop {
        match pc {
            0x8245A0A8 => {
    //   block [0x8245A0A8..0x8245A100)
	// 8245A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A0B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A0B4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8245A0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A0BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A0C0: 4BFFF951  bl 0x82459a10
	ctx.lr = 0x8245A0C4;
	sub_82459A10(ctx, base);
	// 8245A0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A0C8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8245A0CC: 489FF6AD  bl 0x82e59778
	ctx.lr = 0x8245A0D0;
	sub_82E59778(ctx, base);
	// 8245A0D0: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 8245A0D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A0D8: C19F0170  lfs f12, 0x170(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8245A0DC: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A0E0: EC0D603A  fmadds f0, f13, f0, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 8245A0E4: D01F0170  stfs f0, 0x170(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 8245A0E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A0F4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245A100 size=440
    let mut pc: u32 = 0x8245A100;
    'dispatch: loop {
        match pc {
            0x8245A100 => {
    //   block [0x8245A100..0x8245A2B8)
	// 8245A100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A104: 48D4E065  bl 0x831a8168
	ctx.lr = 0x8245A108;
	sub_831A8130(ctx, base);
	// 8245A108: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8245A10C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A110: 48CFA4F1  bl 0x83154600
	ctx.lr = 0x8245A114;
	sub_83154600(ctx, base);
	// 8245A114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A118: 48CFA4E9  bl 0x83154600
	ctx.lr = 0x8245A11C;
	sub_83154600(ctx, base);
	// 8245A11C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8245A120: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8245A124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A128: 4BFFE891  bl 0x824589b8
	ctx.lr = 0x8245A12C;
	sub_824589B8(ctx, base);
	// 8245A12C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A130: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8245A134: C3EB9450  lfs f31, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8245A138: 41820010  beq 0x8245a148
	if ctx.cr[0].eq {
	pc = 0x8245A148; continue 'dispatch;
	}
	// 8245A13C: C01F0160  lfs f0, 0x160(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A140: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8245A144: 48000020  b 0x8245a164
	pc = 0x8245A164; continue 'dispatch;
	// 8245A148: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8245A14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A150: 4BFFE869  bl 0x824589b8
	ctx.lr = 0x8245A154;
	sub_824589B8(ctx, base);
	// 8245A154: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245A158: 41820010  beq 0x8245a168
	if ctx.cr[0].eq {
	pc = 0x8245A168; continue 'dispatch;
	}
	// 8245A15C: C01F0160  lfs f0, 0x160(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A160: EC00F828  fsubs f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8245A164: D01F0160  stfs f0, 0x160(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 8245A168: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A16C: C01F0160  lfs f0, 0x160(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A170: C1AB959C  lfs f13, -0x6a64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245A174: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8245A178: 4099000C  ble cr6, 0x8245a184
	if !ctx.cr[6].gt {
	pc = 0x8245A184; continue 'dispatch;
	}
	// 8245A17C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8245A180: 48000010  b 0x8245a190
	pc = 0x8245A190; continue 'dispatch;
	// 8245A184: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8245A188: 40980008  bge cr6, 0x8245a190
	if !ctx.cr[6].lt {
	pc = 0x8245A190; continue 'dispatch;
	}
	// 8245A18C: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 8245A190: D01F0160  stfs f0, 0x160(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 8245A194: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8245A198: 4800A981  bl 0x82464b18
	ctx.lr = 0x8245A19C;
	sub_82464B18(ctx, base);
	// 8245A19C: 3BBF0150  addi r29, r31, 0x150
	ctx.r[29].s64 = ctx.r[31].s64 + 336;
	// 8245A1A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245A1A4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8245A1A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245A1AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8245A1B0: 4BFFF161  bl 0x82459310
	ctx.lr = 0x8245A1B4;
	sub_82459310(ctx, base);
	// 8245A1B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A1B8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8245A1BC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245A1C4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8245A1C8: 419A0024  beq cr6, 0x8245a1ec
	if ctx.cr[6].eq {
	pc = 0x8245A1EC; continue 'dispatch;
	}
	// 8245A1CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8245A1D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8245A1D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8245A1D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8245A1DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8245A1E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8245A1E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8245A1E8: 4082FFE8  bne 0x8245a1d0
	if !ctx.cr[0].eq {
	pc = 0x8245A1D0; continue 'dispatch;
	}
	// 8245A1EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8245A1F0: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8245A1F4: 388B7910  addi r4, r11, 0x7910
	ctx.r[4].s64 = ctx.r[11].s64 + 30992;
	// 8245A1F8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245A1FC: 38A00045  li r5, 0x45
	ctx.r[5].s64 = 69;
	// 8245A200: 387C0028  addi r3, r28, 0x28
	ctx.r[3].s64 = ctx.r[28].s64 + 40;
	// 8245A204: 489FCDE5  bl 0x82e56fe8
	ctx.lr = 0x8245A208;
	sub_82E56FE8(ctx, base);
	// 8245A208: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8245A20C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A210: 419A0008  beq cr6, 0x8245a218
	if ctx.cr[6].eq {
	pc = 0x8245A218; continue 'dispatch;
	}
	// 8245A214: 4BE6667D  bl 0x822c0890
	ctx.lr = 0x8245A218;
	sub_822C0890(ctx, base);
	// 8245A218: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8245A21C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A220: 419A0008  beq cr6, 0x8245a228
	if ctx.cr[6].eq {
	pc = 0x8245A228; continue 'dispatch;
	}
	// 8245A224: 4BE6666D  bl 0x822c0890
	ctx.lr = 0x8245A228;
	sub_822C0890(ctx, base);
	// 8245A228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A22C: 4BFFFE7D  bl 0x8245a0a8
	ctx.lr = 0x8245A230;
	sub_8245A0A8(ctx, base);
	// 8245A230: C01F0154  lfs f0, 0x154(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A234: C1BF0170  lfs f13, 0x170(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245A238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A23C: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8245A240: D01F0154  stfs f0, 0x154(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), tmp.u32 ) };
	// 8245A244: 4BFFFDA5  bl 0x82459fe8
	ctx.lr = 0x8245A248;
	sub_82459FE8(ctx, base);
	// 8245A248: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245A24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A250: 4BFFFC71  bl 0x82459ec0
	ctx.lr = 0x8245A254;
	sub_82459EC0(ctx, base);
	// 8245A254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245A258: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8245A25C: C07F0164  lfs f3, 0x164(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8245A260: C05F0168  lfs f2, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8245A264: C03F016C  lfs f1, 0x16c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8245A268: 481453F9  bl 0x8259f660
	ctx.lr = 0x8245A26C;
	sub_8259F660(ctx, base);
	// 8245A26C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8245A270: 3BDF0130  addi r30, r31, 0x130
	ctx.r[30].s64 = ctx.r[31].s64 + 304;
	// 8245A274: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8245A278: 3B9F0140  addi r28, r31, 0x140
	ctx.r[28].s64 = ctx.r[31].s64 + 320;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245A2B8 size=12
    let mut pc: u32 = 0x8245A2B8;
    'dispatch: loop {
        match pc {
            0x8245A2B8 => {
    //   block [0x8245A2B8..0x8245A2C4)
	// 8245A2B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A2BC: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 8245A2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A2C8 size=124
    let mut pc: u32 = 0x8245A2C8;
    'dispatch: loop {
        match pc {
            0x8245A2C8 => {
    //   block [0x8245A2C8..0x8245A344)
	// 8245A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A2D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A2D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A2D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A2DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8245A2E0: 419A004C  beq cr6, 0x8245a32c
	if ctx.cr[6].eq {
	pc = 0x8245A32C; continue 'dispatch;
	}
	// 8245A2E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A2E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A2EC: 419A0018  beq cr6, 0x8245a304
	if ctx.cr[6].eq {
	pc = 0x8245A304; continue 'dispatch;
	}
	// 8245A2F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A2F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A2F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245A2FC: 4E800421  bctrl
	ctx.lr = 0x8245A300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245A300: 4800000C  b 0x8245a30c
	pc = 0x8245A30C; continue 'dispatch;
	// 8245A304: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8245A308: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 8245A30C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245A310: 388B3CA4  addi r4, r11, 0x3ca4
	ctx.r[4].s64 = ctx.r[11].s64 + 15524;
	// 8245A314: 48D4DDE5  bl 0x831a80f8
	ctx.lr = 0x8245A318;
	sub_831A80F8(ctx, base);
	// 8245A318: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245A31C: 41820010  beq 0x8245a32c
	if ctx.cr[0].eq {
	pc = 0x8245A32C; continue 'dispatch;
	}
	// 8245A320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A324: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8245A328: 48000008  b 0x8245a330
	pc = 0x8245A330; continue 'dispatch;
	// 8245A32C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245A330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245A334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A33C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A348 size=124
    let mut pc: u32 = 0x8245A348;
    'dispatch: loop {
        match pc {
            0x8245A348 => {
    //   block [0x8245A348..0x8245A3C4)
	// 8245A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A354: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A358: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A35C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8245A360: 419A004C  beq cr6, 0x8245a3ac
	if ctx.cr[6].eq {
	pc = 0x8245A3AC; continue 'dispatch;
	}
	// 8245A364: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A36C: 419A0018  beq cr6, 0x8245a384
	if ctx.cr[6].eq {
	pc = 0x8245A384; continue 'dispatch;
	}
	// 8245A370: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A374: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A378: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245A37C: 4E800421  bctrl
	ctx.lr = 0x8245A380;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245A380: 4800000C  b 0x8245a38c
	pc = 0x8245A38C; continue 'dispatch;
	// 8245A384: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8245A388: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 8245A38C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8245A390: 388B7618  addi r4, r11, 0x7618
	ctx.r[4].s64 = ctx.r[11].s64 + 30232;
	// 8245A394: 48D4DD65  bl 0x831a80f8
	ctx.lr = 0x8245A398;
	sub_831A80F8(ctx, base);
	// 8245A398: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245A39C: 41820010  beq 0x8245a3ac
	if ctx.cr[0].eq {
	pc = 0x8245A3AC; continue 'dispatch;
	}
	// 8245A3A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A3A4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8245A3A8: 48000008  b 0x8245a3b0
	pc = 0x8245A3B0; continue 'dispatch;
	// 8245A3AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245A3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245A3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A3BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245A3C8 size=12
    let mut pc: u32 = 0x8245A3C8;
    'dispatch: loop {
        match pc {
            0x8245A3C8 => {
    //   block [0x8245A3C8..0x8245A3D4)
	// 8245A3C8: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245A3CC: 386B3CA4  addi r3, r11, 0x3ca4
	ctx.r[3].s64 = ctx.r[11].s64 + 15524;
	// 8245A3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245A3D8 size=12
    let mut pc: u32 = 0x8245A3D8;
    'dispatch: loop {
        match pc {
            0x8245A3D8 => {
    //   block [0x8245A3D8..0x8245A3E4)
	// 8245A3D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8245A3DC: 386B7618  addi r3, r11, 0x7618
	ctx.r[3].s64 = ctx.r[11].s64 + 30232;
	// 8245A3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A3E8 size=108
    let mut pc: u32 = 0x8245A3E8;
    'dispatch: loop {
        match pc {
            0x8245A3E8 => {
    //   block [0x8245A3E8..0x8245A454)
	// 8245A3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A3F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245A3F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A3F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A3FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245A400: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8245A404: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245A408: 4BE66531  bl 0x822c0938
	ctx.lr = 0x8245A40C;
	sub_822C0938(ctx, base);
	// 8245A40C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8245A410: 41820020  beq 0x8245a430
	if ctx.cr[0].eq {
	pc = 0x8245A430; continue 'dispatch;
	}
	// 8245A414: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8245A418: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245A41C: 394A79AC  addi r10, r10, 0x79ac
	ctx.r[10].s64 = ctx.r[10].s64 + 31148;
	// 8245A420: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245A424: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A428: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8245A42C: 48000008  b 0x8245a434
	pc = 0x8245A434; continue 'dispatch;
	// 8245A430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245A434: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245A43C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A448: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A44C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A458 size=108
    let mut pc: u32 = 0x8245A458;
    'dispatch: loop {
        match pc {
            0x8245A458 => {
    //   block [0x8245A458..0x8245A4C4)
	// 8245A458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245A464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A46C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245A470: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8245A474: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245A478: 4BE664C1  bl 0x822c0938
	ctx.lr = 0x8245A47C;
	sub_822C0938(ctx, base);
	// 8245A47C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8245A480: 41820020  beq 0x8245a4a0
	if ctx.cr[0].eq {
	pc = 0x8245A4A0; continue 'dispatch;
	}
	// 8245A484: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8245A488: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245A48C: 394A79BC  addi r10, r10, 0x79bc
	ctx.r[10].s64 = ctx.r[10].s64 + 31164;
	// 8245A490: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245A494: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A498: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8245A49C: 48000008  b 0x8245a4a4
	pc = 0x8245A4A4; continue 'dispatch;
	// 8245A4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245A4A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A4A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245A4AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A4B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A4B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A4B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A4BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A4C8 size=84
    let mut pc: u32 = 0x8245A4C8;
    'dispatch: loop {
        match pc {
            0x8245A4C8 => {
    //   block [0x8245A4C8..0x8245A51C)
	// 8245A4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A4D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A4D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A4D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245A4DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A4E0: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8245A4E4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8245A4E8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8245A4EC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A4F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245A4F4: 4BE69F6D  bl 0x822c4460
	ctx.lr = 0x8245A4F8;
	sub_822C4460(ctx, base);
	// 8245A4F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A4FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A500: 419A0008  beq cr6, 0x8245a508
	if ctx.cr[6].eq {
	pc = 0x8245A508; continue 'dispatch;
	}
	// 8245A504: 4BE6638D  bl 0x822c0890
	ctx.lr = 0x8245A508;
	sub_822C0890(ctx, base);
	// 8245A508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245A50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A520 size=104
    let mut pc: u32 = 0x8245A520;
    'dispatch: loop {
        match pc {
            0x8245A520 => {
    //   block [0x8245A520..0x8245A588)
	// 8245A520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245A52C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A538: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 8245A53C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 8245A540: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8245A544: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8245A548: 38870004  addi r4, r7, 4
	ctx.r[4].s64 = ctx.r[7].s64 + 4;
	// 8245A54C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8245A550: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8245A554: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A558: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8245A55C: 4BEB930D  bl 0x82313868
	ctx.lr = 0x8245A560;
	sub_82313868(ctx, base);
	// 8245A560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245A564: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245A568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A56C: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8245A570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A57C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A588 size=96
    let mut pc: u32 = 0x8245A588;
    'dispatch: loop {
        match pc {
            0x8245A588 => {
    //   block [0x8245A588..0x8245A5E8)
	// 8245A588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A594: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A598: 4BFFFD31  bl 0x8245a2c8
	ctx.lr = 0x8245A59C;
	sub_8245A2C8(ctx, base);
	// 8245A59C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245A5A0: 40820030  bne 0x8245a5d0
	if !ctx.cr[0].eq {
	pc = 0x8245A5D0; continue 'dispatch;
	}
	// 8245A5A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A5A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8245A5AC: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 8245A5B0: 394A1014  addi r10, r10, 0x1014
	ctx.r[10].s64 = ctx.r[10].s64 + 4116;
	// 8245A5B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245A5B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A5BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8245A5C0: 4BE65A41  bl 0x822c0000
	ctx.lr = 0x8245A5C4;
	sub_822C0000(ctx, base);
	// 8245A5C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245A5C8: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 8245A5CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245A5D0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A5D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A5E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A5E8 size=100
    let mut pc: u32 = 0x8245A5E8;
    'dispatch: loop {
        match pc {
            0x8245A5E8 => {
    //   block [0x8245A5E8..0x8245A64C)
	// 8245A5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A5F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A5F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A5F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A5FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A600: 4BFFFDE9  bl 0x8245a3e8
	ctx.lr = 0x8245A604;
	sub_8245A3E8(ctx, base);
	// 8245A604: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A608: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A60C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A610: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245A614: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A618: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245A61C: 419A0018  beq cr6, 0x8245a634
	if ctx.cr[6].eq {
	pc = 0x8245A634; continue 'dispatch;
	}
	// 8245A620: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A624: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245A628: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A62C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245A630: 4E800421  bctrl
	ctx.lr = 0x8245A634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245A634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A638: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A650 size=96
    let mut pc: u32 = 0x8245A650;
    'dispatch: loop {
        match pc {
            0x8245A650 => {
    //   block [0x8245A650..0x8245A6B0)
	// 8245A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A65C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A660: 4BFFFCE9  bl 0x8245a348
	ctx.lr = 0x8245A664;
	sub_8245A348(ctx, base);
	// 8245A664: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245A668: 40820030  bne 0x8245a698
	if !ctx.cr[0].eq {
	pc = 0x8245A698; continue 'dispatch;
	}
	// 8245A66C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A670: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8245A674: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 8245A678: 394A1014  addi r10, r10, 0x1014
	ctx.r[10].s64 = ctx.r[10].s64 + 4116;
	// 8245A67C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245A680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A684: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8245A688: 4BE65979  bl 0x822c0000
	ctx.lr = 0x8245A68C;
	sub_822C0000(ctx, base);
	// 8245A68C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245A690: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 8245A694: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245A698: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A69C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A6A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A6B0 size=100
    let mut pc: u32 = 0x8245A6B0;
    'dispatch: loop {
        match pc {
            0x8245A6B0 => {
    //   block [0x8245A6B0..0x8245A714)
	// 8245A6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A6B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A6BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A6C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A6C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A6C8: 4BFFFD91  bl 0x8245a458
	ctx.lr = 0x8245A6CC;
	sub_8245A458(ctx, base);
	// 8245A6CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A6D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A6D4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A6D8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245A6DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245A6E0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245A6E4: 419A0018  beq cr6, 0x8245a6fc
	if ctx.cr[6].eq {
	pc = 0x8245A6FC; continue 'dispatch;
	}
	// 8245A6E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A6EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245A6F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A6F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245A6F8: 4E800421  bctrl
	ctx.lr = 0x8245A6FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245A6FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245A700: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A70C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A718 size=96
    let mut pc: u32 = 0x8245A718;
    'dispatch: loop {
        match pc {
            0x8245A718 => {
    //   block [0x8245A718..0x8245A778)
	// 8245A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245A724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A72C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245A730: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 8245A734: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8245A738: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8245A73C: 419A0024  beq cr6, 0x8245a760
	if ctx.cr[6].eq {
	pc = 0x8245A760; continue 'dispatch;
	}
	// 8245A740: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A744: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8245A748: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8245A74C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A750: 4808E941  bl 0x824e9090
	ctx.lr = 0x8245A754;
	sub_824E9090(ctx, base);
	// 8245A754: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8245A758: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8245A75C: 409AFFE4  bne cr6, 0x8245a740
	if !ctx.cr[6].eq {
	pc = 0x8245A740; continue 'dispatch;
	}
	// 8245A760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A76C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A778 size=84
    let mut pc: u32 = 0x8245A778;
    'dispatch: loop {
        match pc {
            0x8245A778 => {
    //   block [0x8245A778..0x8245A7CC)
	// 8245A778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A78C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8245A790: 4BE661A9  bl 0x822c0938
	ctx.lr = 0x8245A794;
	sub_822C0938(ctx, base);
	// 8245A794: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8245A798: 4182001C  beq 0x8245a7b4
	if ctx.cr[0].eq {
	pc = 0x8245A7B4; continue 'dispatch;
	}
	// 8245A79C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8245A7A0: 396B79AC  addi r11, r11, 0x79ac
	ctx.r[11].s64 = ctx.r[11].s64 + 31148;
	// 8245A7A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A7A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A7AC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245A7B0: 48000008  b 0x8245a7b8
	pc = 0x8245A7B8; continue 'dispatch;
	// 8245A7B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245A7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245A7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A7D0 size=84
    let mut pc: u32 = 0x8245A7D0;
    'dispatch: loop {
        match pc {
            0x8245A7D0 => {
    //   block [0x8245A7D0..0x8245A824)
	// 8245A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A7E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245A7E4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8245A7E8: 4BE66151  bl 0x822c0938
	ctx.lr = 0x8245A7EC;
	sub_822C0938(ctx, base);
	// 8245A7EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8245A7F0: 4182001C  beq 0x8245a80c
	if ctx.cr[0].eq {
	pc = 0x8245A80C; continue 'dispatch;
	}
	// 8245A7F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8245A7F8: 396B79BC  addi r11, r11, 0x79bc
	ctx.r[11].s64 = ctx.r[11].s64 + 31164;
	// 8245A7FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245A800: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A804: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245A808: 48000008  b 0x8245a810
	pc = 0x8245A810; continue 'dispatch;
	// 8245A80C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245A810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245A814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A81C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245A828 size=96
    let mut pc: u32 = 0x8245A828;
    'dispatch: loop {
        match pc {
            0x8245A828 => {
    //   block [0x8245A828..0x8245A888)
	// 8245A828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A834: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A838: 4BEE3C19  bl 0x8233e450
	ctx.lr = 0x8245A83C;
	sub_8233E450(ctx, base);
	// 8245A83C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245A840: 40820030  bne 0x8245a870
	if !ctx.cr[0].eq {
	pc = 0x8245A870; continue 'dispatch;
	}
	// 8245A844: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A848: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8245A84C: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 8245A850: 394A1014  addi r10, r10, 0x1014
	ctx.r[10].s64 = ctx.r[10].s64 + 4116;
	// 8245A854: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245A858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A85C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8245A860: 4BE657A1  bl 0x822c0000
	ctx.lr = 0x8245A864;
	sub_822C0000(ctx, base);
	// 8245A864: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245A868: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 8245A86C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245A870: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8245A874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A888 size=96
    let mut pc: u32 = 0x8245A888;
    'dispatch: loop {
        match pc {
            0x8245A888 => {
    //   block [0x8245A888..0x8245A8E8)
	// 8245A888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A894: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A898: 4BED9699  bl 0x82333f30
	ctx.lr = 0x8245A89C;
	sub_82333F30(ctx, base);
	// 8245A89C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245A8A0: 40820030  bne 0x8245a8d0
	if !ctx.cr[0].eq {
	pc = 0x8245A8D0; continue 'dispatch;
	}
	// 8245A8A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245A8A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8245A8AC: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 8245A8B0: 394A1014  addi r10, r10, 0x1014
	ctx.r[10].s64 = ctx.r[10].s64 + 4116;
	// 8245A8B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245A8B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A8BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8245A8C0: 4BE65741  bl 0x822c0000
	ctx.lr = 0x8245A8C4;
	sub_822C0000(ctx, base);
	// 8245A8C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245A8C8: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 8245A8CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245A8D0: 887F0000  lbz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A8D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245A8E8 size=124
    let mut pc: u32 = 0x8245A8E8;
    'dispatch: loop {
        match pc {
            0x8245A8E8 => {
    //   block [0x8245A8E8..0x8245A964)
	// 8245A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245A8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A8FC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A900: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8245A904: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245A908: 48000014  b 0x8245a91c
	pc = 0x8245A91C; continue 'dispatch;
	// 8245A90C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8245A910: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245A914: 419A0038  beq cr6, 0x8245a94c
	if ctx.cr[6].eq {
	pc = 0x8245A94C; continue 'dispatch;
	}
	// 8245A918: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A91C: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245A920: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245A924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A928: 480C1881  bl 0x8251c1a8
	ctx.lr = 0x8245A92C;
	sub_8251C1A8(ctx, base);
	// 8245A92C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245A930: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A934: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245A938: 419AFFD4  beq cr6, 0x8245a90c
	if ctx.cr[6].eq {
	pc = 0x8245A90C; continue 'dispatch;
	}
	// 8245A93C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8245A940: 4BEE3B11  bl 0x8233e450
	ctx.lr = 0x8245A944;
	sub_8233E450(ctx, base);
	// 8245A944: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245A948: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8245A94C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A958: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245A95C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A968 size=92
    let mut pc: u32 = 0x8245A968;
    'dispatch: loop {
        match pc {
            0x8245A968 => {
    //   block [0x8245A968..0x8245A9C4)
	// 8245A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A974: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A978: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245A97C: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245A980: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A984: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A988: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245A98C: 480C181D  bl 0x8251c1a8
	ctx.lr = 0x8245A990;
	sub_8251C1A8(ctx, base);
	// 8245A990: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A994: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245A998: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245A99C: 419A0010  beq cr6, 0x8245a9ac
	if ctx.cr[6].eq {
	pc = 0x8245A9AC; continue 'dispatch;
	}
	// 8245A9A0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8245A9A4: 4BEE3AAD  bl 0x8233e450
	ctx.lr = 0x8245A9A8;
	sub_8233E450(ctx, base);
	// 8245A9A8: 48000008  b 0x8245a9b0
	pc = 0x8245A9B0; continue 'dispatch;
	// 8245A9AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245A9B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245A9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245A9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245A9BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245A9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245A9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245A9C8 size=92
    let mut pc: u32 = 0x8245A9C8;
    'dispatch: loop {
        match pc {
            0x8245A9C8 => {
    //   block [0x8245A9C8..0x8245AA24)
	// 8245A9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245A9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245A9D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245A9D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245A9D8: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245A9DC: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245A9E0: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245A9E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245A9E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245A9EC: 480C17BD  bl 0x8251c1a8
	ctx.lr = 0x8245A9F0;
	sub_8251C1A8(ctx, base);
	// 8245A9F0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245A9F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245A9F8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245A9FC: 419A0010  beq cr6, 0x8245aa0c
	if ctx.cr[6].eq {
	pc = 0x8245AA0C; continue 'dispatch;
	}
	// 8245AA00: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8245AA04: 4BFFF8C5  bl 0x8245a2c8
	ctx.lr = 0x8245AA08;
	sub_8245A2C8(ctx, base);
	// 8245AA08: 48000008  b 0x8245aa10
	pc = 0x8245AA10; continue 'dispatch;
	// 8245AA0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245AA10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AA1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245AA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AA28 size=92
    let mut pc: u32 = 0x8245AA28;
    'dispatch: loop {
        match pc {
            0x8245AA28 => {
    //   block [0x8245AA28..0x8245AA84)
	// 8245AA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AA30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245AA34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AA38: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245AA3C: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245AA40: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AA44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245AA48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245AA4C: 480C175D  bl 0x8251c1a8
	ctx.lr = 0x8245AA50;
	sub_8251C1A8(ctx, base);
	// 8245AA50: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245AA54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AA58: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245AA5C: 419A0010  beq cr6, 0x8245aa6c
	if ctx.cr[6].eq {
	pc = 0x8245AA6C; continue 'dispatch;
	}
	// 8245AA60: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8245AA64: 4BED944D  bl 0x82333eb0
	ctx.lr = 0x8245AA68;
	sub_82333EB0(ctx, base);
	// 8245AA68: 48000008  b 0x8245aa70
	pc = 0x8245AA70; continue 'dispatch;
	// 8245AA6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245AA70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AA7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245AA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AA88 size=92
    let mut pc: u32 = 0x8245AA88;
    'dispatch: loop {
        match pc {
            0x8245AA88 => {
    //   block [0x8245AA88..0x8245AAE4)
	// 8245AA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AA90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245AA94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AA98: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245AA9C: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245AAA0: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AAA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245AAA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245AAAC: 480C16FD  bl 0x8251c1a8
	ctx.lr = 0x8245AAB0;
	sub_8251C1A8(ctx, base);
	// 8245AAB0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245AAB4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AAB8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245AABC: 419A0010  beq cr6, 0x8245aacc
	if ctx.cr[6].eq {
	pc = 0x8245AACC; continue 'dispatch;
	}
	// 8245AAC0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8245AAC4: 4BFFF885  bl 0x8245a348
	ctx.lr = 0x8245AAC8;
	sub_8245A348(ctx, base);
	// 8245AAC8: 48000008  b 0x8245aad0
	pc = 0x8245AAD0; continue 'dispatch;
	// 8245AACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245AAD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245AAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AAE8 size=92
    let mut pc: u32 = 0x8245AAE8;
    'dispatch: loop {
        match pc {
            0x8245AAE8 => {
    //   block [0x8245AAE8..0x8245AB44)
	// 8245AAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AAF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245AAF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AAF8: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245AAFC: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245AB00: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AB04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245AB08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245AB0C: 480C169D  bl 0x8251c1a8
	ctx.lr = 0x8245AB10;
	sub_8251C1A8(ctx, base);
	// 8245AB10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245AB14: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AB18: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245AB1C: 419A0010  beq cr6, 0x8245ab2c
	if ctx.cr[6].eq {
	pc = 0x8245AB2C; continue 'dispatch;
	}
	// 8245AB20: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8245AB24: 4BED940D  bl 0x82333f30
	ctx.lr = 0x8245AB28;
	sub_82333F30(ctx, base);
	// 8245AB28: 48000008  b 0x8245ab30
	pc = 0x8245AB30; continue 'dispatch;
	// 8245AB2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245AB30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AB3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245AB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AB48 size=104
    let mut pc: u32 = 0x8245AB48;
    'dispatch: loop {
        match pc {
            0x8245AB48 => {
    //   block [0x8245AB48..0x8245ABB0)
	// 8245AB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AB50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245AB54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245AB58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AB5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245AB60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8245AB64: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8245AB68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245AB6C: 419A0018  beq cr6, 0x8245ab84
	if ctx.cr[6].eq {
	pc = 0x8245AB84; continue 'dispatch;
	}
	// 8245AB70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AB74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245AB78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245AB80: 4E800421  bctrl
	ctx.lr = 0x8245AB84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245AB84: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AB88: 4182000C  beq 0x8245ab94
	if ctx.cr[0].eq {
	pc = 0x8245AB94; continue 'dispatch;
	}
	// 8245AB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245AB90: 4BE656D9  bl 0x822c0268
	ctx.lr = 0x8245AB94;
	sub_822C0268(ctx, base);
	// 8245AB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245AB98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245ABA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245ABA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245ABA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245ABAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245ABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245ABB0 size=108
    let mut pc: u32 = 0x8245ABB0;
    'dispatch: loop {
        match pc {
            0x8245ABB0 => {
    //   block [0x8245ABB0..0x8245AC1C)
	// 8245ABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245ABB4: 48D4D5AD  bl 0x831a8160
	ctx.lr = 0x8245ABB8;
	sub_831A8130(ctx, base);
	// 8245ABB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245ABBC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245ABC0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8245ABC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8245ABC8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8245ABCC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8245ABD0: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8245ABD4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8245ABD8: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8245ABDC: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8245ABE0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8245ABE4: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8245ABE8: 489974E1  bl 0x82df20c8
	ctx.lr = 0x8245ABEC;
	sub_82DF20C8(ctx, base);
	// 8245ABEC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245ABF0: 41820020  beq 0x8245ac10
	if ctx.cr[0].eq {
	pc = 0x8245AC10; continue 'dispatch;
	}
	// 8245ABF4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 8245ABF8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8245ABFC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8245AC00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8245AC04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8245AC08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245AC0C: 4BFFF915  bl 0x8245a520
	ctx.lr = 0x8245AC10;
	sub_8245A520(ctx, base);
	// 8245AC10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245AC14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8245AC18: 48D4D598  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AC20 size=88
    let mut pc: u32 = 0x8245AC20;
    'dispatch: loop {
        match pc {
            0x8245AC20 => {
    //   block [0x8245AC20..0x8245AC78)
	// 8245AC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AC28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245AC2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AC30: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8245AC34: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8245AC38: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AC3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245AC40: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8245AC44: 480C1565  bl 0x8251c1a8
	ctx.lr = 0x8245AC48;
	sub_8251C1A8(ctx, base);
	// 8245AC48: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8245AC4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AC50: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245AC54: 419A000C  beq cr6, 0x8245ac60
	if ctx.cr[6].eq {
	pc = 0x8245AC60; continue 'dispatch;
	}
	// 8245AC58: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8245AC5C: 48000008  b 0x8245ac64
	pc = 0x8245AC64; continue 'dispatch;
	// 8245AC60: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8245AC64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AC70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245AC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AC78 size=144
    let mut pc: u32 = 0x8245AC78;
    'dispatch: loop {
        match pc {
            0x8245AC78 => {
    //   block [0x8245AC78..0x8245AD08)
	// 8245AC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AC7C: 48D4D4E5  bl 0x831a8160
	ctx.lr = 0x8245AC80;
	sub_831A8130(ctx, base);
	// 8245AC80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AC84: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8245AC88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245AC8C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8245AC90: 936100BC  stw r27, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[27].u32 ) };
	// 8245AC94: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8245AC98: 48000010  b 0x8245aca8
	pc = 0x8245ACA8; continue 'dispatch;
	// 8245AC9C: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8245ACA0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8245ACA4: 419A0058  beq cr6, 0x8245acfc
	if ctx.cr[6].eq {
	pc = 0x8245ACFC; continue 'dispatch;
	}
	// 8245ACA8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245ACAC: 38A100BC  addi r5, r1, 0xbc
	ctx.r[5].s64 = ctx.r[1].s64 + 188;
	// 8245ACB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245ACB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245ACB8: 480C14F1  bl 0x8251c1a8
	ctx.lr = 0x8245ACBC;
	sub_8251C1A8(ctx, base);
	// 8245ACBC: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245ACC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245ACC4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245ACC8: 419AFFD4  beq cr6, 0x8245ac9c
	if ctx.cr[6].eq {
	pc = 0x8245AC9C; continue 'dispatch;
	}
	// 8245ACCC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8245ACD0: 419A0014  beq cr6, 0x8245ace4
	if ctx.cr[6].eq {
	pc = 0x8245ACE4; continue 'dispatch;
	}
	// 8245ACD4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8245ACD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245ACDC: 4BFFFF45  bl 0x8245ac20
	ctx.lr = 0x8245ACE0;
	sub_8245AC20(ctx, base);
	// 8245ACE0: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8245ACE4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8245ACE8: 419A000C  beq cr6, 0x8245acf4
	if ctx.cr[6].eq {
	pc = 0x8245ACF4; continue 'dispatch;
	}
	// 8245ACEC: 397D0010  addi r11, r29, 0x10
	ctx.r[11].s64 = ctx.r[29].s64 + 16;
	// 8245ACF0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245ACF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8245ACF8: 48000008  b 0x8245ad00
	pc = 0x8245AD00; continue 'dispatch;
	// 8245ACFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245AD00: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8245AD04: 48D4D4AC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245AD08 size=148
    let mut pc: u32 = 0x8245AD08;
    'dispatch: loop {
        match pc {
            0x8245AD08 => {
    //   block [0x8245AD08..0x8245AD9C)
	// 8245AD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AD10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245AD14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245AD18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AD1C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8245AD20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8245AD24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245AD28: 4BFFFF51  bl 0x8245ac78
	ctx.lr = 0x8245AD2C;
	sub_8245AC78(ctx, base);
	// 8245AD2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AD30: 41820050  beq 0x8245ad80
	if ctx.cr[0].eq {
	pc = 0x8245AD80; continue 'dispatch;
	}
	// 8245AD34: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AD38: 2F06FFFF  cmpwi cr6, r6, -1
	ctx.cr[6].compare_i32(ctx.r[6].s32, -1, &mut ctx.xer);
	// 8245AD3C: 409A0018  bne cr6, 0x8245ad54
	if !ctx.cr[6].eq {
	pc = 0x8245AD54; continue 'dispatch;
	}
	// 8245AD40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245AD44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8245AD48: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245AD4C: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8245AD50: 48000034  b 0x8245ad84
	pc = 0x8245AD84; continue 'dispatch;
	// 8245AD54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245AD58: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8245AD5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245AD60: 4BFFFF19  bl 0x8245ac78
	ctx.lr = 0x8245AD64;
	sub_8245AC78(ctx, base);
	// 8245AD64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AD68: 4182FFD8  beq 0x8245ad40
	if ctx.cr[0].eq {
	pc = 0x8245AD40; continue 'dispatch;
	}
	// 8245AD6C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8245AD70: 4BFFFAB9  bl 0x8245a828
	ctx.lr = 0x8245AD74;
	sub_8245A828(ctx, base);
	// 8245AD74: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8245AD78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8245AD7C: 48000008  b 0x8245ad84
	pc = 0x8245AD84; continue 'dispatch;
	// 8245AD80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245AD84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AD90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245AD94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245AD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245ADA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245ADA0 size=68
    let mut pc: u32 = 0x8245ADA0;
    'dispatch: loop {
        match pc {
            0x8245ADA0 => {
    //   block [0x8245ADA0..0x8245ADE4)
	// 8245ADA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245ADA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245ADA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245ADAC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245ADB0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8245ADB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245ADB8: 4BFFFF51  bl 0x8245ad08
	ctx.lr = 0x8245ADBC;
	sub_8245AD08(ctx, base);
	// 8245ADBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245ADC0: 41820010  beq 0x8245add0
	if ctx.cr[0].eq {
	pc = 0x8245ADD0; continue 'dispatch;
	}
	// 8245ADC4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245ADC8: 4BFFFAC1  bl 0x8245a888
	ctx.lr = 0x8245ADCC;
	sub_8245A888(ctx, base);
	// 8245ADCC: 48000008  b 0x8245add4
	pc = 0x8245ADD4; continue 'dispatch;
	// 8245ADD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8245ADD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245ADD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245ADDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245ADE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245ADE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245ADE8 size=80
    let mut pc: u32 = 0x8245ADE8;
    'dispatch: loop {
        match pc {
            0x8245ADE8 => {
    //   block [0x8245ADE8..0x8245AE38)
	// 8245ADE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245ADEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245ADF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245ADF4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245ADF8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8245ADFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245AE00: 4BFFFF09  bl 0x8245ad08
	ctx.lr = 0x8245AE04;
	sub_8245AD08(ctx, base);
	// 8245AE04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AE08: 41820018  beq 0x8245ae20
	if ctx.cr[0].eq {
	pc = 0x8245AE20; continue 'dispatch;
	}
	// 8245AE0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AE10: 4BFFFA19  bl 0x8245a828
	ctx.lr = 0x8245AE14;
	sub_8245A828(ctx, base);
	// 8245AE14: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245AE18: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8245AE1C: 4800000C  b 0x8245ae28
	pc = 0x8245AE28; continue 'dispatch;
	// 8245AE20: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245AE24: C02B6244  lfs f1, 0x6244(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25156 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8245AE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245AE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245AE38 size=112
    let mut pc: u32 = 0x8245AE38;
    'dispatch: loop {
        match pc {
            0x8245AE38 => {
    //   block [0x8245AE38..0x8245AEA8)
	// 8245AE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AE40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AE44: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245AE48: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8245AE4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245AE50: 4BFFFEB9  bl 0x8245ad08
	ctx.lr = 0x8245AE54;
	sub_8245AD08(ctx, base);
	// 8245AE54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AE58: 41820038  beq 0x8245ae90
	if ctx.cr[0].eq {
	pc = 0x8245AE90; continue 'dispatch;
	}
	// 8245AE5C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AE60: 4BFFF729  bl 0x8245a588
	ctx.lr = 0x8245AE64;
	sub_8245A588(ctx, base);
	// 8245AE64: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8245AE68: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245AE6C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8245AE70: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8245AE74: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8245AE78: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8245AE7C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8245AE80: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8245AE84: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8245AE88: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8245AE8C: 4800000C  b 0x8245ae98
	pc = 0x8245AE98; continue 'dispatch;
	// 8245AE90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245AE94: 806B6248  lwz r3, 0x6248(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25160 as u32) ) } as u64;
	// 8245AE98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245AEA8 size=112
    let mut pc: u32 = 0x8245AEA8;
    'dispatch: loop {
        match pc {
            0x8245AEA8 => {
    //   block [0x8245AEA8..0x8245AF18)
	// 8245AEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AEB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AEB4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245AEB8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8245AEBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245AEC0: 4BFFFE49  bl 0x8245ad08
	ctx.lr = 0x8245AEC4;
	sub_8245AD08(ctx, base);
	// 8245AEC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AEC8: 41820038  beq 0x8245af00
	if ctx.cr[0].eq {
	pc = 0x8245AF00; continue 'dispatch;
	}
	// 8245AECC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AED0: 4BED91F9  bl 0x823340c8
	ctx.lr = 0x8245AED4;
	sub_823340C8(ctx, base);
	// 8245AED4: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 8245AED8: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245AEDC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8245AEE0: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8245AEE4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8245AEE8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8245AEEC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8245AEF0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8245AEF4: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8245AEF8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8245AEFC: 4800000C  b 0x8245af08
	pc = 0x8245AF08; continue 'dispatch;
	// 8245AF00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245AF04: 806B624C  lwz r3, 0x624c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25164 as u32) ) } as u64;
	// 8245AF08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245AF18 size=112
    let mut pc: u32 = 0x8245AF18;
    'dispatch: loop {
        match pc {
            0x8245AF18 => {
    //   block [0x8245AF18..0x8245AF88)
	// 8245AF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245AF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AF24: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245AF28: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8245AF2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245AF30: 4BFFFDD9  bl 0x8245ad08
	ctx.lr = 0x8245AF34;
	sub_8245AD08(ctx, base);
	// 8245AF34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245AF38: 41820038  beq 0x8245af70
	if ctx.cr[0].eq {
	pc = 0x8245AF70; continue 'dispatch;
	}
	// 8245AF3C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245AF40: 4BFFF711  bl 0x8245a650
	ctx.lr = 0x8245AF44;
	sub_8245A650(ctx, base);
	// 8245AF44: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8245AF48: C0010058  lfs f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245AF4C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8245AF50: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8245AF54: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8245AF58: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8245AF5C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8245AF60: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8245AF64: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8245AF68: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8245AF6C: 4800000C  b 0x8245af78
	pc = 0x8245AF78; continue 'dispatch;
	// 8245AF70: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245AF74: 806B6250  lwz r3, 0x6250(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25168 as u32) ) } as u64;
	// 8245AF78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245AF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245AF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245AF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AF88 size=104
    let mut pc: u32 = 0x8245AF88;
    'dispatch: loop {
        match pc {
            0x8245AF88 => {
    //   block [0x8245AF88..0x8245AFF0)
	// 8245AF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AF8C: 48D4D1DD  bl 0x831a8168
	ctx.lr = 0x8245AF90;
	sub_831A8130(ctx, base);
	// 8245AF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AF94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8245AF98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8245AF9C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8245AFA0: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245AFA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245AFA8: 409A0040  bne cr6, 0x8245afe8
	if !ctx.cr[6].eq {
	pc = 0x8245AFE8; continue 'dispatch;
	}
	// 8245AFAC: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 8245AFB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8245AFB4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245AFB8: 4BFFFFD1  bl 0x8245af88
	ctx.lr = 0x8245AFBC;
	sub_8245AF88(ctx, base);
	// 8245AFBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8245AFC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245AFC4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245AFC8: 4BFFFB81  bl 0x8245ab48
	ctx.lr = 0x8245AFCC;
	sub_8245AB48(ctx, base);
	// 8245AFCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8245AFD0: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8245AFD4: 489971B5  bl 0x82df2188
	ctx.lr = 0x8245AFD8;
	sub_82DF2188(ctx, base);
	// 8245AFD8: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245AFDC: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8245AFE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245AFE4: 419AFFCC  beq cr6, 0x8245afb0
	if ctx.cr[6].eq {
	pc = 0x8245AFB0; continue 'dispatch;
	}
	// 8245AFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245AFEC: 48D4D1CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245AFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245AFF0 size=1028
    let mut pc: u32 = 0x8245AFF0;
    'dispatch: loop {
        match pc {
            0x8245AFF0 => {
    //   block [0x8245AFF0..0x8245B3F4)
	// 8245AFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245AFF4: 48D4D165  bl 0x831a8158
	ctx.lr = 0x8245AFF8;
	sub_831A8130(ctx, base);
	// 8245AFF8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245AFFC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8245B000: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8245B004: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8245B008: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8245B00C: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B014: 419A0048  beq cr6, 0x8245b05c
	if ctx.cr[6].eq {
	pc = 0x8245B05C; continue 'dispatch;
	}
	// 8245B018: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245B01C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B020: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8245B024: 4BE6A8A5  bl 0x822c58c8
	ctx.lr = 0x8245B028;
	sub_822C58C8(ctx, base);
	// 8245B028: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245B02C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8245B030: 4BE6EE81  bl 0x822c9eb0
	ctx.lr = 0x8245B034;
	sub_822C9EB0(ctx, base);
	// 8245B034: 4BE6927D  bl 0x822c42b0
	ctx.lr = 0x8245B038;
	sub_822C42B0(ctx, base);
	// 8245B038: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245B03C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8245B040: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8245B044: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8245B048: 4BE6A429  bl 0x822c5470
	ctx.lr = 0x8245B04C;
	sub_822C5470(ctx, base);
	// 8245B04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245B050: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245B054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B058: 4BE69C89  bl 0x822c4ce0
	ctx.lr = 0x8245B05C;
	sub_822C4CE0(ctx, base);
	// 8245B05C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8245B060: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8245B064: 4808E02D  bl 0x824e9090
	ctx.lr = 0x8245B068;
	sub_824E9090(ctx, base);
	// 8245B068: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B06C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B070: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B074: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8245B078: 419A000C  beq cr6, 0x8245b084
	if ctx.cr[6].eq {
	pc = 0x8245B084; continue 'dispatch;
	}
	// 8245B07C: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B080: 48000028  b 0x8245b0a8
	pc = 0x8245B0A8; continue 'dispatch;
	// 8245B084: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B088: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B08C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B090: 419A000C  beq cr6, 0x8245b09c
	if ctx.cr[6].eq {
	pc = 0x8245B09C; continue 'dispatch;
	}
	// 8245B094: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8245B098: 48000010  b 0x8245b0a8
	pc = 0x8245B0A8; continue 'dispatch;
	// 8245B09C: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B0A0: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B0A4: 409A00DC  bne cr6, 0x8245b180
	if !ctx.cr[6].eq {
	pc = 0x8245B180; continue 'dispatch;
	}
	// 8245B0A8: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B0AC: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B0B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B0B4: 409A0008  bne cr6, 0x8245b0bc
	if !ctx.cr[6].eq {
	pc = 0x8245B0BC; continue 'dispatch;
	}
	// 8245B0B8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8245B0BC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B0C0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B0C4: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B0C8: 409A000C  bne cr6, 0x8245b0d4
	if !ctx.cr[6].eq {
	pc = 0x8245B0D4; continue 'dispatch;
	}
	// 8245B0CC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8245B0D0: 4800001C  b 0x8245b0ec
	pc = 0x8245B0EC; continue 'dispatch;
	// 8245B0D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B0D8: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B0DC: 409A000C  bne cr6, 0x8245b0e8
	if !ctx.cr[6].eq {
	pc = 0x8245B0E8; continue 'dispatch;
	}
	// 8245B0E0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8245B0E4: 48000008  b 0x8245b0ec
	pc = 0x8245B0EC; continue 'dispatch;
	// 8245B0E8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8245B0EC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B0F0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B0F4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B0F8: 409A003C  bne cr6, 0x8245b134
	if !ctx.cr[6].eq {
	pc = 0x8245B134; continue 'dispatch;
	}
	// 8245B0FC: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B104: 419A000C  beq cr6, 0x8245b110
	if ctx.cr[6].eq {
	pc = 0x8245B110; continue 'dispatch;
	}
	// 8245B108: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8245B10C: 48000024  b 0x8245b130
	pc = 0x8245B130; continue 'dispatch;
	// 8245B110: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B114: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8245B118: 4800000C  b 0x8245b124
	pc = 0x8245B124; continue 'dispatch;
	// 8245B11C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8245B120: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B124: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B128: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8245B12C: 419AFFF0  beq cr6, 0x8245b11c
	if ctx.cr[6].eq {
	pc = 0x8245B11C; continue 'dispatch;
	}
	// 8245B130: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245B134: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B138: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B13C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B140: 409A00D4  bne cr6, 0x8245b214
	if !ctx.cr[6].eq {
	pc = 0x8245B214; continue 'dispatch;
	}
	// 8245B144: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B148: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B14C: 419A000C  beq cr6, 0x8245b158
	if ctx.cr[6].eq {
	pc = 0x8245B158; continue 'dispatch;
	}
	// 8245B150: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8245B154: 48000024  b 0x8245b178
	pc = 0x8245B178; continue 'dispatch;
	// 8245B158: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B15C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8245B160: 4800000C  b 0x8245b16c
	pc = 0x8245B16C; continue 'dispatch;
	// 8245B164: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8245B168: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B16C: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B170: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8245B174: 419AFFF0  beq cr6, 0x8245b164
	if ctx.cr[6].eq {
	pc = 0x8245B164; continue 'dispatch;
	}
	// 8245B178: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8245B17C: 48000098  b 0x8245b214
	pc = 0x8245B214; continue 'dispatch;
	// 8245B180: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8245B184: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B188: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245B18C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B190: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B194: 409A000C  bne cr6, 0x8245b1a0
	if !ctx.cr[6].eq {
	pc = 0x8245B1A0; continue 'dispatch;
	}
	// 8245B198: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8245B19C: 4800002C  b 0x8245b1c8
	pc = 0x8245B1C8; continue 'dispatch;
	// 8245B1A0: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B1A4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B1A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B1AC: 409A0008  bne cr6, 0x8245b1b4
	if !ctx.cr[6].eq {
	pc = 0x8245B1B4; continue 'dispatch;
	}
	// 8245B1B0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8245B1B4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8245B1B8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B1BC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245B1C0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B1C4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8245B1C8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B1CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B1D0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B1D4: 409A000C  bne cr6, 0x8245b1e0
	if !ctx.cr[6].eq {
	pc = 0x8245B1E0; continue 'dispatch;
	}
	// 8245B1D8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8245B1DC: 48000020  b 0x8245b1fc
	pc = 0x8245B1FC; continue 'dispatch;
	// 8245B1E0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B1E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B1E8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B1EC: 409A000C  bne cr6, 0x8245b1f8
	if !ctx.cr[6].eq {
	pc = 0x8245B1F8; continue 'dispatch;
	}
	// 8245B1F0: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8245B1F4: 48000008  b 0x8245b1fc
	pc = 0x8245B1FC; continue 'dispatch;
	// 8245B1F8: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8245B1FC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B200: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245B204: 897B0014  lbz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B208: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B20C: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8245B210: 995B0014  stb r10, 0x14(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8245B214: 897B0014  lbz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B218: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8245B21C: 409A0198  bne cr6, 0x8245b3b4
	if !ctx.cr[6].eq {
	pc = 0x8245B3B4; continue 'dispatch;
	}
	// 8245B220: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B224: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8245B228: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B22C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B230: 419A0180  beq cr6, 0x8245b3b0
	if ctx.cr[6].eq {
	pc = 0x8245B3B0; continue 'dispatch;
	}
	// 8245B234: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8245B238: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B23C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8245B240: 409A0170  bne cr6, 0x8245b3b0
	if !ctx.cr[6].eq {
	pc = 0x8245B3B0; continue 'dispatch;
	}
	// 8245B244: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B248: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B24C: 409A00A8  bne cr6, 0x8245b2f4
	if !ctx.cr[6].eq {
	pc = 0x8245B2F4; continue 'dispatch;
	}
	// 8245B250: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B254: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B258: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B25C: 409A001C  bne cr6, 0x8245b278
	if !ctx.cr[6].eq {
	pc = 0x8245B278; continue 'dispatch;
	}
	// 8245B260: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B264: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B268: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B26C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B270: 4875B961  bl 0x82bb6bd0
	ctx.lr = 0x8245B274;
	sub_82BB6BD0(ctx, base);
	// 8245B274: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B278: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B27C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B280: 409A00C8  bne cr6, 0x8245b348
	if !ctx.cr[6].eq {
	pc = 0x8245B348; continue 'dispatch;
	}
	// 8245B284: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B288: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B28C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8245B290: 409A0014  bne cr6, 0x8245b2a4
	if !ctx.cr[6].eq {
	pc = 0x8245B2A4; continue 'dispatch;
	}
	// 8245B294: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B298: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B29C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8245B2A0: 419A00A4  beq cr6, 0x8245b344
	if ctx.cr[6].eq {
	pc = 0x8245B344; continue 'dispatch;
	}
	// 8245B2A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B2A8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B2AC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8245B2B0: 409A0020  bne cr6, 0x8245b2d0
	if !ctx.cr[6].eq {
	pc = 0x8245B2D0; continue 'dispatch;
	}
	// 8245B2B4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B2B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8245B2BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B2C0: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B2C4: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B2C8: 4BE93249  bl 0x822ee510
	ctx.lr = 0x8245B2CC;
	sub_822EE510(ctx, base);
	// 8245B2CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B2D0: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B2D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B2D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B2DC: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8245B2E0: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B2E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B2E8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B2EC: 4875B8E5  bl 0x82bb6bd0
	ctx.lr = 0x8245B2F0;
	sub_82BB6BD0(ctx, base);
	// 8245B2F0: 480000C0  b 0x8245b3b0
	pc = 0x8245B3B0; continue 'dispatch;
	// 8245B2F4: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B2F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B2FC: 409A001C  bne cr6, 0x8245b318
	if !ctx.cr[6].eq {
	pc = 0x8245B318; continue 'dispatch;
	}
	// 8245B300: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B304: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B308: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B30C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B310: 4BE93201  bl 0x822ee510
	ctx.lr = 0x8245B314;
	sub_822EE510(ctx, base);
	// 8245B314: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B318: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B31C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B320: 409A0028  bne cr6, 0x8245b348
	if !ctx.cr[6].eq {
	pc = 0x8245B348; continue 'dispatch;
	}
	// 8245B324: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B328: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B32C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8245B330: 409A0034  bne cr6, 0x8245b364
	if !ctx.cr[6].eq {
	pc = 0x8245B364; continue 'dispatch;
	}
	// 8245B334: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B338: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B33C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8245B340: 409A0024  bne cr6, 0x8245b364
	if !ctx.cr[6].eq {
	pc = 0x8245B364; continue 'dispatch;
	}
	// 8245B344: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B348: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B34C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8245B350: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B354: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B358: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B35C: 409AFEDC  bne cr6, 0x8245b238
	if !ctx.cr[6].eq {
	pc = 0x8245B238; continue 'dispatch;
	}
	// 8245B360: 48000050  b 0x8245b3b0
	pc = 0x8245B3B0; continue 'dispatch;
	// 8245B364: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B368: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B36C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8245B370: 409A0020  bne cr6, 0x8245b390
	if !ctx.cr[6].eq {
	pc = 0x8245B390; continue 'dispatch;
	}
	// 8245B374: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B378: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8245B37C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B380: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B384: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B388: 4875B849  bl 0x82bb6bd0
	ctx.lr = 0x8245B38C;
	sub_82BB6BD0(ctx, base);
	// 8245B38C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B390: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B394: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B398: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B39C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8245B3A0: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B3A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B3A8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B3AC: 4BE93165  bl 0x822ee510
	ctx.lr = 0x8245B3B0;
	sub_822EE510(ctx, base);
	// 8245B3B0: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8245B3B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8245B3B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8245B3BC: 4BFFF78D  bl 0x8245ab48
	ctx.lr = 0x8245B3C0;
	sub_8245AB48(ctx, base);
	// 8245B3C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245B3C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8245B3C8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8245B3CC: 48996DBD  bl 0x82df2188
	ctx.lr = 0x8245B3D0;
	sub_82DF2188(ctx, base);
	// 8245B3D0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B3D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B3D8: 419A000C  beq cr6, 0x8245b3e4
	if ctx.cr[6].eq {
	pc = 0x8245B3E4; continue 'dispatch;
	}
	// 8245B3DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8245B3E0: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245B3E4: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8245B3E8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8245B3EC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8245B3F0: 48D4CDB8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245B3F8 size=84
    let mut pc: u32 = 0x8245B3F8;
    'dispatch: loop {
        match pc {
            0x8245B3F8 => {
    //   block [0x8245B3F8..0x8245B44C)
	// 8245B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245B400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245B404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B408: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245B40C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B410: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B414: 4BFFFB75  bl 0x8245af88
	ctx.lr = 0x8245B418;
	sub_8245AF88(ctx, base);
	// 8245B418: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B41C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245B420: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8245B424: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245B428: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B42C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8245B430: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B434: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245B438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245B43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245B440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245B444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245B448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245B450 size=548
    let mut pc: u32 = 0x8245B450;
    'dispatch: loop {
        match pc {
            0x8245B450 => {
    //   block [0x8245B450..0x8245B674)
	// 8245B450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B454: 48D4CD0D  bl 0x831a8160
	ctx.lr = 0x8245B458;
	sub_831A8130(ctx, base);
	// 8245B458: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B45C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8245B460: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 8245B464: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8245B468: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 8245B46C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8245B470: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B474: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8245B478: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8245B47C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B480: 41980048  blt cr6, 0x8245b4c8
	if ctx.cr[6].lt {
	pc = 0x8245B4C8; continue 'dispatch;
	}
	// 8245B484: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245B488: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B48C: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 8245B490: 4BE6A439  bl 0x822c58c8
	ctx.lr = 0x8245B494;
	sub_822C58C8(ctx, base);
	// 8245B494: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245B498: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8245B49C: 4BE6A37D  bl 0x822c5818
	ctx.lr = 0x8245B4A0;
	sub_822C5818(ctx, base);
	// 8245B4A0: 4BE68E11  bl 0x822c42b0
	ctx.lr = 0x8245B4A4;
	sub_822C42B0(ctx, base);
	// 8245B4A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8245B4A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8245B4AC: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 8245B4B0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8245B4B4: 4BE69FBD  bl 0x822c5470
	ctx.lr = 0x8245B4B8;
	sub_822C5470(ctx, base);
	// 8245B4B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245B4BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245B4C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B4C4: 4BE6981D  bl 0x822c4ce0
	ctx.lr = 0x8245B4C8;
	sub_822C4CE0(ctx, base);
	// 8245B4C8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8245B4D0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245B4D4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8245B4D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8245B4DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245B4E0: 4BFFF6D1  bl 0x8245abb0
	ctx.lr = 0x8245B4E4;
	sub_8245ABB0(ctx, base);
	// 8245B4E4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B4E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B4EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8245B4F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8245B4F4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B4F8: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8245B4FC: 409A0018  bne cr6, 0x8245b514
	if !ctx.cr[6].eq {
	pc = 0x8245B514; continue 'dispatch;
	}
	// 8245B500: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8245B504: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B508: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8245B50C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B510: 4800003C  b 0x8245b54c
	pc = 0x8245B54C; continue 'dispatch;
	// 8245B514: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245B518: 41820020  beq 0x8245b538
	if ctx.cr[0].eq {
	pc = 0x8245B538; continue 'dispatch;
	}
	// 8245B51C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8245B520: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B524: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B528: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245B52C: 409A0024  bne cr6, 0x8245b550
	if !ctx.cr[6].eq {
	pc = 0x8245B550; continue 'dispatch;
	}
	// 8245B530: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8245B534: 4800001C  b 0x8245b550
	pc = 0x8245B550; continue 'dispatch;
	// 8245B538: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8245B53C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B540: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B544: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245B548: 409A0008  bne cr6, 0x8245b550
	if !ctx.cr[6].eq {
	pc = 0x8245B550; continue 'dispatch;
	}
	// 8245B54C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8245B550: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B554: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8245B558: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8245B55C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8245B560: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B564: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B568: 409A00F0  bne cr6, 0x8245b658
	if !ctx.cr[6].eq {
	pc = 0x8245B658; continue 'dispatch;
	}
	// 8245B56C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8245B570: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B574: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B578: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B57C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245B580: 409A0054  bne cr6, 0x8245b5d4
	if !ctx.cr[6].eq {
	pc = 0x8245B5D4; continue 'dispatch;
	}
	// 8245B584: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B588: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B58C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8245B590: 419A0054  beq cr6, 0x8245b5e4
	if ctx.cr[6].eq {
	pc = 0x8245B5E4; continue 'dispatch;
	}
	// 8245B594: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B598: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B59C: 409A0010  bne cr6, 0x8245b5ac
	if !ctx.cr[6].eq {
	pc = 0x8245B5AC; continue 'dispatch;
	}
	// 8245B5A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245B5A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245B5A8: 4875B629  bl 0x82bb6bd0
	ctx.lr = 0x8245B5AC;
	sub_82BB6BD0(ctx, base);
	// 8245B5AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B5B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245B5B4: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B5B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B5BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B5C0: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 8245B5C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B5C8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B5CC: 4BE92F45  bl 0x822ee510
	ctx.lr = 0x8245B5D0;
	sub_822EE510(ctx, base);
	// 8245B5D0: 48000074  b 0x8245b644
	pc = 0x8245B644; continue 'dispatch;
	// 8245B5D4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B5D8: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B5DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8245B5E0: 409A0028  bne cr6, 0x8245b608
	if !ctx.cr[6].eq {
	pc = 0x8245B608; continue 'dispatch;
	}
	// 8245B5E4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B5E8: 9BA90014  stb r29, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B5EC: 9BAA0014  stb r29, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B5F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B5F4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B5F8: 9B6A0014  stb r27, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 8245B5FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B600: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B604: 48000040  b 0x8245b644
	pc = 0x8245B644; continue 'dispatch;
	// 8245B608: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B60C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B610: 409A0010  bne cr6, 0x8245b620
	if !ctx.cr[6].eq {
	pc = 0x8245B620; continue 'dispatch;
	}
	// 8245B614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245B618: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245B61C: 4BE92EF5  bl 0x822ee510
	ctx.lr = 0x8245B620;
	sub_822EE510(ctx, base);
	// 8245B620: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245B628: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B62C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B630: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B634: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 8245B638: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B63C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B640: 4875B591  bl 0x82bb6bd0
	ctx.lr = 0x8245B644;
	sub_82BB6BD0(ctx, base);
	// 8245B644: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B648: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8245B64C: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8245B650: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B654: 419AFF1C  beq cr6, 0x8245b570
	if ctx.cr[6].eq {
	pc = 0x8245B570; continue 'dispatch;
	}
	// 8245B658: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B65C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8245B660: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8245B664: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B668: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8245B66C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8245B670: 48D4CB40  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245B678 size=132
    let mut pc: u32 = 0x8245B678;
    'dispatch: loop {
        match pc {
            0x8245B678 => {
    //   block [0x8245B678..0x8245B6FC)
	// 8245B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B67C: 48D4CAED  bl 0x831a8168
	ctx.lr = 0x8245B680;
	sub_831A8130(ctx, base);
	// 8245B680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8245B688: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8245B68C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8245B690: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8245B694: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B698: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B69C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245B6A0: 409A0044  bne cr6, 0x8245b6e4
	if !ctx.cr[6].eq {
	pc = 0x8245B6E4; continue 'dispatch;
	}
	// 8245B6A4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B6A8: 409A003C  bne cr6, 0x8245b6e4
	if !ctx.cr[6].eq {
	pc = 0x8245B6E4; continue 'dispatch;
	}
	// 8245B6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245B6B0: 4BFFFD49  bl 0x8245b3f8
	ctx.lr = 0x8245B6B4;
	sub_8245B3F8(ctx, base);
	// 8245B6B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B6B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B6BC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245B6C0: 48000030  b 0x8245b6f0
	pc = 0x8245B6F0; continue 'dispatch;
	// 8245B6C4: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8245B6C8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8245B6CC: 4808D9C5  bl 0x824e9090
	ctx.lr = 0x8245B6D0;
	sub_824E9090(ctx, base);
	// 8245B6D0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8245B6D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B6D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B6DC: 4BFFF915  bl 0x8245aff0
	ctx.lr = 0x8245B6E0;
	sub_8245AFF0(ctx, base);
	// 8245B6E0: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8245B6E4: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8245B6E8: 409AFFDC  bne cr6, 0x8245b6c4
	if !ctx.cr[6].eq {
	pc = 0x8245B6C4; continue 'dispatch;
	}
	// 8245B6EC: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8245B6F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8245B6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245B6F8: 48D4CAC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245B700 size=88
    let mut pc: u32 = 0x8245B700;
    'dispatch: loop {
        match pc {
            0x8245B700 => {
    //   block [0x8245B700..0x8245B758)
	// 8245B700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245B708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245B70C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245B714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B718: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B71C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B720: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B724: 4BFFFF55  bl 0x8245b678
	ctx.lr = 0x8245B728;
	sub_8245B678(ctx, base);
	// 8245B728: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8245B72C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B730: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8245B734: 48996A55  bl 0x82df2188
	ctx.lr = 0x8245B738;
	sub_82DF2188(ctx, base);
	// 8245B738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245B73C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8245B740: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8245B744: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245B748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245B74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245B750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245B754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8245B758 size=236
    let mut pc: u32 = 0x8245B758;
    'dispatch: loop {
        match pc {
            0x8245B758 => {
    //   block [0x8245B758..0x8245B844)
	// 8245B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B75C: 48D4CA05  bl 0x831a8160
	ctx.lr = 0x8245B760;
	sub_831A8130(ctx, base);
	// 8245B760: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B764: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8245B768: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8245B76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245B770: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8245B774: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8245B778: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B77C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B780: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B784: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245B788: 409A0038  bne cr6, 0x8245b7c0
	if !ctx.cr[6].eq {
	pc = 0x8245B7C0; continue 'dispatch;
	}
	// 8245B78C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B790: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B794: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8245B798: 7D295010  subfc r9, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8245B79C: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8245B7A0: 553D07FF  clrlwi. r29, r9, 0x1f
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8245B7A4: 4182000C  beq 0x8245b7b0
	if ctx.cr[0].eq {
	pc = 0x8245B7B0; continue 'dispatch;
	}
	// 8245B7A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B7AC: 48000008  b 0x8245b7b4
	pc = 0x8245B7B4; continue 'dispatch;
	// 8245B7B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B7B4: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B7B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8245B7BC: 419AFFD4  beq cr6, 0x8245b790
	if ctx.cr[6].eq {
	pc = 0x8245B790; continue 'dispatch;
	}
	// 8245B7C0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8245B7C4: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8245B7C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245B7CC: 41820044  beq 0x8245b810
	if ctx.cr[0].eq {
	pc = 0x8245B810; continue 'dispatch;
	}
	// 8245B7D0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B7D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B7D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B7DC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B7E0: 409A0028  bne cr6, 0x8245b808
	if !ctx.cr[6].eq {
	pc = 0x8245B808; continue 'dispatch;
	}
	// 8245B7E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8245B7E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245B7EC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245B7F0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8245B7F4: 4BFFFC5D  bl 0x8245b450
	ctx.lr = 0x8245B7F8;
	sub_8245B450(ctx, base);
	// 8245B7F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245B7FC: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 8245B800: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B804: 48000030  b 0x8245b834
	pc = 0x8245B834; continue 'dispatch;
	// 8245B808: 4875B431  bl 0x82bb6c38
	ctx.lr = 0x8245B80C;
	sub_82BB6C38(ctx, base);
	// 8245B80C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245B810: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B814: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B818: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245B81C: 40980010  bge cr6, 0x8245b82c
	if !ctx.cr[6].lt {
	pc = 0x8245B82C; continue 'dispatch;
	}
	// 8245B820: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8245B824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B828: 4BFFFFC0  b 0x8245b7e8
	pc = 0x8245B7E8; continue 'dispatch;
	// 8245B82C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245B830: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 8245B834: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245B838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245B83C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8245B840: 48D4C970  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245B848 size=100
    let mut pc: u32 = 0x8245B848;
    'dispatch: loop {
        match pc {
            0x8245B848 => {
    //   block [0x8245B848..0x8245B8AC)
	// 8245B848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B84C: 48D4C921  bl 0x831a816c
	ctx.lr = 0x8245B850;
	sub_831A8130(ctx, base);
	// 8245B850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B854: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8245B858: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8245B85C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8245B860: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8245B864: 4845CC8D  bl 0x828b84f0
	ctx.lr = 0x8245B868;
	sub_828B84F0(ctx, base);
	// 8245B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245B86C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8245B870: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245B874: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8245B878: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8245B87C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8245B880: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245B884: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245B888: 4BFFEE91  bl 0x8245a718
	ctx.lr = 0x8245B88C;
	sub_8245A718(ctx, base);
	// 8245B88C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8245B890: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8245B894: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8245B898: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8245B89C: 4BFFFDDD  bl 0x8245b678
	ctx.lr = 0x8245B8A0;
	sub_8245B678(ctx, base);
	// 8245B8A0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8245B8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245B8A8: 48D4C914  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245B8B0 size=356
    let mut pc: u32 = 0x8245B8B0;
    'dispatch: loop {
        match pc {
            0x8245B8B0 => {
    //   block [0x8245B8B0..0x8245BA14)
	// 8245B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245B8B4: 48D4C8A9  bl 0x831a815c
	ctx.lr = 0x8245B8B8;
	sub_831A8130(ctx, base);
	// 8245B8B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245B8BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8245B8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245B8C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8245B8C8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8245B8CC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B8D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B8D4: 409A0020  bne cr6, 0x8245b8f4
	if !ctx.cr[6].eq {
	pc = 0x8245B8F4; continue 'dispatch;
	}
	// 8245B8D8: 80DC0004  lwz r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B8DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8245B8E0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245B8E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245B8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245B8EC: 4BFFFB65  bl 0x8245b450
	ctx.lr = 0x8245B8F0;
	sub_8245B450(ctx, base);
	// 8245B8F0: 48000118  b 0x8245ba08
	pc = 0x8245BA08; continue 'dispatch;
	// 8245B8F4: 833C0004  lwz r25, 4(r28)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245B8F8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B8FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B900: 409A001C  bne cr6, 0x8245b91c
	if !ctx.cr[6].eq {
	pc = 0x8245B91C; continue 'dispatch;
	}
	// 8245B904: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B908: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B90C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245B910: 409800DC  bge cr6, 0x8245b9ec
	if !ctx.cr[6].lt {
	pc = 0x8245B9EC; continue 'dispatch;
	}
	// 8245B914: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245B918: 4BFFFFC4  b 0x8245b8dc
	pc = 0x8245B8DC; continue 'dispatch;
	// 8245B91C: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 8245B920: 409A0020  bne cr6, 0x8245b940
	if !ctx.cr[6].eq {
	pc = 0x8245B940; continue 'dispatch;
	}
	// 8245B924: 80D90008  lwz r6, 8(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B928: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B92C: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B930: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B934: 409800B8  bge cr6, 0x8245b9ec
	if !ctx.cr[6].lt {
	pc = 0x8245B9EC; continue 'dispatch;
	}
	// 8245B938: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245B93C: 4BFFFFA4  b 0x8245b8e0
	pc = 0x8245B8E0; continue 'dispatch;
	// 8245B940: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245B944: 835E000C  lwz r26, 0xc(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B948: 7F1AD840  cmplw cr6, r26, r27
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B94C: 40990054  ble cr6, 0x8245b9a0
	if !ctx.cr[6].gt {
	pc = 0x8245B9A0; continue 'dispatch;
	}
	// 8245B950: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8245B954: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B958: 4875B2E1  bl 0x82bb6c38
	ctx.lr = 0x8245B95C;
	sub_82BB6C38(ctx, base);
	// 8245B95C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245B960: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B964: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B968: 40980034  bge cr6, 0x8245b99c
	if !ctx.cr[6].lt {
	pc = 0x8245B99C; continue 'dispatch;
	}
	// 8245B96C: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B970: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245B974: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245B978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245B97C: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B984: 419A000C  beq cr6, 0x8245b990
	if ctx.cr[6].eq {
	pc = 0x8245B990; continue 'dispatch;
	}
	// 8245B988: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245B98C: 4BFFFF60  b 0x8245b8ec
	pc = 0x8245B8EC; continue 'dispatch;
	// 8245B990: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245B994: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8245B998: 4BFFFF54  b 0x8245b8ec
	pc = 0x8245B8EC; continue 'dispatch;
	// 8245B99C: 7F1AD840  cmplw cr6, r26, r27
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245B9A0: 4098004C  bge cr6, 0x8245b9ec
	if !ctx.cr[6].lt {
	pc = 0x8245B9EC; continue 'dispatch;
	}
	// 8245B9A4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8245B9A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B9AC: 4808D6E5  bl 0x824e9090
	ctx.lr = 0x8245B9B0;
	sub_824E9090(ctx, base);
	// 8245B9B0: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245B9B4: 7F06C840  cmplw cr6, r6, r25
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[25].u32, &mut ctx.xer);
	// 8245B9B8: 419A0010  beq cr6, 0x8245b9c8
	if ctx.cr[6].eq {
	pc = 0x8245B9C8; continue 'dispatch;
	}
	// 8245B9BC: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245B9C0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245B9C4: 40980028  bge cr6, 0x8245b9ec
	if !ctx.cr[6].lt {
	pc = 0x8245B9EC; continue 'dispatch;
	}
	// 8245B9C8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245B9CC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245B9D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245B9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245B9D8: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245B9DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245B9E0: 419AFFB4  beq cr6, 0x8245b994
	if ctx.cr[6].eq {
	pc = 0x8245B994; continue 'dispatch;
	}
	// 8245B9E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245B9E8: 4BFFFFA0  b 0x8245b988
	pc = 0x8245B988; continue 'dispatch;
	// 8245B9EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8245B9F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245B9F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245B9F8: 4BFFFD61  bl 0x8245b758
	ctx.lr = 0x8245B9FC;
	sub_8245B758(ctx, base);
	// 8245B9FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245BA00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BA04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245BA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BA0C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8245BA10: 48D4C79C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BA18 size=356
    let mut pc: u32 = 0x8245BA18;
    'dispatch: loop {
        match pc {
            0x8245BA18 => {
    //   block [0x8245BA18..0x8245BB7C)
	// 8245BA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BA1C: 48D4C741  bl 0x831a815c
	ctx.lr = 0x8245BA20;
	sub_831A8130(ctx, base);
	// 8245BA20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BA24: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8245BA28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245BA2C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8245BA30: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8245BA34: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245BA38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245BA3C: 409A0020  bne cr6, 0x8245ba5c
	if !ctx.cr[6].eq {
	pc = 0x8245BA5C; continue 'dispatch;
	}
	// 8245BA40: 80DC0004  lwz r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BA44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8245BA48: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245BA4C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245BA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BA54: 480C0A15  bl 0x8251c468
	ctx.lr = 0x8245BA58;
	sub_8251C468(ctx, base);
	// 8245BA58: 48000118  b 0x8245bb70
	pc = 0x8245BB70; continue 'dispatch;
	// 8245BA5C: 833C0004  lwz r25, 4(r28)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BA60: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BA64: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245BA68: 409A001C  bne cr6, 0x8245ba84
	if !ctx.cr[6].eq {
	pc = 0x8245BA84; continue 'dispatch;
	}
	// 8245BA6C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BA70: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BA74: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245BA78: 409800DC  bge cr6, 0x8245bb54
	if !ctx.cr[6].lt {
	pc = 0x8245BB54; continue 'dispatch;
	}
	// 8245BA7C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245BA80: 4BFFFFC4  b 0x8245ba44
	pc = 0x8245BA44; continue 'dispatch;
	// 8245BA84: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 8245BA88: 409A0020  bne cr6, 0x8245baa8
	if !ctx.cr[6].eq {
	pc = 0x8245BAA8; continue 'dispatch;
	}
	// 8245BA8C: 80D90008  lwz r6, 8(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245BA90: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BA94: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BA98: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245BA9C: 409800B8  bge cr6, 0x8245bb54
	if !ctx.cr[6].lt {
	pc = 0x8245BB54; continue 'dispatch;
	}
	// 8245BAA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245BAA4: 4BFFFFA4  b 0x8245ba48
	pc = 0x8245BA48; continue 'dispatch;
	// 8245BAA8: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BAAC: 835E000C  lwz r26, 0xc(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BAB0: 7F1AD840  cmplw cr6, r26, r27
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245BAB4: 40990054  ble cr6, 0x8245bb08
	if !ctx.cr[6].gt {
	pc = 0x8245BB08; continue 'dispatch;
	}
	// 8245BAB8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8245BABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245BAC0: 4875B179  bl 0x82bb6c38
	ctx.lr = 0x8245BAC4;
	sub_82BB6C38(ctx, base);
	// 8245BAC4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245BAC8: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BACC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245BAD0: 40980034  bge cr6, 0x8245bb04
	if !ctx.cr[6].lt {
	pc = 0x8245BB04; continue 'dispatch;
	}
	// 8245BAD4: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245BAD8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245BADC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245BAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BAE4: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245BAE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245BAEC: 419A000C  beq cr6, 0x8245baf8
	if ctx.cr[6].eq {
	pc = 0x8245BAF8; continue 'dispatch;
	}
	// 8245BAF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8245BAF4: 4BFFFF60  b 0x8245ba54
	pc = 0x8245BA54; continue 'dispatch;
	// 8245BAF8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245BAFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8245BB00: 4BFFFF54  b 0x8245ba54
	pc = 0x8245BA54; continue 'dispatch;
	// 8245BB04: 7F1AD840  cmplw cr6, r26, r27
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8245BB08: 4098004C  bge cr6, 0x8245bb54
	if !ctx.cr[6].lt {
	pc = 0x8245BB54; continue 'dispatch;
	}
	// 8245BB0C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8245BB10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245BB14: 4808D57D  bl 0x824e9090
	ctx.lr = 0x8245BB18;
	sub_824E9090(ctx, base);
	// 8245BB18: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8245BB1C: 7F06C840  cmplw cr6, r6, r25
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[25].u32, &mut ctx.xer);
	// 8245BB20: 419A0010  beq cr6, 0x8245bb30
	if ctx.cr[6].eq {
	pc = 0x8245BB30; continue 'dispatch;
	}
	// 8245BB24: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BB28: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245BB2C: 40980028  bge cr6, 0x8245bb54
	if !ctx.cr[6].lt {
	pc = 0x8245BB54; continue 'dispatch;
	}
	// 8245BB30: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245BB34: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8245BB38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245BB3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BB40: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245BB44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245BB48: 419AFFB4  beq cr6, 0x8245bafc
	if ctx.cr[6].eq {
	pc = 0x8245BAFC; continue 'dispatch;
	}
	// 8245BB4C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8245BB50: 4BFFFFA0  b 0x8245baf0
	pc = 0x8245BAF0; continue 'dispatch;
	// 8245BB54: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8245BB58: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8245BB5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245BB60: 480C1029  bl 0x8251cb88
	ctx.lr = 0x8245BB64;
	sub_8251CB88(ctx, base);
	// 8245BB64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8245BB68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BB6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245BB70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BB74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8245BB78: 48D4C634  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BB80 size=44
    let mut pc: u32 = 0x8245BB80;
    'dispatch: loop {
        match pc {
            0x8245BB80 => {
    //   block [0x8245BB80..0x8245BBAC)
	// 8245BB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BB8C: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8245BB90: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8245BB94: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BB98: 4BFFFCB1  bl 0x8245b848
	ctx.lr = 0x8245BB9C;
	sub_8245B848(ctx, base);
	// 8245BB9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245BBA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BBA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BBB0 size=236
    let mut pc: u32 = 0x8245BBB0;
    'dispatch: loop {
        match pc {
            0x8245BBB0 => {
    //   block [0x8245BBB0..0x8245BC9C)
	// 8245BBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BBB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245BBBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BBC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245BBC8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BBCC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BBD0: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245BBD4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8245BBD8: 409A0030  bne cr6, 0x8245bc08
	if !ctx.cr[6].eq {
	pc = 0x8245BC08; continue 'dispatch;
	}
	// 8245BBDC: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BBE0: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BBE4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245BBE8: 4098000C  bge cr6, 0x8245bbf4
	if !ctx.cr[6].lt {
	pc = 0x8245BBF4; continue 'dispatch;
	}
	// 8245BBEC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245BBF0: 4800000C  b 0x8245bbfc
	pc = 0x8245BBFC; continue 'dispatch;
	// 8245BBF4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8245BBF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BBFC: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245BC00: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8245BC04: 419AFFDC  beq cr6, 0x8245bbe0
	if ctx.cr[6].eq {
	pc = 0x8245BBE0; continue 'dispatch;
	}
	// 8245BC08: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BC0C: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 8245BC10: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245BC14: 419A0014  beq cr6, 0x8245bc28
	if ctx.cr[6].eq {
	pc = 0x8245BC28; continue 'dispatch;
	}
	// 8245BC18: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BC1C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BC20: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245BC24: 4098005C  bge cr6, 0x8245bc80
	if !ctx.cr[6].lt {
	pc = 0x8245BC80; continue 'dispatch;
	}
	// 8245BC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245BC2C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BC30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245BC34: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245BC38: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8245BC3C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8245BC40: 4BEB7C29  bl 0x82313868
	ctx.lr = 0x8245BC44;
	sub_82313868(ctx, base);
	// 8245BC44: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8245BC48: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8245BC4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8245BC50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8245BC54: 4BFFFC5D  bl 0x8245b8b0
	ctx.lr = 0x8245BC58;
	sub_8245B8B0(ctx, base);
	// 8245BC58: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8245BC5C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BC60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8245BC64: 419A001C  beq cr6, 0x8245bc80
	if ctx.cr[6].eq {
	pc = 0x8245BC80; continue 'dispatch;
	}
	// 8245BC68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BC6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8245BC70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245BC74: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BC78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8245BC7C: 4E800421  bctrl
	ctx.lr = 0x8245BC80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8245BC80: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8245BC84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245BC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BC90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245BC94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BCA0 size=164
    let mut pc: u32 = 0x8245BCA0;
    'dispatch: loop {
        match pc {
            0x8245BCA0 => {
    //   block [0x8245BCA0..0x8245BD44)
	// 8245BCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BCA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BCAC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BCB0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BCB4: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245BCB8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8245BCBC: 409A0030  bne cr6, 0x8245bcec
	if !ctx.cr[6].eq {
	pc = 0x8245BCEC; continue 'dispatch;
	}
	// 8245BCC0: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BCC4: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BCC8: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245BCCC: 4098000C  bge cr6, 0x8245bcd8
	if !ctx.cr[6].lt {
	pc = 0x8245BCD8; continue 'dispatch;
	}
	// 8245BCD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8245BCD4: 4800000C  b 0x8245bce0
	pc = 0x8245BCE0; continue 'dispatch;
	// 8245BCD8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8245BCDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BCE0: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8245BCE4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8245BCE8: 419AFFDC  beq cr6, 0x8245bcc4
	if ctx.cr[6].eq {
	pc = 0x8245BCC4; continue 'dispatch;
	}
	// 8245BCEC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8245BCF0: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8245BCF4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8245BCF8: 419A0014  beq cr6, 0x8245bd0c
	if ctx.cr[6].eq {
	pc = 0x8245BD0C; continue 'dispatch;
	}
	// 8245BCFC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BD00: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8245BD04: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245BD08: 40980028  bge cr6, 0x8245bd30
	if !ctx.cr[6].lt {
	pc = 0x8245BD30; continue 'dispatch;
	}
	// 8245BD0C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245BD14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8245BD18: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8245BD1C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8245BD20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8245BD24: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8245BD28: 4BFFFCF1  bl 0x8245ba18
	ctx.lr = 0x8245BD2C;
	sub_8245BA18(ctx, base);
	// 8245BD2C: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BD30: 38650010  addi r3, r5, 0x10
	ctx.r[3].s64 = ctx.r[5].s64 + 16;
	// 8245BD34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245BD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BD48 size=64
    let mut pc: u32 = 0x8245BD48;
    'dispatch: loop {
        match pc {
            0x8245BD48 => {
    //   block [0x8245BD48..0x8245BD88)
	// 8245BD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BD50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BD54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BD58: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8245BD5C: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8245BD60: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BD64: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8245BD68: 4BFFFE49  bl 0x8245bbb0
	ctx.lr = 0x8245BD6C;
	sub_8245BBB0(ctx, base);
	// 8245BD6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245BD70: 4BF18DE1  bl 0x82374b50
	ctx.lr = 0x8245BD74;
	sub_82374B50(ctx, base);
	// 8245BD74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245BD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BD80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BD88 size=84
    let mut pc: u32 = 0x8245BD88;
    'dispatch: loop {
        match pc {
            0x8245BD88 => {
    //   block [0x8245BD88..0x8245BDDC)
	// 8245BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BD8C: 48D4C3E1  bl 0x831a816c
	ctx.lr = 0x8245BD90;
	sub_831A8130(ctx, base);
	// 8245BD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BD94: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8245BD98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245BD9C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8245BDA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8245BDA4: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8245BDA8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8245BDAC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8245BDB0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BDB4: 4BFFFDFD  bl 0x8245bbb0
	ctx.lr = 0x8245BDB8;
	sub_8245BBB0(ctx, base);
	// 8245BDB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8245BDBC: 4BF18D95  bl 0x82374b50
	ctx.lr = 0x8245BDC0;
	sub_82374B50(ctx, base);
	// 8245BDC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BDC4: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 8245BDC8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8245BDCC: 4BFFFED5  bl 0x8245bca0
	ctx.lr = 0x8245BDD0;
	sub_8245BCA0(ctx, base);
	// 8245BDD0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8245BDD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8245BDD8: 48D4C3E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BDE0 size=64
    let mut pc: u32 = 0x8245BDE0;
    'dispatch: loop {
        match pc {
            0x8245BDE0 => {
    //   block [0x8245BDE0..0x8245BE20)
	// 8245BDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BDEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BDF0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8245BDF4: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8245BDF8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BDFC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8245BE00: 4BFFFDB1  bl 0x8245bbb0
	ctx.lr = 0x8245BE04;
	sub_8245BBB0(ctx, base);
	// 8245BE04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245BE08: 4BFFE7E1  bl 0x8245a5e8
	ctx.lr = 0x8245BE0C;
	sub_8245A5E8(ctx, base);
	// 8245BE0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245BE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BE18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BE20 size=64
    let mut pc: u32 = 0x8245BE20;
    'dispatch: loop {
        match pc {
            0x8245BE20 => {
    //   block [0x8245BE20..0x8245BE60)
	// 8245BE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BE28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BE2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BE30: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8245BE34: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8245BE38: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BE3C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8245BE40: 4BFFFD71  bl 0x8245bbb0
	ctx.lr = 0x8245BE44;
	sub_8245BBB0(ctx, base);
	// 8245BE44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245BE48: 4BEDA9D9  bl 0x82336820
	ctx.lr = 0x8245BE4C;
	sub_82336820(ctx, base);
	// 8245BE4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245BE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BE58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BE60 size=64
    let mut pc: u32 = 0x8245BE60;
    'dispatch: loop {
        match pc {
            0x8245BE60 => {
    //   block [0x8245BE60..0x8245BEA0)
	// 8245BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BE68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BE6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BE70: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8245BE74: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8245BE78: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BE7C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8245BE80: 4BFFFD31  bl 0x8245bbb0
	ctx.lr = 0x8245BE84;
	sub_8245BBB0(ctx, base);
	// 8245BE84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245BE88: 4BFFE829  bl 0x8245a6b0
	ctx.lr = 0x8245BE8C;
	sub_8245A6B0(ctx, base);
	// 8245BE8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245BE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BE98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BEA0 size=64
    let mut pc: u32 = 0x8245BEA0;
    'dispatch: loop {
        match pc {
            0x8245BEA0 => {
    //   block [0x8245BEA0..0x8245BEE0)
	// 8245BEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BEA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BEAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BEB0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8245BEB4: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 8245BEB8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BEBC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8245BEC0: 4BFFFCF1  bl 0x8245bbb0
	ctx.lr = 0x8245BEC4;
	sub_8245BBB0(ctx, base);
	// 8245BEC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8245BEC8: 4BEC1571  bl 0x8231d438
	ctx.lr = 0x8245BECC;
	sub_8231D438(ctx, base);
	// 8245BECC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8245BED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BEE0 size=132
    let mut pc: u32 = 0x8245BEE0;
    'dispatch: loop {
        match pc {
            0x8245BEE0 => {
    //   block [0x8245BEE0..0x8245BF64)
	// 8245BEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BEE4: 48D4C289  bl 0x831a816c
	ctx.lr = 0x8245BEE8;
	sub_831A8130(ctx, base);
	// 8245BEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BEEC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8245BEF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245BEF4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8245BEF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8245BEFC: 388B79C8  addi r4, r11, 0x79c8
	ctx.r[4].s64 = ctx.r[11].s64 + 31176;
	// 8245BF00: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 8245BF04: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8245BF08: 489964E1  bl 0x82df23e8
	ctx.lr = 0x8245BF0C;
	sub_82DF23E8(ctx, base);
	// 8245BF0C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245BF10: 41820028  beq 0x8245bf38
	if ctx.cr[0].eq {
	pc = 0x8245BF38; continue 'dispatch;
	}
	// 8245BF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BF18: 4BEAD511  bl 0x82309428
	ctx.lr = 0x8245BF1C;
	sub_82309428(ctx, base);
	// 8245BF1C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8245BF20: 4BEAD509  bl 0x82309428
	ctx.lr = 0x8245BF24;
	sub_82309428(ctx, base);
	// 8245BF24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245BF28: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8245BF2C: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8245BF30: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8245BF34: 48000008  b 0x8245bf3c
	pc = 0x8245BF3C; continue 'dispatch;
	// 8245BF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245BF3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245BF40: 394B0018  addi r10, r11, 0x18
	ctx.r[10].s64 = ctx.r[11].s64 + 24;
	// 8245BF44: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245BF48: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8245BF4C: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8245BF50: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8245BF54: 4BE6850D  bl 0x822c4460
	ctx.lr = 0x8245BF58;
	sub_822C4460(ctx, base);
	// 8245BF58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245BF5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245BF60: 48D4C25C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BF68 size=104
    let mut pc: u32 = 0x8245BF68;
    'dispatch: loop {
        match pc {
            0x8245BF68 => {
    //   block [0x8245BF68..0x8245BFD0)
	// 8245BF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BF70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245BF74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BF78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BF7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8245BF80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8245BF84: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8245BF88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245BF8C: 419A0008  beq cr6, 0x8245bf94
	if ctx.cr[6].eq {
	pc = 0x8245BF94; continue 'dispatch;
	}
	// 8245BF90: 4BE64901  bl 0x822c0890
	ctx.lr = 0x8245BF94;
	sub_822C0890(ctx, base);
	// 8245BF94: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8245BF98: 48351251  bl 0x827ad1e8
	ctx.lr = 0x8245BF9C;
	sub_827AD1E8(ctx, base);
	// 8245BF9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BFA0: 4BFFF761  bl 0x8245b700
	ctx.lr = 0x8245BFA4;
	sub_8245B700(ctx, base);
	// 8245BFA4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245BFA8: 4182000C  beq 0x8245bfb4
	if ctx.cr[0].eq {
	pc = 0x8245BFB4; continue 'dispatch;
	}
	// 8245BFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BFB0: 48996429  bl 0x82df23d8
	ctx.lr = 0x8245BFB4;
	sub_82DF23D8(ctx, base);
	// 8245BFB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245BFB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245BFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245BFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245BFC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245BFC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245BFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245BFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8245BFD0 size=128
    let mut pc: u32 = 0x8245BFD0;
    'dispatch: loop {
        match pc {
            0x8245BFD0 => {
    //   block [0x8245BFD0..0x8245C050)
	// 8245BFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8245BFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8245BFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8245BFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8245BFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8245BFE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8245BFE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8245BFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8245BFF0: 388B79C8  addi r4, r11, 0x79c8
	ctx.r[4].s64 = ctx.r[11].s64 + 31176;
	// 8245BFF4: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8245BFF8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8245BFFC: 489963ED  bl 0x82df23e8
	ctx.lr = 0x8245C000;
	sub_82DF23E8(ctx, base);
	// 8245C000: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8245C004: 41820028  beq 0x8245c02c
	if ctx.cr[0].eq {
	pc = 0x8245C02C; continue 'dispatch;
	}
	// 8245C008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8245C00C: 4BEAD41D  bl 0x82309428
	ctx.lr = 0x8245C010;
	sub_82309428(ctx, base);
	// 8245C010: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8245C014: 4BEAD415  bl 0x82309428
	ctx.lr = 0x8245C018;
	sub_82309428(ctx, base);
	// 8245C018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8245C01C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8245C020: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8245C024: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8245C028: 48000008  b 0x8245c030
	pc = 0x8245C030; continue 'dispatch;
	// 8245C02C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245C030: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8245C034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8245C038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8245C03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8245C040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8245C044: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8245C048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8245C04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245C050 size=12
    let mut pc: u32 = 0x8245C050;
    'dispatch: loop {
        match pc {
            0x8245C050 => {
    //   block [0x8245C050..0x8245C05C)
	// 8245C050: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8245C054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8245C058: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C05C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245C05C size=8
    let mut pc: u32 = 0x8245C05C;
    'dispatch: loop {
        match pc {
            0x8245C05C => {
    //   block [0x8245C05C..0x8245C064)
	// 8245C05C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8245C060: 4BFFFF08  b 0x8245bf68
	sub_8245BF68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C064(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245C064 size=4
    let mut pc: u32 = 0x8245C064;
    'dispatch: loop {
        match pc {
            0x8245C064 => {
    //   block [0x8245C064..0x8245C068)
	// 8245C064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245C068 size=16
    let mut pc: u32 = 0x8245C068;
    'dispatch: loop {
        match pc {
            0x8245C068 => {
    //   block [0x8245C068..0x8245C078)
	// 8245C068: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245C06C: 816B3CFC  lwz r11, 0x3cfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15612 as u32) ) } as u64;
	// 8245C070: 7C6B21D6  mullw r3, r11, r4
	ctx.r[3].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8245C074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C078 size=24
    let mut pc: u32 = 0x8245C078;
    'dispatch: loop {
        match pc {
            0x8245C078 => {
    //   block [0x8245C078..0x8245C090)
	// 8245C078: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8245C07C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245C080: 409A0010  bne cr6, 0x8245c090
	if !ctx.cr[6].eq {
		sub_8245C090(ctx, base);
		return;
	}
	// 8245C084: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8245C088: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8245C08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C090 size=24
    let mut pc: u32 = 0x8245C090;
    'dispatch: loop {
        match pc {
            0x8245C090 => {
    //   block [0x8245C090..0x8245C0A8)
	// 8245C090: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245C094: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245C098: 396B3D00  addi r11, r11, 0x3d00
	ctx.r[11].s64 = ctx.r[11].s64 + 15616;
	// 8245C09C: 4182000C  beq 0x8245c0a8
	if ctx.cr[0].eq {
		sub_8245C0A8(ctx, base);
		return;
	}
	// 8245C0A0: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245C0A4: 48000008  b 0x8245c0ac
	sub_8245C0A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C0A8 size=64
    let mut pc: u32 = 0x8245C0A8;
    'dispatch: loop {
        match pc {
            0x8245C0A8 => {
    //   block [0x8245C0A8..0x8245C0E8)
	// 8245C0A8: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245C0AC: EC200072  fmuls f1, f0, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8245C0B0: 54C9063F  clrlwi. r9, r6, 0x18
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8245C0B4: 4182000C  beq 0x8245c0c0
	if ctx.cr[0].eq {
	pc = 0x8245C0C0; continue 'dispatch;
	}
	// 8245C0B8: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245C0BC: EC200072  fmuls f1, f0, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8245C0C0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8245C0C4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8245C0C8: 40980020  bge cr6, 0x8245c0e8
	if !ctx.cr[6].lt {
		sub_8245C0E8(ctx, base);
		return;
	}
	// 8245C0CC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8245C0D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8245C0D4: 7C2B1D2E  stfsx f1, r11, r3
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 8245C0D8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8245C0DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8245C0E0: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8245C0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C0E8 size=76
    let mut pc: u32 = 0x8245C0E8;
    'dispatch: loop {
        match pc {
            0x8245C0E8 => {
    //   block [0x8245C0E8..0x8245C134)
	// 8245C0E8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8245C0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8245C0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8245C0F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8245C0F8: C0099F60  lfs f0, -0x60a0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-24736 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245C0FC: 419A0030  beq cr6, 0x8245c12c
	if ctx.cr[6].eq {
	pc = 0x8245C12C; continue 'dispatch;
	}
	// 8245C100: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8245C104: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 8245C108: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245C10C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8245C110: 4098000C  bge cr6, 0x8245c11c
	if !ctx.cr[6].lt {
	pc = 0x8245C11C; continue 'dispatch;
	}
	// 8245C114: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8245C118: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8245C11C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8245C120: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8245C124: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8245C128: 4198FFE0  blt cr6, 0x8245c108
	if ctx.cr[6].lt {
	pc = 0x8245C108; continue 'dispatch;
	}
	// 8245C12C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8245C130: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C134 size=16
    let mut pc: u32 = 0x8245C134;
    'dispatch: loop {
        match pc {
            0x8245C134 => {
    //   block [0x8245C134..0x8245C144)
	// 8245C134: 39680008  addi r11, r8, 8
	ctx.r[11].s64 = ctx.r[8].s64 + 8;
	// 8245C138: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8245C13C: 7C2B1D2E  stfsx f1, r11, r3
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 8245C140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C148 size=24
    let mut pc: u32 = 0x8245C148;
    'dispatch: loop {
        match pc {
            0x8245C148 => {
    //   block [0x8245C148..0x8245C160)
	// 8245C148: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8245C14C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8245C150: 396B3D00  addi r11, r11, 0x3d00
	ctx.r[11].s64 = ctx.r[11].s64 + 15616;
	// 8245C154: 4182000C  beq 0x8245c160
	if ctx.cr[0].eq {
		sub_8245C160(ctx, base);
		return;
	}
	// 8245C158: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245C15C: 48000008  b 0x8245c164
	sub_8245C160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8245C160 size=32
    let mut pc: u32 = 0x8245C160;
    'dispatch: loop {
        match pc {
            0x8245C160 => {
    //   block [0x8245C160..0x8245C180)
	// 8245C160: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8245C164: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8245C168: 54CA063F  clrlwi. r10, r6, 0x18
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8245C16C: 4182000C  beq 0x8245c178
	if ctx.cr[0].eq {
	pc = 0x8245C178; continue 'dispatch;
	}
	// 8245C170: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8245C174: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8245C178: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8245C17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245C180 size=20
    let mut pc: u32 = 0x8245C180;
    'dispatch: loop {
        match pc {
            0x8245C180 => {
    //   block [0x8245C180..0x8245C194)
	// 8245C180: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8245C184: 54AA063F  clrlwi. r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8245C188: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8245C18C: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8245C190: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8245C194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8245C194 size=20
    let mut pc: u32 = 0x8245C194;
    'dispatch: loop {
        match pc {
            0x8245C194 => {
    //   block [0x8245C194..0x8245C1A8)
	// 8245C194: 3D408327  lis r10, -0x7cd9
	ctx.r[10].s64 = -2094596096;
	// 8245C198: 814A3D14  lwz r10, 0x3d14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(15636 as u32) ) } as u64;
	// 8245C19C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8245C1A0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8245C1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


